/// Mock DEX for testing yield aggregator swap functionality
/// Simulates token swapping with configurable rates

use odra::prelude::*;
use odra::{Address, Event, Mapping, Var};
use odra::casper_types::U512;
use crate::types::*;

/// Mock DEX for testing token swaps
#[odra::module]
pub struct MockDEX {
    /// Exchange rate for token A to token B (scaled by 1e9)
    /// Default 1:1 = 1_000_000_000
    exchange_rate: Var<U512>,
    
    /// Slippage percentage (basis points, e.g., 100 = 1%)
    slippage_bps: Var<u32>,
    
    /// Liquidity reserves for token A
    reserve_a: Var<U512>,
    
    /// Liquidity reserves for token B
    reserve_b: Var<U512>,
    
    /// Trading fee (basis points, e.g., 30 = 0.3%)
    trading_fee_bps: Var<u32>,
    
    /// Total fees collected
    fees_collected: Var<U512>,
    
    /// Is paused
    paused: Var<bool>,
}

#[odra::module]
impl MockDEX {
    /// Initialize the mock DEX
    pub fn init(&mut self) {
        // Default 1:1 exchange rate
        self.exchange_rate.set(U512::from(1_000_000_000u64));
        
        // Default 1% slippage
        self.slippage_bps.set(100);
        
        // Default 0.3% trading fee
        self.trading_fee_bps.set(30);
        
        // Initial liquidity: 1M tokens each
        self.reserve_a.set(U512::from(1_000_000_000_000_000u64));
        self.reserve_b.set(U512::from(1_000_000_000_000_000u64));
        
        self.fees_collected.set(U512::zero());
        self.paused.set(false);
    }
    
    /// Swap token A for token B
    pub fn swap_a_to_b(&mut self, amount_in: U512) -> U512 {
        if self.paused.get_or_default() {
            self.env().revert(VaultError::ContractPaused);
        }
        
        if amount_in == U512::zero() {
            self.env().revert(VaultError::ZeroAmount);
        }
        
        let fee_bps = self.trading_fee_bps.get_or_default();
        let fee = amount_in * U512::from(fee_bps) / U512::from(10000u64);
        let amount_after_fee = amount_in - fee;
        
        let current_fees = self.fees_collected.get_or_default();
        self.fees_collected.set(current_fees + fee);
        
        let rate = self.exchange_rate.get_or_default();
        let amount_out = amount_after_fee * rate / U512::from(1_000_000_000u64);
        
        let slippage = self.slippage_bps.get_or_default();
        let slippage_amount = amount_out * U512::from(slippage) / U512::from(10000u64);
        let final_amount = amount_out - slippage_amount;
        
        let reserve_a = self.reserve_a.get_or_default();
        let reserve_b = self.reserve_b.get_or_default();
        
        if final_amount > reserve_b {
            self.env().revert(VaultError::InsufficientLiquidity);
        }
        
        self.reserve_a.set(reserve_a + amount_after_fee);
        self.reserve_b.set(reserve_b - final_amount);
        
        self.env().emit_event(SwapExecuted {
            token_in: Address::from([0u8; 32]), // Mock address
            token_out: Address::from([1u8; 32]), // Mock address
            amount_in,
            amount_out: final_amount,
            fee,
        });
        
        final_amount
    }
    
    /// Swap token B for token A
    pub fn swap_b_to_a(&mut self, amount_in: U512) -> U512 {
        if self.paused.get_or_default() {
            self.env().revert(VaultError::ContractPaused);
        }
        
        if amount_in == U512::zero() {
            self.env().revert(VaultError::ZeroAmount);
        }
        
        let fee_bps = self.trading_fee_bps.get_or_default();
        let fee = amount_in * U512::from(fee_bps) / U512::from(10000u64);
        let amount_after_fee = amount_in - fee;
        
        let current_fees = self.fees_collected.get_or_default();
        self.fees_collected.set(current_fees + fee);
        
        let rate = self.exchange_rate.get_or_default();
        let amount_out = amount_after_fee * U512::from(1_000_000_000u64) / rate;
        
        let slippage = self.slippage_bps.get_or_default();
        let slippage_amount = amount_out * U512::from(slippage) / U512::from(10000u64);
        let final_amount = amount_out - slippage_amount;
        
        let reserve_a = self.reserve_a.get_or_default();
        let reserve_b = self.reserve_b.get_or_default();
        
        if final_amount > reserve_a {
            self.env().revert(VaultError::InsufficientLiquidity);
        }
        
        self.reserve_b.set(reserve_b + amount_after_fee);
        self.reserve_a.set(reserve_a - final_amount);
        
        self.env().emit_event(SwapExecuted {
            token_in: Address::from([1u8; 32]),
            token_out: Address::from([0u8; 32]),
            amount_in,
            amount_out: final_amount,
            fee,
        });
        
        final_amount
    }
    
    /// Get quote for swapping A to B
    pub fn get_quote_a_to_b(&self, amount_in: U512) -> U512 {
        if amount_in == U512::zero() {
            return U512::zero();
        }
        
        let fee_bps = self.trading_fee_bps.get_or_default();
        let fee = amount_in * U512::from(fee_bps) / U512::from(10000u64);
        let amount_after_fee = amount_in - fee;
        
        let rate = self.exchange_rate.get_or_default();
        let amount_out = amount_after_fee * rate / U512::from(1_000_000_000u64);
        
        let slippage = self.slippage_bps.get_or_default();
        let slippage_amount = amount_out * U512::from(slippage) / U512::from(10000u64);
        
        amount_out - slippage_amount
    }
    
    /// Add liquidity to the pool
    pub fn add_liquidity(&mut self, amount_a: U512, amount_b: U512) {
        let reserve_a = self.reserve_a.get_or_default();
        let reserve_b = self.reserve_b.get_or_default();
        
        self.reserve_a.set(reserve_a + amount_a);
        self.reserve_b.set(reserve_b + amount_b);
        
        self.env().emit_event(LiquidityAdded {
            amount_a,
            amount_b,
            provider: self.env().caller(),
        });
    }
    
    /// Get current reserves
    pub fn get_reserves(&self) -> (U512, U512) {
        (
            self.reserve_a.get_or_default(),
            self.reserve_b.get_or_default(),
        )
    }
    
    /// Get total fees collected
    pub fn get_fees_collected(&self) -> U512 {
        self.fees_collected.get_or_default()
    }
    
    /// Admin: Set exchange rate
    pub fn set_exchange_rate(&mut self, rate: U512) {
        self.exchange_rate.set(rate);
    }
    
    /// Admin: Set slippage
    pub fn set_slippage(&mut self, slippage_bps: u32) {
        if slippage_bps > 1000 { // Max 10% slippage
            self.env().revert(VaultError::InvalidFee);
        }
        self.slippage_bps.set(slippage_bps);
    }
    
    /// Admin: Set trading fee
    pub fn set_trading_fee(&mut self, fee_bps: u32) {
        if fee_bps > 1000 { // Max 10% fee
            self.env().revert(VaultError::InvalidFee);
        }
        self.trading_fee_bps.set(fee_bps);
    }
    
    /// Admin: Pause DEX
    pub fn pause(&mut self) {
        self.paused.set(true);
    }
    
    /// Admin: Unpause DEX
    pub fn unpause(&mut self) {
        self.paused.set(false);
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SwapExecuted {
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: U512,
    pub amount_out: U512,
    pub fee: U512,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct LiquidityAdded {
    pub amount_a: U512,
    pub amount_b: U512,
    pub provider: Address,
}
