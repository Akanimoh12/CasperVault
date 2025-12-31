use odra::prelude::*;
use odra::{casper_types::U512, Mapping, Var};

#[odra::module]
pub struct MockPriceOracle {
    prices: Mapping<String, U512>,
    last_update: Mapping<String, u64>,
    price_feeds: Var<u8>,
}

#[odra::module]
impl MockPriceOracle {
    pub fn init(&mut self) {
        self.price_feeds.set(0);
        
        self.set_price("CSPR".to_string(), U512::from(50_000_000u64));
        self.set_price("lstCSPR".to_string(), U512::from(50_000_000u64));
        self.set_price("ETH".to_string(), U512::from(3500_000_000_000u64));
        self.set_price("USDC".to_string(), U512::from(1_000_000u64));
    }

    pub fn set_price(&mut self, token: String, price: U512) {
        self.prices.set(&token, price);
        self.last_update.set(&token, self.env().get_block_time());
    }

    pub fn get_price(&self, token: String) -> U512 {
        self.prices.get(&token).unwrap_or(U512::zero())
    }

    pub fn get_price_with_decimals(&self, token: String, decimals: u8) -> U512 {
        let base_price = self.get_price(token);
        let multiplier = U512::from(10u64.pow(decimals as u32));
        base_price * multiplier / U512::from(1_000_000u64)
    }

    pub fn convert(&self, from_token: String, to_token: String, amount: U512) -> U512 {
        let from_price = self.get_price(from_token);
        let to_price = self.get_price(to_token);
        
        if to_price == U512::zero() {
            return U512::zero();
        }
        
        (amount * from_price) / to_price
    }

    pub fn get_twap(&self, token: String, _period: u64) -> U512 {
        self.get_price(token)
    }

    pub fn is_price_fresh(&self, token: String, max_age: u64) -> bool {
        let last_update = self.last_update.get(&token).unwrap_or(0);
        let current_time = self.env().get_block_time();
        
        current_time - last_update <= max_age
    }

    pub fn get_last_update(&self, token: String) -> u64 {
        self.last_update.get(&token).unwrap_or(0)
    }

    pub fn simulate_price_movement(&mut self, token: String, change_bps: i32) {
        let current_price = self.get_price(token.clone());
        
        let change = if change_bps >= 0 {
            (current_price * U512::from(change_bps as u64)) / U512::from(10000u64)
        } else {
            (current_price * U512::from((-change_bps) as u64)) / U512::from(10000u64)
        };
        
        let new_price = if change_bps >= 0 {
            current_price + change
        } else {
            if current_price > change {
                current_price - change
            } else {
                U512::from(1u64)
            }
        };
        
        self.set_price(token, new_price);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::{Deployer, HostRef};

    #[test]
    fn test_price_setting() {
        let env = odra_test::env();
        let mut contract = MockPriceOracleHostRef::deploy(&env);
        
        let price = U512::from(100_000_000u64);
        contract.set_price("TEST".to_string(), price);
        
        assert_eq!(contract.get_price("TEST".to_string()), price);
    }

    #[test]
    fn test_conversion() {
        let env = odra_test::env();
        let contract = MockPriceOracleHostRef::deploy(&env);
        
        let cspr_amount = U512::from(1000u64);
        let usdc_amount = contract.convert("CSPR".to_string(), "USDC".to_string(), cspr_amount);
        
        assert!(usdc_amount > U512::zero());
    }

    #[test]
    fn test_price_movement() {
        let env = odra_test::env();
        let mut contract = MockPriceOracleHostRef::deploy(&env);
        
        let initial_price = contract.get_price("CSPR".to_string());
        contract.simulate_price_movement("CSPR".to_string(), 1000);
        let new_price = contract.get_price("CSPR".to_string());
        
        assert!(new_price > initial_price);
    }
}
