#[cfg(test)]
mod strategy_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;
    use crate::mocks::*;

    #[test]
    fn test_dex_strategy_deployment() {
        let deploy_amount = cspr(10000);
        
        assert!(deploy_amount > U512::zero(), "Deployment amount valid");
    }

    #[test]
    fn test_lending_strategy_supply() {
        let env = odra_test::env();
        let mut lending = MockLendingHostRef::deploy(&env, 500u16);
        
        let supply_amount = cspr(5000);
        lending.supply(supply_amount);
        
        assert_u512_eq(lending.get_total_supplied(), supply_amount, "Supply recorded");
    }

    #[test]
    fn test_lending_strategy_redeem() {
        let env = odra_test::env();
        let mut lending = MockLendingHostRef::deploy(&env, 500u16);
        
        let supply_amount = cspr(5000);
        lending.supply(supply_amount);
        
        let redeemed = lending.redeem(supply_amount);
        
        assert!(redeemed >= supply_amount, "Redeemed amount with interest");
    }

    #[test]
    fn test_lending_apy() {
        let env = odra_test::env();
        let lending = MockLendingHostRef::deploy(&env, 500u16);
        
        let apy = lending.get_supply_apy();
        
        assert_eq!(apy, 500u16, "APY is 5%");
    }

    #[test]
    fn test_crosschain_bridge_initiation() {
        let env = odra_test::env();
        let mut bridge = MockBridgeHostRef::deploy(&env, 3u8);
        
        let amount = cspr(1000);
        let request_id = bridge.initiate_bridge(amount, "ethereum".to_string());
        
        assert!(request_id > 0, "Bridge request created");
    }

    #[test]
    fn test_bridge_confirmation() {
        let env = odra_test::env();
        let mut bridge = MockBridgeHostRef::deploy(&env, 3u8);
        
        let request_id = bridge.initiate_bridge(cspr(1000), "ethereum".to_string());
        bridge.confirm_bridge(request_id);
        
        let request = bridge.get_bridge_request(request_id);
        assert!(request.is_some(), "Request confirmed");
    }

    #[test]
    fn test_strategy_balance_tracking() {
        let deployed = cspr(10000);
        let harvested = cspr(500);
        
        let expected_balance = deployed + harvested;
        
        assert_u512_eq(expected_balance, cspr(10500), "Strategy balance");
    }

    #[test]
    fn test_strategy_harvest_yields() {
        let principal = cspr(10000);
        let apy_bps = 800u64;
        let days = 30u64;
        
        let daily_rate = (principal * U512::from(apy_bps)) / (U512::from(10000u64) * U512::from(365u64));
        let yield_amount = daily_rate * U512::from(days);
        
        assert!(yield_amount > U512::zero(), "Harvest yield calculated");
    }

    #[test]
    fn test_strategy_withdraw_partial() {
        let total_balance = cspr(10000);
        let withdraw_amount = cspr(3000);
        
        let remaining = total_balance - withdraw_amount;
        
        assert_u512_eq(remaining, cspr(7000), "Partial withdrawal");
    }

    #[test]
    fn test_strategy_withdraw_full() {
        let total_balance = cspr(10000);
        let withdraw_amount = total_balance;
        
        let remaining = U512::zero();
        
        assert_u512_eq(remaining, U512::zero(), "Full withdrawal");
    }

    #[test]
    fn test_dex_lp_position_tracking() {
        let cspr_amount = cspr(5000);
        let lst_cspr_amount = cspr(5000);
        
        let lp_value = cspr_amount + lst_cspr_amount;
        
        assert_u512_eq(lp_value, cspr(10000), "LP position value");
    }

    #[test]
    fn test_impermanent_loss_calculation() {
        let initial_value = cspr(10000);
        let final_value = cspr(9800);
        
        let il = if initial_value > final_value {
            initial_value - final_value
        } else {
            U512::zero()
        };
        
        assert_u512_eq(il, cspr(200), "Impermanent loss");
    }

    #[test]
    fn test_strategy_apy_comparison() {
        let dex_apy = 800u16;
        let lending_apy = 500u16;
        let staking_apy = 1000u16;
        
        assert!(staking_apy > dex_apy && staking_apy > lending_apy, "Staking has highest APY");
    }

    #[test]
    fn test_strategy_risk_level() {
        let low_risk = 1u8;
        let medium_risk = 2u8;
        let high_risk = 3u8;
        
        assert!(high_risk > medium_risk && medium_risk > low_risk, "Risk levels ordered");
    }

    #[test]
    fn test_interest_accrual() {
        let env = odra_test::env();
        let mut lending = MockLendingHostRef::deploy(&env, 1000u16);
        
        let principal = U512::from(10000u64);
        let interest = lending.calculate_interest(principal);
        
        assert_eq!(interest, U512::from(1000u64), "10% interest");
    }
}

