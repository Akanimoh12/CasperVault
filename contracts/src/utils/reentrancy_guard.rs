use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::VaultError;

/// Reentrancy guard to prevent reentrancy attacks
/// 
/// This module prevents recursive calls to functions by tracking
/// the execution status. Functions protected by this guard cannot
/// be called recursively.
#[odra::module]
pub struct ReentrancyGuard {
    /// Status flag: 1 = NOT_ENTERED, 2 = ENTERED
    status: Var<u8>,
}

/// Constants for reentrancy status
const NOT_ENTERED: u8 = 1;
const ENTERED: u8 = 2;

#[odra::module]
impl ReentrancyGuard {
    /// Initialize the reentrancy guard
    pub fn init(&mut self) {
        self.status.set(NOT_ENTERED);
    }

    /// Enter the guarded section
    /// 
    /// This should be called at the beginning of any function that
    /// needs reentrancy protection. It will revert if already entered.
    pub fn enter(&mut self) {
        let current_status = self.status.get_or_default();
        
        if current_status == ENTERED {
            self.env().revert(VaultError::ReentrancyGuard);
        }
        
        // Set status to ENTERED
        self.status.set(ENTERED);
    }

    /// Exit the guarded section
    /// 
    /// This should be called at the end of any function that called enter().
    /// It resets the status to allow future calls.
    pub fn exit(&mut self) {
        self.status.set(NOT_ENTERED);
    }

    /// Check if currently in a guarded section
    pub fn is_entered(&self) -> bool {
        self.status.get_or_default() == ENTERED
    }
}

/// Macro to use reentrancy guard in a function
/// 
/// Usage:
/// ```ignore
/// use_reentrancy_guard!(self, {
///     // Your function logic here
/// });
/// ```
#[macro_export]
macro_rules! use_reentrancy_guard {
    ($self:expr, $body:block) => {{
        $self.reentrancy_guard.enter();
        let result = $body;
        $self.reentrancy_guard.exit();
        result
    }};
}
