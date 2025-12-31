use odra::prelude::*;

/// Mock Validator Contract
/// 
/// Simulates a Casper validator for testing purposes.
/// In production, LiquidStaking would interact with real Casper validators
/// via system contracts. This mock allows testing without a full node.
#[odra::module]
pub struct MockValidator {
    /// Validator address
    validator_address: Var<Address>,
    
    /// Total CSPR delegated to this validator
    total_delegated: Var<U512>,
    
    /// Delegations per account (delegator -> amount)
    delegations: Mapping<Address, U512>,
    
    /// Simulated rewards pool
    rewards_pool: Var<U512>,
    
    /// Uptime percentage (0-100)
    uptime: Var<u8>,
    
    /// Commission rate (0-100)
    commission: Var<u8>,
    
    /// Is validator active
    is_active: Var<bool>,
    
    /// Rewards rate per epoch (basis points)
    rewards_rate_bps: Var<u16>,
    
    /// Last rewards distribution timestamp
    last_distribution: Var<u64>,
}

#[odra::module]
impl MockValidator {
    /// Initialize the mock validator
    pub fn init(
        &mut self,
        validator_address: Address,
        uptime: u8,
        commission: u8,
        rewards_rate_bps: u16,
    ) {
        self.validator_address.set(validator_address);
        self.uptime.set(uptime);
        self.commission.set(commission);
        self.rewards_rate_bps.set(rewards_rate_bps);
        self.total_delegated.set(U512::zero());
        self.rewards_pool.set(U512::zero());
        self.is_active.set(true);
        self.last_distribution.set(self.env().get_block_time());
    }

    /// Delegate CSPR to this validator
    /// 
    /// In real Casper, this would be a system contract call.
    /// Here we simulate by tracking delegations.
    pub fn delegate(&mut self, delegator: Address, amount: U512) -> Result<(), String> {
        if !self.is_active.get_or_default() {
            return Err("Validator is not active".to_string());
        }
        
        if amount.is_zero() {
            return Err("Cannot delegate zero amount".to_string());
        }
        
        // Update delegator's balance
        let current = self.delegations.get(&delegator).unwrap_or(U512::zero());
        self.delegations.set(&delegator, current + amount);
        
        // Update total delegated
        let total = self.total_delegated.get_or_default();
        self.total_delegated.set(total + amount);
        
        Ok(())
    }

    /// Undelegate CSPR from this validator
    /// 
    /// In real Casper, this initiates unbonding period.
    /// Here we simulate immediate undelegation for testing.
    pub fn undelegate(&mut self, delegator: Address, amount: U512) -> Result<(), String> {
        let current = self.delegations.get(&delegator).unwrap_or(U512::zero());
        
        if amount > current {
            return Err("Insufficient delegation balance".to_string());
        }
        
        // Update delegator's balance
        self.delegations.set(&delegator, current - amount);
        
        // Update total delegated
        let total = self.total_delegated.get_or_default();
        self.total_delegated.set(total - amount);
        
        Ok(())
    }

    /// Claim rewards for a delegator
    /// 
    /// Calculates and returns accrued rewards based on:
    /// - Delegation amount
    /// - Time since last claim
    /// - Rewards rate
    /// - Commission
    pub fn claim_rewards(&mut self, delegator: Address) -> U512 {
        let delegation = self.delegations.get(&delegator).unwrap_or(U512::zero());
        
        if delegation.is_zero() {
            return U512::zero();
        }
        
        let rewards = self.calculate_rewards(delegation);
        
        // Apply commission
        let commission_rate = self.commission.get_or_default();
        let commission_amount = (U256::from(rewards) * U256::from(commission_rate) / U256::from(100u64))
            .try_into()
            .unwrap_or(U512::zero());
        
        let net_rewards = rewards - commission_amount;
        
        // Add to rewards pool (simulated)
        let pool = self.rewards_pool.get_or_default();
        self.rewards_pool.set(pool + net_rewards);
        
        net_rewards
    }

    /// Calculate rewards for an amount
    /// 
    /// Formula: amount * rewards_rate * time_factor
    fn calculate_rewards(&self, amount: U512) -> U512 {
        let rate_bps = self.rewards_rate_bps.get_or_default();
        
        // Simulate daily rewards: amount * rate / 365 days / 10000 (bps)
        // For testing, assume 1 day has passed
        let rewards = (U256::from(amount) * U256::from(rate_bps) / U256::from(365 * 10000u64))
            .try_into()
            .unwrap_or(U512::zero());
        
        rewards
    }

    /// Distribute rewards to all delegators
    /// 
    /// Simulates epoch rewards distribution
    pub fn distribute_epoch_rewards(&mut self) {
        let total_delegated = self.total_delegated.get_or_default();
        
        if total_delegated.is_zero() {
            return;
        }
        
        // Calculate total rewards for this epoch
        let total_rewards = self.calculate_rewards(total_delegated);
        
        // Add to rewards pool
        let pool = self.rewards_pool.get_or_default();
        self.rewards_pool.set(pool + total_rewards);
        
        self.last_distribution.set(self.env().get_block_time());
    }

    /// Simulate slashing event
    /// 
    /// Reduces delegated stake by a percentage
    pub fn simulate_slashing(&mut self, percentage: u8) {
        if percentage > 100 {
            return;
        }
        
        let total = self.total_delegated.get_or_default();
        let slash_amount = (U256::from(total) * U256::from(percentage) / U256::from(100u64))
            .try_into()
            .unwrap_or(U512::zero());
        
        if slash_amount >= total {
            self.total_delegated.set(U512::zero());
        } else {
            self.total_delegated.set(total - slash_amount);
        }
        
        // Mark as inactive after slashing
        self.is_active.set(false);
    }

