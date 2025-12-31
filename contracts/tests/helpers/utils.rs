use odra::casper_types::U512;

pub fn cspr(amount: u64) -> U512 {
    U512::from(amount) * U512::from(1_000_000_000u64)
}

pub fn milli_cspr(amount: u64) -> U512 {
    U512::from(amount) * U512::from(1_000_000u64)
}

pub fn to_cspr(amount: U512) -> u64 {
    (amount / U512::from(1_000_000_000u64)).as_u64()
}

pub fn calculate_apy(initial: U512, final_amount: U512, days: u64) -> u64 {
    if initial == U512::zero() || days == 0 {
        return 0;
    }
    
    let profit = if final_amount > initial {
        final_amount - initial
    } else {
        return 0;
    };
    
    let daily_rate = (profit * U512::from(10000u64)) / initial;
    let annual_rate = daily_rate * U512::from(365u64) / U512::from(days);
    
    annual_rate.as_u64()
}

pub fn calculate_performance_fee(profit: U512, fee_bps: u64) -> U512 {
    (profit * U512::from(fee_bps)) / U512::from(10000u64)
}

pub fn calculate_management_fee(tvl: U512, fee_bps: u64, days: u64) -> U512 {
    let annual_fee = (tvl * U512::from(fee_bps)) / U512::from(10000u64);
    (annual_fee * U512::from(days)) / U512::from(365u64)
}

pub fn calculate_compound_growth(principal: U512, rate_bps: u64, compounds: u32) -> U512 {
    let mut amount = principal;
    
    for _ in 0..compounds {
        let growth = (amount * U512::from(rate_bps)) / U512::from(10000u64);
        amount = amount + growth;
    }
    
    amount
}

pub fn simulate_time_passage(days: u64) -> u64 {
    days * 24 * 60 * 60
}

pub fn bps_to_percentage(bps: u64) -> f64 {
    bps as f64 / 100.0
}

pub fn percentage_to_bps(percentage: f64) -> u64 {
    (percentage * 100.0) as u64
}

pub fn calculate_slippage(expected: U512, actual: U512) -> u64 {
    if expected == U512::zero() {
        return 0;
    }
    
    let diff = if expected > actual {
        expected - actual
    } else {
        actual - expected
    };
    
    ((diff * U512::from(10000u64)) / expected).as_u64()
}

pub fn apply_slippage(amount: U512, slippage_bps: u64) -> U512 {
    let slippage = (amount * U512::from(slippage_bps)) / U512::from(10000u64);
    if amount > slippage {
        amount - slippage
    } else {
        U512::zero()
    }
}

pub fn weighted_average(values: &[(U512, u64)]) -> U512 {
    let mut total_value = U512::zero();
    let mut total_weight = 0u64;
    
    for (value, weight) in values {
        total_value = total_value + (*value * U512::from(*weight));
        total_weight += weight;
    }
    
    if total_weight == 0 {
        U512::zero()
    } else {
        total_value / U512::from(total_weight)
    }
}

pub fn generate_random_amount(min: u64, max: u64, seed: u64) -> U512 {
    let range = max - min;
    let amount = min + (seed % range);
    U512::from(amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cspr_conversion() {
        let amount = cspr(100);
        assert_eq!(to_cspr(amount), 100);
    }

    #[test]
    fn test_apy_calculation() {
        let initial = U512::from(10000u64);
        let final_amount = U512::from(11000u64);
        let apy = calculate_apy(initial, final_amount, 365);
        
        assert!(apy >= 950 && apy <= 1050);
    }

    #[test]
    fn test_performance_fee() {
        let profit = U512::from(1000u64);
        let fee = calculate_performance_fee(profit, 1000);
        
        assert_eq!(fee, U512::from(100u64));
    }

    #[test]
    fn test_compound_growth() {
        let principal = U512::from(10000u64);
        let final_amount = calculate_compound_growth(principal, 100, 12);
        
        assert!(final_amount > principal);
    }

    #[test]
    fn test_slippage_calculation() {
        let expected = U512::from(1000u64);
        let actual = U512::from(990u64);
        let slippage = calculate_slippage(expected, actual);
        
        assert_eq!(slippage, 100);
    }

    #[test]
    fn test_weighted_average() {
        let values = vec![
            (U512::from(100u64), 50),
            (U512::from(200u64), 30),
            (U512::from(300u64), 20),
        ];
        
        let avg = weighted_average(&values);
        assert_eq!(avg, U512::from(160u64));
    }
}
