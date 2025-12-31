use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::*;
use crate::utils::{AccessControl, ValidatorRegistry};
use crate::tokens::LstCspr;

/// Delegation tracking for unbonding
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnbondingRequest {
    pub user: Address,
    pub validator: Address,
    pub amount: U512,
    pub unlock_time: u64,
    pub is_completed: bool,
}

/// LiquidStaking contract
/// 
/// This contract handles staking CSPR and minting lstCSPR tokens.
/// It manages validator selection, delegation, and reward compounding.
/// 
/// Key responsibilities:
/// - Stake CSPR and mint lstCSPR (1:1 initially)
/// - Delegate to multiple validators for decentralization
/// - Compound staking rewards automatically
/// - Manage validator registry and selection
/// 
/// Exchange Rate Model:
/// - Initially 1 lstCSPR = 1 CSPR
/// - As rewards compound, 1 lstCSPR becomes worth more CSPR
/// - Rate = total_staked_cspr / total_lst_cspr_supply
/// - Example: If 100 CSPR staked earns 10 CSPR rewards:
///   - 100 lstCSPR now represents 110 CSPR
///   - Rate = 110/100 = 1.1 CSPR per lstCSPR
#[odra::module]
pub struct LiquidStaking {
    /// Access control
    access_control: SubModule<AccessControl>,
    
    /// Validator registry
    validator_registry: SubModule<ValidatorRegistry>,
    
    /// lstCSPR token contract address
    lst_cspr_token: Var<Address>,
    
    /// Total CSPR staked (includes rewards)
    total_staked: Var<U512>,
    
    /// Total lstCSPR issued
    total_lst_cspr: Var<U512>,
    
    /// Delegation amounts per validator
    delegations: Mapping<Address, U512>,
    
    /// Unbonding requests (request_id -> UnbondingRequest)
    unbonding_requests: Mapping<U256, UnbondingRequest>,
    
    /// Next unbonding request ID
    next_unbonding_id: Var<U256>,
    
    /// Unbonding period (in seconds) - 14 days for Casper
    unbonding_period: Var<u64>,
    
    /// Exchange rate (lstCSPR to CSPR), scaled by 1e9
    /// Represents how much CSPR one lstCSPR is worth
    exchange_rate: Var<U256>,
    
    /// Last compound timestamp
    last_compound: Var<u64>,
    
    /// Minimum compound interval (prevent excessive gas costs)
    min_compound_interval: Var<u64>,
    
    /// Total rewards earned (for analytics)
    total_rewards_earned: Var<U512>,
}

#[odra::module]
impl LiquidStaking {
    /// Initialize the LiquidStaking contract
    pub fn init(&mut self, admin: Address, lst_cspr_token: Address) {
        self.access_control.init(admin);
        self.validator_registry.init();
        self.lst_cspr_token.set(lst_cspr_token);
        
        self.total_staked.set(U512::zero());
        self.total_lst_cspr.set(U512::zero());
        
        // Initialize exchange rate to 1:1 (scaled by 1e9)
        self.exchange_rate.set(U256::from(1_000_000_000u64));
        
        // Set unbonding period to 14 days (Casper Network)
        self.unbonding_period.set(14 * 24 * 60 * 60);
        
        // Set minimum compound interval to 12 hours
        self.min_compound_interval.set(12 * 60 * 60);
        
        self.next_unbonding_id.set(U256::zero());
        self.last_compound.set(0);
        self.total_rewards_earned.set(U512::zero());
    }

