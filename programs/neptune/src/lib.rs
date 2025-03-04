use anchor_lang::prelude::*;
use lock_voter::cpi::{self, accounts::NewEscrow};
declare_id!("DxQiCxj7hPw5oCXt4uMxXrsp1CLBmRUXzZczUwH9C5VU");

declare_program!(lock_voter);

#[program]
pub mod neptune {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        let cpi_ctx = CpiContext::new(
            ctx.accounts.locked_voter.to_account_info(),
            NewEscrow {
                payer: ctx.accounts.signer.to_account_info(),
                locker: ctx.accounts.locker.to_account_info(),
                escrow: ctx.accounts.escrow.to_account_info(),
                escrow_owner: ctx.accounts.escrow_owner.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );

        // Invoke the initialize instruction
        cpi::new_escrow(cpi_ctx)?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Accounts)]
pub struct Initialize<'info>{
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
