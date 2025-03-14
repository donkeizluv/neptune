use crate::{
    locked_voter::{
        self,
        accounts::Locker,
        cpi::accounts::{NewEscrow, ToggleMaxLock},
    },
    state::Vault,
    vault_seeds, NeptuneError,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::extension::transfer_fee::MAX_FEE_BASIS_POINTS,
    token_interface::{Mint, TokenInterface},
};

impl<'info> CreateVault<'info> {
    pub fn create_vault(&mut self, vault_bump: u8, fees_bps: u16) -> Result<()> {
        require!(fees_bps < MAX_FEE_BASIS_POINTS, NeptuneError::InvalidBPS);
        let vault = &mut self.vault;
        vault.escrow = self.escrow.key();
        vault.lst_mint = self.lst_mint.key();
        vault.owner = self.vault_owner.key();
        vault.fees_bps = fees_bps;
        vault.bump = vault_bump;

        let new_escrow_cpi = CpiContext::new(
            self.locked_voter_program.to_account_info(),
            NewEscrow {
                payer: self.signer.to_account_info(),
                locker: self.locker.to_account_info(),
                escrow: self.escrow.to_account_info(),
                escrow_owner: self.vault.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        );
        locked_voter::cpi::new_escrow(new_escrow_cpi)?;

        // toggle max lock
        let locker_key = self.locker.key();
        let vault_owner = self.vault.owner.key();
        let vault_seeds: &[&[&[u8]]] = vault_seeds!(self.vault, locker_key, vault_owner);

        let toggle_max_lock_cpi = CpiContext::new_with_signer(
            self.locked_voter_program.to_account_info(),
            ToggleMaxLock {
                locker: self.locker.to_account_info(),
                escrow: self.escrow.to_account_info(),
                escrow_owner: self.vault.to_account_info(),
            },
            vault_seeds,
        );
        locked_voter::cpi::toggle_max_lock(toggle_max_lock_cpi, true)?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Accounts)]
pub struct CreateVault<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = Vault::DISCRIMINATOR.len() + Vault::INIT_SPACE,
        seeds = [
            &Vault::VAULT_SEED,
            locker.key().as_ref(),
            vault_owner.key().as_ref(),
        ],
        bump,
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        init,
        payer = signer,
        mint::freeze_authority = vault,
        mint::decimals = utoken_mint.decimals,
        mint::authority = vault,
        seeds = [Vault::VAULT_LST_MINT.as_ref(),
                    vault.key().as_ref()],
        bump
    )]
    pub lst_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        address = locker.token_mint
    )]
    pub utoken_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub locker: Box<Account<'info, Locker>>,

    // escrow seeds
    // b"Escrow".as_ref(),
    // locker.key().as_ref(),
    // escrow_owner.key().as_ref()
    /// CHECK: checked in cpi
    #[account(mut)]
    pub escrow: UncheckedAccount<'info>,

    /// CHECK: new vault owner
    pub vault_owner: UncheckedAccount<'info>,

    // programs
    /// CHECK: check in attr
    #[account(address = locked_voter::ID)]
    pub locked_voter_program: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