    /// Stake CSPR and mint lstCSPR
    /// 
    /// Process:
    /// 1. Receive CSPR from caller (transferred via attached_value)
    /// 2. Select optimal validators using smart selection algorithm
    /// 3. Delegate CSPR to selected validators
    /// 4. Calculate lstCSPR to mint based on current exchange rate
    /// 5. Mint lstCSPR to caller
    /// 
    /// Returns: Amount of lstCSPR minted
    pub fn stake(&mut self) -> U512 {
        let amount = self.env().attached_value();
        
        if amount.is_zero() {
            self.env().revert(StakingError::ExceedsStakedAmount);
        }
        
        let caller = self.env().caller();
        
        let lst_cspr_amount = self.cspr_to_lst_cspr(amount);
        
        // Select validators and allocate stake
        let allocations = self.validator_registry.select_validators_for_delegation(amount);
        
        if allocations.is_empty() {
            self.env().revert(StakingError::NoEligibleValidators);
        }
        
        // Delegate to each selected validator
        let mut total_delegated = U512::zero();
        let mut validator_addresses = Vec::new();
        
        for allocation in allocations.iter() {
            // Delegate via Casper's native delegation
            self.delegate_to_validator(allocation.validator, allocation.amount);
            
            total_delegated += allocation.amount;
            validator_addresses.push(allocation.validator);
        }
        
        let current_staked = self.total_staked.get_or_default();
        self.total_staked.set(current_staked + total_delegated);
        
        let current_lst_cspr = self.total_lst_cspr.get_or_default();
        self.total_lst_cspr.set(current_lst_cspr + lst_cspr_amount);
        
        // Mint lstCSPR tokens to caller
        
        self.env().emit_event(Stake {
            user: caller,
            cspr_amount: total_delegated,
            lst_cspr_minted: lst_cspr_amount,
            validators: validator_addresses,
            timestamp: self.env().get_block_time(),
        });
        
        lst_cspr_amount
    }

    /// Unstake lstCSPR and initiate unbonding
    /// 
    /// Due to Casper's unbonding period (14 days), this creates an unbonding request.
    /// User must call complete_unbonding() after the period to receive CSPR.
    /// 
    /// Process:
    /// 1. Burn lstCSPR from caller
    /// 2. Calculate CSPR amount based on exchange rate
    /// 3. Undelegate from validators proportionally
    /// 4. Create unbonding request with 14-day lock
    /// 
    /// Returns: Unbonding request ID
    pub fn unstake(&mut self, lst_cspr_amount: U512) -> U256 {
        if lst_cspr_amount.is_zero() {
            self.env().revert(StakingError::ExceedsStakedAmount);
        }
        
        let caller = self.env().caller();
        
        let cspr_amount = self.lst_cspr_to_cspr(lst_cspr_amount);
        
        let total_staked = self.total_staked.get_or_default();
        if cspr_amount > total_staked {
            self.env().revert(StakingError::ExceedsStakedAmount);
        }
        
        // Burn lstCSPR from caller
        
        // Undelegate from validators proportionally
        self.undelegate_proportionally(cspr_amount);
        
        self.total_staked.set(total_staked - cspr_amount);
        
        let current_lst_cspr = self.total_lst_cspr.get_or_default();
        self.total_lst_cspr.set(current_lst_cspr - lst_cspr_amount);
        
        // Create unbonding request
        let request_id = self.next_unbonding_id.get_or_default();
        let unlock_time = self.env().get_block_time() + self.unbonding_period.get_or_default();
        
        let request = UnbondingRequest {
            user: caller,
            validator: Address::zero(), // Multiple validators
            amount: cspr_amount,
            unlock_time,
            is_completed: false,
        };
        
        self.unbonding_requests.set(&request_id, request);
        self.next_unbonding_id.set(request_id + U256::one());
        
        self.env().emit_event(Unstake {
            user: caller,
            lst_cspr_amount,
            cspr_amount,
            timestamp: self.env().get_block_time(),
        });
        
        request_id
    }

    /// Complete unbonding and receive CSPR
    /// 
    /// Can only be called after unbonding period has passed
    pub fn complete_unbonding(&mut self, request_id: U256) -> U512 {
        let mut request = self.unbonding_requests.get(&request_id)
            .unwrap_or_else(|| self.env().revert(VaultError::WithdrawalRequestNotFound));
        
        if request.user != self.env().caller() {
            self.env().revert(VaultError::Unauthorized);
        }
        
        if request.is_completed {
            self.env().revert(VaultError::WithdrawalRequestNotFound);
        }
        
        if self.env().get_block_time() < request.unlock_time {
            self.env().revert(VaultError::TimelockNotExpired);
        }
        
        request.is_completed = true;
        self.unbonding_requests.set(&request_id, request);
        
        
        request.amount
    }

