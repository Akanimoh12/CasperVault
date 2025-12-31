use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::*;

/// Validator performance metrics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorMetrics {
    pub validator: Address,
    pub uptime_percentage: u8,
    pub commission_rate: u8,
    pub current_stake: U512,
    pub max_stake_cap: U512,
    pub is_verified: bool,
    pub risk_score: u8,
    pub total_delegators: u32,
    pub last_performance_check: u64,
    pub consecutive_good_epochs: u32,
    pub total_rewards_earned: U512,
}

/// Validator selection result
#[derive(Debug, Clone)]
pub struct ValidatorAllocation {
    pub validator: Address,
    pub amount: U512,
}

/// ValidatorRegistry - Manages validator information and selection
/// 
/// This module provides:
/// - Validator registration and deregistration
/// - Performance monitoring
/// - Smart validator selection algorithm
/// - Risk scoring and health checks
#[odra::module]
pub struct ValidatorRegistry {
    /// Validator metrics storage
    validators: Mapping<Address, ValidatorMetrics>,
    
    /// Active validators list
    active_validators: Var<Vec<Address>>,
    
    /// Blacklisted validators (removed due to poor performance)
    blacklisted: Mapping<Address, bool>,
    
    /// Configuration: Minimum uptime requirement (%)
    min_uptime: Var<u8>,
    
    /// Configuration: Maximum commission rate (%)
    max_commission: Var<u8>,
    
    /// Configuration: Maximum stake per validator (% of total)
    max_per_validator_pct: Var<u8>,
    
    /// Configuration: Minimum consecutive good epochs before auto-add
    min_good_epochs: Var<u32>,
    
    /// Total stake across all validators
    total_stake: Var<U512>,
}

#[odra::module]
impl ValidatorRegistry {
    /// Initialize the validator registry
    pub fn init(&mut self) {
        self.min_uptime.set(95);
        self.max_commission.set(10);
        self.max_per_validator_pct.set(10);
        self.min_good_epochs.set(10);
        self.total_stake.set(U512::zero());
        self.active_validators.set(Vec::new());
    }

    /// Register a new validator
    /// 
    /// Validators must meet minimum requirements to be registered
    pub fn register_validator(
        &mut self,
        validator: Address,
        uptime_percentage: u8,
        commission_rate: u8,
        max_stake_cap: U512,
        is_verified: bool,
    ) -> Result<(), StakingError> {
        if self.validators.get(&validator).is_some() {
            return Err(StakingError::ValidatorNotEligible);
        }
        
        // Validate minimum requirements
        if uptime_percentage < self.min_uptime.get_or_default() {
            return Err(StakingError::ValidatorNotEligible);
        }
        
        if commission_rate > self.max_commission.get_or_default() {
            return Err(StakingError::ValidatorNotEligible);
        }
        
        if self.blacklisted.get(&validator).unwrap_or(false) {
            return Err(StakingError::ValidatorNotEligible);
        }
        
        let risk_score = self.calculate_risk_score(uptime_percentage, commission_rate, 0);
        
        // Create validator metrics
        let metrics = ValidatorMetrics {
            validator,
            uptime_percentage,
            commission_rate,
            current_stake: U512::zero(),
            max_stake_cap,
            is_verified,
            risk_score,
            total_delegators: 0,
            last_performance_check: self.env().get_block_time(),
            consecutive_good_epochs: 0,
            total_rewards_earned: U512::zero(),
        };
        
        self.validators.set(&validator, metrics);
        
        // Add to active list
        let mut active = self.active_validators.get_or_default();
        active.push(validator);
        self.active_validators.set(active);
        
        self.env().emit_event(ValidatorAdded {
            validator,
            uptime_percentage,
            commission_rate,
        });
        
        Ok(())
    }

    /// Deregister a validator
    /// 
    /// Removes validator from active set. Should trigger undelegation.
    pub fn deregister_validator(&mut self, validator: Address, reason: String) {
        // Remove from active list
        let mut active = self.active_validators.get_or_default();
        active.retain(|v| v != &validator);
        self.active_validators.set(active);
        
        if let Some(metrics) = self.validators.get(&validator) {
            let total = self.total_stake.get_or_default();
            if total >= metrics.current_stake {
                self.total_stake.set(total - metrics.current_stake);
            }
        }
        
        self.env().emit_event(ValidatorRemoved {
            validator,
            reason,
        });
    }

    /// Update validator performance metrics
    /// 
    /// Should be called periodically (e.g., every epoch)
    pub fn update_validator_metrics(
        &mut self,
        validator: Address,
        uptime_percentage: u8,
        commission_rate: u8,
    ) -> Result<(), StakingError> {
        let mut metrics = self.validators.get(&validator)
            .ok_or(StakingError::ValidatorNotFound)?;
        
        metrics.uptime_percentage = uptime_percentage;
        metrics.commission_rate = commission_rate;
        metrics.last_performance_check = self.env().get_block_time();
        
        if uptime_percentage >= self.min_uptime.get_or_default()
            && commission_rate <= self.max_commission.get_or_default()
        {
            metrics.consecutive_good_epochs += 1;
        } else {
            metrics.consecutive_good_epochs = 0;
        }
        
        // Recalculate risk score
        metrics.risk_score = self.calculate_risk_score(
            uptime_percentage,
            commission_rate,
            metrics.consecutive_good_epochs,
        );
        
        self.validators.set(&validator, metrics);
        
        // Auto-remove if performance drops
        if uptime_percentage < self.min_uptime.get_or_default() {
            self.deregister_validator(validator, "Low uptime".to_string());
        }
        
        Ok(())
    }

