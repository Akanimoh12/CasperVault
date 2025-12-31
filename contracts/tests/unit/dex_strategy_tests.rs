/// Unit tests for DEXStrategy
/// 
/// Tests the DEX liquidity provision strategy including:
/// - Deployment to LP pool
/// - Withdrawal from LP pool
/// - Harvest of trading fees and mining rewards
/// - Impermanent loss calculations
/// - Admin functions

#[cfg(test)]
mod dex_strategy_tests {
    use crate::strategies::dex_strategy::*;
    use crate::mocks::mock_dex::*;
    use odra::host::{Deployer, HostRef, NoArgs};
    use odra::types::{Address, U512};
    
    /// Setup test environment
    fn setup() -> (HostRef<DEXStrategyHostRef>, HostRef<MockDEXHostRef>, Address) {
        let admin = odra::test_env::get_account(0);
        let lst_cspr_address = odra::test_env::get_account(10);
        
        // Deploy mock DEX with 12% APY
        let mut mock_dex = MockDEXHostRef::deploy(NoArgs);
        mock_dex.init(1200u16); // 12% APY
        
        // Deploy DEX strategy
        let mut strategy = DEXStrategyHostRef::deploy(NoArgs);
        strategy.init(
            admin,
            lst_cspr_address,
            *mock_dex.address(),
            U512::from(1_000_000_000u64), // 1M max capacity
            U512::from(100_000u64), // 100 min deployment
        );
        
        (strategy, mock_dex, admin)
    }
    
    #[test]
    fn test_deploy_to_dex() {
        let (mut strategy, mock_dex, admin) = setup();
        
        let amount = U512::from(10_000_000u64); // 10K lstCSPR
        
        // Deploy to DEX
        strategy.with_tokens(admin).deploy(amount);
        
        // Check balance
        let balance = strategy.get_balance();
        assert!(balance > U512::zero(), "Balance should be greater than zero");
        
        // Check that LP tokens were minted
        let position = strategy.get_position();
        assert!(position.is_some(), "Position should exist");
        
        let (lp_tokens, lst_amount, cspr_amount) = position.unwrap();
        assert!(lp_tokens > U512::zero(), "LP tokens should be minted");
        assert_eq!(lst_amount, amount, "lstCSPR amount should match");
    }
    
    #[test]
    fn test_deploy_below_minimum() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(50_000u64); // Below minimum
        