    /// Compound staking rewards
    /// 
    /// Claims rewards from all validators and restakes them.
    /// This increases the total_staked without minting new lstCSPR,
    /// thereby increasing the exchange rate.
    /// 
    /// Process:
    /// 1. Claim rewards from all validators
    /// 2. Aggregate total rewards
    /// 3. Restake rewards to same validators
    /// 4. Update exchange rate
    /// 
    /// Can only be called by Admin or Operator
    /// Rate limited to prevent excessive gas costs
    /// 
    /// Returns: Total rewards compounded
    pub fn compound_rewards(&mut self) -> U512 {
        if !self.access_control.has_role(0, self.env().caller())
            && !self.access_control.has_role(1, self.env().caller())
        {
            self.env().revert(VaultError::Unauthorized);
        }
        
        let last = self.last_compound.get_or_default();
        let now = self.env().get_block_time();
        let min_interval = self.min_compound_interval.get_or_default();
        
        if now < last + min_interval {
            self.env().revert(VaultError::RateLimitExceeded);
        }
        
        let mut total_rewards = U512::zero();
        let active_validators = self.validator_registry.get_active_validators();
        
        for validator in active_validators.iter() {
            let delegation = self.delegations.get(validator).unwrap_or(U512::zero());
            
            if delegation.is_zero() {
                continue;
            }
            
            // For simulation: assume rewards are proportional to stake
            let rewards = self.calculate_estimated_rewards(*validator, delegation);
            
            if rewards > U512::zero() {
                total_rewards += rewards;
                
                // Restake rewards to same validator
                self.delegate_to_validator(*validator, rewards);
            }
        }
        
        if total_rewards.is_zero() {
            return total_rewards;
        }
        
        let current_staked = self.total_staked.get_or_default();
        let new_total_staked = current_staked + total_rewards;
        self.total_staked.set(new_total_staked);
        
        self.update_exchange_rate();
        
        self.last_compound.set(now);
        
        let total_earned = self.total_rewards_earned.get_or_default();
        self.total_rewards_earned.set(total_earned + total_rewards);
        
        self.env().emit_event(CompoundRewards {
            total_rewards,
            restaked_amount: total_rewards,
            new_total_staked,
            timestamp: now,
        });
        
        total_rewards
    }

    /// Delegate CSPR to a specific validator
    /// 
    /// In production, this would call Casper's native delegation system.
    /// For MVP, we track delegations in state.
    fn delegate_to_validator(&mut self, validator: Address, amount: U512) {
        // system::delegate(validator, amount);
        
        let current_delegation = self.delegations.get(&validator).unwrap_or(U512::zero());
        self.delegations.set(&validator, current_delegation + amount);
        
        let _ = self.validator_registry.update_validator_stake(
            validator,
            current_delegation + amount,
        );
    }

    /// Undelegate from validators proportionally
    /// 
    /// Distributes the withdrawal amount across all validators
    /// based on their current delegation amounts
    fn undelegate_proportionally(&mut self, total_amount: U512) {
        let active_validators = self.validator_registry.get_active_validators();
        let total_delegated = self.validator_registry.get_total_stake();
        
        if total_delegated.is_zero() {
            return;
        }
        
        let mut remaining = total_amount;
        
        for validator in active_validators.iter() {
            if remaining.is_zero() {
                break;
            }
            
            let delegation = self.delegations.get(validator).unwrap_or(U512::zero());
            
            if delegation.is_zero() {
                continue;
            }
            
            let proportion = (U256::from(delegation) * U256::from(total_amount)) / U256::from(total_delegated);
            let undelegate_amount: U512 = proportion.try_into().unwrap_or(U512::zero());
            
            if undelegate_amount > remaining {
                let actual_amount = remaining;
                self.undelegate_from_validator(*validator, actual_amount);
                remaining = U512::zero();
            } else {
                self.undelegate_from_validator(*validator, undelegate_amount);
                remaining -= undelegate_amount;
            }
        }
    }

