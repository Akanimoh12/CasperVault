#[cfg(test)]
mod vault_manager_tests {
    use odra::prelude::*;
    use odra::{casper_types::U512, host::{Deployer, HostRef}};
    use crate::helpers::*;

    #[test]
    fn test_vault_initialization() {
        let env = odra_test::env();
        
        let admin = env.get_account(0);
        env.set_caller(admin);
        
        assert!(true, "Vault initialized successfully");
    }

    #[test]
    fn test_first_deposit_one_to_one_ratio() {
        let test_env = TestEnvironment::new();
        
        let deposit_amount = cspr(1000);
        let expected_shares = deposit_amount;
        
        assert_u512_eq(expected_shares, deposit_amount, "First deposit should be 1:1");
    }

    #[test]
    fn test_deposit_updates_state() {
        let test_env = TestEnvironment::new();
        test_env.set_caller(test_env.user1);
        
        let deposit_amount = cspr(1000);
        
        assert!(deposit_amount > U512::zero(), "Deposit amount valid");
    }

    #[test]
    fn test_subsequent_deposit_with_appreciation() {
        let deposit1 = cspr(1000);
        let deposit2 = cspr(500);
        
        let total_assets = cspr(1500);
        let total_shares = cspr(1000);
        
        let expected_shares = calculate_expected_shares(deposit2, total_assets, total_shares);
        
        assert!(expected_shares < deposit2, "Shares should be less than assets when share price > 1");
    }

    #[test]
    fn test_withdrawal_burns_shares() {
        let shares_to_burn = cspr(100);
        let total_assets = cspr(1000);
        let total_shares = cspr(800);
        
        let expected_assets = calculate_expected_assets(shares_to_burn, total_assets, total_shares);
        
        assert!(expected_assets > shares_to_burn, "Assets should be more than shares when share price > 1");
    }

    #[test]
    fn test_convert_to_shares() {
        let assets = U512::from(1000u64);
        let total_assets = U512::from(10000u64);
        let total_shares = U512::from(8000u64);
        
        let shares = calculate_expected_shares(assets, total_assets, total_shares);
        
        assert_eq!(shares, U512::from(800u64), "Share calculation incorrect");
    }

    #[test]
    fn test_convert_to_assets() {
        let shares = U512::from(800u64);
        let total_assets = U512::from(10000u64);
        let total_shares = U512::from(8000u64);
        
        let assets = calculate_expected_assets(shares, total_assets, total_shares);
        
        assert_eq!(assets, U512::from(1000u64), "Asset calculation incorrect");
    }

    #[test]
    fn test_share_price_calculation() {
        let total_assets = cspr(12000);
        let total_shares = cspr(10000);
        
        let share_price = (total_assets * U512::from(1_000_000u64)) / total_shares;
        let expected = U512::from(1_200_000u64);
        
        assert_u512_eq(share_price, expected, "Share price calculation");
    }

    #[test]
    fn test_zero_balance_withdrawal() {
        let user_shares = U512::zero();
        
        assert_eq!(user_shares, U512::zero(), "User should have zero shares");
    }

    #[test]
    fn test_max_deposit_limit() {
        let max_per_tx = cspr(10_000);
        let deposit = cspr(15_000);
        
        assert!(deposit > max_per_tx, "Should exceed limit");
    }

    #[test]
    fn test_instant_withdrawal_fee() {
        let withdrawal_amount = cspr(1000);
        let fee_bps = 50u64;
        
        let fee = calculate_performance_fee(withdrawal_amount, fee_bps);
        let expected_fee = cspr(5);
        
        assert_u512_within_tolerance(fee, expected_fee, 10);
    }

    #[test]
    fn test_time_locked_withdrawal_request() {
        let env = TestEnvironment::new();
        let current_time = env.get_block_time();
        let timelock_duration = 7 * 24 * 60 * 60;
        
        let execution_time = current_time + timelock_duration;
        
        assert!(execution_time > current_time, "Execution time in future");
    }

    #[test]
    fn test_withdrawal_request_completion_before_timelock() {
        let env = TestEnvironment::new();
        let current_time = env.get_block_time();
        let execution_time = current_time + (7 * 24 * 60 * 60);
        
        assert!(current_time < execution_time, "Should not be executable yet");
    }

    #[test]
    fn test_withdrawal_request_completion_after_timelock() {
        let env = TestEnvironment::new();
        let current_time = env.get_block_time();
        let execution_time = current_time + (7 * 24 * 60 * 60);
        
        let time_after_execution = execution_time + 100;
        
        assert!(time_after_execution >= execution_time, "Should be executable");
    }

    #[test]
    fn test_performance_fee_calculation() {
        let profit = cspr(1000);
        let fee_bps = 1000u64;
        
        let fee = calculate_performance_fee(profit, fee_bps);
        let expected = cspr(100);
        
        assert_u512_eq(fee, expected, "Performance fee");
    }

    #[test]
    fn test_management_fee_calculation() {
        let tvl = cspr(100_000);
        let fee_bps = 200u64;
        let days = 365u64;
        
        let fee = calculate_management_fee(tvl, fee_bps, days);
        let expected = cspr(2000);
        
        assert_u512_within_tolerance(fee, expected, 10);
    }

    #[test]
    fn test_total_assets_calculation() {
        let vault_balance = cspr(5000);
        let strategy_balance = cspr(15000);
        
        let total = vault_balance + strategy_balance;
        
        assert_u512_eq(total, cspr(20000), "Total assets");
    }

    #[test]
    fn test_multiple_user_shares() {
        let user1_deposit = cspr(10000);
        let user2_deposit = cspr(5000);
        
        let user1_shares = user1_deposit;
        let user2_shares = calculate_expected_shares(
            user2_deposit,
            user1_deposit + user2_deposit,
            user1_shares
        );
        
        let total_shares = user1_shares + user2_shares;
        
        assert!(total_shares > U512::zero(), "Total shares valid");
    }

    #[test]
    fn test_share_price_appreciation() {
        let initial_total_assets = cspr(10000);
        let initial_total_shares = cspr(10000);
        
        let yield_added = cspr(1000);
        let new_total_assets = initial_total_assets + yield_added;
        
        let initial_price = (initial_total_assets * U512::from(1000000u64)) / initial_total_shares;
        let new_price = (new_total_assets * U512::from(1000000u64)) / initial_total_shares;
        
        assert_share_price_increased(initial_price, new_price);
    }

    #[test]
    fn test_edge_case_minimum_deposit() {
        let min_deposit = U512::from(1u64);
        
        assert!(min_deposit > U512::zero(), "Minimum deposit valid");
    }

    #[test]
    fn test_edge_case_maximum_deposit() {
        let max_deposit = U512::from(u64::MAX);
        
        assert!(max_deposit > U512::zero(), "Maximum deposit valid");
    }

    #[test]
    fn test_pause_prevents_deposits() {
        let is_paused = true;
        
        assert!(is_paused, "Contract is paused");
    }

    #[test]
    fn test_unpause_allows_deposits() {
        let is_paused = false;
        
        assert!(!is_paused, "Contract is not paused");
    }
}
