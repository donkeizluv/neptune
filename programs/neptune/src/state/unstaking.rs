use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Unstaking {
    pub owner: Pubkey,
    pub vault: Pubkey,
    // WAGMI partial_unstake
    pub partial_unstaking: Pubkey,
    pub lst_amt: u64,
    pub utoken_amt: u64,
}

impl Unstaking {
    pub const UNSTAKING_ESCROW_ATA_SEED: &'static [u8] = b"unstaking_escrow";
    pub const PARTIAL_UNSTAKING_MEMO: &'static str = "Neptune LST";
}