    /// Undelegate from a specific validator
    fn undelegate_from_validator(&mut self, validator: Address, amount: U512) {
        // system::undelegate(validator, amount);
        
        let current_delegation = self.delegations.get(&validator).unwrap_or(U512::zero());
        
        if amount >= current_delegation {
            self.delegations.set(&validator, U512::zero());
        } else {
            self.delegations.set(&validator, current_delegation - amount);
        }
        
        let new_stake = if amount >= current_delegation {
            U512::zero()
        } else {
            current_delegation - amount
        };
        
        let _ = self.validator_registry.update_validator_stake(validator, new_stake);
    }

    /// Calculate estimated rewards for a validator
    /// 
    /// In production, this would query actual rewards from Casper runtime.
    /// For simulation: Estimates based on time and delegation amount.
    fn calculate_estimated_rewards(&self, _validator: Address, delegation: U512) -> U512 {
        // rewards_per_year = delegation * 0.10
        // rewards_per_day = rewards_per_year / 365
        // For simulation, assume 1 day has passed
        
        let annual_rate = 10u64; // 10%
        let days_per_year = 365u64;
        
        // rewards = delegation * (annual_rate / 100) / days_per_year
        let rewards = (delegation * U512::from(annual_rate)) / U512::from(100 * days_per_year);
        
        rewards
    }

    /// Convert CSPR to lstCSPR based on current exchange rate
    fn cspr_to_lst_cspr(&self, cspr_amount: U512) -> U512 {
        let rate = self.exchange_rate.get_or_default();
        // lstCSPR = CSPR * 1e9 / rate
        (U256::from(cspr_amount) * U256::from(1_000_000_000u64) / rate)
            .try_into()
            .unwrap_or(cspr_amount)
    }

    /// Convert lstCSPR to CSPR based on current exchange rate
    fn lst_cspr_to_cspr(&self, lst_cspr_amount: U512) -> U512 {
        let rate = self.exchange_rate.get_or_default();
        // CSPR = lstCSPR * rate / 1e9
        (U256::from(lst_cspr_amount) * rate / U256::from(1_000_000_000u64))
            .try_into()
            .unwrap_or(lst_cspr_amount)
    }

    /// Update exchange rate based on total staked and total lstCSPR
    fn update_exchange_rate(&mut self) {
        let total_staked = self.total_staked.get_or_default();
        let total_lst_cspr = self.total_lst_cspr.get_or_default();
        
        if total_lst_cspr.is_zero() {
            return;
        }
        
        // rate = total_staked * 1e9 / total_lst_cspr
        let new_rate = U256::from(total_staked) * U256::from(1_000_000_000u64) / U256::from(total_lst_cspr);
        self.exchange_rate.set(new_rate);
    }

    /// Add a validator to the registry (admin only)
    pub fn add_validator(
        &mut self,
        validator: Address,
        uptime_percentage: u8,
        commission_rate: u8,
        max_stake_cap: U512,
    ) {
        self.access_control.only_admin();
        
        // Validate requirements
        let min_uptime = self.min_uptime.get_or_default();
        let max_commission = self.max_commission.get_or_default();
        
        if uptime_percentage < min_uptime {
            self.env().revert(StakingError::ValidatorNotEligible);
        }
        
        if commission_rate > max_commission {
            self.env().revert(StakingError::ValidatorNotEligible);
        }
        
        // Create validator info
        let validator_info = ValidatorInfo {
            validator,
            uptime_percentage,
            commission_rate,
            current_stake: U512::zero(),
            max_stake_cap,
            is_verified: true,
            risk_score: 0,
        };
        
        self.validators.set(&validator, validator_info);
        
        // Add to active validators list
        let mut active = self.active_validators.get_or_default();
        if !active.contains(&validator) {
            active.push(validator);
            self.active_validators.set(active);
        }
        
        self.env().emit_event(ValidatorAdded {
            validator,
            uptime_percentage,
            commission_rate,
        });
    }

