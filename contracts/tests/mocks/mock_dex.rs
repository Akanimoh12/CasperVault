/// Mock DEX Contract for Testing
/// 
/// Simulates a decentralized exchange with configurable APY
/// for testing the DEXStrategy without external dependencies.

use odra::prelude::*;
use odra::types::{Address, U512};

/// LP Position tracking
#[derive(Debug, Clone)]
struct LPPosition {
    /// LP tokens minted
    lp_tokens: U512,
    
    /// lstCSPR amount
    lst_cspr_amount: U512,
    
    /// CSPR amount
    cspr_amount: U512,
    
    /// Deposit timestamp
    deposit_time: u64,
}

/// Mock DEX for testing
#[odra::module]
pub struct MockDEX {
    /// LP positions (user -> position)
    positions: Mapping<Address, LPPosition>,
    
    /// Staked LP tokens (user -> amount)
    staked_lp: Mapping<Address, U512>,
    
    /// Total LP supply
    total_lp_supply: Var<U512>,
    
    /// Pool reserves
    lst_cspr_reserve: Var<U512>,
    cspr_reserve: Var<U512>,
    
    /// Configuration
    target_apy_bps: Var<u16>, // In basis points
    trading_fee_bps: Var<u16>, // 30 = 0.3%
    
    /// Rewards accumulated (user -> amount)
    rewards: Mapping<Address, U512>,
    
    /// Last harvest time (user -> timestamp)
    last_harvest: Mapping<Address, u64>,
}

#[odra::module]
impl MockDEX {
    /// Initialize the mock DEX
    pub fn init(&mut self, target_apy_bps: u16) {
        self.target_apy_bps.set(target_apy_bps);
        self.trading_fee_bps.set(30); // 0.3%
        self.total_lp_supply.set(U512::zero());
        self.lst_cspr_reserve.set(U512::from(1_000_000_000u64)); // 1M initial liquidity
        self.cspr_reserve.set(U512::from(1_000_000_000u64));
    }
    
    /// Add liquidity to the pool
    /// 
    /// Returns: (LP tokens minted, lstCSPR used, CSPR used)
    pub fn add_liquidity(&mut self, lst_cspr_amount: U512, cspr_amount: U512) -> (U512, U512, U512) {
        let caller = self.env().caller();
        
        if lst_cspr_amount.is_zero() || cspr_amount.is_zero() {
            return (U512::zero(), U512::zero(), U512::zero());
        }
        
        // Calculate LP tokens to mint
        let total_supply = self.total_lp_supply.get_or_default();
        let lp_tokens = if total_supply.is_zero() {
            // First liquidity provider
            lst_cspr_amount
                .checked_mul(cspr_amount)
                .and_then(|v| v.integer_sqrt())
                .unwrap_or(U512::zero())
        } else {
            // Proportional to existing pool
            let lst_reserve = self.lst_cspr_reserve.get_or_default();
            let cspr_reserve = self.cspr_reserve.get_or_default();
            
            let lp_from_lst = lst_cspr_amount
                .checked_mul(total_supply)
                .and_then(|v| v.checked_div(lst_reserve))
                .unwrap_or(U512::zero());
            
            let lp_from_cspr = cspr_amount
                .checked_mul(total_supply)
                .and_then(|v| v.checked_div(cspr_reserve))
                .unwrap_or(U512::zero());
            
            if lp_from_lst < lp_from_cspr {
                lp_from_lst
            } else {
                lp_from_cspr
            }
        };
        
        // Update reserves
        let new_lst_reserve = self.lst_cspr_reserve.get_or_default()
            .checked_add(lst_cspr_amount)
            .unwrap();
        let new_cspr_reserve = self.cspr_reserve.get_or_default()
            .checked_add(cspr_amount)
            .unwrap();
        
        self.lst_cspr_reserve.set(new_lst_reserve);
        self.cspr_reserve.set(new_cspr_reserve);
        
        // Update total supply
        self.total_lp_supply.set(total_supply.checked_add(lp_tokens).unwrap());
        
        // Store position
        let position = LPPosition {
            lp_tokens,
            lst_cspr_amount,
            cspr_amount,
            deposit_time: self.env().get_block_time(),
        };
        self.positions.set(&caller, position);
        
        // Emit event
        self.env().emit_event(LiquidityAdded {
            user: caller,
            lst_cspr_amount,
            cspr_amount,
            lp_tokens,
            timestamp: self.env().get_block_time(),
        });
        
        (lp_tokens, lst_cspr_amount, cspr_amount)
    }
    
    /// Remove liquidity from the pool
    /// 
    /// Returns: (lstCSPR amount, CSPR amount)
    pub fn remove_liquidity(&mut self, lp_tokens: U512) -> (U512, U512) {
        let caller = self.env().caller();
        
        if lp_tokens.is_zero() {
            return (U512::zero(), U512::zero());
        }
        
        let position = self.positions.get(&caller);
        if position.is_none() {
            return (U512::zero(), U512::zero());
        }
        
        let total_supply = self.total_lp_supply.get_or_default();
        if total_supply.is_zero() {
            return (U512::zero(), U512::zero());
        }
        
        // Calculate amounts to return
        let lst_reserve = self.lst_cspr_reserve.get_or_default();
        let cspr_reserve = self.cspr_reserve.get_or_default();
        
        let lst_amount = lp_tokens
            .checked_mul(lst_reserve)
            .and_then(|v| v.checked_div(total_supply))
            .unwrap_or(U512::zero());
        
        let cspr_amount = lp_tokens
            .checked_mul(cspr_reserve)
            .and_then(|v| v.checked_div(total_supply))
            .unwrap_or(U512::zero());
        
        // Update reserves
        self.lst_cspr_reserve.set(lst_reserve.checked_sub(lst_amount).unwrap());
        self.cspr_reserve.set(cspr_reserve.checked_sub(cspr_amount).unwrap());
        
        // Update total supply
        self.total_lp_supply.set(total_supply.checked_sub(lp_tokens).unwrap());
        
        // Clear position
        self.positions.remove(&caller);
        
        // Emit event
        self.env().emit_event(LiquidityRemoved {
            user: caller,
            lst_cspr_amount: lst_amount,
            cspr_amount,
            lp_tokens,
            timestamp: self.env().get_block_time(),
        });
        
        (lst_amount, cspr_amount)
    }
    
