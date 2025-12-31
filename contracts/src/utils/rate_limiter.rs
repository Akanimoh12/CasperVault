use odra::prelude::*;
use odra::{Address, Event, Mapping, Var};
use odra::casper_types::U512;
use crate::types::VaultError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimit {
    pub max_per_tx: U512,
    pub max_per_day: U512,
    pub current_day_total: U512,
    pub last_reset: u64,
}

#[odra::module]
pub struct RateLimiter {
    per_user_limits: Mapping<Address, RateLimit>,
    max_per_tx: Var<U512>,
    max_per_day_per_user: Var<U512>,
    global_hourly_deposits: Var<U512>,
    global_hourly_withdrawals: Var<U512>,
    last_hourly_reset: Var<u64>,
    max_global_deposits_per_hour: Var<U512>,
    max_global_withdrawals_per_hour: Var<U512>,
}

#[odra::module]
impl RateLimiter {
    pub fn init(&mut self) {
        self.max_per_tx.set(U512::from(10_000_000_000_000u64));
        self.max_per_day_per_user.set(U512::from(50_000_000_000_000u64));
        self.max_global_deposits_per_hour.set(U512::from(1_000_000_000_000_000u64));
        self.max_global_withdrawals_per_hour.set(U512::from(500_000_000_000_000u64));
        self.global_hourly_deposits.set(U512::zero());
        self.global_hourly_withdrawals.set(U512::zero());
        self.last_hourly_reset.set(self.env().get_block_time());
    }
    
    pub fn check_deposit_limit(&mut self, user: Address, amount: U512) -> Result<(), VaultError> {
        let max_tx = self.max_per_tx.get_or_default();
        if amount > max_tx {
            return Err(VaultError::RateLimitExceeded);
        }
        
        self.reset_if_needed();
        
        let current_time = self.env().get_block_time();
        let mut user_limit = self.per_user_limits.get(&user).unwrap_or(RateLimit {
            max_per_tx,
            max_per_day: self.max_per_day_per_user.get_or_default(),
            current_day_total: U512::zero(),
            last_reset: current_time,
        });
        
        if current_time >= user_limit.last_reset + 86400 {
            user_limit.current_day_total = U512::zero();
            user_limit.last_reset = current_time;
        }
        
        let new_total = user_limit.current_day_total + amount;
        if new_total > user_limit.max_per_day {
            return Err(VaultError::RateLimitExceeded);
        }
        
        let global_deposits = self.global_hourly_deposits.get_or_default();
        let max_global = self.max_global_deposits_per_hour.get_or_default();
        if global_deposits + amount > max_global {
            return Err(VaultError::RateLimitExceeded);
        }
        
        user_limit.current_day_total = new_total;
        self.per_user_limits.set(&user, user_limit);
        self.global_hourly_deposits.set(global_deposits + amount);
        
        Ok(())
    }
    
    pub fn check_withdrawal_limit(&mut self, amount: U512) -> Result<(), VaultError> {
        self.reset_if_needed();
        
        let global_withdrawals = self.global_hourly_withdrawals.get_or_default();
        let max_global = self.max_global_withdrawals_per_hour.get_or_default();
        
        if global_withdrawals + amount > max_global {
            return Err(VaultError::RateLimitExceeded);
        }
        
        self.global_hourly_withdrawals.set(global_withdrawals + amount);
        Ok(())
    }
    
    fn reset_if_needed(&mut self) {
        let current_time = self.env().get_block_time();
        let last_reset = self.last_hourly_reset.get_or_default();
        
        if current_time >= last_reset + 3600 {
            self.global_hourly_deposits.set(U512::zero());
            self.global_hourly_withdrawals.set(U512::zero());
            self.last_hourly_reset.set(current_time);
        }
    }
    
    pub fn set_max_per_tx(&mut self, amount: U512) {
        self.max_per_tx.set(amount);
    }
    
    pub fn set_max_per_day(&mut self, amount: U512) {
        self.max_per_day_per_user.set(amount);
    }
    
    pub fn get_user_remaining_daily_limit(&self, user: Address) -> U512 {
        let current_time = self.env().get_block_time();
        let user_limit = self.per_user_limits.get(&user).unwrap_or(RateLimit {
            max_per_tx: self.max_per_tx.get_or_default(),
            max_per_day: self.max_per_day_per_user.get_or_default(),
            current_day_total: U512::zero(),
            last_reset: current_time,
        });
        
        if current_time >= user_limit.last_reset + 86400 {
            return user_limit.max_per_day;
        }
        
        if user_limit.current_day_total >= user_limit.max_per_day {
            U512::zero()
        } else {
            user_limit.max_per_day - user_limit.current_day_total
        }
    }
}
