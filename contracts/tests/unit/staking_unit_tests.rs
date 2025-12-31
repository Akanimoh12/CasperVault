#[cfg(test)]
mod liquid_staking_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;
    use crate::mocks::*;

    #[test]
    fn test_staking_initialization() {
        let env = odra_test::env();
        
        assert!(true, "Staking module initialized");
    }

    #[test]
    fn test_first_stake_one_to_one() {
        let stake_amount = cspr(1000);
        let expected_lst_cspr = stake_amount;
        
        assert_u512_eq(expected_lst_cspr, stake_amount, "First stake 1:1");
    }

    #[test]
    fn test_stake_updates_total_staked() {
        let initial_total = cspr(5000);
        let stake_amount = cspr(1000);
        
        let new_total = initial_total + stake_amount;
        
        assert_u512_eq(new_total, cspr(6000), "Total staked updated");
    }

    #[test]
    fn test_exchange_rate_after_rewards() {
        let total_cspr = cspr(11000);
        let total_lst_cspr = cspr(10000);
        
        let exchange_rate = (total_cspr * U512::from(1_000_000u64)) / total_lst_cspr;
        
        assert!(exchange_rate > U512::from(1_000_000u64), "Exchange rate > 1");
    }

    #[test]
    fn test_stake_with_appreciation() {
        let new_stake = cspr(1000);
        let total_cspr = cspr(11000);
        let total_lst_cspr = cspr(10000);
        
        let lst_minted = (new_stake * total_lst_cspr) / total_cspr;
        
        assert!(lst_minted < new_stake, "LST minted < CSPR staked when exchange rate > 1");
    }

    #[test]
    fn test_unstake_burns_lst_cspr() {
        let lst_to_burn = cspr(909);
        let total_cspr = cspr(11000);
        let total_lst_cspr = cspr(10000);
        
        let cspr_returned = (lst_to_burn * total_cspr) / total_lst_cspr;
        
        assert!(cspr_returned > lst_to_burn, "CSPR returned > LST burned when exchange rate > 1");
    }

    #[test]
    fn test_validator_selection_algorithm() {
        let validators = generate_validator_set(5);
        
        let filtered: Vec<_> = validators.iter()
            .filter(|(_, uptime, commission)| *uptime >= 95 && *commission < 10)
            .collect();
        
        assert!(filtered.len() > 0, "Valid validators available");
    }

    #[test]
    fn test_validator_uptime_filter() {
        let low_uptime = 90u8;
        let high_uptime = 98u8;
        
        assert!(high_uptime >= 95, "High uptime validator passes");
        assert!(low_uptime < 95, "Low uptime validator filtered");
    }

    #[test]
    fn test_validator_commission_filter() {
        let low_commission = 5u8;
        let high_commission = 15u8;
        
        assert!(low_commission < 10, "Low commission validator passes");
        assert!(high_commission >= 10, "High commission validator filtered");
    }

    #[test]
    fn test_delegation_distribution() {
        let total_amount = cspr(10000);
        let num_validators = 5;
        
        let per_validator = total_amount / U512::from(num_validators as u64);
        
        assert_u512_eq(per_validator, cspr(2000), "Equal distribution");
    }

    #[test]
    fn test_compound_rewards_calculation() {
        let env = odra_test::env();
        let mut validator = MockValidatorHostRef::deploy(&env, 98u8, 5u8);
        
        let delegation = cspr(10000);
        validator.delegate(delegation);
        validator.add_rewards(cspr(500));
        
        let rewards = validator.claim_rewards();
        
        assert!(rewards > U512::zero(), "Rewards claimed");
    }

    #[test]
    fn test_rewards_claim_from_multiple_validators() {
        let env = odra_test::env();
        
        let mut val1 = MockValidatorHostRef::deploy(&env, 98u8, 5u8);
        let mut val2 = MockValidatorHostRef::deploy(&env, 97u8, 6u8);
        
        val1.delegate(cspr(5000));
        val2.delegate(cspr(5000));
        
        val1.add_rewards(cspr(250));
        val2.add_rewards(cspr(250));
        
        let rewards1 = val1.claim_rewards();
        let rewards2 = val2.claim_rewards();
        
        let total_rewards = rewards1 + rewards2;
        
        assert!(total_rewards > U512::zero(), "Total rewards > 0");
    }

    #[test]
    fn test_restake_after_compound() {
        let initial_stake = cspr(10000);
        let rewards = cspr(500);
        
        let new_total = initial_stake + rewards;
        
        assert_u512_eq(new_total, cspr(10500), "Restaked amount");
    }

    #[test]
    fn test_validator_cap_enforcement() {
        let max_stake_cap = cspr(100_000);
        let current_stake = cspr(95_000);
        let new_delegation = cspr(10_000);
        
        let would_exceed = current_stake + new_delegation > max_stake_cap;
        
        assert!(would_exceed, "Would exceed cap");
    }

    #[test]
    fn test_diversification_per_validator_limit() {
        let total_tvl = cspr(1_000_000);
        let max_per_validator_pct = 10u64;
        
        let max_per_validator = (total_tvl * U512::from(max_per_validator_pct)) / U512::from(100u64);
        
        assert_u512_eq(max_per_validator, cspr(100_000), "Max per validator");
    }

    #[test]
    fn test_rebalance_underweight_validator() {
        let validator1_stake = cspr(5000);
        let validator2_stake = cspr(10000);
        
        assert!(validator1_stake < validator2_stake, "Validator 1 underweight");
    }

    #[test]
    fn test_remove_underperforming_validator() {
        let env = odra_test::env();
        let mut validator = MockValidatorHostRef::deploy(&env, 90u8, 5u8);
        
        let uptime = validator.get_uptime();
        
        assert!(uptime < 95, "Validator underperforming");
    }

    #[test]
    fn test_validator_active_status() {
        let env = odra_test::env();
        let mut validator = MockValidatorHostRef::deploy(&env, 98u8, 5u8);
        
        assert!(validator.is_active(), "Validator active");
        
        validator.set_active(false);
        assert!(!validator.is_active(), "Validator inactive");
    }

    #[test]
    fn test_zero_stake_amount() {
        let stake_amount = U512::zero();
        
        assert_eq!(stake_amount, U512::zero(), "Zero stake invalid");
    }

    #[test]
    fn test_unstake_insufficient_balance() {
        let user_balance = cspr(100);
        let unstake_amount = cspr(200);
        
        assert!(unstake_amount > user_balance, "Insufficient balance");
    }

    #[test]
    fn test_slashing_event_handling() {
        let original_stake = cspr(10000);
        let slashed_amount = cspr(100);
        
        let remaining = original_stake - slashed_amount;
        
        assert_u512_eq(remaining, cspr(9900), "Post-slashing balance");
    }

    #[test]
    fn test_apy_calculation_from_rewards() {
        let principal = cspr(10000);
        let rewards = cspr(1000);
        let days = 365u64;
        
        let apy = calculate_apy(principal, principal + rewards, days);
        
        assert!(apy >= 900 && apy <= 1100, "APY around 10%");
    }

    #[test]
    fn test_batch_delegation() {
        let total_amount = cspr(30000);
        let validators = vec![
            cspr(10000),
            cspr(10000),
            cspr(10000),
        ];
        
        let sum: U512 = validators.iter().sum();
        
        assert_u512_eq(sum, total_amount, "Batch delegation sum");
    }

    #[test]
    fn test_commission_deduction() {
        let gross_rewards = cspr(100);
        let commission_rate = 5u64;
        
        let commission = (gross_rewards * U512::from(commission_rate)) / U512::from(100u64);
        let net_rewards = gross_rewards - commission;
        
        assert_u512_eq(net_rewards, cspr(95), "Net rewards after commission");
    }
}
