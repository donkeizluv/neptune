use crate::{
    lock_voter::{
        self,
        accounts::{Escrow, Locker},
        cpi::{accounts::MergePartialUnstaking, merge_partial_unstaking},
    },
    state::Vault,
};
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::TokenInterface};

impl<'info> MergeUnstake<'info> {
    pub fn merge_stake(&mut self) -> Result<()> {
        todo!();
    }
}

#[rustfmt::skip]
#[derive(Accounts)]
pub struct MergeUnstake<'info>{
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

    // programs
    /// CHECK: check in attr
    #[account(address = lock_voter::ID)]
    pub locked_voter: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
