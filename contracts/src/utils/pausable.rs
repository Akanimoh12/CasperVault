use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::VaultError;

/// Pausable functionality for emergency situations
/// 
/// This module allows authorized users to pause critical functions
/// in case of security issues or other emergencies.
#[odra::module]
pub struct Pausable {
    /// Whether the contract is currently paused
    paused: Var<bool>,
}

#[odra::module]
impl Pausable {
    /// Initialize pausable state (unpaused by default)
    pub fn init(&mut self) {
        self.paused.set(false);
    }

    /// Pause the contract
    /// Should only be called by authorized roles (Guardian or Admin)
    pub fn pause(&mut self) {
        if self.is_paused() {
            self.env().revert(VaultError::Paused);
        }
        
        self.paused.set(true);
        
        self.env().emit_event(Paused {
            by: self.env().caller(),
            timestamp: self.env().get_block_time(),
        });
    }

    /// Unpause the contract
    /// Should only be called by Admin
    pub fn unpause(&mut self) {
        if !self.is_paused() {
            self.env().revert(VaultError::NotPaused);
        }
        
        self.paused.set(false);
        
        self.env().emit_event(Unpaused {
            by: self.env().caller(),
            timestamp: self.env().get_block_time(),
        });
    }

    /// Check if the contract is paused
    pub fn is_paused(&self) -> bool {
        self.paused.get_or_default()
    }

    /// Modifier: Require contract to not be paused
    pub fn when_not_paused(&self) {
        if self.is_paused() {
            self.env().revert(VaultError::Paused);
        }
    }

    /// Modifier: Require contract to be paused
    pub fn when_paused(&self) {
        if !self.is_paused() {
            self.env().revert(VaultError::NotPaused);
        }
    }
}

#[derive(Event)]
struct Paused {
    by: Address,
    timestamp: u64,
}

#[derive(Event)]
struct Unpaused {
    by: Address,
    timestamp: u64,
}
