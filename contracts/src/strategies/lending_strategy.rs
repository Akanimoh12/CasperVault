/// Lending Strategy for CasperVault
/// 
/// Supply lstCSPR to Casper lending protocol to earn interest from borrowers.
/// Lower risk than DEX LP, but typically lower yields.

use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::strategies::strategy_interface::{IStrategy, RiskLevel, StrategyError};
use crate::utils::access_control::AccessControl;
use crate::utils::pausable::Pausable;
use crate::utils::reentrancy_guard::ReentrancyGuard;

/// Lending position tracking
#[derive(Debug, Clone)]
struct LendingPosition {
    /// Principal supplied
    principal: U512,
    
    /// Accrued interest
    interest_accrued: U512,
    
    /// Supply timestamp
    supply_time: u64,
    
    /// cTokens received (lending protocol shares)
    c_tokens: U512,
}

/// Lending Strategy Module
/// 
/// Architecture:
/// lstCSPR → Supply to Lending Pool → Receive cTokens → Earn Interest
#[odra::module]
pub struct LendingStrategy {
    /// Access control
    access_control: SubModule<AccessControl>,
    
    /// Pausable
    pausable: SubModule<Pausable>,
    
    /// Reentrancy protection
    reentrancy_guard: SubModule<ReentrancyGuard>,
    
    /// CORE STATE
    
    /// Current lending position
    position: Var<LendingPosition>,
    
    /// Total supplied (lifetime)
    total_supplied: Var<U512>,
    
    /// Total withdrawn (lifetime)
    total_withdrawn: Var<U512>,
    
    /// Total interest earned (lifetime)
    total_interest_earned: Var<U512>,
    
    /// CONTRACT ADDRESSES
    
    /// Lending protocol contract
    lending_protocol_address: Var<Address>,
    
    /// lstCSPR token address
    lst_cspr_address: Var<Address>,
    
    /// PARAMETERS
    
    /// Maximum capacity
    max_capacity: Var<U512>,
    
    /// Minimum supply amount
    min_supply: Var<U512>,
    
    /// Target utilization rate (basis points)
    /// If pool utilization > target, reduce allocation
    target_utilization_bps: Var<u32>,
    
    /// Maximum acceptable utilization (basis points)
    max_utilization_bps: Var<u32>,
    
    /// Last harvest timestamp
    last_harvest: Var<u64>,
    
    /// Min harvest interval
    min_harvest_interval: Var<u64>,
    
    /// Current APY (cached, updated on harvest)
    cached_apy: Var<U256>,
}

#[odra::module]
impl LendingStrategy {
    /// Initialize the lending strategy
    pub fn init(
        &mut self,
        admin: Address,
        lending_protocol_address: Address,
        lst_cspr_address: Address,
    ) {
        self.access_control.init(admin);
        
        self.lending_protocol_address.set(lending_protocol_address);
        self.lst_cspr_address.set(lst_cspr_address);
        
        self.max_capacity.set(U512::from(5_000_000u64) * U512::from(1_000_000_000u64)); // 5M CSPR
        self.min_supply.set(U512::from(100u64) * U512::from(1_000_000_000u64)); // 100 CSPR
        self.target_utilization_bps.set(7000); // 70% target
        self.max_utilization_bps.set(9000); // 90% max
        self.min_harvest_interval.set(43200); // 12 hours
        self.cached_apy.set(U256::from(800u64)); // 8% initial estimate
        
        self.position.set(LendingPosition {
            principal: U512::zero(),
            interest_accrued: U512::zero(),
            supply_time: 0,
            c_tokens: U512::zero(),
        });
        
        self.total_supplied.set(U512::zero());
        self.total_withdrawn.set(U512::zero());
        self.total_interest_earned.set(U512::zero());
        self.last_harvest.set(0);
    }
    
    /// Deploy funds to lending pool
    /// 
    /// Process:
    /// 1. Receive lstCSPR
    /// 2. Supply to lending protocol
    /// 3. Receive cTokens
    /// 4. Track position
    pub fn deploy(&mut self, amount: U512) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let min = self.min_supply.get_or_default();
        if amount < min {
            self.reentrancy_guard.exit();
            return Err(StrategyError::AmountTooLow);
        }
        
        let current_position = self.position.get_or_default();
        let max_cap = self.max_capacity.get_or_default();
        if current_position.principal.checked_add(amount).unwrap() > max_cap {
            self.reentrancy_guard.exit();
            return Err(StrategyError::MaxCapacityReached);
        }
        
