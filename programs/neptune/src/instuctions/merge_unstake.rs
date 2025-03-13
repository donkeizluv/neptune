use crate::{
    lock_voter::{
        self,
        accounts::Escrow,
        cpi::{self as locked_voter, accounts::MergePartialUnstaking},
    },
    state::{Unstaking, Vault},
    vault_seeds,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

impl<'info> MergeUnstake<'info> {
    pub fn merge_unstaking(&mut self) -> Result<()> {
        let vault_key = self.vault.key();
        let vault_seeds: &[&[&[u8]]] = vault_seeds!(self.vault, vault_key);

        // return lst to user
        let xfer_lst_to_user_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            TransferChecked {
                authority: self.vault.to_account_info(),
                from: self.lst_escrow_ata.to_account_info(),
                to: self.lst_ata.to_account_info(),
                mint: self.lst_mint.to_account_info(),
            },
            vault_seeds,
        );
        token::transfer_checked(
            xfer_lst_to_user_cpi,
            self.unstaking.lst_amt,
            self.lst_mint.decimals,
        )?;

        // merge parital_unstaking
        let merge_partial_unstaking_cpi = CpiContext::new_with_signer(
            self.locked_voter.to_account_info(),
            MergePartialUnstaking {
                locker: self.locker.to_account_info(),
                escrow: self.escrow.to_account_info(),
                partial_unstake: self.partial_unstaking.to_account_info(),
                owner: self.vault.to_account_info(),
            },
            vault_seeds,
        );
        locked_voter::merge_partial_unstaking(merge_partial_unstaking_cpi)?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Accounts)]
pub struct MergeUnstake<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: check in cpi
    #[account(mut)]
    pub locker: UncheckedAccount<'info>,

    #[account(
        mut,
        has_one = locker,
        constraint = escrow.owner == vault.key()
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    #[account(
        has_one = escrow
    )]
    pub vault: Box<Account<'info, Vault>>,

    /// CHECK: check in cpi
    #[account(mut)]
    pub partial_unstaking: UncheckedAccount<'info>,

    #[account(
        address = vault.lst_mint,
    )]
    pub lst_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = lst_mint,
        associated_token::authority = signer,
    )]
    pub lst_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        has_one = partial_unstaking,
        has_one = vault,
        constraint = unstaking.owner == signer.key(),
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

    // programs
    /// CHECK: check in attr
    #[account(address = lock_voter::ID)]
    pub locked_voter: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
