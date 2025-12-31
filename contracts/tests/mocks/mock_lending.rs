/// Mock Lending Protocol for Testing
/// 
/// Simulates a lending protocol with interest accrual
/// for testing the LendingStrategy without external dependencies.

use odra::prelude::*;
use odra::types::{Address, U512};

/// Lending position tracking
#[derive(Debug, Clone)]
struct LendingPosition {
    /// Principal supplied
    principal: U512,
    
    /// cTokens minted (receipt tokens)
    c_tokens: U512,
    
    /// Supply timestamp
    supply_time: u64,
    
    /// Interest accrued
    interest_accrued: U512,
}

/// Mock Lending Protocol for testing
#[odra::module]
pub struct MockLending {
    /// User positions (user -> position)
    positions: Mapping<Address, LendingPosition>,
    
    /// Total supplied to the protocol
    total_supplied: Var<U512>,
    
    /// Total borrowed from the protocol
    total_borrowed: Var<U512>,
    
    /// Total cTokens minted
    total_c_tokens: Var<U512>,
    
    /// Configuration
    base_supply_apy_bps: Var<u16>, // Base APY
    utilization_multiplier: Var<u16>, // Utilization impact
    exchange_rate: Var<U512>, // cToken to underlying exchange rate (scaled by 1e18)
    
    /// Last interest update timestamp
    last_update: Var<u64>,
}

#[odra::module]
impl MockLending {
    /// Initialize the mock lending protocol
    pub fn init(&mut self, base_apy_bps: u16) {
        self.base_supply_apy_bps.set(base_apy_bps);
        self.utilization_multiplier.set(1000); // 10% additional per 1% utilization
        self.total_supplied.set(U512::zero());
        self.total_borrowed.set(U512::zero());
        self.total_c_tokens.set(U512::zero());
        
        // Initialize exchange rate to 1:1 (scaled by 1e18)
        self.exchange_rate.set(U512::from(1_000_000_000_000_000_000u128));
        self.last_update.set(0);
    }
    
    /// Supply assets to the lending pool
    /// 
    /// Returns: cTokens minted
    pub fn supply(&mut self, amount: U512) -> U512 {
        let caller = self.env().caller();
        
        if amount.is_zero() {
            return U512::zero();
        }
        
        // Accrue interest before state changes
        self.accrue_interest();
        
        // Calculate cTokens to mint based on current exchange rate
        let exchange_rate = self.exchange_rate.get_or_default();
        let c_tokens = amount
            .checked_mul(U512::from(1_000_000_000_000_000_000u128))
            .and_then(|v| v.checked_div(exchange_rate))
            .unwrap_or(U512::zero());
        
        // Update position
        let mut position = self.positions.get(&caller).unwrap_or(LendingPosition {
            principal: U512::zero(),
            c_tokens: U512::zero(),
            supply_time: self.env().get_block_time(),
            interest_accrued: U512::zero(),
        });
        
        position.principal = position.principal.checked_add(amount).unwrap();
        position.c_tokens = position.c_tokens.checked_add(c_tokens).unwrap();
        self.positions.set(&caller, position);
        
        // Update totals
        let total_supplied = self.total_supplied.get_or_default();
        self.total_supplied.set(total_supplied.checked_add(amount).unwrap());
        
        let total_c = self.total_c_tokens.get_or_default();
        self.total_c_tokens.set(total_c.checked_add(c_tokens).unwrap());
        
        // Emit event
        self.env().emit_event(Supplied {
            user: caller,
            amount,
            c_tokens,
            timestamp: self.env().get_block_time(),
        });
        
        c_tokens
    }
    
    /// Redeem cTokens for underlying assets
    /// 
    /// Returns: Amount redeemed
    pub fn redeem(&mut self, c_tokens: U512) -> U512 {
        let caller = self.env().caller();
        
        if c_tokens.is_zero() {
            return U512::zero();
        }
        
        // Accrue interest before state changes
        self.accrue_interest();
        
        let position = self.positions.get(&caller);
        if position.is_none() {
            return U512::zero();
        }
        
        let mut position = position.unwrap();
        
        if c_tokens > position.c_tokens {
            return U512::zero();
        }
        
        // Calculate underlying amount based on current exchange rate
        let exchange_rate = self.exchange_rate.get_or_default();
        let amount = c_tokens
            .checked_mul(exchange_rate)
            .and_then(|v| v.checked_div(U512::from(1_000_000_000_000_000_000u128)))
            .unwrap_or(U512::zero());
        
        // Update position
        position.c_tokens = position.c_tokens.checked_sub(c_tokens).unwrap();
        position.principal = if position.principal > amount {
            position.principal.checked_sub(amount).unwrap()
        } else {
            U512::zero()
        };
        
        if position.c_tokens.is_zero() {
            self.positions.remove(&caller);
        } else {
            self.positions.set(&caller, position);
        }
        
        // Update totals
        let total_supplied = self.total_supplied.get_or_default();
        self.total_supplied.set(total_supplied.checked_sub(amount).unwrap_or(U512::zero()));
        
        let total_c = self.total_c_tokens.get_or_default();
        self.total_c_tokens.set(total_c.checked_sub(c_tokens).unwrap());
        
        // Emit event
        self.env().emit_event(Redeemed {
            user: caller,
            amount,
            c_tokens,
            timestamp: self.env().get_block_time(),
        });
        
        amount
    }
    