    /// Select validators for stake delegation
    /// 
    /// Algorithm optimized for decentralization and risk distribution:
    /// 1. Filter by eligibility (uptime, commission, verified, not at capacity)
    /// 2. Calculate score for each validator (lower stake = higher score)
    /// 3. Sort by score (prioritize underweight validators)
    /// 4. Distribute stake proportionally with caps
    /// 
    /// Returns: Vec of (validator_address, stake_amount)
    pub fn select_validators_for_delegation(
        &self,
        amount_to_stake: U512,
    ) -> Vec<ValidatorAllocation> {
        let active_validators = self.active_validators.get_or_default();
        let min_uptime = self.min_uptime.get_or_default();
        let max_commission = self.max_commission.get_or_default();
        let total_stake = self.total_stake.get_or_default();
        let max_per_validator_pct = self.max_per_validator_pct.get_or_default();
        
        // Step 1: Filter eligible validators
        let mut eligible: Vec<(Address, ValidatorMetrics, u64)> = Vec::new();
        
        for validator_addr in active_validators.iter() {
            if let Some(metrics) = self.validators.get(validator_addr) {
                if metrics.uptime_percentage >= min_uptime
                    && metrics.commission_rate <= max_commission
                    && metrics.is_verified
                    && !self.blacklisted.get(validator_addr).unwrap_or(false)
                {
                    let remaining_capacity = if metrics.current_stake < metrics.max_stake_cap {
                        metrics.max_stake_cap - metrics.current_stake
                    } else {
                        U512::zero()
                    };
                    
                    // Only include if has capacity
                    if remaining_capacity > U512::zero() {
                        let score = self.calculate_decentralization_score(&metrics, total_stake);
                        eligible.push((*validator_addr, metrics, score));
                    }
                }
            }
        }
        
        if eligible.is_empty() {
            return Vec::new();
        }
        
        // Step 2: Sort by score (highest first)
        eligible.sort_by(|a, b| b.2.cmp(&a.2));
        
        // Step 3: Distribute stake
        let mut allocations: Vec<ValidatorAllocation> = Vec::new();
        let mut remaining = amount_to_stake;
        
        let new_total = total_stake + amount_to_stake;
        let max_per_validator = (U256::from(new_total) * U256::from(max_per_validator_pct) / U256::from(100u64))
            .try_into()
            .unwrap_or(U512::MAX);
        
        // First pass: Distribute evenly with caps
        let num_validators = eligible.len();
        let base_allocation = amount_to_stake / U512::from(num_validators);
        
        for (validator, metrics, _score) in eligible.iter() {
            if remaining.is_zero() {
                break;
            }
            
            let capacity_limit = if metrics.current_stake < metrics.max_stake_cap {
                metrics.max_stake_cap - metrics.current_stake
            } else {
                U512::zero()
            };
            
            let percentage_limit = if metrics.current_stake < max_per_validator {
                max_per_validator - metrics.current_stake
            } else {
                U512::zero()
            };
            
            // Take minimum of: base allocation, remaining, capacity limit, percentage limit
            let mut allocation = base_allocation;
            if allocation > remaining {
                allocation = remaining;
            }
            if allocation > capacity_limit {
                allocation = capacity_limit;
            }
            if allocation > percentage_limit {
                allocation = percentage_limit;
            }
            
            if allocation > U512::zero() {
                allocations.push(ValidatorAllocation {
                    validator: *validator,
                    amount: allocation,
                });
                remaining -= allocation;
            }
        }
        
        // Second pass: Distribute any remaining amount
        // (if some validators hit capacity)
        if remaining > U512::zero() {
            let mut round = 0;
            while remaining > U512::zero() && round < 10 {
                // Prevent infinite loop
                let mut distributed_this_round = false;
                
                for alloc in allocations.iter_mut() {
                    if remaining.is_zero() {
                        break;
                    }
                    
                    if let Some(metrics) = self.validators.get(&alloc.validator) {
                        let new_stake = metrics.current_stake + alloc.amount;
                        
                        if new_stake < metrics.max_stake_cap && new_stake < max_per_validator {
                            let additional = U512::from(1_000_000_000u64); // 1 CSPR
                            if additional <= remaining {
                                alloc.amount += additional;
                                remaining -= additional;
                                distributed_this_round = true;
                            }
                        }
                    }
                }
                
                if !distributed_this_round {
                    break; // All validators at capacity
                }
                
                round += 1;
            }
        }
        
        allocations
    }

