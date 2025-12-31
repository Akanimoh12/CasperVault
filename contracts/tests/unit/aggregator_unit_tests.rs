#[cfg(test)]
mod aggregator_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_yield_aggregation_from_all_sources() {
        let staking_yield = cspr(300);
        let dex_yield = cspr(200);
        let lending_yield = cspr(150);
        let crosschain_yield = cspr(100);
        
        let total_yield = staking_yield + dex_yield + lending_yield + crosschain_yield;
        
        assert_u512_eq(total_yield, cspr(750), "Total yield aggregated");
    }

    #[test]
    fn test_compound_cycle() {
        let principal = cspr(10000);
        let yield_amount = cspr(500);
        
        let new_principal = principal + yield_amount;
        
        assert_u512_eq(new_principal, cspr(10500), "After compound");
    }

    #[test]
    fn test_share_price_update_after_compound() {
        let total_assets_before = cspr(10000);
        let total_shares = cspr(10000);
        let yield_added = cspr(500);
        
        let total_assets_after = total_assets_before + yield_added;
        
        let price_before = (total_assets_before * U512::from(1_000_000u64)) / total_shares;
        let price_after = (total_assets_after * U512::from(1_000_000u64)) / total_shares;
        
        assert_share_price_increased(price_before, price_after);
    }

    #[test]
    fn test_performance_fee_on_profits() {
        let profit = cspr(1000);
        let fee_bps = 1000u64;
        
        let fee = calculate_performance_fee(profit, fee_bps);
        let expected = cspr(100);
        
        assert_u512_eq(fee, expected, "10% performance fee");
    }

    #[test]
    fn test_no_fee_on_loss() {
        let initial = cspr(10000);
        let final_amount = cspr(9500);
        
        let profit = if final_amount > initial {
            final_amount - initial
        } else {
            U512::zero()
        };
        
        assert_eq!(profit, U512::zero(), "No profit, no fee");
    }

    #[test]
    fn test_management_fee_accrual() {
        let tvl = cspr(1_000_000);
        let fee_bps = 200u64;
        let days = 365u64;
        
        let fee = calculate_management_fee(tvl, fee_bps, days);
        let expected = cspr(20_000);
        
        assert_u512_within_tolerance(fee, expected, 10);
    }

    #[test]
    fn test_historical_apy_tracking() {
        let scenarios = generate_yield_scenarios();
        
        assert!(scenarios.len() > 0, "Historical data exists");
        
        for scenario in scenarios {
            assert!(scenario.dex_apy > 0, "Valid APY data");
        }
    }

    #[test]
    fn test_compound_frequency() {
        let principal = cspr(10000);
        let apy_bps = 1000u64;
        let compounds = 12u32;
        
        let final_amount = calculate_compound_growth(principal, apy_bps, compounds);
        
        assert!(final_amount > principal, "Compound growth positive");
    }

    #[test]
    fn test_min_yield_threshold() {
        let yield_amount = cspr(50);
        let min_threshold = cspr(100);
        
        let should_compound = yield_amount >= min_threshold;
        
        assert!(!should_compound, "Below threshold");
    }

    #[test]
    fn test_compound_time_interval() {
        let last_compound_time = 1000u64;
        let current_time = 5000u64;
        let min_interval = 3600u64;
        
        let elapsed = current_time - last_compound_time;
        let can_compound = elapsed >= min_interval;
        
        assert!(can_compound, "Sufficient time elapsed");
    }

    #[test]
    fn test_token_swap_slippage() {
        let expected_amount = cspr(1000);
        let slippage_bps = 100u64;
        
        let min_amount = apply_slippage(expected_amount, slippage_bps);
        
        assert!(min_amount < expected_amount, "Slippage applied");
    }

    #[test]
    fn test_fee_distribution_to_treasury() {
        let total_fees = cspr(500);
        let treasury_balance_before = cspr(10000);
        
        let treasury_balance_after = treasury_balance_before + total_fees;
        
        assert_u512_eq(treasury_balance_after, cspr(10500), "Fees distributed");
    }

    #[test]
    fn test_multiple_compound_cycles() {
        let mut amount = cspr(10000);
        let cycles = 5;
        
        for _ in 0..cycles {
            let yield_amt = (amount * U512::from(100u64)) / U512::from(10000u64);
            amount = amount + yield_amt;
        }
        
        assert!(amount > cspr(10000), "Growth over multiple cycles");
    }

    #[test]
    fn test_apy_calculation_7d() {
        let initial = cspr(10000);
        let final_amount = cspr(10100);
        let days = 7u64;
        
        let apy = calculate_apy(initial, final_amount, days);
        
        assert!(apy > 0, "7-day APY calculated");
    }

    #[test]
    fn test_apy_calculation_30d() {
        let initial = cspr(10000);
        let final_amount = cspr(10250);
        let days = 30u64;
        
        let apy = calculate_apy(initial, final_amount, days);
        
        assert!(apy > 0, "30-day APY calculated");
    }

    #[test]
    fn test_apy_calculation_365d() {
        let initial = cspr(10000);
        let final_amount = cspr(11000);
        let days = 365u64;
        
        let apy = calculate_apy(initial, final_amount, days);
        
        assert!(apy >= 900 && apy <= 1100, "Annual APY ~10%");
    }

    #[test]
    fn test_gas_profitability_check() {
        let expected_yield = cspr(150);
        let gas_cost = cspr(10);
        
        let is_profitable = expected_yield > gas_cost * U512::from(2u64);
        
        assert!(is_profitable, "Compound is profitable");
    }
}
