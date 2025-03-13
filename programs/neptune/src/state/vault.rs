use anchor_lang::prelude::*;

use crate::{unwrap_ops, NeptuneError};

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Vault {
    pub owner: Pubkey,
    // WAGMI escrow
    pub escrow: Pubkey,
    pub lst_mint: Pubkey,
    pub total_lst_minted: u64,
    pub total_utoken_staked: u64,
    pub fees_bps: u16,
    pub bump: u8,
}

impl Vault {
    pub const VAULT_SEED: &'static [u8] = b"vault";
    pub const VAULT_LST_MINT: &'static [u8] = b"lst";
    // price = total_underlying / total_lst
    // new_lst_amt = underlying_amt / price
    // new_underlying_amt = minted_amt * price

    // new_lst_amt = underlying_amt / (total_underlying / total_lst)
    // or new_lst_amt = underlying_amt * total_lst / total_underlying
    // new_underlying_amt = lst_amt * total_underlying / total_lst

    pub fn get_lst_amt(&self, utoken_amt: u64) -> Result<u64> {
        if self.total_utoken_staked == 0 {
            return Ok(utoken_amt);
        }

        u64::try_from(
            utoken_amt as u128 * self.total_lst_minted as u128 / self.total_utoken_staked as u128,
        )
        .map_err(|_| NeptuneError::ArithmeticOverflow.into())
    }

    pub fn get_utoken_amt(&self, lst_amt: u64) -> Result<u64> {
        if self.total_lst_minted == 0 {
            return Ok(lst_amt);
        }

        u64::try_from(
            lst_amt as u128 * self.total_utoken_staked as u128 / self.total_lst_minted as u128,
        )
        .map_err(|_| NeptuneError::ArithmeticOverflow.into())
    }

    pub fn unstake(&mut self, lst_amt: u64, utoken_amt: u64) -> Result<u64> {
        self.total_utoken_staked = unwrap_ops!(
            self.total_utoken_staked.checked_sub(utoken_amt),
            NeptuneError::InvalidUnstakeAmt
        );

        self.total_lst_minted = unwrap_ops!(
            self.total_lst_minted.checked_sub(lst_amt),
            NeptuneError::InvalidUnstakeAmt
        );

        Ok(utoken_amt)
    }

    pub fn stake(&mut self, utoken_amt: u64, lst_amt: u64) -> Result<u64> {
        self.total_lst_minted = unwrap_ops!(
            self.total_lst_minted.checked_add(lst_amt),
            NeptuneError::InvalidStakeAmt
        );

        self.total_utoken_staked = unwrap_ops!(
            self.total_utoken_staked.checked_add(utoken_amt),
            NeptuneError::InvalidStakeAmt
        );

        Ok(lst_amt)
    }

    pub fn add_reward(&mut self, utoken_amt: u64) -> Result<()> {
        self.total_utoken_staked = unwrap_ops!(self.total_utoken_staked.checked_add(utoken_amt));

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use super::Vault;
    use anchor_lang::prelude::Pubkey;

    fn new_vault() -> Vault {
        Vault {
            escrow: Pubkey::new_unique(),
            lst_mint: Pubkey::new_unique(),
            bump: 255,
            fees_bps: 100,
            owner: Pubkey::new_unique(),
            total_lst_minted: 0,
            total_utoken_staked: 0,
        }
    }

    #[test]
    fn test_lst_amt() {
        let mut vault = new_vault();

        assert_eq!(
            vault.stake(1, vault.get_lst_amt(1).unwrap()).unwrap(),
            1,
            "should receive 1 lst"
        );
        assert_eq!(
            vault.total_lst_minted, vault.total_utoken_staked,
            "total minted & total staked should be eq"
        );

        vault.add_reward(2).unwrap();

        assert_eq!(
            vault.unstake(1, vault.get_utoken_amt(1).unwrap()).unwrap(),
            3,
            "should receive 3 utoken"
        );

        assert_eq!(
            vault.total_lst_minted, 0,
            "total lst minted should be 0 after unstaked all"
        );
        assert_eq!(
            vault.total_utoken_staked, 0,
            "total utoken staked should be 0 after unstaked all"
        );

        vault
            .stake(100_000_000, vault.get_lst_amt(100_000_000).unwrap())
            .unwrap();
        vault.add_reward(10_000_000).unwrap();

        assert_eq!(
            vault
                .unstake(1_000_000, vault.get_utoken_amt(1_000_000).unwrap())
                .unwrap(),
            1_100_000_u64,
            "should unstake 1_000_000 with extra 10%"
        );

        assert_eq!(
            vault
                .unstake(4_000_000_u64, vault.get_utoken_amt(4_000_000).unwrap())
                .unwrap(),
            4_400_000_u64,
            "should unstake 4_000_000 with extra 10%"
        );

        vault.add_reward(10_000_000).unwrap();
        vault
            .unstake(33_333_333_u64, vault.get_utoken_amt(33_333_333).unwrap())
            .unwrap();

        let total_reward_dis = (1_000_000_u64 * 10_u64 / 100_u64)
            + (4_000_000_u64 * 10_u64 / 100_u64)
            + (vault.get_utoken_amt(33_333_333_u64).unwrap() - 33_333_333_u64);
        assert_eq!(
            100_000_000_u64 + 10_000_000_u64 + 10_000_000_u64
                - 1_000_000_u64
                - 4_000_000_u64
                - 33_333_333_u64,
            total_reward_dis as u64 + vault.total_utoken_staked,
            "final utoken amt should match up"
        );

        vault
            .stake(2_222_222_u64, vault.get_lst_amt(2_222_222_u64).unwrap())
            .unwrap();
        let final_lst_minted = 100_000_000_u64 - 1_000_000_u64 - 4_000_000_u64 - 33_333_333_u64
            + vault.get_lst_amt(2_222_222_u64).unwrap();

        assert_eq!(
            final_lst_minted, vault.total_lst_minted,
            "final lst minted should match"
        );

        vault
            .unstake(
                vault.total_lst_minted,
                vault.get_utoken_amt(vault.total_lst_minted).unwrap(),
            )
            .unwrap();
        assert_eq!(vault.total_lst_minted, 0, "vault should empty");
        assert_eq!(vault.total_utoken_staked, 0, "vault should empty");
    }
}
