use anchor_lang::prelude::*;
use instuctions::*;

pub mod instuctions;
pub mod macros;
pub mod state;

declare_id!("DxQiCxj7hPw5oCXt4uMxXrsp1CLBmRUXzZczUwH9C5VU");

declare_program!(locked_voter);

#[program]
pub mod neptune {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>, fees_bps: u16) -> Result<()> {
        ctx.accounts.create_vault(ctx.bumps.vault, fees_bps)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        ctx.accounts.stake(amount)
    }

    pub fn begin_unstaking(ctx: Context<BeginUnstaking>, amount: u64) -> Result<()> {
        ctx.accounts.begin_unstaking(amount)
    }

    pub fn merge_unstaking(ctx: Context<MergeUnstake>) -> Result<()> {
        ctx.accounts.merge_unstaking()
    }

    pub fn withdraw_unstake(ctx: Context<WithdrawUnstake>) -> Result<()> {
        ctx.accounts.withdraw_unstake()
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
    InvalidStakeAmt,
    AmtMustGreaterThanZero,
    InvalidBPS,
    EscrowAmtIsNotCorrect,
    InvalidOwner
}
