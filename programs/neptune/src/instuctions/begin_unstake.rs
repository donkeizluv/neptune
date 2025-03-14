use crate::{
    locked_voter::{self, accounts::Escrow, cpi::accounts::OpenPartialUnstaking},
    state::{Unstaking, Vault},
    NeptuneError,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

impl<'info> BeginUnstaking<'info> {
    pub fn begin_unstaking(&mut self, lst_amt: u64) -> Result<()> {
        require!(lst_amt > 0, NeptuneError::AmtMustGreaterThanZero);

        let utoken_amt = self.vault.get_utoken_amt(lst_amt)?;
        // update unstaking state
        let unstaking = &mut self.unstaking;
        unstaking.utoken_amt = utoken_amt;
        unstaking.lst_amt = lst_amt;
        unstaking.owner = self.signer.key();
        unstaking.partial_unstaking = self.partial_unstaking.key();
        unstaking.vault = self.vault.key();

        // xfer lst to our escrow
        let xfer_lst_to_escrow_cpi = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.lst_source_ata.to_account_info(),
                to: self.lst_escrow_ata.to_account_info(),
                mint: self.lst_mint.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        );
        token::transfer_checked(xfer_lst_to_escrow_cpi, lst_amt, self.lst_mint.decimals)?;

        // open partial unstaking
        let open_partial_unstaking_cpi = CpiContext::new(
            self.locked_voter_program.to_account_info(),
            OpenPartialUnstaking {
                locker: self.locker.to_account_info(),
                escrow: self.escrow.to_account_info(),
                owner: self.vault.to_account_info(),
                partial_unstake: self.partial_unstaking.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        );
        locked_voter::cpi::open_partial_unstaking(
            open_partial_unstaking_cpi,
            utoken_amt,
            Unstaking::PARTIAL_UNSTAKING_MEMO.to_string(),
        )?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Accounts)]
pub struct BeginUnstaking<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        has_one = escrow
    )]
    pub vault: Box<Account<'info, Vault>>,

    /// CHECK: checked in cpi
    #[account(mut)]
    pub locker: UncheckedAccount<'info>,

    #[account(
        mut,
        has_one = locker,
        constraint = escrow.owner == vault.key()
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    #[account(
        address = vault.lst_mint
    )]
    pub lst_mint: Box<InterfaceAccount<'info, Mint>>,
    
    // unstaking must sign
    #[account(
        init,
        payer = signer,
        space = Unstaking::DISCRIMINATOR.len() + Unstaking::INIT_SPACE
    )]
    pub unstaking: Box<Account<'info, Unstaking>>,

    // partial_unstaking must sign
    /// CHECK: checked in cpi
    #[account(mut)]
    pub partial_unstaking: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = lst_mint,
        associated_token::authority = signer,
    )]
    pub lst_source_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = signer,
        seeds = [
            &Unstaking::UNSTAKING_ESCROW_ATA_SEED,
            unstaking.key().as_ref()
        ],
        bump,
        token::mint = lst_mint,
        token::authority = vault
    )]
    pub lst_escrow_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    // programs
    /// CHECK: check in attr
    #[account(address = locked_voter::ID)]
    pub locked_voter_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