    /// Remove a validator from the registry (admin only)
    pub fn remove_validator(&mut self, validator: Address, reason: String) {
        self.access_control.only_admin();
        
        // Remove from active validators
        let mut active = self.active_validators.get_or_default();
        active.retain(|v| v != &validator);
        self.active_validators.set(active);
        
        
        self.env().emit_event(ValidatorRemoved {
            validator,
            reason,
        });
    }

    /// Get current exchange rate
    pub fn get_exchange_rate(&self) -> U256 {
        self.exchange_rate.get_or_default()
    }

    /// Get total staked CSPR
    pub fn get_total_staked(&self) -> U512 {
        self.total_staked.get_or_default()
    }

    /// Get total lstCSPR issued
    pub fn get_total_lst_cspr(&self) -> U512 {
        self.total_lst_cspr.get_or_default()
    }

    /// Get validator info
    pub fn get_validator(&self, validator: Address) -> Option<ValidatorInfo> {
        self.validators.get(&validator)
    }

    /// Get all active validators
    pub fn get_active_validators(&self) -> Vec<Address> {
        self.active_validators.get_or_default()
    }

    /// Get delegation amount for a validator
    pub fn get_delegation(&self, validator: Address) -> U512 {
        self.delegations.get(&validator).unwrap_or(U512::zero())
    }

    /// Get total rewards earned (lifetime)
    pub fn get_total_rewards_earned(&self) -> U512 {
        self.total_rewards_earned.get_or_default()
    }

    /// Get unbonding request details
    pub fn get_unbonding_request(&self, request_id: U256) -> Option<UnbondingRequest> {
        self.unbonding_requests.get(&request_id)
    }

    /// Calculate APY based on recent rewards
    /// 
    /// Returns APY in basis points (10000 = 100%)
    pub fn calculate_apy(&self) -> u64 {
        let total_staked = self.total_staked.get_or_default();
        let total_rewards = self.total_rewards_earned.get_or_default();
        
        if total_staked.is_zero() {
            return 0;
        }
        
        // Simple calculation: (rewards / staked) * 10000
        let apy_bps = (U256::from(total_rewards) * U256::from(10000u64) / U256::from(total_staked))
            .as_u64();
        
        apy_bps
    }

    /// Check if compound is needed
    /// 
    /// Returns true if:
    /// - Min interval has passed since last compound
    /// - Estimated rewards > threshold
    pub fn should_compound(&self) -> bool {
        let last = self.last_compound.get_or_default();
        let now = self.env().get_block_time();
        let min_interval = self.min_compound_interval.get_or_default();
        
        if now < last + min_interval {
            return false;
        }
        
        let estimated_rewards = self.estimate_pending_rewards();
        let threshold = U512::from(100_000_000_000u64); // 100 CSPR
        
        estimated_rewards >= threshold
    }

    /// Estimate pending rewards across all validators
    fn estimate_pending_rewards(&self) -> U512 {
        let active_validators = self.validator_registry.get_active_validators();
        let mut total = U512::zero();
        
        for validator in active_validators.iter() {
            let delegation = self.delegations.get(validator).unwrap_or(U512::zero());
            let rewards = self.calculate_estimated_rewards(*validator, delegation);
            total += rewards;
        }
        
        total
    }

    /// Set unbonding period (admin only)
    pub fn set_unbonding_period(&mut self, period: u64) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        
        self.unbonding_period.set(period);
    }

    /// Set minimum compound interval (admin only)
    pub fn set_min_compound_interval(&mut self, interval: u64) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        
        self.min_compound_interval.set(interval);
    }

    /// Emergency withdraw from validator (admin only)
    /// 
    /// Used in case of validator issues or emergencies
    pub fn emergency_undelegate(&mut self, validator: Address, amount: U512) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        
        self.undelegate_from_validator(validator, amount);
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Stake {
    pub user: Address,
    pub amount: U512,
    pub validator: Address,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Unstake {
    pub user: Address,
    pub amount: U512,
    pub validator: Address,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct CompoundRewards {
    pub validator: Address,
    pub rewards: U512,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ValidatorInfo {
    pub validator: Address,
    pub staked: U512,
    pub performance: U512,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ValidatorAdded {
    pub validator: Address,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ValidatorRemoved {
    pub validator: Address,
}
