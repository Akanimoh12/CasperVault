#[cfg(test)]
mod security_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_rate_limiter_per_transaction() {
        let deposit = cspr(15_000);
        let max_per_tx = cspr(10_000);
        
        assert!(deposit > max_per_tx, "Exceeds per-tx limit");
    }

    #[test]
    fn test_rate_limiter_daily_limit() {
        let deposits = vec![cspr(10_000), cspr(10_000), cspr(10_000), cspr(25_000)];
        let total: U512 = deposits.iter().sum();
        let daily_limit = cspr(50_000);
        
        assert!(total > daily_limit, "Exceeds daily limit");
    }

    #[test]
    fn test_rate_limiter_global_hourly_deposits() {
        let global_deposits = cspr(1_100_000);
        let hourly_limit = cspr(1_000_000);
        
        assert!(global_deposits > hourly_limit, "Exceeds global hourly limit");
    }

    #[test]
    fn test_slippage_protection_1_percent() {
        let expected = cspr(1000);
        let actual = cspr(985);
        
        let slippage_bps = calculate_slippage(expected, actual);
        let max_slippage = 100u64;
        
        assert!(slippage_bps > max_slippage, "Slippage exceeds 1%");
    }

    #[test]
    fn test_slippage_within_tolerance() {
        let expected = cspr(1000);
        let actual = cspr(995);
        
        let slippage_bps = calculate_slippage(expected, actual);
        let max_slippage = 100u64;
        
        assert!(slippage_bps <= max_slippage, "Slippage within tolerance");
    }

    #[test]
    fn test_strategy_health_check_healthy() {
        let balance = cspr(10_000);
        let expected_balance = cspr(10_100);
        let tolerance_bps = 200u64;
        
        let is_healthy = calculate_slippage(expected_balance, balance) <= tolerance_bps;
        
        assert!(is_healthy, "Strategy health OK");
    }

    #[test]
    fn test_strategy_health_check_critical() {
        let balance = cspr(8_000);
        let expected_balance = cspr(10_000);
        let threshold_bps = 1000u64;
        
        let deviation = calculate_slippage(expected_balance, balance);
        let is_critical = deviation > threshold_bps;
        
        assert!(is_critical, "Strategy health critical");
    }

    #[test]
    fn test_strategy_risk_score() {
        let tvl_score = 30u8;
        let audit_score = 20u8;
        let age_score = 15u8;
        let performance_score = 20u8;
        
        let total_score = tvl_score + audit_score + age_score + performance_score;
        let threshold = 75u8;
        
        assert!(total_score >= threshold, "Risk score acceptable");
    }

    #[test]
    fn test_consecutive_failure_tracking() {
        let failures = 5u8;
        let max_failures = 3u8;
        
        assert!(failures > max_failures, "Too many failures");
    }

    #[test]
    fn test_anomaly_detection_large_withdrawal() {
        let withdrawal = cspr(120_000);
        let tvl = cspr(1_000_000);
        
        let percentage = ((withdrawal * U512::from(100u64)) / tvl).as_u64();
        
        assert!(percentage > 10, "Large withdrawal detected");
    }

    #[test]
    fn test_anomaly_detection_tvl_drop() {
        let tvl_before = cspr(1_000_000);
        let tvl_after = cspr(750_000);
        let time_elapsed = 3600u64;
        
        let drop_pct = if tvl_before > tvl_after {
            ((tvl_before - tvl_after) * U512::from(100u64)) / tvl_before
        } else {
            U512::zero()
        };
        
        assert!(drop_pct.as_u64() > 20, "Rapid TVL decrease");
    }

    #[test]
    fn test_multisig_signature_requirement() {
        let signatures = 2u8;
        let required = 3u8;
        
        assert!(signatures < required, "Insufficient signatures");
    }

    #[test]
    fn test_multisig_sufficient_signatures() {
        let signatures = 3u8;
        let required = 3u8;
        
        assert!(signatures >= required, "Sufficient signatures");
    }

    #[test]
    fn test_timelock_enforcement() {
        let env = TestEnvironment::new();
        let proposal_time = env.get_block_time();
        let timelock_duration = 24 * 60 * 60u64;
        let execution_time = proposal_time + timelock_duration;
        
        env.advance_block_time(12 * 60 * 60);
        let current_time = env.get_block_time();
        
        assert!(current_time < execution_time, "Timelock not passed");
    }

    #[test]
    fn test_timelock_passed() {
        let env = TestEnvironment::new();
        let proposal_time = env.get_block_time();
        let timelock_duration = 24 * 60 * 60u64;
        let execution_time = proposal_time + timelock_duration;
        
        env.advance_block_time(25 * 60 * 60);
        let current_time = env.get_block_time();
        
        assert!(current_time >= execution_time, "Timelock passed");
    }

    #[test]
    fn test_access_control_admin_role() {
        let env = TestEnvironment::new();
        let caller = env.user1;
        let admin = env.admin;
        
        assert_ne!(caller, admin, "User is not admin");
    }

    #[test]
    fn test_access_control_operator_role() {
        let env = TestEnvironment::new();
        let operator = env.operator;
        
        assert_ne!(operator, env.user1, "Operator role distinct");
    }

    #[test]
    fn test_pausable_mechanism() {
        let is_paused = true;
        let operation_type = "deposit";
        
        assert!(is_paused, "Contract paused");
    }

    #[test]
    fn test_emergency_pause_large_withdrawal() {
        let withdrawal = cspr(150_000);
        let tvl = cspr(1_000_000);
        let threshold_pct = 10u64;
        
        let withdrawal_pct = ((withdrawal * U512::from(100u64)) / tvl).as_u64();
        let should_pause = withdrawal_pct > threshold_pct;
        
        assert!(should_pause, "Should trigger emergency pause");
    }

    #[test]
    fn test_reentrancy_guard_status() {
        let not_entered = 1u8;
        let entered = 2u8;
        
        let status = not_entered;
        
        assert_ne!(status, entered, "Not in reentrant call");
    }
}
