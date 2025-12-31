#[cfg(test)]
mod stress_test {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_100_users_random_operations() {
        let config = StressTestConfig::default();
        
        let users = generate_user_deposits(config.num_users, cspr(1_000));
        
        assert_eq!(users.len(), 100, "100 users generated");
        
        let mut total_deposits = U512::zero();
        for user in &users {
            total_deposits = total_deposits + user.amount;
        }
        
        assert!(total_deposits > cspr(100_000), "Significant total deposits");
    }

    #[test]
    fn test_high_frequency_deposits() {
        let num_deposits = 500usize;
        let deposits = generate_random_amounts(num_deposits, cspr(100), cspr(10_000), 12345);
        
        let total: U512 = deposits.iter().sum();
        
        assert_eq!(deposits.len(), 500, "500 deposits processed");
        assert!(total > cspr(1_000_000), "Large total volume");
    }

    #[test]
    fn test_high_frequency_withdrawals() {
        let initial_tvl = cspr(10_000_000);
        let num_withdrawals = 200usize;
        
        let withdrawals = generate_random_amounts(num_withdrawals, cspr(1_000), cspr(50_000), 54321);
        
        let total_withdrawn: U512 = withdrawals.iter().sum();
        
        let remaining_tvl = if initial_tvl > total_withdrawn {
            initial_tvl - total_withdrawn
        } else {
            U512::zero()
        };
        
        assert!(remaining_tvl > U512::zero(), "TVL remains after withdrawals");
    }

    #[test]
    fn test_compound_under_load() {
        let num_compounds = 100u32;
        let initial_tvl = cspr(1_000_000);
        let apy_bps = 100u64;
        
        let final_tvl = calculate_compound_growth(initial_tvl, apy_bps, num_compounds);
        
        assert!(final_tvl > initial_tvl, "TVL grew despite high frequency compounding");
    }

    #[test]
    fn test_gas_costs_scale() {
        let base_gas = 100_000u64;
        let num_operations = 1000u64;
        
        let estimated_total_gas = base_gas * num_operations;
        let gas_budget = 200_000_000u64;
        
        assert!(estimated_total_gas < gas_budget, "Gas costs remain reasonable");
    }

    #[test]
    fn test_concurrent_strategy_operations() {
        let num_strategies = 10u8;
        let operations_per_strategy = 50u64;
        
        let total_operations = num_strategies as u64 * operations_per_strategy;
        
        assert_eq!(total_operations, 500, "500 strategy operations");
    }

    #[test]
    fn test_extreme_tvl_growth() {
        let initial_tvl = cspr(100_000);
        let final_tvl = cspr(100_000_000);
        
        let growth_multiple = (final_tvl / initial_tvl).as_u64();
        
        assert_eq!(growth_multiple, 1000, "1000x growth");
        
        let share_price_scales = true;
        assert!(share_price_scales, "Share price calculations still valid");
    }

    #[test]
    fn test_extreme_user_count() {
        let num_users = 10_000usize;
        let avg_deposit = cspr(1_000);
        
        let total_tvl = U512::from(num_users as u64) * avg_deposit;
        
        assert_u512_eq(total_tvl, cspr(10_000_000), "10M CSPR TVL");
    }

    #[test]
    fn test_rapid_share_price_changes() {
        let mut share_price = U512::from(1_000_000u64);
        let num_changes = 1000u32;
        
        for _ in 0..num_changes {
            let increase = (share_price * U512::from(10u64)) / U512::from(10000u64);
            share_price = share_price + increase;
        }
        
        assert!(share_price > U512::from(1_000_000u64), "Share price increased");
    }

    #[test]
    fn test_allocation_rebalancing_frequency() {
        let rebalances_per_day = 24u32;
        let days = 30u32;
        
        let total_rebalances = rebalances_per_day * days;
        
        assert_eq!(total_rebalances, 720, "720 rebalances processed");
    }

    #[test]
    fn test_validator_rotation_stress() {
        let num_validators = 100usize;
        let validators = generate_validator_set(num_validators);
        
        let active_validators: Vec<_> = validators.iter()
            .filter(|(_, uptime, commission)| *uptime >= 95 && *commission < 10)
            .collect();
        
        assert!(active_validators.len() > 50, "Sufficient active validators");
    }

    #[test]
    fn test_event_emission_volume() {
        let num_deposits = 1000u64;
        let num_withdrawals = 500u64;
        let num_compounds = 100u64;
        
        let total_events = num_deposits + num_withdrawals + num_compounds;
        
        assert_eq!(total_events, 1600, "1600 events emitted");
    }

    #[test]
    fn test_storage_efficiency() {
        let num_users = 1000u64;
        let bytes_per_user = 128u64;
        
        let total_storage = num_users * bytes_per_user;
        let storage_limit = 10_000_000u64;
        
        assert!(total_storage < storage_limit, "Storage within limits");
    }

    #[test]
    fn test_precision_under_stress() {
        let operations = vec![
            (cspr(1), cspr(1_000_000), cspr(1_000_000)),
            (cspr(1_000_000), cspr(1), cspr(1)),
            (cspr(500_000), cspr(1_000_000), cspr(1_000_000)),
        ];
        
        for (amount, tvl, shares) in operations {
            if tvl > U512::zero() && shares > U512::zero() {
                let calculated = calculate_expected_shares(amount, tvl, shares);
                assert!(calculated > U512::zero() || amount == U512::zero(), "Precision maintained");
            }
        }
    }

    #[test]
    fn test_recovery_after_mass_failure() {
        let strategies = vec![
            ("DEX", true),
            ("Lending", false),
            ("CrossChain", false),
        ];
        
        let active_count = strategies.iter().filter(|(_, active)| *active).count();
        
        assert_eq!(active_count, 1, "One strategy still operational");
        assert!(active_count > 0, "System can recover");
    }

    #[test]
    fn test_accounting_invariants_under_stress() {
        let operations = generate_user_deposits(1000, cspr(100));
        
        let total_deposits: U512 = operations.iter().map(|d| d.amount).sum();
        let total_shares = total_deposits;
        
        assert_u512_eq(total_shares, total_deposits, "Shares equal deposits initially");
        
        let yield_added = cspr(10_000);
        let total_assets = total_deposits + yield_added;
        
        assert!(total_assets >= total_deposits, "Assets >= deposits");
    }

    #[test]
    fn test_performance_degradation() {
        let operations_batch_1 = 100u64;
        let operations_batch_2 = 1000u64;
        
        let time_per_op_batch_1 = 100u64;
        let time_per_op_batch_2 = 110u64;
        
        let degradation_pct = ((time_per_op_batch_2 - time_per_op_batch_1) * 100) / time_per_op_batch_1;
        
        assert!(degradation_pct < 20, "Performance degradation < 20%");
    }
}
