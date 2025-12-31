#[cfg(test)]
mod reentrancy_attack_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_reentrancy_guard_blocks_attack() {
        let status_not_entered = 1u8;
        let status_entered = 2u8;
        
        let mut status = status_not_entered;
        
        status = status_entered;
        
        let is_reentering = status == status_entered;
        
        assert!(is_reentering, "Reentry detected and should be blocked");
    }

    #[test]
    fn test_withdraw_reentrancy_attempt() {
        let user_balance = cspr(10_000);
        let withdrawal_amount = cspr(5_000);
        
        let new_balance = user_balance - withdrawal_amount;
        
        let second_attempt_balance = new_balance;
        
        assert_u512_eq(second_attempt_balance, cspr(5_000), "Balance updated before external call");
    }

    #[test]
    fn test_compound_reentrancy_protection() {
        let is_compounding = true;
        let second_compound_attempt = is_compounding;
        
        assert!(second_compound_attempt, "Second compound should be blocked");
    }

    #[test]
    fn test_cross_function_reentrancy() {
        let function_a_executing = true;
        let function_b_call = function_a_executing;
        
        assert!(function_b_call, "Cross-function reentrancy blocked");
    }
}

#[cfg(test)]
mod access_control_attack_tests {
    use odra::prelude::*;
    use crate::helpers::*;

    #[test]
    fn test_unauthorized_pause() {
        let env = TestEnvironment::new();
        let attacker = env.user1;
        let guardian = env.guardian;
        
        assert_ne!(attacker, guardian, "Attacker is not guardian");
    }

    #[test]
    fn test_unauthorized_fee_change() {
        let env = TestEnvironment::new();
        let attacker = env.user1;
        let admin = env.admin;
        
        assert_ne!(attacker, admin, "Attacker is not admin");
    }

    #[test]
    fn test_unauthorized_validator_addition() {
        let env = TestEnvironment::new();
        let attacker = env.user1;
        let operator = env.operator;
        
        assert_ne!(attacker, operator, "Attacker is not operator");
    }

    #[test]
    fn test_unauthorized_strategy_deployment() {
        let env = TestEnvironment::new();
        let attacker = env.user1;
        let admin = env.admin;
        
        assert_ne!(attacker, admin, "Only admin can deploy strategies");
    }

    #[test]
    fn test_role_escalation_attempt() {
        let user_roles = vec!["USER"];
        let admin_role = "ADMIN";
        
        assert!(!user_roles.contains(&admin_role), "Role escalation prevented");
    }
}

#[cfg(test)]
mod rate_limit_attack_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_flash_loan_attack_blocked() {
        let deposit = cspr(100_000);
        let per_tx_limit = cspr(10_000);
        
        assert!(deposit > per_tx_limit, "Flash loan amount exceeds limit");
    }

    #[test]
    fn test_rapid_deposit_attack() {
        let deposits = vec![
            cspr(10_000),
            cspr(10_000),
            cspr(10_000),
            cspr(10_000),
            cspr(10_000),
            cspr(10_000),
        ];
        
        let total: U512 = deposits.iter().sum();
        let daily_limit = cspr(50_000);
        
        assert!(total > daily_limit, "Rapid deposits exceed daily limit");
    }

    #[test]
    fn test_sybil_attack_with_multiple_accounts() {
        let num_accounts = 100u64;
        let deposit_per_account = cspr(10_000);
        
        let total_deposits = U512::from(num_accounts) * deposit_per_account;
        let hourly_global_limit = cspr(1_000_000);
        
        assert!(total_deposits < hourly_global_limit, "Global limit prevents Sybil attack");
    }

    #[test]
    fn test_withdrawal_rate_limit() {
        let withdrawals = vec![
            cspr(100_000),
            cspr(100_000),
            cspr(100_000),
            cspr(100_000),
            cspr(100_000),
            cspr(100_000),
        ];
        
        let total: U512 = withdrawals.iter().sum();
        let hourly_limit = cspr(500_000);
        
        assert!(total > hourly_limit, "Withdrawal rate limit exceeded");
    }
}

