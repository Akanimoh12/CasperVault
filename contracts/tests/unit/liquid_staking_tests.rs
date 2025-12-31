#[cfg(test)]
mod liquid_staking_tests {
    use odra::prelude::*;
    use caspervault_contracts::*;

    /// Test helper to create a test environment
    fn setup() -> (LiquidStaking, Address, Address) {
        let admin = Address::from([1u8; 32]);
        let user = Address::from([2u8; 32]);
        let lst_token = Address::from([3u8; 32]);
        
        let mut staking = LiquidStaking::new();
        staking.init(admin, lst_token);
        
        (staking, admin, user)
    }

    #[test]
    fn test_initialization() {
        let (staking, admin, _) = setup();
        
        // Check initial state
        assert_eq!(staking.get_total_staked(), U512::zero());
        assert_eq!(staking.get_total_lst_cspr(), U512::zero());
        assert_eq!(staking.get_exchange_rate(), U256::from(1_000_000_000u64));
        assert_eq!(staking.get_total_rewards_earned(), U512::zero());
    }

    #[test]
    fn test_add_validator() {
        let (mut staking, admin, _) = setup();
        
        let validator = Address::from([10u8; 32]);
        
        // Add validator
        staking.add_validator(
            validator,
            98,  // 98% uptime
            5,   // 5% commission
            U512::from(1_000_000_000_000_000u64), // 1M CSPR cap
        );
        
        // Verify validator was added
        let active_validators = staking.get_active_validators();
        assert_eq!(active_validators.len(), 1);
        assert_eq!(active_validators[0], validator);
    }

    #[test]
    fn test_first_stake_one_to_one() {
        let (mut staking, admin, user) = setup();
        
        // Add a validator first
        let validator = Address::from([10u8; 32]);
        staking.add_validator(
            validator,
            98,
            5,
            U512::from(1_000_000_000_000_000u64),
        );
        
        // Stake 100 CSPR
        let stake_amount = U512::from(100_000_000_000u64); // 100 CSPR
        
        // TODO: Mock attached_value and caller
        // let lst_amount = staking.stake();
        
        // For first stake, should mint 1:1
        // assert_eq!(lst_amount, stake_amount);
        // assert_eq!(staking.get_total_staked(), stake_amount);
        // assert_eq!(staking.get_total_lst_cspr(), stake_amount);
        // assert_eq!(staking.get_exchange_rate(), U256::from(1_000_000_000u64));
    }

    #[test]
    fn test_exchange_rate_after_rewards() {
        let (mut staking, admin, user) = setup();
        
        // Setup: Add validator and initial stake
        let validator = Address::from([10u8; 32]);
        staking.add_validator(validator, 98, 5, U512::from(1_000_000_000_000_000u64));
        
        // Simulate: 100 CSPR staked, 100 lstCSPR minted
        // (In real test, would call stake())
        
        // Simulate: Rewards compound, now 110 CSPR for 100 lstCSPR
        // Exchange rate should be 110/100 = 1.1 = 1,100,000,000
        
        // TODO: Complete once stake() can be properly tested
    }

    #[test]
    fn test_validator_selection_algorithm() {
        let (mut staking, admin, _) = setup();
        
        // Add 3 validators with different current stakes
        let validator1 = Address::from([10u8; 32]);
        let validator2 = Address::from([11u8; 32]);
        let validator3 = Address::from([12u8; 32]);
        
        staking.add_validator(validator1, 98, 5, U512::from(1_000_000_000_000_000u64));
        staking.add_validator(validator2, 99, 3, U512::from(1_000_000_000_000_000u64));
        staking.add_validator(validator3, 97, 7, U512::from(1_000_000_000_000_000u64));
        
        // All validators should be eligible
        let active = staking.get_active_validators();
        assert_eq!(active.len(), 3);
        
        // TODO: Test validator selection with allocations
        // Should distribute evenly to promote decentralization
    }

    #[test]
    fn test_compound_rewards() {
        let (mut staking, admin, _) = setup();
        
        // Setup
        let validator = Address::from([10u8; 32]);
        staking.add_validator(validator, 98, 5, U512::from(1_000_000_000_000_000u64));
        
        // TODO: Stake initial amount
        // TODO: Call compound_rewards()
        // TODO: Verify total_staked increased
        // TODO: Verify exchange_rate updated
        // TODO: Verify event emitted
    }

    #[test]
    fn test_unstake_creates_unbonding_request() {
        let (mut staking, admin, user) = setup();
        
        // Setup: Add validator and stake
        let validator = Address::from([10u8; 32]);
        staking.add_validator(validator, 98, 5, U512::from(1_000_000_000_000_000u64));
        
        // TODO: Stake 100 CSPR
        // TODO: Unstake 50 lstCSPR
        // TODO: Verify unbonding request created
        // TODO: Verify unlock_time = now + 14 days
    }

    #[test]
    fn test_complete_unbonding_success() {
        let (mut staking, admin, user) = setup();
        
        // TODO: Create unbonding request
        // TODO: Fast-forward time past unlock_time
        // TODO: Call complete_unbonding()
        // TODO: Verify CSPR transferred to user
        // TODO: Verify request marked as completed
    }

