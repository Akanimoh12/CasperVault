/// Strategy Interface for CasperVault
/// 
/// All strategies must implement this interface to integrate with the StrategyRouter.
/// Strategies deploy vault funds to external protocols to generate yield.

use odra::prelude::*;
use odra::{Address, Var};
use odra::casper_types::{U256, U512};

/// Risk level categorization for strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Low risk: Stable protocols, high liquidity, audited
    /// Examples: Blue-chip lending protocols, stable DEX LPs
    Low,
    
    /// Medium risk: Moderate complexity, good liquidity
    /// Examples: Mid-tier DEXes, newer lending protocols
    Medium,
    
    /// High risk: Complex strategies, lower liquidity, cross-chain
    /// Examples: Exotic LP pairs, cross-chain bridges, leverage
    High,
}

/// Strategy-specific errors
#[derive(Debug, PartialEq, Eq)]
pub enum StrategyError {
    /// Insufficient balance in strategy
    InsufficientBalance,
    
    /// Strategy is paused
    Paused,
    
    /// External protocol call failed
    ProtocolCallFailed,
    
    /// Slippage exceeded maximum allowed
    SlippageExceeded,
    
    /// Deployment amount below minimum
    AmountTooLow,
    
    /// Withdrawal amount exceeds available
    WithdrawalTooLarge,
    
    /// Strategy at maximum capacity
    MaxCapacityReached,
    
    /// Health check failed
    UnhealthyStrategy,
    
    /// Unauthorized caller
    Unauthorized,
}

/// Core strategy interface that all strategies must implement
/// 
/// This trait defines the standard operations for interacting with
/// external yield-generating protocols.
pub trait IStrategy {
    /// Deploy funds to the strategy
    /// 
    /// # Arguments
    /// * `amount` - Amount of lstCSPR to deploy
    /// 
    /// # Returns
    /// * `Ok(U512)` - Actual amount deployed (may differ due to slippage)
    /// * `Err(StrategyError)` - If deployment fails
    /// 
    /// # Example Flow
    /// For DEX strategy:
    /// 1. Receive lstCSPR
    /// 2. Add liquidity to lstCSPR/CSPR pool
    /// 3. Stake LP tokens for rewards
    /// 4. Return amount deployed
    fn deploy(&mut self, amount: U512) -> Result<U512, StrategyError>;
    
    /// Withdraw funds from the strategy
    /// 
    /// # Arguments
    /// * `amount` - Amount of lstCSPR to withdraw
    /// 
    /// # Returns
    /// * `Ok(U512)` - Actual amount withdrawn
    /// * `Err(StrategyError)` - If withdrawal fails
    /// 
    /// # Example Flow
    /// For DEX strategy:
    /// 1. Unstake LP tokens
    /// 2. Remove liquidity from pool
    /// 3. Return lstCSPR to router
    /// 4. Return amount withdrawn
    fn withdraw(&mut self, amount: U512) -> Result<U512, StrategyError>;
    
    /// Harvest accrued yields from the strategy
    /// 
    /// # Returns
    /// * `Ok(U512)` - Amount of yield harvested
    /// * `Err(StrategyError)` - If harvest fails
    /// 
    /// # Example Flow
    /// 1. Claim rewards from external protocol
    /// 2. Swap rewards to lstCSPR if needed
    /// 3. Return harvested amount
    fn harvest(&mut self) -> Result<U512, StrategyError>;
    
    /// Get current balance deployed in strategy
    /// 
    /// # Returns
    /// Total lstCSPR value in the strategy
    /// 
    /// This should include:
    /// - Principal deployed
    /// - Accrued but unclaimed yields
    /// - Value of LP positions
    fn get_balance(&self) -> U512;
    
    /// Get current Annual Percentage Yield
    /// 
    /// # Returns
    /// APY as basis points (e.g., 1500 = 15%)
    /// 
    /// Calculation should include:
    /// - Base protocol yield
    /// - Reward token value
    /// - Trading fees (for DEX)
    /// - Compounding frequency
    fn get_apy(&self) -> U256;
    
    /// Get risk level of this strategy
    /// 
    /// # Returns
    /// RiskLevel enum (Low, Medium, High)
    /// 
    /// Factors considered:
    /// - Protocol maturity
    /// - Audit status
    /// - Liquidity depth
    /// - Complexity
    /// - Cross-chain exposure
    fn get_risk_level(&self) -> RiskLevel;
    
    /// Get human-readable strategy name
    /// 
    /// # Returns
    /// Strategy name for display/logging
    fn name(&self) -> String;
    
    /// Check if strategy is healthy
    /// 
    /// # Returns
    /// * `true` - Strategy is operating normally
    /// * `false` - Strategy has issues (paused, unhealthy protocol, etc.)
    /// 
    /// Health checks may include:
    /// - Protocol responsiveness
    /// - Balance consistency
    /// - APY within reasonable range
    /// - No emergency pause
    fn is_healthy(&self) -> bool;
    