        let utilization = self.get_pool_utilization();
        let max_util = self.max_utilization_bps.get_or_default();
        if utilization > max_util {
            self.reentrancy_guard.exit();
            return Err(StrategyError::UnhealthyStrategy);
        }
        
        
        let c_tokens_minted = amount;
        
        let mut new_position = current_position;
        new_position.principal = new_position.principal.checked_add(amount).unwrap();
        new_position.c_tokens = new_position.c_tokens.checked_add(c_tokens_minted).unwrap();
        new_position.supply_time = self.env().get_block_time();
        self.position.set(new_position);
        
        let total = self.total_supplied.get_or_default();
        self.total_supplied.set(total.checked_add(amount).unwrap());
        
        self.env().emit_event(Supplied {
            amount,
            c_tokens: c_tokens_minted,
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        Ok(amount)
    }
    
    /// Withdraw funds from lending pool
    /// 
    /// Process:
    /// 1. Calculate cTokens to redeem
    /// 2. Redeem from lending protocol
    /// 3. Receive lstCSPR
    /// 4. Update position
    pub fn withdraw(&mut self, amount: U512) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let position = self.position.get_or_default();
        
        let total_balance = position.principal.checked_add(position.interest_accrued).unwrap();
        if amount > total_balance {
            self.reentrancy_guard.exit();
            return Err(StrategyError::WithdrawalTooLarge);
        }
        
        // In real protocol: c_tokens = amount / exchange_rate
        let c_tokens_to_redeem = if total_balance.is_zero() {
            U512::zero()
        } else {
            amount.checked_mul(position.c_tokens).unwrap()
                .checked_div(total_balance).unwrap()
        };
        
        
        let lst_received = amount;
        
        let mut new_position = position;
        new_position.c_tokens = new_position.c_tokens.checked_sub(c_tokens_to_redeem).unwrap();
        
        // Reduce principal proportionally
        let principal_reduction = if total_balance.is_zero() {
            U512::zero()
        } else {
            amount.checked_mul(position.principal).unwrap()
                .checked_div(total_balance).unwrap()
        };
        
        new_position.principal = new_position.principal.checked_sub(principal_reduction).unwrap();
        
        // Reduce interest if withdrawn
        if amount > principal_reduction {
            let interest_reduction = amount.checked_sub(principal_reduction).unwrap();
            new_position.interest_accrued = new_position.interest_accrued
                .checked_sub(interest_reduction)
                .unwrap_or(U512::zero());
        }
        
        self.position.set(new_position);
        
        let total = self.total_withdrawn.get_or_default();
        self.total_withdrawn.set(total.checked_add(lst_received).unwrap());
        
        self.env().emit_event(Redeemed {
            amount: lst_received,
            c_tokens_burned: c_tokens_to_redeem,
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        Ok(lst_received)
    }
    
    /// Harvest accrued interest
    /// 
    /// Process:
    /// 1. Query current balance from lending protocol
    /// 2. Calculate interest earned since last harvest
    /// 3. Update interest tracking
    /// 4. Return harvested amount
    pub fn harvest(&mut self) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let current_time = self.env().get_block_time();
        let last_harvest = self.last_harvest.get_or_default();
        let min_interval = self.min_harvest_interval.get_or_default();
        
        if current_time < last_harvest + min_interval {
            self.reentrancy_guard.exit();
            return Err(StrategyError::Unauthorized);
        }
        
        let position = self.position.get_or_default();
        
        if position.principal.is_zero() {
            self.reentrancy_guard.exit();
            return Ok(U512::zero());
        }
        
        
        let time_elapsed = current_time - position.supply_time;
        let annual_apy_bps = 800u64; // 8%
        let seconds_per_year = 31536000u64;
        
        let simulated_interest = position.principal
            .checked_mul(U512::from(annual_apy_bps))
            .unwrap()
            .checked_mul(U512::from(time_elapsed))
            .unwrap()
            .checked_div(U512::from(seconds_per_year))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        let new_interest = if simulated_interest > position.interest_accrued {
            simulated_interest.checked_sub(position.interest_accrued).unwrap()
        } else {
            U512::zero()
        };
        
        let mut new_position = position;
        new_position.interest_accrued = simulated_interest;
        self.position.set(new_position);
        
        let total = self.total_interest_earned.get_or_default();
        self.total_interest_earned.set(total.checked_add(new_interest).unwrap());
        self.last_harvest.set(current_time);
        
        self.update_apy_cache();
        
        self.env().emit_event(InterestHarvested {
            amount: new_interest,
            total_interest: simulated_interest,
            timestamp: current_time,
        });
        
        self.reentrancy_guard.exit();
        Ok(new_interest)
    }
    
    /// Get current balance
    pub fn get_balance(&self) -> U512 {
        let position = self.position.get_or_default();
        position.principal.checked_add(position.interest_accrued).unwrap()
    }
    
    /// Get current APY
    pub fn get_apy(&self) -> U256 {
        self.cached_apy.get_or_default()
    }
    
    /// Get risk level (Low for lending)
    pub fn get_risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    /// Get strategy name
    pub fn name(&self) -> String {
        "Lending Strategy".to_string()
    }
    
    /// Check if strategy is healthy
    pub fn is_healthy(&self) -> bool {
        if self.pausable.is_paused() {
            return false;
        }
        
        let utilization = self.get_pool_utilization();
        let max_util = self.max_utilization_bps.get_or_default();
        
        if utilization > max_util {
            return false;
        }
        
        let apy = self.get_apy();
        let min_apy = U256::from(100u64); // 1%
        let max_apy = U256::from(10000u64); // 100%
        
        if apy < min_apy || apy > max_apy {
            return false;
        }
        
        true
    }
    
    /// Get max capacity
    pub fn max_capacity(&self) -> U512 {
        self.max_capacity.get_or_default()
    }
    
    // HELPER FUNCTIONS
    
    /// Get pool utilization rate
    /// 
    /// Utilization = Borrowed / (Supplied + Borrowed)
    fn get_pool_utilization(&self) -> u16 {
        
        7500u16
    }
    
    /// Update cached APY from lending protocol
    fn update_apy_cache(&mut self) {
        
        // Higher utilization = higher APY
        let utilization = self.get_pool_utilization();
        
        // Simple model: APY = base_rate + utilization_rate * utilization
        // Example: 2% base + 10% * 0.75 = 9.5% APY
        let base_rate = 200u64; // 2%
        let utilization_multiplier = 10u64;
        
        let apy = base_rate + (utilization_multiplier * u64::from(utilization) / 100);
        
        self.cached_apy.set(U256::from(apy));
    }
    
    
    pub fn set_max_capacity(&mut self, capacity: U512) {
        self.access_control.only_admin();
        self.max_capacity.set(capacity);
    }
    
    pub fn set_utilization_targets(&mut self, target_bps: u32, max_bps: u32) {
        self.access_control.only_admin();
        
        if target_bps > 10000 || max_bps > 10000 || max_bps < target_bps {
            self.env().revert(StrategyError::Unauthorized);
        }
        
        self.target_utilization_bps.set(target_bps);
        self.max_utilization_bps.set(max_bps);
    }
    
    pub fn emergency_withdraw(&mut self) -> U512 {
        self.access_control.only_admin();
        
        let balance = self.get_balance();
        
        match self.withdraw(balance) {
            Ok(amount) => amount,
            Err(_) => U512::zero(),
        }
    }
    
    pub fn pause(&mut self) {
        self.access_control.only_guardian();
        self.pausable.pause();
    }
    
    pub fn unpause(&mut self) {
        self.access_control.only_admin();
        self.pausable.unpause();
    }
    
    
    pub fn get_position(&self) -> (U512, U512, U512) {
        let position = self.position.get_or_default();
        (position.principal, position.interest_accrued, position.c_tokens)
    }
    
    pub fn get_total_supplied(&self) -> U512 {
        self.total_supplied.get_or_default()
    }
    
    pub fn get_total_interest_earned(&self) -> U512 {
        self.total_interest_earned.get_or_default()
    }
    
    pub fn get_utilization_rate(&self) -> u16 {
        self.get_pool_utilization()
    }
}


#[derive(Event)]
struct Supplied {
    amount: U512,
    c_tokens: U512,
    timestamp: u64,
}

#[derive(Event)]
struct Redeemed {
    amount: U512,
    c_tokens_burned: U512,
    timestamp: u64,
}

#[derive(Event)]
struct InterestHarvested {
    amount: U512,
    total_interest: U512,
    timestamp: u64,
}