    #[test]
    #[should_panic]
    fn test_complete_unbonding_before_unlock() {
        let (mut staking, admin, user) = setup();
        
        // TODO: Create unbonding request
        // TODO: Try to complete before unlock_time
        // Should panic with TimelockNotExpired
    }

    #[test]
    fn test_remove_validator_undelegates() {
        let (mut staking, admin, _) = setup();
        
        let validator = Address::from([10u8; 32]);
        staking.add_validator(validator, 98, 5, U512::from(1_000_000_000_000_000u64));
        
        // TODO: Delegate some stake to validator
        // TODO: Remove validator
        // TODO: Verify delegation removed
        // TODO: Verify validator not in active list
    }

    #[test]
    fn test_apy_calculation() {
        let (mut staking, admin, _) = setup();
        
        // TODO: Setup with staked amount and rewards
        // TODO: Calculate APY
        // TODO: Verify APY is reasonable (e.g., 8-12%)
    }

    #[test]
    fn test_should_compound_logic() {
        let (staking, admin, _) = setup();
        
        // Initially should not compound (no stake, no time passed)
        assert_eq!(staking.should_compound(), false);
        
        // TODO: Add stake and rewards
        // TODO: Fast-forward time
        // TODO: Verify should_compound returns true
    }

    #[test]
    #[should_panic]
    fn test_stake_zero_amount_fails() {
        let (mut staking, admin, _) = setup();
        
        let validator = Address::from([10u8; 32]);
        staking.add_validator(validator, 98, 5, U512::from(1_000_000_000_000_000u64));
        
        // TODO: Try to stake 0 CSPR
        // Should panic with ExceedsStakedAmount or ZeroAmount
    }

    #[test]
    #[should_panic]
    fn test_unstake_more_than_balance() {
        let (mut staking, admin, user) = setup();
        
        let validator = Address::from([10u8; 32]);
        staking.add_validator(validator, 98, 5, U512::from(1_000_000_000_000_000u64));
        
        // TODO: Stake 100 CSPR
        // TODO: Try to unstake 200 lstCSPR
        // Should panic with ExceedsStakedAmount
    }

    #[test]
    fn test_validator_cap_enforced() {
        let (mut staking, admin, _) = setup();
        
        let validator = Address::from([10u8; 32]);
        let small_cap = U512::from(1_000_000_000_000u64); // 1000 CSPR
        staking.add_validator(validator, 98, 5, small_cap);
        
        // TODO: Try to stake more than validator cap
        // TODO: Verify allocation respects cap
        // TODO: Verify excess goes to other validators or fails gracefully
    }

    #[test]
    fn test_proportional_undelegation() {
        let (mut staking, admin, _) = setup();
        
        // Add 3 validators
        let validator1 = Address::from([10u8; 32]);
        let validator2 = Address::from([11u8; 32]);
        let validator3 = Address::from([12u8; 32]);
        
        staking.add_validator(validator1, 98, 5, U512::from(1_000_000_000_000_000u64));
        staking.add_validator(validator2, 99, 3, U512::from(1_000_000_000_000_000u64));
        staking.add_validator(validator3, 97, 7, U512::from(1_000_000_000_000_000u64));
        
        // TODO: Stake 300 CSPR (should split evenly: 100 each)
        // TODO: Unstake 150 CSPR
        // TODO: Verify proportional withdrawal (50 from each)
    }

    #[test]
    fn test_emergency_undelegate() {
        let (mut staking, admin, _) = setup();
        
        let validator = Address::from([10u8; 32]);
        staking.add_validator(validator, 98, 5, U512::from(1_000_000_000_000_000u64));
        
        // TODO: Stake some amount
        // TODO: Admin calls emergency_undelegate()
        // TODO: Verify undelegation occurred
        // TODO: Verify delegation amount decreased
    }

    #[test]
    #[should_panic]
    fn test_only_admin_can_add_validator() {
        let (mut staking, admin, user) = setup();
        
        let validator = Address::from([10u8; 32]);
        
        // TODO: Mock caller as user (not admin)
        // TODO: Try to add validator
        // Should panic with Unauthorized
    }

    #[test]
    #[should_panic]
    fn test_only_admin_or_operator_can_compound() {
        let (mut staking, admin, user) = setup();
        
        // TODO: Mock caller as regular user
        // TODO: Try to compound rewards
        // Should panic with Unauthorized
    }

    #[test]
    fn test_exchange_rate_increases_with_rewards() {
        let (mut staking, admin, _) = setup();
        
        let initial_rate = staking.get_exchange_rate();
        assert_eq!(initial_rate, U256::from(1_000_000_000u64));
        
        // TODO: Stake initial amount
        // TODO: Simulate rewards accrual
        // TODO: Compound rewards
        // TODO: Get new rate
        // TODO: Verify new_rate > initial_rate
    }

    #[test]
    fn test_multiple_users_fair_share() {
        let (mut staking, admin, _) = setup();
        
        let user1 = Address::from([20u8; 32]);
        let user2 = Address::from([21u8; 32]);
        
        // TODO: User1 stakes 100 CSPR (gets 100 lstCSPR at 1:1)
        // TODO: Rewards compound (rate becomes 1.1:1)
        // TODO: User2 stakes 110 CSPR (gets 100 lstCSPR at 1.1:1)
        // TODO: Both users now have 100 lstCSPR
        // TODO: Both should be able to withdraw equal CSPR
    }
}
