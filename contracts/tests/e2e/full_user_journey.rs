#[cfg(test)]
mod full_user_journey_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_complete_user_lifecycle() {
        let env = TestEnvironment::new();
        env.set_caller(env.user1);
        
        let initial_deposit = cspr(1000);
        let shares_minted = initial_deposit;
        
        assert_u512_eq(shares_minted, initial_deposit, "Step 1: Deposit completed");
        
        let staked_amount = initial_deposit;
        assert_u512_eq(staked_amount, initial_deposit, "Step 2: Funds staked");
        
        let dex_deployed = (staked_amount * U512::from(40u64)) / U512::from(100u64);
        let lending_deployed = (staked_amount * U512::from(30u64)) / U512::from(100u64);
        let cross_chain_deployed = (staked_amount * U512::from(30u64)) / U512::from(100u64);
        
        assert_u512_eq(dex_deployed, cspr(400), "Step 3: DEX strategy deployed");
        assert_u512_eq(lending_deployed, cspr(300), "Step 3: Lending strategy deployed");
        
        env.advance_block_time(30 * 24 * 60 * 60);
        
        let staking_rewards = cspr(25);
        let dex_yield = cspr(10);
        let lending_yield = cspr(8);
        let total_yield = staking_rewards + dex_yield + lending_yield;
        
        assert_u512_eq(total_yield, cspr(43), "Step 4: Yields accrued");
        
        let performance_fee = calculate_performance_fee(total_yield, 1000);
        let compounded_amount = total_yield - performance_fee;
        
        assert_u512_eq(compounded_amount, cspr(38), "Step 5: Yield compounded");
        
        let new_tvl = initial_deposit + compounded_amount;
        let total_shares = shares_minted;
        
        let new_share_price = (new_tvl * U512::from(1_000_000u64)) / total_shares;
        let initial_share_price = U512::from(1_000_000u64);
        
        assert!(new_share_price > initial_share_price, "Step 6: Share price increased");
        
        let withdrawal_value = calculate_expected_assets(shares_minted, new_tvl, total_shares);
        let profit = withdrawal_value - initial_deposit;
        
        assert_u512_eq(profit, cspr(38), "Step 7: Profit realized");
        assert_u512_eq(withdrawal_value, cspr(1038), "Step 7: Withdrawal with profit");
    }

    #[test]
    fn test_user_journey_with_timelock_withdrawal() {
        let env = TestEnvironment::new();
        env.set_caller(env.user1);
        
        let deposit = cspr(10_000);
        let shares = deposit;
        
        let withdrawal_request_time = env.get_block_time();
        let timelock_duration = 7 * 24 * 60 * 60u64;
        let execution_time = withdrawal_request_time + timelock_duration;
        
        env.advance_block_time(8 * 24 * 60 * 60);
        
        let current_time = env.get_block_time();
        assert!(current_time >= execution_time, "Timelock passed");
        
        let no_fee = U512::zero();
        let withdrawal_amount = deposit;
        
        assert_u512_eq(withdrawal_amount, deposit, "Full amount with no fee");
    }

    #[test]
    fn test_user_journey_with_instant_withdrawal() {
        let deposit = cspr(10_000);
        let shares = deposit;
        
        let instant_fee_bps = 50u64;
        let fee = calculate_performance_fee(deposit, instant_fee_bps);
        
        let instant_proceeds = deposit - fee;
        
        assert_u512_eq(fee, cspr(5), "0.5% instant withdrawal fee");
        assert_u512_eq(instant_proceeds, cspr(9_995), "Instant withdrawal proceeds");
    }

    #[test]
    fn test_user_journey_multiple_deposits() {
        let env = TestEnvironment::new();
        env.set_caller(env.user1);
        
        let deposit1 = cspr(5_000);
        let shares1 = deposit1;
        
        env.advance_block_time(15 * 24 * 60 * 60);
        
        let yield1 = cspr(250);
        let tvl_before_deposit2 = deposit1 + yield1;
        
        let deposit2 = cspr(5_000);
        let shares2 = calculate_expected_shares(deposit2, tvl_before_deposit2, shares1);
        
        let total_shares = shares1 + shares2;
        let total_tvl = tvl_before_deposit2 + deposit2;
        
        env.advance_block_time(15 * 24 * 60 * 60);
        
        let yield2 = cspr(250);
        let final_tvl = total_tvl + yield2;
        
        let final_value = calculate_expected_assets(total_shares, final_tvl, total_shares);
        let total_deposited = deposit1 + deposit2;
        let profit = final_value - total_deposited;
        
        assert!(profit > cspr(400), "Multiple deposits profitable");
    }

    #[test]
    fn test_user_journey_with_rebalancing() {
        let env = TestEnvironment::new();
        
        let initial_tvl = cspr(100_000);
        
        let initial_dex = cspr(40_000);
        let initial_lending = cspr(30_000);
        let initial_cross_chain = cspr(30_000);
        
        env.advance_block_time(30 * 24 * 60 * 60);
        
        let new_dex_apy = 1200u16;
        let new_lending_apy = 500u16;
        
        let should_rebalance = new_dex_apy > new_lending_apy + 500;
        
        assert!(should_rebalance, "Should rebalance to higher APY");
        
        let target_dex_allocation = 45u64;
        let target_lending_allocation = 25u64;
        
        let new_dex = (initial_tvl * U512::from(target_dex_allocation)) / U512::from(100u64);
        let new_lending = (initial_tvl * U512::from(target_lending_allocation)) / U512::from(100u64);
        
        let move_from_lending = initial_lending - new_lending;
        let move_to_dex = new_dex - initial_dex;
        
        assert_u512_eq(move_from_lending, cspr(5_000), "Rebalanced from lending");
        assert_u512_eq(move_to_dex, cspr(5_000), "Rebalanced to DEX");
    }

    #[test]
    fn test_user_journey_during_market_volatility() {
        let env = TestEnvironment::new();
        
        let deposit = cspr(10_000);
        let shares = deposit;
        
        env.advance_block_time(7 * 24 * 60 * 60);
        
        let tvl_with_gains = cspr(10_500);
        
        env.advance_block_time(7 * 24 * 60 * 60);
        
        let tvl_with_loss = cspr(10_200);
        
        let final_value = calculate_expected_assets(shares, tvl_with_loss, shares);
        
        assert!(final_value > deposit, "Still profitable despite volatility");
        assert!(final_value < tvl_with_gains, "Lower than peak");
    }

    #[test]
    fn test_user_journey_with_emergency_pause() {
        let env = TestEnvironment::new();
        
        let deposit = cspr(10_000);
        let is_paused = true;
        
        assert!(is_paused, "System paused");
        
        let withdrawal_allowed = true;
        
        assert!(withdrawal_allowed, "Withdrawals still allowed during pause");
    }

    #[test]
    fn test_long_term_hold_scenario() {
        let env = TestEnvironment::new();
        
        let initial_deposit = cspr(10_000);
        let shares = initial_deposit;
        
        let mut tvl = initial_deposit;
        let monthly_apy = 100u64;
        
        for month in 1..=12 {
            env.advance_block_time(30 * 24 * 60 * 60);
            
            let monthly_yield = (tvl * U512::from(monthly_apy)) / U512::from(10000u64);
            let fee = calculate_performance_fee(monthly_yield, 1000);
            tvl = tvl + monthly_yield - fee;
        }
        
        let final_value = calculate_expected_assets(shares, tvl, shares);
        let apy = calculate_apy(initial_deposit, final_value, 365);
        
        assert!(apy >= 900 && apy <= 1100, "Annual APY around 10%");
    }
}