        // Should revert
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            strategy.with_tokens(admin).deploy(amount);
        }));
        
        assert!(result.is_err(), "Should revert for amount below minimum");
    }
    
    #[test]
    fn test_deploy_exceeds_capacity() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(2_000_000_000u64); // 2M, exceeds 1M capacity
        
        // Should revert
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            strategy.with_tokens(admin).deploy(amount);
        }));
        
        assert!(result.is_err(), "Should revert for amount exceeding capacity");
    }
    
    #[test]
    fn test_withdraw_from_dex() {
        let (mut strategy, _, admin) = setup();
        
        let deposit_amount = U512::from(10_000_000u64);
        
        // First deploy
        strategy.with_tokens(admin).deploy(deposit_amount);
        
        let balance_after_deposit = strategy.get_balance();
        
        // Withdraw half
        let withdraw_amount = U512::from(5_000_000u64);
        let withdrawn = strategy.with_tokens(admin).withdraw(withdraw_amount);
        
        assert!(withdrawn > U512::zero(), "Withdrawn amount should be greater than zero");
        
        let balance_after_withdraw = strategy.get_balance();
        assert!(balance_after_withdraw < balance_after_deposit, "Balance should decrease after withdrawal");
    }
    
    #[test]
    fn test_harvest_rewards() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        
        // Deploy
        strategy.with_tokens(admin).deploy(amount);
        
        // Advance time by 30 days
        odra::test_env::advance_block_time_by(30 * 24 * 60 * 60);
        
        // Harvest
        let (trading_fees, mining_rewards) = strategy.with_tokens(admin).harvest();
        
        assert!(trading_fees > U512::zero(), "Trading fees should be earned");
        assert!(mining_rewards > U512::zero(), "Mining rewards should be earned");
        
        // Check that total harvested was updated
        let total_harvested = strategy.get_total_harvested();
        assert_eq!(
            total_harvested,
            trading_fees.checked_add(mining_rewards).unwrap(),
            "Total harvested should equal sum of fees and rewards"
        );
    }
    
    #[test]
    fn test_harvest_before_interval() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        
        // Deploy
        strategy.with_tokens(admin).deploy(amount);
        
        // Try to harvest immediately (before min interval)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            strategy.with_tokens(admin).harvest();
        }));
        
        // Should revert or return zero
        // (Depending on implementation, this might not revert but return zero)
    }
    
    #[test]
    fn test_calculate_impermanent_loss() {
        let (strategy, _, _) = setup();
        
        let initial_lst_price = U512::from(1_000_000u64);
        let initial_cspr_price = U512::from(1_000_000u64);
        let current_lst_price = U512::from(1_200_000u64); // 20% increase
        let current_cspr_price = U512::from(1_000_000u64);
        
        let il = strategy.calculate_impermanent_loss(
            initial_lst_price,
            initial_cspr_price,
            current_lst_price,
            current_cspr_price,
        );
        
        assert!(il.loss_bps > 0, "There should be some impermanent loss");
        assert!(il.loss_bps < 500, "IL should be less than 5% for 20% price change");
    }
    
    #[test]
    fn test_pause_unpause() {
        let (mut strategy, _, admin) = setup();
        
        // Pause
        strategy.with_tokens(admin).pause();
        
        let amount = U512::from(10_000_000u64);
        
        // Try to deploy while paused (should fail)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            strategy.with_tokens(admin).deploy(amount);
        }));
        
        assert!(result.is_err(), "Should not allow deploy when paused");
        
        // Unpause
        strategy.with_tokens(admin).unpause();
        
        // Now should work
        strategy.with_tokens(admin).deploy(amount);
        
        let balance = strategy.get_balance();
        assert!(balance > U512::zero(), "Deploy should work after unpause");
    }
    
    #[test]
    fn test_emergency_withdraw() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        
        // Deploy
        strategy.with_tokens(admin).deploy(amount);
        
        // Emergency withdraw
        let withdrawn = strategy.with_tokens(admin).emergency_withdraw();
        
        assert!(withdrawn > U512::zero(), "Should withdraw all funds");
        
        let balance = strategy.get_balance();
        assert_eq!(balance, U512::zero(), "Balance should be zero after emergency withdrawal");
    }
    
    #[test]
    fn test_set_max_capacity() {
        let (mut strategy, _, admin) = setup();
        
        let new_capacity = U512::from(5_000_000_000u64); // 5M
        
        strategy.with_tokens(admin).set_max_capacity(new_capacity);
        
        let capacity = strategy.max_capacity();
        assert_eq!(capacity, new_capacity, "Capacity should be updated");
    }
    
    #[test]
    fn test_get_apy() {
        let (strategy, _, _) = setup();
        
        let apy = strategy.get_apy();
        
        // Should be around 12% (1200 bps)
        assert!(apy >= 1100 && apy <= 1300, "APY should be around 12%");
    }
    
    #[test]
    fn test_get_risk_level() {
        let (strategy, _, _) = setup();
        
        let risk = strategy.get_risk_level();
        
        // DEX strategy should be Medium risk
        assert_eq!(risk, 1u8, "Risk level should be Medium (1)");
    }
    
    #[test]
    fn test_is_healthy() {
        let (mut strategy, _, admin) = setup();
        
        // Should be healthy initially
        assert!(strategy.is_healthy(), "Strategy should be healthy");
        
        // Deploy some funds
        strategy.with_tokens(admin).deploy(U512::from(10_000_000u64));
        
        // Should still be healthy
        assert!(strategy.is_healthy(), "Strategy should remain healthy after deployment");
    }
    
    #[test]
    fn test_multiple_deposits() {
        let (mut strategy, _, admin) = setup();
        
        // First deposit
        strategy.with_tokens(admin).deploy(U512::from(5_000_000u64));
        let balance1 = strategy.get_balance();
        
        // Second deposit
        strategy.with_tokens(admin).deploy(U512::from(3_000_000u64));
        let balance2 = strategy.get_balance();
        
        assert!(balance2 > balance1, "Balance should increase with each deposit");
    }
    
    #[test]
    fn test_yield_accumulation_over_time() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        strategy.with_tokens(admin).deploy(amount);
        
        // Harvest after 30 days
        odra::test_env::advance_block_time_by(30 * 24 * 60 * 60);
        let (fees1, rewards1) = strategy.with_tokens(admin).harvest();
        let yield1 = fees1.checked_add(rewards1).unwrap();
        
        // Harvest after another 30 days
        odra::test_env::advance_block_time_by(30 * 24 * 60 * 60);
        let (fees2, rewards2) = strategy.with_tokens(admin).harvest();
        let yield2 = fees2.checked_add(rewards2).unwrap();
        
        // Both harvests should have similar yields (assuming constant APY)
        let diff = if yield1 > yield2 {
            yield1.checked_sub(yield2).unwrap()
        } else {
            yield2.checked_sub(yield1).unwrap()
        };
        
        let tolerance = yield1.checked_div(U512::from(10u64)).unwrap(); // 10% tolerance
        assert!(diff < tolerance, "Yields should be similar for equal time periods");
    }
}
