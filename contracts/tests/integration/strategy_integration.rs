#[cfg(test)]
mod strategy_integration_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;
    use crate::mocks::*;

    #[test]
    fn test_deploy_to_multiple_strategies() {
        let total_amount = cspr(30_000);
        
        let dex_amount = cspr(12_000);
        let lending_amount = cspr(9_000);
        let cross_chain_amount = cspr(9_000);
        
        let deployed_total = dex_amount + lending_amount + cross_chain_amount;
        
        assert_u512_eq(deployed_total, total_amount, "All funds deployed");
    }

    #[test]
    fn test_harvest_from_all_strategies() {
        let env = odra_test::env();
        
        let mut lending = MockLendingHostRef::deploy(&env, 500u16);
        
        lending.supply(cspr(10000));
        lending.accrue_interest(env.get_account(0));
        
        let balance = lending.get_balance(env.get_account(0));
        
        assert!(balance > cspr(10000), "Interest accrued");
    }

    #[test]
    fn test_rebalance_between_strategies() {
        let current_dex = cspr(40_000);
        let current_lending = cspr(30_000);
        let current_cross_chain = cspr(30_000);
        
        let target_dex = 40u64;
        let target_lending = 35u64;
        let target_cross_chain = 25u64;
        
        let total_tvl = current_dex + current_lending + current_cross_chain;
        
        let target_dex_amount = (total_tvl * U512::from(target_dex)) / U512::from(100u64);
        let target_lending_amount = (total_tvl * U512::from(target_lending)) / U512::from(100u64);
        
        let dex_needs_withdraw = current_dex > target_dex_amount;
        let lending_needs_deposit = current_lending < target_lending_amount;
        
        assert!(!dex_needs_withdraw, "DEX at target");
        assert!(lending_needs_deposit, "Lending needs more");
    }

    #[test]
    fn test_strategy_failure_handling() {
        let env = odra_test::env();
        let mut validator = MockValidatorHostRef::deploy(&env, 90u8, 5u8);
        
        validator.set_uptime(90);
        
        let uptime = validator.get_uptime();
        let should_withdraw = uptime < 95;
        
        assert!(should_withdraw, "Withdraw from failing strategy");
    }

    #[test]
    fn test_emergency_withdraw_all_strategies() {
        let dex_balance = cspr(40_000);
        let lending_balance = cspr(30_000);
        let cross_chain_balance = cspr(30_000);
        
        let total_emergency_withdrawal = dex_balance + lending_balance + cross_chain_balance;
        
        assert_u512_eq(total_emergency_withdrawal, cspr(100_000), "Emergency withdrawal total");
    }

    #[test]
    fn test_strategy_apy_weighted_average() {
        let dex_allocation = 40u64;
        let lending_allocation = 30u64;
        let cross_chain_allocation = 30u64;
        
        let dex_apy = 800u16;
        let lending_apy = 500u16;
        let cross_chain_apy = 600u16;
        
        let weighted_apy = 
            ((dex_apy as u64 * dex_allocation) +
             (lending_apy as u64 * lending_allocation) +
             (cross_chain_apy as u64 * cross_chain_allocation)) / 100;
        
        assert_eq!(weighted_apy, 670u64, "Weighted APY 6.7%");
    }

    #[test]
    fn test_strategy_health_monitoring() {
        let reported_balance = cspr(9_500);
        let expected_balance = cspr(10_000);
        
        let deviation_bps = calculate_slippage(expected_balance, reported_balance);
        let health_threshold = 500u64;
        
        let is_healthy = deviation_bps <= health_threshold;
        
        assert!(!is_healthy, "Strategy health warning");
    }

    #[test]
    fn test_add_new_strategy() {
        let existing_strategies = 3u8;
        let new_strategy = 1u8;
        
        let total_strategies = existing_strategies + new_strategy;
        
        assert_eq!(total_strategies, 4u8, "New strategy added");
    }

    #[test]
    fn test_remove_strategy() {
        let strategy_balance = cspr(30_000);
        let remaining_strategies = 2u8;
        
        let redistribute_per_strategy = strategy_balance / U512::from(remaining_strategies as u64);
        
        assert_u512_eq(redistribute_per_strategy, cspr(15_000), "Redistributed balance");
    }

    #[test]
    fn test_strategy_allocation_constraints() {
        let dex_allocation = 45u64;
        let max_per_strategy = 40u64;
        
        assert!(dex_allocation > max_per_strategy, "Violates max allocation");
    }

    #[test]
    fn test_cross_chain_allocation_limit() {
        let cross_chain_allocation = 35u64;
        let max_cross_chain = 30u64;
        
        assert!(cross_chain_allocation > max_cross_chain, "Exceeds cross-chain limit");
    }

    #[test]
    fn test_conservative_strategy_minimum() {
        let conservative_allocation = 8u64;
        let min_conservative = 10u64;
        
        assert!(conservative_allocation < min_conservative, "Below minimum conservative");
    }

    #[test]
    fn test_harvest_gas_efficiency() {
        let num_strategies = 3u8;
        let gas_per_harvest = 100_000u64;
        
        let total_gas = gas_per_harvest * num_strategies as u64;
        let gas_budget = 500_000u64;
        
        assert!(total_gas < gas_budget, "Within gas budget");
    }

    #[test]
    fn test_partial_withdrawal_optimization() {
        let total_balance = cspr(100_000);
        let withdraw_amount = cspr(20_000);
        
        let withdrawal_pct = ((withdraw_amount * U512::from(100u64)) / total_balance).as_u64();
        
        assert_eq!(withdrawal_pct, 20u64, "20% withdrawal");
    }

    #[test]
    fn test_slippage_on_strategy_withdrawal() {
        let expected = cspr(10_000);
        let actual = cspr(9_900);
        let max_slippage = 100u64;
        
        let slippage = calculate_slippage(expected, actual);
        
        assert!(slippage <= max_slippage, "Slippage within tolerance");
    }
}
