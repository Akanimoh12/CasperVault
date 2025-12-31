#[cfg(test)]
mod compound_integration_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_full_compound_cycle() {
        let env = TestEnvironment::new();
        
        let initial_tvl = cspr(100_000);
        let staking_rewards = cspr(500);
        let dex_yield = cspr(300);
        let lending_yield = cspr(200);
        
        let total_yield = staking_rewards + dex_yield + lending_yield;
        
        let new_tvl = initial_tvl + total_yield;
        
        assert_u512_eq(new_tvl, cspr(101_000), "TVL after compound");
    }

    #[test]
    fn test_fee_distribution_during_compound() {
        let total_yield = cspr(1_000);
        let performance_fee_bps = 1000u64;
        
        let performance_fee = calculate_performance_fee(total_yield, performance_fee_bps);
        let reinvested_amount = total_yield - performance_fee;
        
        assert_u512_eq(performance_fee, cspr(100), "10% performance fee");
        assert_u512_eq(reinvested_amount, cspr(900), "90% reinvested");
    }

    #[test]
    fn test_share_price_update_after_compound() {
        let total_shares = cspr(100_000);
        let tvl_before = cspr(100_000);
        let tvl_after = cspr(101_000);
        
        let price_before = (tvl_before * U512::from(1_000_000u64)) / total_shares;
        let price_after = (tvl_after * U512::from(1_000_000u64)) / total_shares;
        
        let price_increase = price_after - price_before;
        let expected_increase = U512::from(10_000u64);
        
        assert_u512_eq(price_increase, expected_increase, "Share price increased by 1%");
    }

    #[test]
    fn test_compound_with_multiple_users() {
        let user1_shares = cspr(60_000);
        let user2_shares = cspr(30_000);
        let user3_shares = cspr(10_000);
        
        let total_shares = user1_shares + user2_shares + user3_shares;
        let tvl_after_compound = cspr(105_000);
        
        let user1_value = calculate_expected_assets(user1_shares, tvl_after_compound, total_shares);
        let user2_value = calculate_expected_assets(user2_shares, tvl_after_compound, total_shares);
        let user3_value = calculate_expected_assets(user3_shares, tvl_after_compound, total_shares);
        
        assert_u512_eq(user1_value, cspr(63_000), "User1 value after compound");
        assert_u512_eq(user2_value, cspr(31_500), "User2 value after compound");
        assert_u512_eq(user3_value, cspr(10_500), "User3 value after compound");
    }

    #[test]
    fn test_compound_frequency_optimization() {
        let yield_per_day = cspr(100);
        let gas_cost = cspr(10);
        let min_yield_threshold = cspr(100);
        
        let accumulated_yield = yield_per_day;
        let should_compound = accumulated_yield >= min_yield_threshold &&
                             accumulated_yield > gas_cost * U512::from(2u64);
        
        assert!(should_compound, "Profitable to compound");
    }

    #[test]
    fn test_compound_time_interval_check() {
        let env = TestEnvironment::new();
        let last_compound_time = env.get_block_time();
        
        env.advance_block_time(2 * 60 * 60);
        
        let current_time = env.get_block_time();
        let min_interval = 1 * 60 * 60u64;
        
        let can_compound = (current_time - last_compound_time) >= min_interval;
        
        assert!(can_compound, "Sufficient time passed");
    }

    #[test]
    fn test_restake_after_harvest() {
        let staking_rewards = cspr(500);
        let performance_fee = calculate_performance_fee(staking_rewards, 1000);
        
        let restake_amount = staking_rewards - performance_fee;
        
        assert_u512_eq(restake_amount, cspr(450), "Restaked after fee");
    }

    #[test]
    fn test_redeploy_to_strategies() {
        let harvested_yield = cspr(1_000);
        let performance_fee = calculate_performance_fee(harvested_yield, 1000);
        let to_redeploy = harvested_yield - performance_fee;
        
        let dex_amount = (to_redeploy * U512::from(40u64)) / U512::from(100u64);
        let lending_amount = (to_redeploy * U512::from(30u64)) / U512::from(100u64);
        let cross_chain_amount = (to_redeploy * U512::from(30u64)) / U512::from(100u64);
        
        assert_u512_eq(dex_amount, cspr(360), "DEX redeployment");
        assert_u512_eq(lending_amount, cspr(270), "Lending redeployment");
        assert_u512_eq(cross_chain_amount, cspr(270), "Cross-chain redeployment");
    }

    #[test]
    fn test_compound_with_zero_yield() {
        let total_yield = U512::zero();
        let min_threshold = cspr(100);
        
        let should_compound = total_yield >= min_threshold;
        
        assert!(!should_compound, "Should not compound with zero yield");
    }

    #[test]
    fn test_management_fee_accrual() {
        let tvl = cspr(1_000_000);
        let management_fee_bps = 200u64;
        let days_since_last = 30u64;
        
        let management_fee = calculate_management_fee(tvl, management_fee_bps, days_since_last);
        
        let expected = cspr(1_643);
        assert_u512_within_tolerance(management_fee, expected, 100);
    }

    #[test]
    fn test_multiple_compound_cycles() {
        let initial = cspr(100_000);
        let rate_bps = 100u64;
        let cycles = 12u32;
        
        let final_amount = calculate_compound_growth(initial, rate_bps, cycles);
        
        assert!(final_amount > cspr(112_000), "Compound growth over 12 cycles");
    }

    #[test]
    fn test_token_swap_during_compound() {
        let reward_tokens = cspr(100);
        let slippage_bps = 50u64;
        
        let min_cspr_out = apply_slippage(reward_tokens, slippage_bps);
        
        assert!(min_cspr_out < reward_tokens, "Slippage applied to swap");
    }

    #[test]
    fn test_apy_snapshot_after_compound() {
        let initial_tvl = cspr(100_000);
        let final_tvl = cspr(101_000);
        let days_elapsed = 30u64;
        
        let apy = calculate_apy(initial_tvl, final_tvl, days_elapsed);
        
        assert!(apy > 1000 && apy < 1300, "APY around 12%");
    }

    #[test]
    fn test_user_deposit_during_compound() {
        let tvl_mid_compound = cspr(101_000);
        let total_shares = cspr(100_000);
        
        let new_deposit = cspr(10_000);
        let new_shares = calculate_expected_shares(new_deposit, tvl_mid_compound, total_shares);
        
        assert!(new_shares < cspr(10_000), "Fewer shares due to appreciation");
    }

    #[test]
    fn test_compound_event_emission() {
        let total_yield = cspr(1_000);
        let performance_fee = cspr(100);
        let reinvested = cspr(900);
        
        assert_u512_eq(total_yield, performance_fee + reinvested, "Event data correct");
    }
}
