/// Unit tests for LendingStrategy
/// 
/// Tests the lending protocol strategy including:
/// - Supply to lending pool
/// - Redeem from lending pool
/// - Interest accrual
/// - Utilization rate monitoring
/// - Admin functions

#[cfg(test)]
mod lending_strategy_tests {
    use crate::strategies::lending_strategy::*;
    use crate::mocks::mock_lending::*;
    use odra::host::{Deployer, HostRef, NoArgs};
    use odra::types::{Address, U512};
    
    /// Setup test environment
    fn setup() -> (HostRef<LendingStrategyHostRef>, HostRef<MockLendingHostRef>, Address) {
        let admin = odra::test_env::get_account(0);
        let lst_cspr_address = odra::test_env::get_account(10);
        
        // Deploy mock lending with 8% base APY
        let mut mock_lending = MockLendingHostRef::deploy(NoArgs);
        mock_lending.init(800u16); // 8% base APY
        
        // Deploy lending strategy
        let mut strategy = LendingStrategyHostRef::deploy(NoArgs);
        strategy.init(
            admin,
            lst_cspr_address,
            *mock_lending.address(),
            U512::from(5_000_000_000u64), // 5M max capacity (largest for low-risk)
            U512::from(100_000u64), // 100 min supply
        );
        
        (strategy, mock_lending, admin)
    }
    