    /// Calculate risk score for a validator
    /// 
    /// Lower score = lower risk
    /// Factors: uptime, commission, performance history
    fn calculate_risk_score(
        &self,
        uptime: u8,
        commission: u8,
        good_epochs: u32,
    ) -> u8 {
        let mut score: u16 = 0;
        
        // Uptime factor (0-25 points)
        // 100% uptime = 0 risk, 95% = 5 risk, <95% = 25 risk
        if uptime >= 100 {
            score += 0;
        } else if uptime >= 99 {
            score += 1;
        } else if uptime >= 98 {
            score += 3;
        } else if uptime >= 97 {
            score += 5;
        } else if uptime >= 96 {
            score += 10;
        } else if uptime >= 95 {
            score += 15;
        } else {
            score += 25;
        }
        
        // Commission factor (0-25 points)
        // 0% commission = 0 risk, 10% = 10 risk, >10% = 25 risk
        if commission == 0 {
            score += 0;
        } else if commission <= 5 {
            score += (commission as u16) * 1;
        } else if commission <= 10 {
            score += (commission as u16) * 2;
        } else {
            score += 25;
        }
        
        // Performance history factor (0-50 points)
        // More good epochs = lower risk
        if good_epochs >= 100 {
            score += 0;
        } else if good_epochs >= 50 {
            score += 10;
        } else if good_epochs >= 20 {
            score += 20;
        } else if good_epochs >= 10 {
            score += 30;
        } else if good_epochs >= 5 {
            score += 40;
        } else {
            score += 50;
        }
        
        // Cap at 100
        if score > 100 {
            score = 100;
        }
        
        score as u8
    }

    /// Calculate decentralization score
    /// 
    /// Higher score = more desirable for staking (promotes decentralization)
    /// Validators with lower stake get higher scores
    fn calculate_decentralization_score(
        &self,
        metrics: &ValidatorMetrics,
        total_stake: U512,
    ) -> u64 {
        if total_stake.is_zero() {
            return 100_000_000; // High score if no stake yet
        }
        
        let validator_pct = (U256::from(metrics.current_stake) * U256::from(1_000_000u64) / U256::from(total_stake))
            .as_u64();
        
        // Lower percentage = higher score
        // Score = 1_000_000 - (validator_pct * 100)
        let base_score = 1_000_000u64.saturating_sub(validator_pct * 100);
        
        // Boost score for high uptime
        let uptime_boost = (metrics.uptime_percentage as u64) * 1_000;
        
        // Penalty for high commission
        let commission_penalty = (metrics.commission_rate as u64) * 10_000;
        
        // Boost for low risk score
        let risk_boost = (100 - metrics.risk_score as u64) * 1_000;
        
        base_score + uptime_boost + risk_boost - commission_penalty
    }

    /// Update validator stake amount
    pub fn update_validator_stake(
        &mut self,
        validator: Address,
        new_stake: U512,
    ) -> Result<(), StakingError> {
        let mut metrics = self.validators.get(&validator)
            .ok_or(StakingError::ValidatorNotFound)?;
        
        let old_stake = metrics.current_stake;
        metrics.current_stake = new_stake;
        
        self.validators.set(&validator, metrics);
        
        let total = self.total_stake.get_or_default();
        let new_total = total + new_stake - old_stake;
        self.total_stake.set(new_total);
        
        Ok(())
    }

    /// Check if validator is eligible for delegation
    pub fn is_eligible(&self, validator: Address) -> bool {
        if let Some(metrics) = self.validators.get(&validator) {
            metrics.uptime_percentage >= self.min_uptime.get_or_default()
                && metrics.commission_rate <= self.max_commission.get_or_default()
                && metrics.is_verified
                && !self.blacklisted.get(&validator).unwrap_or(false)
        } else {
            false
        }
    }

    /// Blacklist a validator
    pub fn blacklist_validator(&mut self, validator: Address) {
        self.blacklisted.set(&validator, true);
        self.deregister_validator(validator, "Blacklisted due to misbehavior".to_string());
    }

    /// Get validator metrics
    pub fn get_validator_metrics(&self, validator: Address) -> Option<ValidatorMetrics> {
        self.validators.get(&validator)
    }

    /// Get all active validators
    pub fn get_active_validators(&self) -> Vec<Address> {
        self.active_validators.get_or_default()
    }

    /// Get total stake across all validators
    pub fn get_total_stake(&self) -> U512 {
        self.total_stake.get_or_default()
    }

    /// Set configuration
    pub fn set_min_uptime(&mut self, uptime: u8) {
        self.min_uptime.set(uptime);
    }

    pub fn set_max_commission(&mut self, commission: u8) {
        self.max_commission.set(commission);
    }

    pub fn set_max_per_validator_pct(&mut self, pct: u8) {
        self.max_per_validator_pct.set(pct);
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ValidatorAdded {
    pub validator: Address,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ValidatorRemoved {
    pub validator: Address,
}
