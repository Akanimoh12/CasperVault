use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::*;
use crate::utils::AccessControl;

/// StrategyRouter contract
/// 
/// This contract routes vault funds to different yield-generating strategies.
/// It manages allocation, rebalancing, and yield harvesting across multiple strategies.
/// 
/// Key responsibilities:
/// - Allocate funds to strategies based on target allocations
/// - Rebalance strategies based on performance
/// - Harvest yields from all strategies
/// - Calculate blended APY across all strategies
#[odra::module]
pub struct StrategyRouter {
    /// Access control
    access_control: SubModule<AccessControl>,
    
    /// Strategy contracts (name -> address)
    strategies: Mapping<String, Address>,
    /// Strategy names list
    strategy_names: Var<Vec<String>>,
    
    /// Current allocations per strategy (strategy name -> amount)
    current_allocations: Mapping<String, U512>,
    /// Target allocation percentages (strategy name -> percentage)
    target_allocations: Mapping<String, u8>,
    
    /// Total amount allocated across all strategies
    total_allocated: Var<U512>,
    
    /// Maximum allocation per strategy (percentage)
    max_strategy_allocation: Var<u8>,  // Default: 40%
    /// Maximum cross-chain allocation (percentage)
    max_crosschain_allocation: Var<u8>, // Default: 30%
    
    /// Last rebalance timestamp
    last_rebalance: Var<u64>,
    /// Minimum rebalance interval (seconds)
    min_rebalance_interval: Var<u64>, // Default: 12 hours
}

#[odra::module]
impl StrategyRouter {
    /// Initialize the StrategyRouter
    pub fn init(&mut self, admin: Address) {
        self.access_control.init(admin);
        
        self.total_allocated.set(U512::zero());
        self.max_strategy_allocation.set(40);
        self.max_crosschain_allocation.set(30);
        self.last_rebalance.set(0);
        self.min_rebalance_interval.set(12 * 60 * 60); // 12 hours
        
        self.strategy_names.set(Vec::new());
    }

    /// Allocate funds to strategies
    /// 
    /// Distributes the given amount across strategies based on target allocations
    pub fn allocate(&mut self, amount: U512) {
        if amount.is_zero() {
            return;
        }
        
        let strategy_names = self.strategy_names.get_or_default();
        
        for strategy_name in strategy_names.iter() {
            let target_pct = self.target_allocations.get(strategy_name).unwrap_or(0);
            
            if target_pct == 0 {
                continue;
            }
            
            let allocation = (amount * U512::from(target_pct)) / U512::from(100u64);
            
            if allocation.is_zero() {
                continue;
            }
            
            
            let current = self.current_allocations.get(strategy_name).unwrap_or(U512::zero());
            self.current_allocations.set(strategy_name, current + allocation);
            
            self.env().emit_event(AllocationUpdate {
                strategy_name: strategy_name.clone(),
                amount: allocation,
                total_allocated: current + allocation,
                timestamp: self.env().get_block_time(),
            });
        }
        
        let total = self.total_allocated.get_or_default();
        self.total_allocated.set(total + amount);
    }

    /// Withdraw from strategies proportionally
    pub fn withdraw(&mut self, amount: U512) -> U512 {
        if amount.is_zero() {
            return U512::zero();
        }
        
        let total_allocated = self.total_allocated.get_or_default();
        
        if total_allocated.is_zero() {
            return U512::zero();
        }
        
        let strategy_names = self.strategy_names.get_or_default();
        let mut total_withdrawn = U512::zero();
        
        for strategy_name in strategy_names.iter() {
            let current_allocation = self.current_allocations.get(strategy_name).unwrap_or(U512::zero());
            
            if current_allocation.is_zero() {
                continue;
            }
            
            let withdrawal_amount = (amount * current_allocation) / total_allocated;
            
            if withdrawal_amount.is_zero() {
                continue;
            }
            
            let withdrawn = withdrawal_amount; // Assume successful
            
            self.current_allocations.set(strategy_name, current_allocation - withdrawn);
            total_withdrawn += withdrawn;
        }
        
        self.total_allocated.set(total_allocated - total_withdrawn);
        
        total_withdrawn
    }

    /// Harvest yields from all strategies
    pub fn harvest_all(&mut self) -> U512 {
        self.access_control.only_admin_or_operator();
        
        let strategy_names = self.strategy_names.get_or_default();
        let mut total_yield = U512::zero();
        
        for strategy_name in strategy_names.iter() {
            // For now, simulate yields
            let allocation = self.current_allocations.get(strategy_name).unwrap_or(U512::zero());
            let simulated_yield = allocation / U512::from(100u64); // 1% yield
            
            total_yield += simulated_yield;
            
            self.env().emit_event(YieldHarvested {
                strategy_name: strategy_name.clone(),
                yield_amount: simulated_yield,
                timestamp: self.env().get_block_time(),
            });
        }
        
        total_yield
    }

