use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::AccessError;

/// Role definitions for access control
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Administrator role - full control over the system
    Admin = 0,
    /// Operator role - can rebalance strategies and harvest yields
    Operator = 1,
    /// Guardian role - can only emergency pause
    Guardian = 2,
    /// Keeper role - can trigger compounding
    Keeper = 3,
}

impl Role {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Role::Admin),
            1 => Some(Role::Operator),
            2 => Some(Role::Guardian),
            3 => Some(Role::Keeper),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Access control module for role-based permissions
#[odra::module]
pub struct AccessControl {
    /// Mapping of role -> account -> has_role
    roles: Mapping<(u8, Address), bool>,
    /// Count of admins (to prevent removing last admin)
    admin_count: Var<u32>,
}

#[odra::module]
impl AccessControl {
    /// Initialize access control with initial admin
    pub fn init(&mut self, initial_admin: Address) {
        let admin_role = Role::Admin.to_u8();
        self.roles.set(&(admin_role, initial_admin), true);
        self.admin_count.set(1);
        
        self.env().emit_event(RoleGranted {
            role: admin_role,
            account: initial_admin,
            grantor: initial_admin,
        });
    }

    /// Grant a role to an account
    /// Can only be called by an admin
    pub fn grant_role(&mut self, role: u8, account: Address) {
        self.only_admin();
        
        Role::from_u8(role).unwrap_or_else(|| {
            self.env().revert(AccessError::InvalidRole)
        });
        
        if !self.has_role(role, account) {
            self.roles.set(&(role, account), true);
            
            // Increment admin count if granting admin role
            if role == Role::Admin.to_u8() {
                let count = self.admin_count.get_or_default();
                self.admin_count.set(count + 1);
            }
            
            self.env().emit_event(RoleGranted {
                role,
                account,
                grantor: self.env().caller(),
            });
        }
    }

    /// Revoke a role from an account
    /// Can only be called by an admin
    pub fn revoke_role(&mut self, role: u8, account: Address) {
        self.only_admin();
        
        Role::from_u8(role).unwrap_or_else(|| {
            self.env().revert(AccessError::InvalidRole)
        });
        
        if role == Role::Admin.to_u8() {
            let count = self.admin_count.get_or_default();
            if count <= 1 {
                self.env().revert(AccessError::CannotRenounceLastAdmin);
            }
        }
        
        if self.has_role(role, account) {
            self.roles.set(&(role, account), false);
            
            // Decrement admin count if revoking admin role
            if role == Role::Admin.to_u8() {
                let count = self.admin_count.get_or_default();
                self.admin_count.set(count - 1);
            }
            
            self.env().emit_event(RoleRevoked {
                role,
                account,
                revoker: self.env().caller(),
            });
        }
    }

    /// Renounce a role (caller gives up their own role)
    pub fn renounce_role(&mut self, role: u8) {
        let caller = self.env().caller();
        
        Role::from_u8(role).unwrap_or_else(|| {
            self.env().revert(AccessError::InvalidRole)
        });
        
        if role == Role::Admin.to_u8() {
            let count = self.admin_count.get_or_default();
            if count <= 1 {
                self.env().revert(AccessError::CannotRenounceLastAdmin);
            }
        }
        
        if self.has_role(role, caller) {
            self.roles.set(&(role, caller), false);
            
            // Decrement admin count if renouncing admin role
            if role == Role::Admin.to_u8() {
                let count = self.admin_count.get_or_default();
                self.admin_count.set(count - 1);
            }
            
            self.env().emit_event(RoleRenounced {
                role,
                account: caller,
            });
        }
    }

    /// Check if an account has a specific role
    pub fn has_role(&self, role: u8, account: Address) -> bool {
        self.roles.get(&(role, account)).unwrap_or(false)
    }

    /// Modifier: Only admin can call
    pub fn only_admin(&self) {
        let caller = self.env().caller();
        if !self.has_role(Role::Admin.to_u8(), caller) {
            self.env().revert(AccessError::MissingRole);
        }
    }

    /// Modifier: Only operator can call
    pub fn only_operator(&self) {
        let caller = self.env().caller();
        if !self.has_role(Role::Operator.to_u8(), caller) {
            self.env().revert(AccessError::MissingRole);
        }
    }

    /// Modifier: Only guardian can call
    pub fn only_guardian(&self) {
        let caller = self.env().caller();
        if !self.has_role(Role::Guardian.to_u8(), caller) {
            self.env().revert(AccessError::MissingRole);
        }
    }

    /// Modifier: Only keeper can call
    pub fn only_keeper(&self) {
        let caller = self.env().caller();
        if !self.has_role(Role::Keeper.to_u8(), caller) {
            self.env().revert(AccessError::MissingRole);
        }
    }

    /// Modifier: Only admin or operator can call
    pub fn only_admin_or_operator(&self) {
        let caller = self.env().caller();
        let is_admin = self.has_role(Role::Admin.to_u8(), caller);
        let is_operator = self.has_role(Role::Operator.to_u8(), caller);
        
        if !is_admin && !is_operator {
            self.env().revert(AccessError::MissingRole);
        }
    }

    /// Get the number of admins
    pub fn get_admin_count(&self) -> u32 {
        self.admin_count.get_or_default()
    }
}

#[derive(Event)]
struct RoleGranted {
    role: u8,
    account: Address,
    grantor: Address,
}

#[derive(Event)]
struct RoleRevoked {
    role: u8,
    account: Address,
    revoker: Address,
}

#[derive(Event)]
struct RoleRenounced {
    role: u8,
    account: Address,
}
