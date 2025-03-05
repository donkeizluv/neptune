use anchor_lang::prelude::*;
use instuctions::*;

pub mod instuctions;
pub mod state;
declare_id!("DxQiCxj7hPw5oCXt4uMxXrsp1CLBmRUXzZczUwH9C5VU");

declare_program!(lock_voter);

#[program]
pub mod neptune {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        ctx.accounts.create_vault()
    }
}

#[error_code]
pub enum NeptuneError {
    #[msg("You do not have sufficient permissions to perform this action.")]
    Unauthorized,
    #[msg("Cannot get the bump.")]
    CannotGetBump,
    ArithmeticOverflow,
    InvalidRedeemAmt,
}