    /// Get maximum capacity for this strategy
    /// 
    /// # Returns
    /// Maximum lstCSPR that can be deployed
    /// 
    /// Limits may be based on:
    /// - Protocol TVL caps
    /// - Liquidity constraints
    /// - Risk management limits
    fn max_capacity(&self) -> U512;
}

/// Strategy metadata for tracking and reporting
#[derive(Debug, Clone)]
pub struct StrategyMetadata {
    /// Strategy contract address
    pub address: Address,
    
    /// Human-readable name
    pub name: String,
    
    /// Current APY in basis points
    pub apy: U256,
    
    /// Risk level
    pub risk_level: RiskLevel,
    
    /// Current balance deployed
    pub balance: U512,
    
    /// Maximum capacity
    pub max_capacity: U512,
    
    /// Health status
    pub is_healthy: bool,
    
    /// Last harvest timestamp
    pub last_harvest: u64,
    
    /// Total harvested (lifetime)
    pub total_harvested: U512,
}

impl StrategyMetadata {
    /// Calculate utilization percentage
    /// 
    /// # Returns
    /// Utilization as basis points (0-10000)
    pub fn utilization_bps(&self) -> u16 {
        if self.max_capacity.is_zero() {
            return 0;
        }
        
        let utilization = self.balance
            .checked_mul(U512::from(10000u64))
            .unwrap_or(U512::zero())
            .checked_div(self.max_capacity)
            .unwrap_or(U512::zero());
        
        u16::try_from(utilization).unwrap_or(10000)
    }
    
    /// Check if strategy is at or near capacity
    /// 
    /// # Arguments
    /// * `threshold_bps` - Threshold in basis points (e.g., 9000 = 90%)
    /// 
    /// # Returns
    /// `true` if utilization >= threshold
    pub fn is_near_capacity(&self, threshold_bps: u32) -> bool {
        self.utilization_bps() >= threshold_bps
    }
}

/// Strategy allocation configuration
#[derive(Debug, Clone)]
pub struct AllocationConfig {
    /// Strategy name
    pub name: String,
    
    /// Target allocation percentage (basis points, 0-10000)
    pub target_bps: u32,
    
    /// Minimum allocation (basis points)
    pub min_bps: u32,
    
    /// Maximum allocation (basis points)
    pub max_bps: u32,
    
    /// Enable/disable this strategy
    pub enabled: bool,
}

impl AllocationConfig {
    /// Validate allocation config
    pub fn validate(&self) -> Result<(), String> {
        if self.target_bps > 10000 {
            return Err("Target allocation exceeds 100%".to_string());
        }
        
        if self.min_bps > self.target_bps {
            return Err("Min allocation exceeds target".to_string());
        }
        
        if self.max_bps < self.target_bps {
            return Err("Max allocation below target".to_string());
        }
        
        if self.max_bps > 10000 {
            return Err("Max allocation exceeds 100%".to_string());
        }
        
        Ok(())
    }
    
    /// Check if a proposed allocation is within bounds
    pub fn is_within_bounds(&self, allocation_bps: u32) -> bool {
        allocation_bps >= self.min_bps && allocation_bps <= self.max_bps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strategy_metadata_utilization() {
        let metadata = StrategyMetadata {
            address: Address::from([1u8; 32]),
            name: "Test Strategy".to_string(),
            apy: U256::from(1500u64), // 15%
            risk_level: RiskLevel::Low,
            balance: U512::from(75_000u64),
            max_capacity: U512::from(100_000u64),
            is_healthy: true,
            last_harvest: 0,
            total_harvested: U512::zero(),
        };
        
        // 75k / 100k = 75% = 7500 bps
        assert_eq!(metadata.utilization_bps(), 7500);
        assert!(!metadata.is_near_capacity(8000)); // Below 80% threshold
        assert!(metadata.is_near_capacity(7000)); // Above 70% threshold
    }
    
    #[test]
    fn test_allocation_config_validation() {
        let valid = AllocationConfig {
            name: "DEX Strategy".to_string(),
            target_bps: 4000, // 40%
            min_bps: 2000,    // 20%
            max_bps: 5000,    // 50%
            enabled: true,
        };
        assert!(valid.validate().is_ok());
        
        // Target exceeds 100%
        let invalid1 = AllocationConfig {
            name: "Invalid".to_string(),
            target_bps: 15000,
            min_bps: 0,
            max_bps: 20000,
            enabled: true,
        };
        assert!(invalid1.validate().is_err());
        
        // Min > Target
        let invalid2 = AllocationConfig {
            name: "Invalid".to_string(),
            target_bps: 3000,
            min_bps: 5000,
            max_bps: 8000,
            enabled: true,
        };
        assert!(invalid2.validate().is_err());
    }
    
    #[test]
    fn test_allocation_bounds_check() {
        let config = AllocationConfig {
            name: "Test".to_string(),
            target_bps: 4000,
            min_bps: 2000,
            max_bps: 6000,
            enabled: true,
        };
        
        assert!(!config.is_within_bounds(1000)); // Below min
        assert!(config.is_within_bounds(3000));  // Within bounds
        assert!(config.is_within_bounds(5000));  // Within bounds
        assert!(!config.is_within_bounds(7000)); // Above max
    }
}