    /// Accrue interest based on time elapsed
    fn accrue_interest(&mut self) {
        let current_time = self.env().get_block_time();
        let last_update = self.last_update.get_or_default();
        
        if last_update == 0 {
            self.last_update.set(current_time);
            return;
        }
        
        let time_elapsed = current_time.saturating_sub(last_update);
        
        if time_elapsed == 0 {
            return;
        }
        
        // Calculate current utilization
        let total_supplied = self.total_supplied.get_or_default();
        let total_borrowed = self.total_borrowed.get_or_default();
        
        if total_supplied.is_zero() {
            self.last_update.set(current_time);
            return;
        }
        
        let utilization_bps = total_borrowed
            .checked_mul(U512::from(10000u64))
            .and_then(|v| v.checked_div(total_supplied))
            .unwrap_or(U512::zero());
        
        // Calculate APY based on utilization
        let base_apy = self.base_supply_apy_bps.get_or_default();
        let multiplier = self.utilization_multiplier.get_or_default();
        
        let utilization_impact = u16::try_from(utilization_bps)
            .unwrap_or(0)
            .saturating_mul(multiplier)
            .saturating_div(10000);
        
        let current_apy_bps = base_apy.saturating_add(utilization_impact);
        
        // Calculate interest accrued
        let seconds_per_year = 31_536_000u64;
        let interest = total_supplied
            .checked_mul(U512::from(current_apy_bps))
            .and_then(|v| v.checked_mul(U512::from(time_elapsed)))
            .and_then(|v| v.checked_div(U512::from(10000u64)))
            .and_then(|v| v.checked_div(U512::from(seconds_per_year)))
            .unwrap_or(U512::zero());
        
        // Update exchange rate (increases over time)
        let exchange_rate = self.exchange_rate.get_or_default();
        let total_c = self.total_c_tokens.get_or_default();
        
        if !total_c.is_zero() {
            let new_total_value = total_supplied.checked_add(interest).unwrap();
            let new_exchange_rate = new_total_value
                .checked_mul(U512::from(1_000_000_000_000_000_000u128))
                .and_then(|v| v.checked_div(total_c))
                .unwrap_or(exchange_rate);
            
            self.exchange_rate.set(new_exchange_rate);
        }
        
        // Update last update time
        self.last_update.set(current_time);
        
        // Emit event
        if !interest.is_zero() {
            self.env().emit_event(InterestAccrued {
                amount: interest,
                timestamp: current_time,
            });
        }
    }
    
    /// Get user's position
    pub fn get_position(&self, user: Address) -> Option<(U512, U512, U512)> {
        self.positions.get(&user).map(|p| (p.principal, p.c_tokens, p.interest_accrued))
    }
    
    /// Get current balance (including accrued interest)
    pub fn get_balance(&self, user: Address) -> U512 {
        if let Some(position) = self.positions.get(&user) {
            let exchange_rate = self.exchange_rate.get_or_default();
            position.c_tokens
                .checked_mul(exchange_rate)
                .and_then(|v| v.checked_div(U512::from(1_000_000_000_000_000_000u128)))
                .unwrap_or(position.principal)
        } else {
            U512::zero()
        }
    }
    
    /// Get pool utilization rate (in basis points)
    pub fn get_utilization(&self) -> u16 {
        let total_supplied = self.total_supplied.get_or_default();
        let total_borrowed = self.total_borrowed.get_or_default();
        
        if total_supplied.is_zero() {
            return 0;
        }
        
        let utilization = total_borrowed
            .checked_mul(U512::from(10000u64))
            .and_then(|v| v.checked_div(total_supplied))
            .unwrap_or(U512::zero());
        
        u16::try_from(utilization).unwrap_or(0)
    }
    
    /// Get current supply APY (in basis points)
    pub fn get_supply_apy(&self) -> u16 {
        let base_apy = self.base_supply_apy_bps.get_or_default();
        let utilization = self.get_utilization();
        let multiplier = self.utilization_multiplier.get_or_default();
        
        let utilization_impact = utilization
            .saturating_mul(multiplier)
            .saturating_div(10000);
        
        base_apy.saturating_add(utilization_impact)
    }
    
    /// Get total supplied to the protocol
    pub fn get_total_supplied(&self) -> U512 {
        self.total_supplied.get_or_default()
    }
    
    /// Get total borrowed from the protocol
    pub fn get_total_borrowed(&self) -> U512 {
        self.total_borrowed.get_or_default()
    }
    
    /// Get current exchange rate (cToken to underlying)
    pub fn get_exchange_rate(&self) -> U512 {
        self.exchange_rate.get_or_default()
    }
    
    /// Simulate borrowing (for utilization testing)
    pub fn simulate_borrow(&mut self, amount: U512) {
        let total_borrowed = self.total_borrowed.get_or_default();
        self.total_borrowed.set(total_borrowed.checked_add(amount).unwrap());
    }
    
    /// Simulate repayment (for utilization testing)
    pub fn simulate_repay(&mut self, amount: U512) {
        let total_borrowed = self.total_borrowed.get_or_default();
        self.total_borrowed.set(total_borrowed.checked_sub(amount).unwrap_or(U512::zero()));
    }
}

// ============================================
// EVENTS
// ============================================

#[odra::event]
struct Supplied {
    user: Address,
    amount: U512,
    c_tokens: U512,
    timestamp: u64,
}

#[odra::event]
struct Redeemed {
    user: Address,
    amount: U512,
    c_tokens: U512,
    timestamp: u64,
}

#[odra::event]
struct InterestAccrued {
    amount: U512,
    timestamp: u64,
}
