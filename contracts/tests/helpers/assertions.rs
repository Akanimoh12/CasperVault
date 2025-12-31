use odra::casper_types::U512;

pub fn assert_u512_eq(actual: U512, expected: U512, msg: &str) {
    assert_eq!(actual, expected, "{}: expected {}, got {}", msg, expected, actual);
}

pub fn assert_u512_gt(actual: U512, expected: U512, msg: &str) {
    assert!(actual > expected, "{}: expected > {}, got {}", msg, expected, actual);
}

pub fn assert_u512_gte(actual: U512, expected: U512, msg: &str) {
    assert!(actual >= expected, "{}: expected >= {}, got {}", msg, expected, actual);
}

pub fn assert_u512_lt(actual: U512, expected: U512, msg: &str) {
    assert!(actual < expected, "{}: expected < {}, got {}", msg, expected, actual);
}

pub fn assert_u512_within_tolerance(actual: U512, expected: U512, tolerance_bps: u64) {
    let tolerance = (expected * U512::from(tolerance_bps)) / U512::from(10000u64);
    let lower = if expected > tolerance { expected - tolerance } else { U512::zero() };
    let upper = expected + tolerance;
    
    assert!(
        actual >= lower && actual <= upper,
        "Value {} not within {}bps of expected {} (range: {} - {})",
        actual, tolerance_bps, expected, lower, upper
    );
}

pub fn assert_share_price_increased(old_price: U512, new_price: U512) {
    assert!(
        new_price > old_price,
        "Share price should increase: old={}, new={}",
        old_price, new_price
    );
}

pub fn assert_balance_decreased(old_balance: U512, new_balance: U512, amount: U512) {
    let expected_new = if old_balance >= amount {
        old_balance - amount
    } else {
        U512::zero()
    };
    
    assert_u512_eq(new_balance, expected_new, "Balance decrease mismatch");
}

pub fn assert_balance_increased(old_balance: U512, new_balance: U512, amount: U512) {
    let expected_new = old_balance + amount;
    assert_u512_eq(new_balance, expected_new, "Balance increase mismatch");
}

pub fn assert_percentage(part: U512, whole: U512, expected_percentage: u64) {
    let actual_percentage = if whole > U512::zero() {
        ((part * U512::from(10000u64)) / whole).as_u64()
    } else {
        0
    };
    
    let tolerance = 10;
    assert!(
        actual_percentage >= expected_percentage.saturating_sub(tolerance) &&
        actual_percentage <= expected_percentage.saturating_add(tolerance),
        "Expected {}%, got {}%",
        expected_percentage as f64 / 100.0,
        actual_percentage as f64 / 100.0
    );
}

pub fn calculate_expected_shares(assets: U512, total_assets: U512, total_shares: U512) -> U512 {
    if total_shares == U512::zero() {
        assets
    } else {
        (assets * total_shares) / total_assets
    }
}

pub fn calculate_expected_assets(shares: U512, total_assets: U512, total_shares: U512) -> U512 {
    if total_shares == U512::zero() {
        U512::zero()
    } else {
        (shares * total_assets) / total_shares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_within_tolerance() {
        let value = U512::from(1000u64);
        let expected = U512::from(1005u64);
        
        assert_u512_within_tolerance(value, expected, 100);
    }

    #[test]
    #[should_panic]
    fn test_assert_within_tolerance_fails() {
        let value = U512::from(1000u64);
        let expected = U512::from(1200u64);
        
        assert_u512_within_tolerance(value, expected, 100);
    }

    #[test]
    fn test_calculate_expected_shares() {
        let assets = U512::from(1000u64);
        let total_assets = U512::from(10000u64);
        let total_shares = U512::from(8000u64);
        
        let shares = calculate_expected_shares(assets, total_assets, total_shares);
        assert_eq!(shares, U512::from(800u64));
    }

    #[test]
    fn test_calculate_expected_shares_first_deposit() {
        let assets = U512::from(1000u64);
        let shares = calculate_expected_shares(assets, U512::zero(), U512::zero());
        
        assert_eq!(shares, assets);
    }
}