    /// Update validator performance metrics
    pub fn update_performance(&mut self, uptime: u8, commission: u8) {
        self.uptime.set(uptime);
        self.commission.set(commission);
        
        // Deactivate if performance drops too low
        if uptime < 95 {
            self.is_active.set(false);
        }
    }

    // ==================== VIEW FUNCTIONS ====================

    /// Get validator address
    pub fn get_address(&self) -> Address {
        self.validator_address.get_or_default()
    }

    /// Get total delegated to this validator
    pub fn get_total_delegated(&self) -> U512 {
        self.total_delegated.get_or_default()
    }

    /// Get delegation for specific delegator
    pub fn get_delegation(&self, delegator: Address) -> U512 {
        self.delegations.get(&delegator).unwrap_or(U512::zero())
    }

    /// Get pending rewards for delegator
    pub fn get_pending_rewards(&self, delegator: Address) -> U512 {
        let delegation = self.delegations.get(&delegator).unwrap_or(U512::zero());
        self.calculate_rewards(delegation)
    }

    /// Get uptime percentage
    pub fn get_uptime(&self) -> u8 {
        self.uptime.get_or_default()
    }

    /// Get commission rate
    pub fn get_commission(&self) -> u8 {
        self.commission.get_or_default()
    }

    /// Check if validator is active
    pub fn is_active(&self) -> bool {
        self.is_active.get_or_default()
    }

    /// Get rewards pool
    pub fn get_rewards_pool(&self) -> U512 {
        self.rewards_pool.get_or_default()
    }
}

// ==================== MOCK HELPER FUNCTIONS ====================

/// Helper to create a test validator with good performance
pub fn create_good_validator() -> MockValidator {
    let mut validator = MockValidator::new();
    let address = Address::from([1u8; 32]);
    validator.init(
        address,
        98,    // 98% uptime
        5,     // 5% commission
        1000,  // 10% annual rewards (1000 bps)
    );
    validator
}

/// Helper to create a test validator with poor performance
pub fn create_poor_validator() -> MockValidator {
    let mut validator = MockValidator::new();
    let address = Address::from([2u8; 32]);
    validator.init(
        address,
        93,    // 93% uptime (below threshold)
        15,    // 15% commission (high)
        800,   // 8% annual rewards
    );
    validator
}

/// Helper to create a high-performance validator
pub fn create_excellent_validator() -> MockValidator {
    let mut validator = MockValidator::new();
    let address = Address::from([3u8; 32]);
    validator.init(
        address,
        100,   // 100% uptime
        2,     // 2% commission (low)
        1200,  // 12% annual rewards
    );
    validator
}

#[cfg(test)]
mod mock_validator_tests {
    use super::*;

    #[test]
    fn test_mock_validator_delegation() {
        let mut validator = create_good_validator();
        let delegator = Address::from([10u8; 32]);
        let amount = U512::from(1000_000_000_000u64); // 1000 CSPR
        
        // Delegate
        let result = validator.delegate(delegator, amount);
        assert!(result.is_ok());
        
        // Verify delegation
        assert_eq!(validator.get_delegation(delegator), amount);
        assert_eq!(validator.get_total_delegated(), amount);
    }

    #[test]
    fn test_mock_validator_undelegation() {
        let mut validator = create_good_validator();
        let delegator = Address::from([10u8; 32]);
        let amount = U512::from(1000_000_000_000u64);
        
        // Delegate first
        validator.delegate(delegator, amount).unwrap();
        
        // Undelegate half
        let half = amount / U512::from(2u64);
        let result = validator.undelegate(delegator, half);
        assert!(result.is_ok());
        
        // Verify
        assert_eq!(validator.get_delegation(delegator), half);
        assert_eq!(validator.get_total_delegated(), half);
    }

    #[test]
    fn test_mock_validator_rewards() {
        let mut validator = create_good_validator();
        let delegator = Address::from([10u8; 32]);
        let amount = U512::from(1000_000_000_000u64);
        
        // Delegate
        validator.delegate(delegator, amount).unwrap();
        
        // Claim rewards
        let rewards = validator.claim_rewards(delegator);
        
        // Should have some rewards (based on 10% APY)
        assert!(rewards > U512::zero());
    }

    #[test]
    fn test_mock_validator_slashing() {
        let mut validator = create_good_validator();
        let delegator = Address::from([10u8; 32]);
        let amount = U512::from(1000_000_000_000u64);
        
        validator.delegate(delegator, amount).unwrap();
        
        // Simulate 10% slashing
        validator.simulate_slashing(10);
        
        // Total delegated should be reduced by 10%
        let expected = amount * U512::from(90u64) / U512::from(100u64);
        assert_eq!(validator.get_total_delegated(), expected);
        
        // Validator should be inactive after slashing
        assert_eq!(validator.is_active(), false);
    }

    #[test]
    fn test_mock_validator_performance_update() {
        let mut validator = create_good_validator();
        
        assert_eq!(validator.get_uptime(), 98);
        assert_eq!(validator.is_active(), true);
        
        // Drop uptime below threshold
        validator.update_performance(93, 5);
        
        assert_eq!(validator.get_uptime(), 93);
        assert_eq!(validator.is_active(), false);
    }
}