    #[test]
    fn test_supply_to_lending() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64); // 10K lstCSPR
        
        // Supply to lending pool
        strategy.with_tokens(admin).deploy(amount);
        
        // Check balance
        let balance = strategy.get_balance();
        assert!(balance >= amount, "Balance should be at least the supplied amount");
        
        // Check position
        let position = strategy.get_position();
        assert!(position.is_some(), "Position should exist");
        
        let (principal, c_tokens, interest) = position.unwrap();
        assert_eq!(principal, amount, "Principal should match supplied amount");
        assert!(c_tokens > U512::zero(), "cTokens should be minted");
    }
    
    #[test]
    fn test_supply_below_minimum() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(50_000u64); // Below minimum
        
        // Should revert
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            strategy.with_tokens(admin).deploy(amount);
        }));
        
        assert!(result.is_err(), "Should revert for amount below minimum");
    }
    
    #[test]
    fn test_supply_exceeds_capacity() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(6_000_000_000u64); // 6M, exceeds 5M capacity
        
        // Should revert
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            strategy.with_tokens(admin).deploy(amount);
        }));
        
        assert!(result.is_err(), "Should revert for amount exceeding capacity");
    }
    
    #[test]
    fn test_redeem_from_lending() {
        let (mut strategy, _, admin) = setup();
        
        let supply_amount = U512::from(10_000_000u64);
        
        // First supply
        strategy.with_tokens(admin).deploy(supply_amount);
        
        let balance_after_supply = strategy.get_balance();
        
        // Redeem half
        let redeem_amount = U512::from(5_000_000u64);
        let redeemed = strategy.with_tokens(admin).withdraw(redeem_amount);
        
        assert!(redeemed > U512::zero(), "Redeemed amount should be greater than zero");
        
        let balance_after_redeem = strategy.get_balance();
        assert!(balance_after_redeem < balance_after_supply, "Balance should decrease after redemption");
    }
    
    #[test]
    fn test_interest_accrual() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        
        // Supply
        strategy.with_tokens(admin).deploy(amount);
        
        let balance_initial = strategy.get_balance();
        
        // Advance time by 365 days (1 year)
        odra::test_env::advance_block_time_by(365 * 24 * 60 * 60);
        
        // Harvest (accrue interest)
        let interest = strategy.with_tokens(admin).harvest();
        
        assert!(interest > U512::zero(), "Interest should be earned");
        
        // Balance should have increased
        let balance_after = strategy.get_balance();
        assert!(balance_after > balance_initial, "Balance should increase with interest");
        
        // Interest should be roughly 8% of principal (800 bps)
        let expected_interest = amount
            .checked_mul(U512::from(8u64))
            .unwrap()
            .checked_div(U512::from(100u64))
            .unwrap();
        
        let tolerance = expected_interest.checked_div(U512::from(10u64)).unwrap(); // 10% tolerance
        let diff = if interest > expected_interest {
            interest.checked_sub(expected_interest).unwrap()
        } else {
            expected_interest.checked_sub(interest).unwrap()
        };
        
        assert!(diff < tolerance, "Interest should be approximately 8% per year");
    }
    
    #[test]
    fn test_harvest_multiple_times() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        strategy.with_tokens(admin).deploy(amount);
        
        // First harvest after 30 days
        odra::test_env::advance_block_time_by(30 * 24 * 60 * 60);
        let interest1 = strategy.with_tokens(admin).harvest();
        
        // Second harvest after another 30 days
        odra::test_env::advance_block_time_by(30 * 24 * 60 * 60);
        let interest2 = strategy.with_tokens(admin).harvest();
        
        // Both should have similar interest (for equal time periods)
        assert!(interest1 > U512::zero(), "First harvest should yield interest");
        assert!(interest2 > U512::zero(), "Second harvest should yield interest");
    }
    
    #[test]
    fn test_utilization_monitoring() {
        let (mut strategy, mock_lending, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        strategy.with_tokens(admin).deploy(amount);
        
        // Check initial utilization
        let utilization = strategy.get_pool_utilization();
        assert!(utilization < 9000, "Initial utilization should be below max");
        
        // Simulate borrowing to increase utilization
        mock_lending.simulate_borrow(U512::from(8_000_000u64)); // 80% utilization
        
        let new_utilization = strategy.get_pool_utilization();
        assert!(new_utilization > utilization, "Utilization should increase after borrowing");
    }
    
    #[test]
    fn test_apy_adjusts_with_utilization() {
        let (mut strategy, mock_lending, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        strategy.with_tokens(admin).deploy(amount);
        
        // Get initial APY
        strategy.update_apy_cache();
        let apy_low_util = strategy.get_apy();
        
        // Simulate high utilization
        mock_lending.simulate_borrow(U512::from(9_000_000u64)); // 90% utilization
        
        // Update APY
        strategy.update_apy_cache();
        let apy_high_util = strategy.get_apy();
        
        assert!(apy_high_util > apy_low_util, "APY should increase with higher utilization");
    }
    
    #[test]
    fn test_pause_unpause() {
        let (mut strategy, _, admin) = setup();
        
        // Pause
        strategy.with_tokens(admin).pause();
        
        let amount = U512::from(10_000_000u64);
        
        // Try to supply while paused (should fail)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            strategy.with_tokens(admin).deploy(amount);
        }));
        
        assert!(result.is_err(), "Should not allow supply when paused");
        
        // Unpause
        strategy.with_tokens(admin).unpause();
        
        // Now should work
        strategy.with_tokens(admin).deploy(amount);
        
        let balance = strategy.get_balance();
        assert!(balance > U512::zero(), "Supply should work after unpause");
    }
    
    #[test]
    fn test_emergency_withdraw() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        
        // Supply
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
        
        let new_capacity = U512::from(10_000_000_000u64); // 10M
        
        strategy.with_tokens(admin).set_max_capacity(new_capacity);
        
        let capacity = strategy.max_capacity();
        assert_eq!(capacity, new_capacity, "Capacity should be updated");
    }
    
    #[test]
    fn test_set_utilization_targets() {
        let (mut strategy, _, admin) = setup();
        
        let new_target = 8000u16; // 80%
        let new_max = 9500u16; // 95%
        
        strategy.with_tokens(admin).set_utilization_targets(new_target, new_max);
        
        // Verify targets were updated (would need getter functions)
    }
    
    #[test]
    fn test_get_risk_level() {
        let (strategy, _, _) = setup();
        
        let risk = strategy.get_risk_level();
        
        // Lending strategy should be Low risk
        assert_eq!(risk, 0u8, "Risk level should be Low (0)");
    }
    
    #[test]
    fn test_is_healthy() {
        let (mut strategy, _, admin) = setup();
        
        // Should be healthy initially
        assert!(strategy.is_healthy(), "Strategy should be healthy");
        
        // Supply some funds
        strategy.with_tokens(admin).deploy(U512::from(10_000_000u64));
        
        // Should still be healthy
        assert!(strategy.is_healthy(), "Strategy should remain healthy after supply");
    }
    
    #[test]
    fn test_multiple_supplies() {
        let (mut strategy, _, admin) = setup();
        
        // First supply
        strategy.with_tokens(admin).deploy(U512::from(5_000_000u64));
        let balance1 = strategy.get_balance();
        
        // Second supply
        strategy.with_tokens(admin).deploy(U512::from(3_000_000u64));
        let balance2 = strategy.get_balance();
        
        assert!(balance2 > balance1, "Balance should increase with each supply");
        
        // Total supplied should equal sum
        let expected = U512::from(8_000_000u64);
        assert!(balance2 >= expected, "Total balance should be at least the sum of supplies");
    }
    
    #[test]
    fn test_compound_interest() {
        let (mut strategy, _, admin) = setup();
        
        let amount = U512::from(10_000_000u64);
        strategy.with_tokens(admin).deploy(amount);
        
        // Harvest every 90 days for a year
        let mut total_interest = U512::zero();
        
        for _ in 0..4 {
            odra::test_env::advance_block_time_by(90 * 24 * 60 * 60);
            let interest = strategy.with_tokens(admin).harvest();
            total_interest = total_interest.checked_add(interest).unwrap();
        }
        
        // Total interest should be greater than 8% due to compounding
        let simple_interest = amount
            .checked_mul(U512::from(8u64))
            .unwrap()
            .checked_div(U512::from(100u64))
            .unwrap();
        
        // With compounding, should be slightly higher
        assert!(total_interest >= simple_interest, "Compound interest should be at least as much as simple interest");
    }
}