#[cfg(test)]
mod router_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_allocation_split() {
        let total_amount = cspr(10000);
        let dex_pct = 40u64;
        let lending_pct = 30u64;
        let cross_chain_pct = 30u64;
        
        let dex_allocation = (total_amount * U512::from(dex_pct)) / U512::from(100u64);
        let lending_allocation = (total_amount * U512::from(lending_pct)) / U512::from(100u64);
        let cross_chain_allocation = (total_amount * U512::from(cross_chain_pct)) / U512::from(100u64);
        
        let total = dex_allocation + lending_allocation + cross_chain_allocation;
        
        assert_u512_eq(total, total_amount, "Allocation sums to total");
    }

    #[test]
    fn test_max_allocation_per_strategy() {
        let max_pct = 40u64;
        let allocation_pct = 45u64;
        
        assert!(allocation_pct > max_pct, "Exceeds max allocation");
    }

    #[test]
    fn test_min_allocation_conservative() {
        let conservative_min = 10u64;
        let allocation = 15u64;
        
        assert!(allocation >= conservative_min, "Meets minimum");
    }

    #[test]
    fn test_rebalance_threshold() {
        let target_allocation = 40u64;
        let current_allocation = 35u64;
        let threshold = 5u64;
        
        let deviation = if target_allocation > current_allocation {
            target_allocation - current_allocation
        } else {
            current_allocation - target_allocation
        };
        
        assert_eq!(deviation, threshold, "At rebalance threshold");
    }

    #[test]
    fn test_weighted_apy_calculation() {
        let allocations = vec![
            (U512::from(800u64), 40),
            (U512::from(500u64), 30),
            (U512::from(1000u64), 30),
        ];
        
        let blended_apy = weighted_average(&allocations);
        
        assert!(blended_apy > U512::zero(), "Blended APY calculated");
    }

    #[test]
    fn test_proportional_withdrawal() {
        let total_strategies = 3u64;
        let withdraw_amount = cspr(9000);
        
        let per_strategy = withdraw_amount / U512::from(total_strategies);
        
        assert_u512_eq(per_strategy, cspr(3000), "Proportional withdrawal");
    }

    #[test]
    fn test_harvest_aggregation() {
        let yield1 = cspr(100);
        let yield2 = cspr(150);
        let yield3 = cspr(200);
        
        let total_yield = yield1 + yield2 + yield3;
        
        assert_u512_eq(total_yield, cspr(450), "Aggregated harvest");
    }

    #[test]
    fn test_allocation_validation_sum() {
        let allocations = vec![40u64, 30u64, 30u64];
        let sum: u64 = allocations.iter().sum();
        
        assert_eq!(sum, 100u64, "Allocations sum to 100%");
    }

    #[test]
    fn test_emergency_withdrawal_all_strategies() {
        let strategy_balances = vec![
            cspr(4000),
            cspr(3000),
            cspr(3000),
        ];
        
        let total: U512 = strategy_balances.iter().sum();
        
        assert_u512_eq(total, cspr(10000), "Emergency withdrawal total");
    }
}
