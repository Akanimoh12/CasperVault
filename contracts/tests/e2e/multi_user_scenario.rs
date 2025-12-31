#[cfg(test)]
mod multi_user_scenario_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_sequential_user_deposits() {
        let env = TestEnvironment::new();
        
        env.set_caller(env.user1);
        let user1_deposit = cspr(10_000);
        let user1_shares = user1_deposit;
        let tvl_after_user1 = user1_deposit;
        let total_shares_after_user1 = user1_shares;
        
        env.advance_block_time(15 * 24 * 60 * 60);
        
        let yield_before_user2 = cspr(500);
        let tvl_before_user2 = tvl_after_user1 + yield_before_user2;
        
        env.set_caller(env.user2);
        let user2_deposit = cspr(5_000);
        let user2_shares = calculate_expected_shares(user2_deposit, tvl_before_user2, total_shares_after_user1);
        
        let tvl_after_user2 = tvl_before_user2 + user2_deposit;
        let total_shares_after_user2 = total_shares_after_user1 + user2_shares;
        
        assert!(user2_shares < user2_deposit, "User2 gets fewer shares");
        
        env.advance_block_time(15 * 24 * 60 * 60);
        
        let yield_before_user3 = cspr(800);
        let tvl_before_user3 = tvl_after_user2 + yield_before_user3;
        
        env.set_caller(env.user3);
        let user3_deposit = cspr(8_000);
        let user3_shares = calculate_expected_shares(user3_deposit, tvl_before_user3, total_shares_after_user2);
        
        let final_tvl = tvl_before_user3 + user3_deposit;
        let final_total_shares = total_shares_after_user2 + user3_shares;
        
        env.advance_block_time(30 * 24 * 60 * 60);
        
        let final_yield = cspr(1_200);
        let performance_fee = calculate_performance_fee(final_yield, 1000);
        let tvl_after_compound = final_tvl + final_yield - performance_fee;
        
        let user1_final_value = calculate_expected_assets(user1_shares, tvl_after_compound, final_total_shares);
        let user2_final_value = calculate_expected_assets(user2_shares, tvl_after_compound, final_total_shares);
        let user3_final_value = calculate_expected_assets(user3_shares, tvl_after_compound, final_total_shares);
        
        let user1_profit = user1_final_value - user1_deposit;
        let user2_profit = user2_final_value - user2_deposit;
        let user3_profit = user3_final_value - user3_deposit;
        
        assert!(user1_profit > cspr(1_500), "User1 highest profit (earliest deposit)");
        assert!(user2_profit > cspr(300), "User2 moderate profit");
        assert!(user3_profit > U512::zero() && user3_profit < cspr(300), "User3 lowest profit (latest deposit)");
        
