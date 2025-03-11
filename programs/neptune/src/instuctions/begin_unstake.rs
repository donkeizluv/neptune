use crate::{
    lock_voter::{
        self,
        accounts::Escrow,
        cpi::{self, accounts::OpenPartialUnstaking},
    },
    partial_unstaking_seeds,
    state::{Unstaking, Vault},
    NeptuneError,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

impl<'info> BeginUnstaking<'info> {
    pub fn begin_unstaking(&mut self, lst_amt: u64, unstaking_bump: u8) -> Result<()> {
        require!(lst_amt > 0, NeptuneError::AmtMustGreaterThanZero);

        let utoken_amt = self.vault.get_utoken_amt(lst_amt)?;
        // update unstaking state
        self.unstaking.utoken_amt = utoken_amt;
        self.unstaking.lst_amt = lst_amt;
        self.unstaking.owner = self.signer.key();
        self.unstaking.partial_unstaking = self.partial_unstaking.key();
        self.unstaking.vault = self.vault.key();
        self.unstaking.bump = unstaking_bump;

        // xfer lst to our escrow
        let xfer_lst_to_escrow_cpi = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.unstake_ata.to_account_info(),
                mint: self.lst_mint.to_account_info(),
                to: self.unstaking_escrow_ata.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        );
        transfer_checked(xfer_lst_to_escrow_cpi, lst_amt, self.lst_mint.decimals)?;

        // partial_unstake cpi
        let unstaking_key = self.unstaking.key();
        // MAGIC: does this work tho? since unstaking is initally a rnd kp
        let partial_unstaking_seeds: &[&[&[u8]]] =
            partial_unstaking_seeds!(self.unstaking, unstaking_key);

        let open_partial_unstaking_cpi = CpiContext::new_with_signer(
            self.locked_voter.to_account_info(),
            OpenPartialUnstaking {
                locker: self.locker.to_account_info(),
                escrow: self.escrow.to_account_info(),
                owner: self.unstaking.to_account_info(),
                partial_unstake: self.partial_unstaking.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
            partial_unstaking_seeds,
        );
        cpi::open_partial_unstaking(
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

    #[account(has_one = escrow)]
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

    #[account(address = vault.lst_mint)]
    pub lst_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = signer,
        space = Unstaking::DISCRIMINATOR.len() + Unstaking::INIT_SPACE
    )]
    pub unstaking: Box<Account<'info, Unstaking>>,

    /// CHECK: checked in cpi
    #[account(
        mut,
        seeds = [
            &Unstaking::PARTIAL_UNSTAKING_SEED,
            unstaking.key().as_ref()
        ],
        bump
    )]
    pub partial_unstaking: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = lst_mint,
        associated_token::authority = signer,
    )]
    pub unstake_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = lst_mint,
        associated_token::authority = unstaking
    )]
    pub unstaking_escrow_ata: Box<InterfaceAccount<'info, TokenAccount>>,



    // programs
    /// CHECK: check in attr
    #[account(address = lock_voter::ID)]
    pub locked_voter: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