    /// Rebalance strategies based on target allocations
    /// 
    /// This function should be called periodically (e.g., every 12 hours)
    /// by an off-chain keeper or admin
    pub fn rebalance(&mut self) {
        self.access_control.only_admin_or_operator();
        
        let current_time = self.env().get_block_time();
        let last_rebalance = self.last_rebalance.get_or_default();
        let min_interval = self.min_rebalance_interval.get_or_default();
        
        if current_time < last_rebalance + min_interval {
            return;
        }
        
        
        self.last_rebalance.set(current_time);
        
        self.env().emit_event(Rebalance {
            old_allocations: Vec::new(),
            new_allocations: Vec::new(),
            timestamp: current_time,
        });
    }

    /// Calculate blended APY across all strategies
    pub fn calculate_blended_apy(&self) -> U256 {
        let total_allocated = self.total_allocated.get_or_default();
        
        if total_allocated.is_zero() {
            return U256::zero();
        }
        
        let strategy_names = self.strategy_names.get_or_default();
        let mut weighted_apy = U256::zero();
        
        for strategy_name in strategy_names.iter() {
            let allocation = self.current_allocations.get(strategy_name).unwrap_or(U512::zero());
            
            if allocation.is_zero() {
                continue;
            }
            
            // For now, use simulated APYs
            let strategy_apy = if strategy_name == "dex" {
                U256::from(1200u64) // 12%
            } else if strategy_name == "lending" {
                U256::from(1500u64) // 15%
            } else if strategy_name == "crosschain" {
                U256::from(1850u64) // 18.5%
            } else {
                U256::from(1000u64) // 10%
            };
            
            let weight = (U256::from(allocation) * U256::from(10000u64)) / U256::from(total_allocated);
            
            // Add weighted APY
            weighted_apy += (strategy_apy * weight) / U256::from(10000u64);
        }
        
        weighted_apy
    }

    /// Add a strategy (admin only)
    pub fn add_strategy(&mut self, name: String, strategy_address: Address) {
        self.access_control.only_admin();
        
        self.strategies.set(&name, strategy_address);
        
        let mut names = self.strategy_names.get_or_default();
        if !names.contains(&name) {
            names.push(name.clone());
            self.strategy_names.set(names);
        }
        
        // Initialize allocation to 0
        self.current_allocations.set(&name, U512::zero());
        self.target_allocations.set(&name, 0);
    }

    /// Remove a strategy (admin only)
    pub fn remove_strategy(&mut self, name: String) {
        self.access_control.only_admin();
        
        
        let mut names = self.strategy_names.get_or_default();
        names.retain(|n| n != &name);
        self.strategy_names.set(names);
    }

    /// Set target allocations (admin only)
    /// 
    /// Allocations should sum to 100%
    pub fn set_target_allocations(&mut self, allocations: Vec<(String, u8)>) {
        self.access_control.only_admin();
        
        let max_strategy = self.max_strategy_allocation.get_or_default();
        let max_crosschain = self.max_crosschain_allocation.get_or_default();
        
        let mut total_pct: u16 = 0;
        let mut crosschain_pct: u16 = 0;
        
        for (strategy_name, pct) in allocations.iter() {
            // Validate constraints
            if *pct > max_strategy {
                self.env().revert(StrategyError::AllocationExceedsMax);
            }
            
            if strategy_name == "crosschain" {
                crosschain_pct += *pct as u16;
            }
            
            total_pct += *pct as u16;
            
            // Set target allocation
            self.target_allocations.set(strategy_name, *pct);
        }
        
        // Validate total = 100%
        if total_pct != 100 {
            self.env().revert(StrategyError::InvalidTotalAllocation);
        }
        
        // Validate cross-chain limit
        if crosschain_pct > max_crosschain as u16 {
            self.env().revert(StrategyError::CrossChainExceedsMax);
        }
    }

    /// Get current allocation for a strategy
    pub fn get_current_allocation(&self, strategy_name: String) -> U512 {
        self.current_allocations.get(&strategy_name).unwrap_or(U512::zero())
    }

    /// Get target allocation percentage for a strategy
    pub fn get_target_allocation(&self, strategy_name: String) -> u8 {
        self.target_allocations.get(&strategy_name).unwrap_or(0)
    }

    /// Get total allocated amount
    pub fn get_total_allocated(&self) -> U512 {
        self.total_allocated.get_or_default()
    }

    /// Get all strategy names
    pub fn get_strategy_names(&self) -> Vec<String> {
        self.strategy_names.get_or_default()
    }
}


#[derive(Event)]
struct AllocationUpdate {
    strategy_name: String,
    amount: U512,
    total_allocated: U512,
    timestamp: u64,
}

#[derive(Event)]
struct YieldHarvested {
    strategy_name: String,
    yield_amount: U512,
    timestamp: u64,
}

#[derive(Event)]
struct Rebalance {
    old_allocations: Vec<(String, U512)>,
    new_allocations: Vec<(String, U512)>,
    timestamp: u64,
}

// ERRORS

#[derive(Debug, PartialEq)]
pub enum StrategyError {
    AllocationExceedsMax,
    CrossChainExceedsMax,
    InvalidTotalAllocation,
}
