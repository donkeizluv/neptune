use crate::{
    locked_voter::{
        self,
        accounts::{Escrow, Locker},
        cpi::accounts::WithdrawPartialUnstaking,
    },
    state::{Unstaking, Vault},
    unwrap_ops, vault_seeds, NeptuneError,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, CloseAccount, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

impl<'info> WithdrawUnstake<'info> {
    pub fn withdraw_unstake(&mut self) -> Result<()> {
        let locker_key = self.locker.key();
        let vault_owner = self.vault.owner.key();
        let vault_seeds: &[&[&[u8]]] = vault_seeds!(self.vault, locker_key, vault_owner);

        // withdraw partial unstaking
        let withdraw_partial_unstaking_cpi = CpiContext::new_with_signer(
            self.locked_voter_program.to_account_info(),
            WithdrawPartialUnstaking {
                payer: self.signer.to_account_info(),
                locker: self.locker.to_account_info(),
                escrow: self.escrow.to_account_info(),
                owner: self.vault.to_account_info(),
                partial_unstake: self.partial_unstaking.to_account_info(),
                escrow_tokens: self.utoken_escrow_ata.to_account_info(),
                destination_tokens: self.utoken_target_ata.to_account_info(),
                token_program: self.token_program.to_account_info(),
            },
            vault_seeds,
        );
        locked_voter::cpi::withdraw_partial_unstaking(withdraw_partial_unstaking_cpi)?;

        // update vault state
        self.vault
            .unstake(self.unstaking.lst_amt, self.unstaking.utoken_amt)?;

        // handle ATA amt > escrowed lst amt
        let exceeding_amt = unwrap_ops!(
            self.lst_escrow_ata
                .amount
                .checked_sub(self.unstaking.lst_amt),
            NeptuneError::EscrowAmtIsNotCorrect
        );

        if exceeding_amt > 0 {
            // xfer exceeding back to user
            let xfer_exceeding_cpi = CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.lst_escrow_ata.to_account_info(),
                    to: self.lst_ata.to_account_info(),
                    mint: self.lst_mint.to_account_info(),
                    authority: self.vault.to_account_info(),
                },
            );
            token::transfer_checked(xfer_exceeding_cpi, exceeding_amt, self.lst_mint.decimals)?;
        }
        // burn lst
        let burn_lst_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Burn {
                mint: self.lst_mint.to_account_info(),
                from: self.lst_escrow_ata.to_account_info(),
                authority: self.vault.to_account_info(),
            },
            vault_seeds,
        );
        token::burn(burn_lst_cpi, self.unstaking.lst_amt)?;

        // close lst_escrow_ata
        let close_lst_escrow_ata_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.lst_escrow_ata.to_account_info(),
                destination: self.signer.to_account_info(),
                authority: self.vault.to_account_info(),
            },
            vault_seeds,
        );
        token::close_account(close_lst_escrow_ata_cpi)?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Accounts)]
pub struct WithdrawUnstake<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub locker: Box<Account<'info, Locker>>,

    #[account(
        mut,
        has_one = locker,
        constraint = escrow.owner == vault.key()
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    #[account(
        mut,
        has_one = escrow
    )]
    pub vault: Box<Account<'info, Vault>>,

    /// CHECK: check in cpi
    #[account(mut)]
    pub partial_unstaking: UncheckedAccount<'info>,

    #[account(
        mut,
        address = vault.lst_mint,
    )]
    pub lst_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        has_one = partial_unstaking,
        has_one = vault,
        constraint = unstaking.owner == signer.key() @ NeptuneError::InvalidOwner,
        close = signer
    )]
    pub unstaking: Box<Account<'info, Unstaking>>,

    #[account(
        mut,
        seeds = [
            &Unstaking::UNSTAKING_ESCROW_ATA_SEED,
            unstaking.key().as_ref()
        ],
        bump,
        token::mint = lst_mint,
        token::authority = vault
    )]
    pub lst_escrow_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = lst_mint,
        associated_token::authority = signer,
    )]
    pub lst_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        address = locker.token_mint,
    )]
    pub utoken_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = utoken_mint,
        associated_token::authority = signer,

    )]
    pub utoken_target_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut, 
        address = escrow.tokens
    )]
    pub utoken_escrow_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    // programs
    /// CHECK: check in attr
    #[account(address = locked_voter::ID)]
    pub locked_voter_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
