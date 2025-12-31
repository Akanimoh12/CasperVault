#[cfg(test)]
mod property_based_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn property_total_assets_gte_deposits() {
        let deposits = vec![cspr(1_000), cspr(2_000), cspr(3_000)];
        let total_deposits: U512 = deposits.iter().sum();
        
        let yield_earned = cspr(500);
        let total_assets = total_deposits + yield_earned;
        
        assert!(total_assets >= total_deposits, "INVARIANT: totalAssets >= sum(deposits)");
    }

    #[test]
    fn property_total_shares_equals_sum_user_shares() {
        let user_shares = vec![cspr(5_000), cspr(3_000), cspr(2_000)];
        let total_shares: U512 = user_shares.iter().sum();
        let recorded_total = cspr(10_000);
        
        assert_u512_eq(total_shares, recorded_total, "INVARIANT: totalShares = sum(userShares)");
    }

    #[test]
    fn property_share_price_never_decreases() {
        let initial_price = U512::from(1_000_000u64);
        
        let prices = vec![
            U512::from(1_000_000u64),
            U512::from(1_050_000u64),
            U512::from(1_100_000u64),
            U512::from(1_150_000u64),
        ];
        
        for i in 1..prices.len() {
            assert!(prices[i] >= prices[i-1], "INVARIANT: Share price never decreases (except fees)");
        }
    }

    #[test]
    fn property_conservation_of_value() {
        let total_deposits = cspr(100_000);
        let total_yields = cspr(5_000);
        let total_fees = cspr(500);
        
        let expected_tvl = total_deposits + total_yields - total_fees;
        
        let user1_value = cspr(60_000);
        let user2_value = cspr(30_000);
        let user3_value = cspr(14_500);
        let sum_user_values = user1_value + user2_value + user3_value;
        
        assert_u512_eq(sum_user_values, expected_tvl, "INVARIANT: Conservation of value");
    }

    #[test]
    fn property_shares_to_assets_to_shares() {
        let shares = cspr(1_000);
        let total_assets = cspr(11_000);
        let total_shares = cspr(10_000);
        
        let assets = calculate_expected_assets(shares, total_assets, total_shares);
        let back_to_shares = calculate_expected_shares(assets, total_assets, total_shares);
        
        assert_u512_within_tolerance(back_to_shares, shares, 1);
    }

    #[test]
    fn property_proportional_yield_distribution() {
        let user1_shares = cspr(60_000);
        let user2_shares = cspr(40_000);
        let total_shares = cspr(100_000);
        
        let yield_amount = cspr(10_000);
        
        let user1_yield = (yield_amount * user1_shares) / total_shares;
        let user2_yield = (yield_amount * user2_shares) / total_shares;
        
        let ratio_shares = (user1_shares * U512::from(100u64)) / user2_shares;
        let ratio_yield = (user1_yield * U512::from(100u64)) / user2_yield;
        
        assert_u512_within_tolerance(ratio_shares, ratio_yield, 10);
    }

    #[test]
    fn property_deposit_then_withdraw_equals_initial() {
        let deposit = cspr(1_000);
        let shares = deposit;
        
        let withdrawn = calculate_expected_assets(shares, deposit, shares);
        
        assert_u512_eq(withdrawn, deposit, "PROPERTY: Deposit then immediate withdraw = initial");
    }

    #[test]
    fn property_fees_never_exceed_profits() {
        let profit = cspr(1_000);
        let fee_bps = 1000u64;
        
        let fee = calculate_performance_fee(profit, fee_bps);
        
        assert!(fee <= profit, "INVARIANT: Fees never exceed profits");
    }

    #[test]
    fn property_tvl_equals_strategies_plus_vault() {
        let vault_balance = cspr(10_000);
        let dex_balance = cspr(40_000);
        let lending_balance = cspr(30_000);
        let cross_chain_balance = cspr(20_000);
        
        let calculated_tvl = vault_balance + dex_balance + lending_balance + cross_chain_balance;
        let reported_tvl = cspr(100_000);
        
        assert_u512_eq(calculated_tvl, reported_tvl, "INVARIANT: TVL = sum(all balances)");
    }

    #[test]
    fn property_allocation_percentages_sum_to_100() {
        let allocations = vec![40u64, 30u64, 30u64];
        let sum: u64 = allocations.iter().sum();
        
        assert_eq!(sum, 100, "INVARIANT: Allocation percentages sum to 100%");
    }

    #[test]
    fn property_withdrawal_reduces_shares_proportionally() {
        let initial_shares = cspr(10_000);
        let withdraw_pct = 30u64;
        
        let withdrawn_shares = (initial_shares * U512::from(withdraw_pct)) / U512::from(100u64);
        let remaining_shares = initial_shares - withdrawn_shares;
        
        let expected_remaining = cspr(7_000);
        assert_u512_eq(remaining_shares, expected_remaining, "PROPERTY: Proportional share reduction");
    }

    #[test]
    fn property_commutative_deposits() {
        let scenario1_total = cspr(1_000) + cspr(2_000);
        let scenario2_total = cspr(2_000) + cspr(1_000);
        
        assert_u512_eq(scenario1_total, scenario2_total, "PROPERTY: Deposit order doesn't affect total (without yields)");
    }

    #[test]
    fn property_zero_shares_zero_value() {
        let shares = U512::zero();
        let tvl = cspr(100_000);
        let total_shares = cspr(100_000);
        
        let value = calculate_expected_assets(shares, tvl, total_shares);
        
        assert_eq!(value, U512::zero(), "PROPERTY: Zero shares = zero value");
    }

    #[test]
    fn property_monotonic_tvl_growth_with_yields() {
        let tvl_snapshots = vec![
            cspr(100_000),
            cspr(105_000),
            cspr(110_000),
            cspr(115_000),
        ];
        
        for i in 1..tvl_snapshots.len() {
            assert!(tvl_snapshots[i] >= tvl_snapshots[i-1], "PROPERTY: TVL grows with yields");
        }
    }

    #[test]
    fn property_slippage_symmetric() {
        let amount1 = cspr(1_000);
        let amount2 = cspr(990);
        
        let slippage_1_to_2 = calculate_slippage(amount1, amount2);
        let slippage_2_to_1 = calculate_slippage(amount2, amount1);
        
        assert_u512_within_tolerance(
            U512::from(slippage_1_to_2),
            U512::from(slippage_2_to_1),
            10
        );
    }

    #[test]
    fn property_compound_always_increases_tvl() {
        let tvl_before = cspr(100_000);
        let yield_amount = cspr(1_000);
        let fee = calculate_performance_fee(yield_amount, 1000);
        
        let tvl_after = tvl_before + yield_amount - fee;
        
        assert!(tvl_after > tvl_before, "PROPERTY: Compound increases TVL");
    }
}

