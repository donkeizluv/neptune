use anchor_lang::prelude::*;
use instuctions::*;

pub mod instuctions;
pub mod macros;
pub mod state;

declare_id!("DxQiCxj7hPw5oCXt4uMxXrsp1CLBmRUXzZczUwH9C5VU");

declare_program!(lock_voter);

#[program]
pub mod neptune {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>, fees_bps: u16) -> Result<()> {
        ctx.accounts.create_vault(ctx.bumps.vault, fees_bps)
    }
}

#[error_code]
pub enum NeptuneError {
    #[msg("You do not have sufficient permissions to perform this action.")]
    Unauthorized,
    #[msg("Cannot get the bump.")]
    CannotGetBump,
    ArithmeticOverflow,
    InvalidUnstakeAmt,
    AmtMustGreaterThanZero,
    InvalidBPS,
    EscrowAmtIsNotCorrect
}
