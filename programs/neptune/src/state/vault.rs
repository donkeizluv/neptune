use anchor_lang::prelude::*;

use crate::NeptuneError;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Vault {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub total_lst_minted: u64,
    pub total_utoken_staked: u64,
    pub fees_bps: u16,
    pub bump: u8,
}

impl Vault {
    pub const NEPTUNE_VAULT_SEED: [u8; 5] = *b"vault";
    // price = staked / minted
    // minted = staked / price
    // staked = minted * price

    pub fn get_lst_price(&self) -> f64 {
        if self.total_lst_minted == 0 {
            return 1_f64;
        }

        self.total_utoken_staked as f64 / self.total_lst_minted as f64
    }

    fn get_lst_amt(&self, utoken_amt: u64) -> u64 {
        (utoken_amt as f64 / self.get_lst_price()) as u64
    }

    fn get_utoken_amt(&self, lst_amt: u64) -> u64 {
        (lst_amt as f64 * self.get_lst_price()) as u64
    }

    pub fn redeem(&mut self, lst_amt: u64) -> Result<u64> {
        let utoken_amt = self.get_utoken_amt(lst_amt);

        self.total_utoken_staked = self
            .total_utoken_staked
            .checked_sub(utoken_amt)
            .ok_or(NeptuneError::InvalidRedeemAmt)?;

        self.total_lst_minted = self
            .total_lst_minted
            .checked_sub(lst_amt)
            .ok_or(NeptuneError::InvalidRedeemAmt)?;

        Ok(utoken_amt)
    }

    pub fn stake(&mut self, utoken_amt: u64) -> Result<u64> {
        let lst_amt = self.get_lst_amt(utoken_amt);

        self.total_lst_minted = self
            .total_lst_minted
            .checked_add(lst_amt)
            .ok_or(NeptuneError::ArithmeticOverflow)?;

        self.total_utoken_staked = self
            .total_utoken_staked
            .checked_add(utoken_amt)
            .ok_or(NeptuneError::ArithmeticOverflow)?;

        Ok(lst_amt)
    }

    pub fn add_reward(&mut self, utoken_amt: u64) -> Result<()> {
        self.total_utoken_staked = self
            .total_utoken_staked
            .checked_add(utoken_amt)
            .ok_or(NeptuneError::ArithmeticOverflow)?;

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
            bump: 255,
            fees_bps: 100,
            mint: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            total_lst_minted: 0,
            total_utoken_staked: 0,
        }
    }

    #[test]
    fn test_lst_amt() {
        let mut cf = new_vault();

        assert_eq!(cf.get_lst_price(), 1_f64, "price should be 1 initally");
        assert_eq!(cf.stake(1).unwrap(), 1, "should receive 1 lst");
        assert_eq!(
            cf.total_lst_minted, cf.total_utoken_staked,
            "total minted & total staked should be eq"
        );
        assert_eq!(
            cf.get_lst_price(),
            1_f64,
            "price should be 1 when theres no extras reward"
        );

        cf.add_reward(2).unwrap();
        assert_eq!(
            cf.get_lst_price(),
            3_f64,
            "price should reflect with added rewards"
        );
        assert_eq!(cf.redeem(1).unwrap(), 3, "should receive 3 utoken");
        assert_eq!(
            cf.get_lst_price(),
            1_f64,
            "price should be 1 after redeemed all"
        );
        assert_eq!(
            cf.total_lst_minted, 0,
            "total lst minted should be 0 after redeemed all"
        );
        assert_eq!(
            cf.total_utoken_staked, 0,
            "total utoken staked should be 0 after redeemed all"
        );

        cf.stake(100_000_000).unwrap();
        cf.add_reward(10_000_000).unwrap();

        assert_eq!(
            cf.get_lst_price(),
            1.10_f64,
            "price should be correct with 10% reward"
        );

        assert_eq!(
            cf.redeem(1_000_000).unwrap(),
            (1_000_000 as f64 * 1.10_f64) as u64,
            "should redeem 1_000_000 with extra 10%"
        );

        assert_eq!(
            cf.redeem(4_000_000).unwrap(),
            (4_000_000 as f64 * 1.10_f64) as u64,
            "should redeem 4_000_000 with extra 10%"
        );

        cf.add_reward(10_000_000).unwrap();
        cf.redeem(33_333_333_u64).unwrap();

        let total_reward_dis = (1_000_000_f64 * 10_f64 / 100_f64)
            + (4_000_000_f64 * 10_f64 / 100_f64)
            + ((cf.get_lst_price() * 33_333_333_f64) - 33_333_333_f64);
        // println!("final price: {}", cf.get_lst_price());
        assert_eq!(
            100_000_000_u64 + 10_000_000_u64 + 10_000_000_u64
                - 1_000_000_u64
                - 4_000_000_u64
                - 33_333_333_u64,
            total_reward_dis as u64 + cf.total_utoken_staked,
            "final utoken amt should match up"
        );
        let price_before_stake = cf.get_lst_price();
        cf.stake(2_222_222_u64).unwrap();
        // floating point error allowed down to 6 decimals only
        assert!(
            price_before_stake - cf.get_lst_price() < 0.000001_f64,
            "price should not change when staking more"
        );

        let final_lst_minted = 100_000_000_u64 - 1_000_000_u64 - 4_000_000_u64 - 33_333_333_u64
            + (2_222_222_f64 / cf.get_lst_price()) as u64;
        // println!("total lst minted: {}", final_lst_minted);

        assert_eq!(
            final_lst_minted, cf.total_lst_minted,
            "final lst minted should match"
        )
    }
}