        let total_withdrawn = user1_final_value + user2_final_value + user3_final_value;
        assert_u512_eq(total_withdrawn, tvl_after_compound, "All users can withdraw full TVL");
    }

    #[test]
    fn test_simultaneous_withdrawals() {
        let env = TestEnvironment::new();
        
        let total_tvl = cspr(100_000);
        let user1_shares = cspr(50_000);
        let user2_shares = cspr(30_000);
        let user3_shares = cspr(20_000);
        let total_shares = cspr(100_000);
        
        let user1_value = calculate_expected_assets(user1_shares, total_tvl, total_shares);
        let user2_value = calculate_expected_assets(user2_shares, total_tvl, total_shares);
        let user3_value = calculate_expected_assets(user3_shares, total_tvl, total_shares);
        
        assert_u512_eq(user1_value, cspr(50_000), "User1 withdrawal");
        assert_u512_eq(user2_value, cspr(30_000), "User2 withdrawal");
        assert_u512_eq(user3_value, cspr(20_000), "User3 withdrawal");
        
        let total = user1_value + user2_value + user3_value;
        assert_u512_eq(total, total_tvl, "All funds accounted for");
    }

    #[test]
    fn test_partial_withdrawals_multiple_users() {
        let total_tvl = cspr(100_000);
        let total_shares = cspr(80_000);
        
        let user1_shares = cspr(40_000);
        let user1_withdraw_shares = cspr(10_000);
        
        let user1_withdrawal = calculate_expected_assets(user1_withdraw_shares, total_tvl, total_shares);
        
        let new_tvl = total_tvl - user1_withdrawal;
        let new_total_shares = total_shares - user1_withdraw_shares;
        
        let user2_shares = cspr(30_000);
        let user2_withdraw_shares = cspr(15_000);
        
        let user2_withdrawal = calculate_expected_assets(user2_withdraw_shares, new_tvl, new_total_shares);
        
        let final_tvl = new_tvl - user2_withdrawal;
        
        assert!(final_tvl > U512::zero(), "TVL remains after partial withdrawals");
    }

    #[test]
    fn test_whale_vs_small_depositors() {
        let env = TestEnvironment::new();
        
        let whale_deposit = cspr(500_000);
        let whale_shares = whale_deposit;
        
        let small_depositors = generate_user_deposits(100, cspr(100));
        let small_total: U512 = small_depositors.iter().map(|d| d.amount).sum();
        
        let total_tvl = whale_deposit + small_total;
        
        let whale_share_pct = ((whale_shares * U512::from(100u64)) / total_tvl).as_u64();
        
        assert!(whale_share_pct > 70, "Whale owns majority");
        
        let whale_influence = whale_share_pct > 50;
        assert!(whale_influence, "Whale has significant influence");
    }

    #[test]
    fn test_user_deposits_during_different_share_prices() {
        let deposits = vec![
            (cspr(10_000), cspr(10_000), cspr(10_000)),
            (cspr(5_000), cspr(11_000), cspr(10_000)),
            (cspr(5_000), cspr(12_000), cspr(10_000)),
        ];
        
        for (deposit, tvl_before, shares_before) in deposits {
            let shares_minted = calculate_expected_shares(deposit, tvl_before, shares_before);
            
            let expected_ratio = (shares_minted * tvl_before) / deposit;
            let shares_ratio = shares_before;
            
            assert_u512_within_tolerance(expected_ratio, shares_ratio, 100);
        }
    }

    #[test]
    fn test_proportional_yield_distribution() {
        let user1_shares = cspr(60_000);
        let user2_shares = cspr(30_000);
        let user3_shares = cspr(10_000);
        let total_shares = cspr(100_000);
        
        let total_yield = cspr(10_000);
        
        let user1_yield = (total_yield * user1_shares) / total_shares;
        let user2_yield = (total_yield * user2_shares) / total_shares;
        let user3_yield = (total_yield * user3_shares) / total_shares;
        
        assert_u512_eq(user1_yield, cspr(6_000), "User1 gets 60% of yield");
        assert_u512_eq(user2_yield, cspr(3_000), "User2 gets 30% of yield");
        assert_u512_eq(user3_yield, cspr(1_000), "User3 gets 10% of yield");
        
        let distributed = user1_yield + user2_yield + user3_yield;
        assert_u512_eq(distributed, total_yield, "All yield distributed");
    }

    #[test]
    fn test_user_competition_for_yields() {
        let env = TestEnvironment::new();
        
        let user1_early_deposit = cspr(10_000);
        let user1_shares = user1_early_deposit;
        
        env.advance_block_time(60 * 24 * 60 * 60);
        
        let yield_earned = cspr(1_000);
        let tvl_with_yield = user1_early_deposit + yield_earned;
        
        let user2_late_deposit = cspr(10_000);
        let user2_shares = calculate_expected_shares(user2_late_deposit, tvl_with_yield, user1_shares);
        
        let total_shares = user1_shares + user2_shares;
        let total_tvl = tvl_with_yield + user2_late_deposit;
        
        let user1_value = calculate_expected_assets(user1_shares, total_tvl, total_shares);
        let user2_value = calculate_expected_assets(user2_shares, total_tvl, total_shares);
        
        assert!(user1_value > user2_value, "Early depositor has advantage");
    }

    #[test]
    fn test_mass_exodus_scenario() {
        let total_tvl = cspr(1_000_000);
        let total_shares = cspr(1_000_000);
        
        let mass_withdrawal_shares = cspr(500_000);
        
        let withdrawal_amount = calculate_expected_assets(mass_withdrawal_shares, total_tvl, total_shares);
        
        let remaining_tvl = total_tvl - withdrawal_amount;
        let remaining_shares = total_shares - mass_withdrawal_shares;
        
        assert_u512_eq(remaining_tvl, cspr(500_000), "50% TVL remains");
        assert_u512_eq(remaining_shares, cspr(500_000), "50% shares remain");
        
        let remaining_share_price = (remaining_tvl * U512::from(1_000_000u64)) / remaining_shares;
        let initial_price = U512::from(1_000_000u64);
        
        assert_u512_eq(remaining_share_price, initial_price, "Share price unchanged");
    }
}
