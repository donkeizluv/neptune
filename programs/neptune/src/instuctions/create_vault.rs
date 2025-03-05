use crate::lock_voter::cpi::{self, accounts::NewEscrow};
use anchor_lang::prelude::*;

impl<'info> CreateVault<'info> {
    pub fn create_vault(&mut self) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.locked_voter.to_account_info(),
            NewEscrow {
                payer: self.signer.to_account_info(),
                locker: self.locker.to_account_info(),
                escrow: self.escrow.to_account_info(),
                escrow_owner: self.escrow_owner.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        );
        cpi::new_escrow(cpi_ctx)?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Accounts)]
pub struct CreateVault<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: checked in cpi
    #[account(mut)]
    pub locker: UncheckedAccount<'info>,
    /// CHECK: checked in cpi
    #[account(mut)]
    pub escrow: UncheckedAccount<'info>,
    /// CHECK: checked in cpi
    pub escrow_owner: UncheckedAccount<'info>,

    // programs
    /// CHECK: checked in cpi
    pub locked_voter: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
