use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::TokenError;

/// lstCSPR - Liquid Staking Token for Casper
/// 
/// This token represents staked CSPR and can be used in DeFi protocols
/// while the underlying CSPR continues to earn staking rewards.
/// 
/// Initially mints 1:1 with staked CSPR, but the exchange rate may
/// change over time as rewards accrue.
#[odra::module]
pub struct LstCspr {
    /// Token name
    name: Var<String>,
    /// Token symbol
    symbol: Var<String>,
    /// Token decimals
    decimals: Var<u8>,
    /// Total supply of lstCSPR
    total_supply: Var<U512>,
    /// Balances mapping
    balances: Mapping<Address, U512>,
    /// Allowances mapping (owner -> spender -> amount)
    allowances: Mapping<(Address, Address), U512>,
    /// Minter address (LiquidStaking contract)
    minter: Var<Address>,
}

/// Events for lstCSPR token
#[derive(Event)]
pub struct Transfer {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub amount: U512,
}

#[derive(Event)]
pub struct Approval {
    pub owner: Address,
    pub spender: Address,
    pub amount: U512,
}

#[odra::module]
impl LstCspr {
    /// Initialize the lstCSPR token
    pub fn init(&mut self, minter: Address) {
        self.name.set("Liquid Staked CSPR".to_string());
        self.symbol.set("lstCSPR".to_string());
        self.decimals.set(9); // Same as CSPR
        self.total_supply.set(U512::zero());
        self.minter.set(minter);
    }

    /// Get token name
    pub fn name(&self) -> String {
        self.name.get_or_default()
    }

    /// Get token symbol
    pub fn symbol(&self) -> String {
        self.symbol.get_or_default()
    }

    /// Get token decimals
    pub fn decimals(&self) -> u8 {
        self.decimals.get_or_default()
    }

    /// Get total supply
    pub fn total_supply(&self) -> U512 {
        self.total_supply.get_or_default()
    }

    /// Get balance of an account
    pub fn balance_of(&self, account: Address) -> U512 {
        self.balances.get(&account).unwrap_or(U512::zero())
    }

    /// Transfer tokens
    pub fn transfer(&mut self, to: Address, amount: U512) {
        let from = self.env().caller();
        self._transfer(from, to, amount);
    }

    /// Approve spender to spend tokens
    pub fn approve(&mut self, spender: Address, amount: U512) {
        let owner = self.env().caller();
        self.allowances.set(&(owner, spender), amount);
        
        self.env().emit_event(Approval {
            owner,
            spender,
            amount,
        });
    }

    /// Transfer tokens from one account to another (requires allowance)
    pub fn transfer_from(&mut self, from: Address, to: Address, amount: U512) {
        let spender = self.env().caller();
        
        let allowance = self.allowances.get(&(from, spender)).unwrap_or(U512::zero());
        if allowance < amount {
            self.env().revert(TokenError::AllowanceExceeded);
        }
        
        self.allowances.set(&(from, spender), allowance - amount);
        
        self._transfer(from, to, amount);
    }

    /// Get allowance
    pub fn allowance(&self, owner: Address, spender: Address) -> U512 {
        self.allowances.get(&(owner, spender)).unwrap_or(U512::zero())
    }

    /// Mint tokens (only callable by minter)
    pub fn mint(&mut self, to: Address, amount: U512) {
        // Only minter can mint
        let caller = self.env().caller();
        let minter = self.minter.get().unwrap_or_else(|| self.env().revert(TokenError::InsufficientTokenBalance));
        if caller != minter {
            self.env().revert(TokenError::InsufficientTokenBalance); // Use generic error
        }
        
        if amount.is_zero() {
            self.env().revert(TokenError::ZeroMintAmount);
        }
        
        let balance = self.balance_of(to);
        self.balances.set(&to, balance + amount);
        
        let supply = self.total_supply();
        self.total_supply.set(supply + amount);
        
        self.env().emit_event(Transfer {
            from: None,
            to: Some(to),
            amount,
        });
    }

    /// Burn tokens (only callable by minter)
    pub fn burn(&mut self, from: Address, amount: U512) {
        // Only minter can burn
        let caller = self.env().caller();
        let minter = self.minter.get().unwrap_or_else(|| self.env().revert(TokenError::InsufficientTokenBalance));
        if caller != minter {
            self.env().revert(TokenError::InsufficientTokenBalance); // Use generic error
        }
        
        if amount.is_zero() {
            self.env().revert(TokenError::ZeroBurnAmount);
        }
        
        let balance = self.balance_of(from);
        if balance < amount {
            self.env().revert(TokenError::InsufficientTokenBalance);
        }
        
        self.balances.set(&from, balance - amount);
        
        let supply = self.total_supply();
        self.total_supply.set(supply - amount);
        
        self.env().emit_event(Transfer {
            from: Some(from),
            to: None,
            amount,
        });
    }

    /// Internal transfer function
    fn _transfer(&mut self, from: Address, to: Address, amount: U512) {
        if amount.is_zero() {
            self.env().revert(TokenError::ZeroTransferAmount);
        }
        
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            self.env().revert(TokenError::InsufficientTokenBalance);
        }
        
        self.balances.set(&from, from_balance - amount);
        
        let to_balance = self.balance_of(to);
        self.balances.set(&to, to_balance + amount);
        
        self.env().emit_event(Transfer {
            from: Some(from),
            to: Some(to),
            amount,
        });
    }
}