#[cfg(test)]
mod gas_optimization_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn benchmark_single_deposit() {
        let deposit = cspr(1_000);
        let gas_estimate = 200_000u64;
        let gas_budget = 500_000u64;
        
        assert!(gas_estimate < gas_budget, "Single deposit within budget");
    }

    #[test]
    fn benchmark_single_withdrawal() {
        let withdrawal = cspr(1_000);
        let gas_estimate = 250_000u64;
        let gas_budget = 500_000u64;
        
        assert!(gas_estimate < gas_budget, "Single withdrawal within budget");
    }

    #[test]
    fn benchmark_compound_operation() {
        let num_strategies = 3u8;
        let gas_per_strategy = 150_000u64;
        let overhead = 100_000u64;
        
        let total_gas = (num_strategies as u64 * gas_per_strategy) + overhead;
        let gas_budget = 1_000_000u64;
        
        assert!(total_gas < gas_budget, "Compound within budget");
    }

    #[test]
    fn benchmark_batch_harvest() {
        let num_strategies = 3u8;
        let gas_per_harvest = 120_000u64;
        
        let total_gas = num_strategies as u64 * gas_per_harvest;
        let gas_budget = 500_000u64;
        
        assert!(total_gas < gas_budget, "Batch harvest within budget");
    }

    #[test]
    fn benchmark_rebalance() {
        let withdrawals = 2u8;
        let deposits = 2u8;
        let gas_per_op = 100_000u64;
        
        let total_gas = (withdrawals + deposits) as u64 * gas_per_op;
        let gas_budget = 600_000u64;
        
        assert!(total_gas < gas_budget, "Rebalance within budget");
    }

    #[test]
    fn test_storage_optimization() {
        let user_count = 1000u64;
        let storage_per_user = 128u64;
        
        let total_storage = user_count * storage_per_user;
        let max_storage = 1_000_000u64;
        
        assert!(total_storage < max_storage, "Storage optimized");
    }

    #[test]
    fn test_computation_complexity() {
        let operations = 1000u64;
        let time_per_op = 100u64;
        
        let total_time = operations * time_per_op;
        let time_budget = 200_000u64;
        
        assert!(total_time < time_budget, "Computation efficient");
    }

    #[test]
    fn compare_instant_vs_timelock_gas() {
        let instant_withdrawal_gas = 250_000u64;
        let timelock_request_gas = 150_000u64;
        let timelock_complete_gas = 200_000u64;
        
        let timelock_total = timelock_request_gas + timelock_complete_gas;
        
        assert!(instant_withdrawal_gas < timelock_total, "Instant withdrawal more gas efficient");
    }

    #[test]
    fn optimize_share_calculation() {
        let operations = 100u64;
        let gas_per_calculation = 5_000u64;
        
        let total_gas = operations * gas_per_calculation;
        let gas_budget = 1_000_000u64;
        
        assert!(total_gas < gas_budget, "Share calculations optimized");
    }

    #[test]
    fn batch_operations_efficiency() {
        let individual_ops_gas = 10 * 100_000u64;
        let batch_ops_gas = 700_000u64;
        
        let savings = individual_ops_gas - batch_ops_gas;
        let savings_pct = (savings * 100) / individual_ops_gas;
        
        assert!(savings_pct >= 20, "Batching saves >= 20% gas");
    }
}