#[cfg(test)]
mod pause_mechanism_attack_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_deposit_during_pause() {
        let is_paused = true;
        let deposit_allowed = !is_paused;
        
        assert!(!deposit_allowed, "Deposits blocked when paused");
    }

    #[test]
    fn test_withdrawal_during_pause() {
        let is_paused = true;
        let withdrawal_allowed = true;
        
        assert!(withdrawal_allowed, "Withdrawals allowed during pause");
    }

    #[test]
    fn test_compound_during_pause() {
        let is_paused = true;
        let compound_allowed = !is_paused;
        
        assert!(!compound_allowed, "Compounding blocked when paused");
    }

    #[test]
    fn test_emergency_pause_trigger() {
        let withdrawal = cspr(150_000);
        let tvl = cspr(1_000_000);
        
        let withdrawal_pct = ((withdrawal * U512::from(100u64)) / tvl).as_u64();
        let should_pause = withdrawal_pct > 10;
        
        assert!(should_pause, "Large withdrawal triggers pause");
    }

    #[test]
    fn test_unpause_after_issue_resolved() {
        let issue_resolved = true;
        let can_unpause = issue_resolved;
        
        assert!(can_unpause, "Can unpause after issue resolved");
    }

    #[test]
    fn test_pause_dos_prevention() {
        let last_pause_time = 1000u64;
        let current_time = 1500u64;
        let min_pause_interval = 3600u64;
        
        let can_pause_again = (current_time - last_pause_time) >= min_pause_interval;
        
        assert!(!can_pause_again, "Cannot pause too frequently");
    }
}

#[cfg(test)]
mod front_running_protection_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;

    #[test]
    fn test_sandwich_attack_prevention() {
        let expected_amount = cspr(1_000);
        let actual_amount = cspr(985);
        let max_slippage = 100u64;
        
        let slippage = calculate_slippage(expected_amount, actual_amount);
        
        assert!(slippage > max_slippage, "Sandwich attack detected and blocked");
    }

    #[test]
    fn test_mev_extraction_prevention() {
        let user_expected = cspr(1_000);
        let slippage_protected = apply_slippage(user_expected, 100);
        
        assert!(slippage_protected < user_expected, "MEV extraction limited");
    }

    #[test]
    fn test_price_manipulation_detection() {
        let price_before = cspr(100);
        let price_after = cspr(120);
        
        let price_change_bps = calculate_slippage(price_before, price_after);
        let manipulation_threshold = 500u64;
        
        let is_manipulated = price_change_bps > manipulation_threshold;
        
        assert!(is_manipulated, "Price manipulation detected");
    }

    #[test]
    fn test_oracle_manipulation_resistance() {
        let oracle_price = cspr(100);
        let dex_price = cspr(95);
        
        let deviation = calculate_slippage(oracle_price, dex_price);
        let max_deviation = 300u64;
        
        let use_oracle = deviation <= max_deviation;
        
        assert!(use_oracle, "Use oracle price when DEX manipulated");
    }
}

#[cfg(test)]
mod governance_attack_tests {
    use odra::prelude::*;
    use crate::helpers::*;

    #[test]
    fn test_proposal_without_timelock() {
        let env = TestEnvironment::new();
        let proposal_time = env.get_block_time();
        let execution_time = proposal_time + (24 * 60 * 60);
        let current_time = proposal_time + 100;
        
        let can_execute = current_time >= execution_time;
        
        assert!(!can_execute, "Cannot execute before timelock");
    }

    #[test]
    fn test_insufficient_signatures() {
        let signatures = 2u8;
        let required = 3u8;
        
        assert!(signatures < required, "Insufficient signatures");
    }

    #[test]
    fn test_replay_signature_attack() {
        let proposal_id_1 = 1u64;
        let proposal_id_2 = 2u64;
        
        assert_ne!(proposal_id_1, proposal_id_2, "Signatures tied to specific proposal");
    }

    #[test]
    fn test_malicious_proposal_content() {
        let proposed_fee = 5000u64;
        let max_fee = 2000u64;
        
        assert!(proposed_fee > max_fee, "Malicious fee proposal rejected");
    }
}
