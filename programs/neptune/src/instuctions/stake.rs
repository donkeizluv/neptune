use crate::{
    lock_voter::{
        self,
        accounts::{Escrow, Locker},
        cpi::{accounts::IncreaseLockedAmount, increase_locked_amount},
    },
    state::Vault,
    vault_seeds, NeptuneError,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, MintTo},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

impl<'info> Stake<'info> {
    pub fn stake(&mut self, utoken_amt: u64) -> Result<()> {
        require!(utoken_amt > 0, NeptuneError::AmtMustGreaterThanZero);

        // increase stake to locked_voter
        let incease_lock_amt_cpi = CpiContext::new(
            self.locked_voter.to_account_info(),
            IncreaseLockedAmount {
                payer: self.signer.to_account_info(),
                locker: self.locker.to_account_info(),
                escrow: self.escrow.to_account_info(),
                source_tokens: self.utoken_source_ata.to_account_info(),
                escrow_tokens: self.utoken_escrow_ata.to_account_info(),
                token_program: self.token_program.to_account_info(),
            },
        );
        increase_locked_amount(incease_lock_amt_cpi, utoken_amt)?;

        // mint lst to user
        let lst_amt = self.vault.get_lst_amt(utoken_amt)?;
        let wagmi_escrow_key = self.escrow.key();
        let vault_seeds: &[&[&[u8]]] = vault_seeds!(self.vault, wagmi_escrow_key);
        let mint_lst_to_user_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.lst_mint.to_account_info(),
                to: self.lst_ata.to_account_info(),
                authority: self.vault.to_account_info(),
            },
            vault_seeds,
        );
        mint_to(mint_lst_to_user_cpi, lst_amt)?;

        // update vault state
        self.vault.stake(utoken_amt, lst_amt)?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Accounts)]
pub struct Stake<'info>{
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

    #[account(
        mut,
        address = escrow.tokens
    )]
    pub utoken_escrow_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        address = vault.lst_mint
    )]
    pub lst_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = locker.token_mint,
        associated_token::authority = signer,
    )]
    pub utoken_source_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = lst_mint,
        associated_token::authority = signer,
    )]
    pub lst_ata: Box<InterfaceAccount<'info, TokenAccount>>,


    // programs
    /// CHECK: check in attr
    #[account(address = lock_voter::ID)]
    pub locked_voter: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
