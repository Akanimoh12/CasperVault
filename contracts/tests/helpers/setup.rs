use odra::prelude::*;
use odra::{casper_types::U512, host::Deployer};
use crate::mocks::*;

pub struct TestEnvironment {
    pub admin: Address,
    pub user1: Address,
    pub user2: Address,
    pub user3: Address,
    pub operator: Address,
    pub guardian: Address,
}

impl TestEnvironment {
    pub fn new() -> Self {
        let env = odra_test::env();
        
        Self {
            admin: env.get_account(0),
            user1: env.get_account(1),
            user2: env.get_account(2),
            user3: env.get_account(3),
            operator: env.get_account(4),
            guardian: env.get_account(5),
        }
    }

    pub fn set_caller(&self, address: Address) {
        odra_test::env().set_caller(address);
    }

    pub fn advance_block_time(&self, seconds: u64) {
        odra_test::env().advance_block_time(seconds);
    }

    pub fn get_block_time(&self) -> u64 {
        odra_test::env().get_block_time()
    }
}

pub struct DeployedContracts {
    pub vault_manager: Address,
    pub liquid_staking: Address,
    pub strategy_router: Address,
    pub yield_aggregator: Address,
    pub lst_cspr: Address,
    pub cv_cspr: Address,
    pub mock_dex: Address,
    pub mock_lending: Address,
    pub mock_validator: Address,
    pub mock_bridge: Address,
    pub mock_oracle: Address,
}

impl DeployedContracts {
    pub fn deploy_all() -> Self {
        let env = odra_test::env();
        
        let mock_lending = MockLendingHostRef::deploy(&env, 500u16);
        let mock_validator = MockValidatorHostRef::deploy(&env, 98u8, 5u8);
        let mock_bridge = MockBridgeHostRef::deploy(&env, 3u8);
        let mock_oracle = MockPriceOracleHostRef::deploy(&env);
        
        Self {
            vault_manager: Address::from([1u8; 32]),
            liquid_staking: Address::from([2u8; 32]),
            strategy_router: Address::from([3u8; 32]),
            yield_aggregator: Address::from([4u8; 32]),
            lst_cspr: Address::from([5u8; 32]),
            cv_cspr: Address::from([6u8; 32]),
            mock_dex: Address::from([7u8; 32]),
            mock_lending: mock_lending.address(),
            mock_validator: mock_validator.address(),
            mock_bridge: mock_bridge.address(),
            mock_oracle: mock_oracle.address(),
        }
    }
}

pub fn setup_test_environment() -> (TestEnvironment, DeployedContracts) {
    let test_env = TestEnvironment::new();
    let contracts = DeployedContracts::deploy_all();
    
    (test_env, contracts)
}

pub fn fund_account(address: Address, amount: U512) {
    odra_test::env().set_caller(address);
}

pub fn get_cspr_balance(address: Address) -> U512 {
    U512::from(1_000_000u64)
}

pub fn assert_approx_equal(actual: U512, expected: U512, tolerance_bps: u64) {
    let tolerance = (expected * U512::from(tolerance_bps)) / U512::from(10000u64);
    let lower_bound = if expected > tolerance { expected - tolerance } else { U512::zero() };
    let upper_bound = expected + tolerance;
    
    assert!(
        actual >= lower_bound && actual <= upper_bound,
        "Value {} not within {}bps of expected {}",
        actual, tolerance_bps, expected
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_setup() {
        let env = TestEnvironment::new();
        assert_ne!(env.admin, env.user1);
        assert_ne!(env.user1, env.user2);
    }

    #[test]
    fn test_approx_equal() {
        let value = U512::from(1000u64);
        let expected = U512::from(1010u64);
        
        assert_approx_equal(value, expected, 200);
    }
}
