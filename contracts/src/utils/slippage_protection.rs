use odra::prelude::*;
use odra::{Address, Event, Mapping, Var};
use odra::casper_types::U512;
use crate::types::VaultError;

#[odra::module]
pub struct SlippageProtection {
    max_slippage_bps: Var<u32>,
}

#[odra::module]
impl SlippageProtection {
    pub fn init(&mut self) {
        self.max_slippage_bps.set(100);
    }
    
    pub fn check_slippage(&self, expected: U512, actual: U512) {
        if expected == U512::zero() {
            return;
        }
        
        let max_slippage = self.max_slippage_bps.get_or_default();
        let slippage_threshold = expected * U512::from(max_slippage) / U512::from(10000u64);
        
        let deviation = if actual > expected {
            actual - expected
        } else {
            expected - actual
        };
        
        if deviation > slippage_threshold {
            self.env().emit_event(SlippageExceeded {
                expected,
                actual,
                max_slippage_bps: max_slippage,
                timestamp: self.env().get_block_time(),
            });
            self.env().revert(VaultError::SlippageExceeded);
        }
    }
    
    pub fn set_max_slippage(&mut self, slippage_bps: u32) {
        if slippage_bps > 1000 {
            self.env().revert(VaultError::InvalidFee);
        }
        self.max_slippage_bps.set(slippage_bps);
    }
    
    pub fn get_max_slippage(&self) -> u32 {
        self.max_slippage_bps.get_or_default()
    }
    
    pub fn calculate_min_amount_out(&self, amount_in: U512) -> U512 {
        let max_slippage = self.max_slippage_bps.get_or_default();
        let slippage_amount = amount_in * U512::from(max_slippage) / U512::from(10000u64);
        amount_in - slippage_amount
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SlippageExceeded {
    pub expected: U512,
    pub actual: U512,
    pub max_slippage_bps: u32,
    pub timestamp: u64,
}