    /// Stake LP tokens for rewards
    pub fn stake_lp(&mut self, lp_amount: U512) {
        let caller = self.env().caller();
        
        let current = self.staked_lp.get(&caller).unwrap_or(U512::zero());
        self.staked_lp.set(&caller, current.checked_add(lp_amount).unwrap());
        
        // Initialize harvest time
        if self.last_harvest.get(&caller).is_none() {
            self.last_harvest.set(&caller, self.env().get_block_time());
        }
        
        self.env().emit_event(LPStaked {
            user: caller,
            amount: lp_amount,
            timestamp: self.env().get_block_time(),
        });
    }
    
    /// Unstake LP tokens
    pub fn unstake_lp(&mut self, lp_amount: U512) {
        let caller = self.env().caller();
        
        let current = self.staked_lp.get(&caller).unwrap_or(U512::zero());
        if lp_amount > current {
            return;
        }
        
        self.staked_lp.set(&caller, current.checked_sub(lp_amount).unwrap());
        
        self.env().emit_event(LPUnstaked {
            user: caller,
            amount: lp_amount,
            timestamp: self.env().get_block_time(),
        });
    }
    
    /// Claim trading fees and mining rewards
    /// 
    /// Returns: (trading fees, mining rewards)
    pub fn claim_rewards(&mut self) -> (U512, U512) {
        let caller = self.env().caller();
        
        let staked = self.staked_lp.get(&caller).unwrap_or(U512::zero());
        if staked.is_zero() {
            return (U512::zero(), U512::zero());
        }
        
        let last_harvest_time = self.last_harvest.get(&caller).unwrap_or(0);
        let current_time = self.env().get_block_time();
        let time_elapsed = current_time.saturating_sub(last_harvest_time);
        
        if time_elapsed == 0 {
            return (U512::zero(), U512::zero());
        }
        
        // Calculate APY-based rewards
        let apy_bps = self.target_apy_bps.get_or_default();
        
        // Calculate total yield
        let seconds_per_year = 31_536_000u64;
        let yield_amount = staked
            .checked_mul(U512::from(apy_bps))
            .and_then(|v| v.checked_mul(U512::from(time_elapsed)))
            .and_then(|v| v.checked_div(U512::from(10000u64)))
            .and_then(|v| v.checked_div(U512::from(seconds_per_year)))
            .unwrap_or(U512::zero());
        
        // Split 50/50 between trading fees and mining rewards
        let trading_fees = yield_amount.checked_div(U512::from(2u64)).unwrap_or(U512::zero());
        let mining_rewards = yield_amount.checked_sub(trading_fees).unwrap_or(U512::zero());
        
        // Update last harvest time
        self.last_harvest.set(&caller, current_time);
        
        // Store rewards
        let current_rewards = self.rewards.get(&caller).unwrap_or(U512::zero());
        self.rewards.set(&caller, current_rewards.checked_add(yield_amount).unwrap());
        
        // Emit event
        self.env().emit_event(RewardsClaimed {
            user: caller,
            trading_fees,
            mining_rewards,
            timestamp: current_time,
        });
        
        (trading_fees, mining_rewards)
    }
    
    /// Get user's LP position
    pub fn get_position(&self, user: Address) -> Option<(U512, U512, U512)> {
        self.positions.get(&user).map(|p| (p.lp_tokens, p.lst_cspr_amount, p.cspr_amount))
    }
    
    /// Get user's staked LP amount
    pub fn get_staked_lp(&self, user: Address) -> U512 {
        self.staked_lp.get(&user).unwrap_or(U512::zero())
    }
    
    /// Get pool reserves
    pub fn get_reserves(&self) -> (U512, U512) {
        (
            self.lst_cspr_reserve.get_or_default(),
            self.cspr_reserve.get_or_default(),
        )
    }
    
    /// Get total LP supply
    pub fn get_total_supply(&self) -> U512 {
        self.total_lp_supply.get_or_default()
    }
}

// ============================================
// EVENTS
// ============================================

#[odra::event]
struct LiquidityAdded {
    user: Address,
    lst_cspr_amount: U512,
    cspr_amount: U512,
    lp_tokens: U512,
    timestamp: u64,
}

#[odra::event]
struct LiquidityRemoved {
    user: Address,
    lst_cspr_amount: U512,
    cspr_amount: U512,
    lp_tokens: U512,
    timestamp: u64,
}

#[odra::event]
struct LPStaked {
    user: Address,
    amount: U512,
    timestamp: u64,
}

#[odra::event]
struct LPUnstaked {
    user: Address,
    amount: U512,
    timestamp: u64,
}

#[odra::event]
struct RewardsClaimed {
    user: Address,
    trading_fees: U512,
    mining_rewards: U512,
    timestamp: u64,
}
