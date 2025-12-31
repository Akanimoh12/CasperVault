/// Cross-Chain Strategy for CasperVault
/// 
/// Bridge lstCSPR to other chains (Ethereum/Polygon) and deploy to protocols like
/// Aave, Compound, or Curve for additional yield opportunities.

use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::VaultError;
use crate::strategies::strategy_interface::{RiskLevel, StrategyError};
use crate::utils::access_control::AccessControl;
use crate::utils::pausable::Pausable;
use crate::utils::reentrancy_guard::ReentrancyGuard;

/// Supported target chains
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetChain {
    Ethereum,
    Polygon,
    Arbitrum,
    Optimism,
}

/// Bridge operation status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeStatus {
    Initiated,
    Confirmed,
    Deployed,
    Harvesting,
    Withdrawing,
    Completed,
    Failed,
}

/// Cross-chain position tracking
#[derive(Debug, Clone)]
struct CrossChainPosition {
    /// Amount bridged
    bridged_amount: U512,
    
    /// Target chain
    target_chain: TargetChain,
    
    /// Bridge timestamp
    bridge_time: u64,
    
    /// Deployed amount on target chain
    deployed_amount: U512,
    
    /// Accrued yields on target chain
    yields_accrued: U512,
    
    /// Bridge transaction hash
    bridge_tx_hash: String,
    
    /// Status
    status: BridgeStatus,
}

/// Cross-Chain Strategy Module
/// 
/// Architecture:
/// lstCSPR → Bridge → Target Chain → Deploy to Protocol → Earn Yield → Bridge Back
/// 
/// For MVP: Simulates bridging with events and state tracking
#[odra::module]
pub struct CrossChainStrategy {
    /// Access control
    access_control: SubModule<AccessControl>,
    
    /// Pausable
    pausable: SubModule<Pausable>,
    
    /// Reentrancy protection
    reentrancy_guard: SubModule<ReentrancyGuard>,
    
    /// CORE STATE
    
    /// Current cross-chain positions by chain (flattened)
    bridged_amounts: Mapping<u8, U512>, // Amount bridged per chain
    deployed_amounts: Mapping<u8, U512>, // Deployed amount per chain
    yields_accrued: Mapping<u8, U512>, // Yields per chain
    bridge_times: Mapping<u8, u64>, // Bridge timestamp per chain
    bridge_statuses: Mapping<u8, u8>, // Status: 0=Initiated, 1=Confirmed, 2=Deployed, 3=Harvesting, 4=Withdrawing, 5=Completed, 6=Failed
    
    /// Total bridged (lifetime)
    total_bridged: Var<U512>,
    
    /// Total yields earned (lifetime)
    total_yields: Var<U512>,
    
    /// CONTRACT ADDRESSES
    
    /// Bridge contract address
    bridge_address: Var<Address>,
    
    /// lstCSPR token address
    lst_cspr_address: Var<Address>,
    
    /// PARAMETERS
    
    /// Maximum capacity
    max_capacity: Var<U512>,
    
    /// Minimum bridge amount
    min_bridge_amount: Var<U512>,
    
    /// Bridge fee (basis points)
    bridge_fee_bps: Var<u32>,
    
    /// Simulated target chain APY (basis points)
    target_apy_bps: Var<U256>,
    
    /// Last harvest timestamp
    last_harvest: Var<u64>,
    
    /// Min harvest interval
    min_harvest_interval: Var<u64>,
    
    /// Bridge confirmation time (seconds)
    bridge_confirmation_time: Var<u64>,
}

#[odra::module]
impl CrossChainStrategy {
    /// Initialize cross-chain strategy
    pub fn init(
        &mut self,
        admin: Address,
        bridge_address: Address,
        lst_cspr_address: Address,
    ) {
        self.access_control.init(admin);
        
        self.bridge_address.set(bridge_address);
        self.lst_cspr_address.set(lst_cspr_address);
        
        self.max_capacity.set(U512::from(2_000_000u64) * U512::from(1_000_000_000u64)); // 2M CSPR
        self.min_bridge_amount.set(U512::from(1_000u64) * U512::from(1_000_000_000u64)); // 1,000 CSPR
        self.bridge_fee_bps.set(50); // 0.5% bridge fee
        self.target_apy_bps.set(U256::from(1800u64)); // 18% target APY
        self.min_harvest_interval.set(86400); // 24 hours
        self.bridge_confirmation_time.set(3600); // 1 hour
        
        self.total_bridged.set(U512::zero());
        self.total_yields.set(U512::zero());
        self.last_harvest.set(0);
    }
    
    /// Deploy funds to cross-chain strategy
    /// 
    /// Process (MVP Simulation):
    /// 1. Receive lstCSPR
    /// 2. Emit BridgeInitiated event
    /// 3. Store bridged amount in state
    /// 4. Simulate deployment on target chain
    pub fn deploy(&mut self, amount: U512) -> U512 {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let min = self.min_bridge_amount.get_or_default();
        if amount < min {
            self.reentrancy_guard.exit();
            return U512::zero(); // Error: AmountTooLow
        }
        
        let current_total = self.get_balance();
        let max_cap = self.max_capacity.get_or_default();
        if current_total.checked_add(amount).unwrap() > max_cap {
            self.reentrancy_guard.exit();
            return U512::zero(); // Error: MaxCapacityReached
        }
        
        let fee_bps = self.bridge_fee_bps.get_or_default();
        let bridge_fee = amount
            .checked_mul(U512::from(fee_bps))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        let amount_after_fee = amount.checked_sub(bridge_fee).unwrap();
        
        let chain_id = 0u8; // 0 = Ethereum
        let current_time = self.env().get_block_time();
        
        // Update or create position using individual Mappings
        let existing_bridged = self.bridged_amounts.get(&chain_id).unwrap_or(U512::zero());
        let existing_deployed = self.deployed_amounts.get(&chain_id).unwrap_or(U512::zero());
        
        let new_bridged = existing_bridged.checked_add(amount_after_fee).unwrap();
        let new_deployed = existing_deployed.checked_add(amount_after_fee).unwrap();
        
        self.bridged_amounts.set(&chain_id, new_bridged);
        self.deployed_amounts.set(&chain_id, new_deployed);
        self.yields_accrued.set(&chain_id, self.yields_accrued.get(&chain_id).unwrap_or(U512::zero()));
        self.bridge_times.set(&chain_id, current_time);
        self.bridge_statuses.set(&chain_id, 2u8); // 2 = Deployed
        
        let total = self.total_bridged.get_or_default();
        self.total_bridged.set(total.checked_add(amount_after_fee).unwrap());
        
        let chain_name = match chain_id {
            0 => "Ethereum",
            1 => "Polygon",
            2 => "Avalanche",
            _ => "Unknown",
        };
        
        self.env().emit_event(BridgeInitiated {
            amount: amount_after_fee,
            fee: bridge_fee,
            target_chain: chain_name.to_string(),
            bridge_tx: format!("0xsimulated{}", self.env().get_block_time()),
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        amount_after_fee
    }
    
    /// Withdraw funds from cross-chain strategy
    /// 
    /// Process (MVP Simulation):
    /// 1. Initiate withdrawal on target chain
    /// 2. Wait for bridge confirmation
    /// 3. Receive lstCSPR back
    pub fn withdraw(&mut self, amount: U512) -> U512 {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let total_balance = self.get_balance();
        
        if amount > total_balance {
            self.reentrancy_guard.exit();
            return U512::zero(); // Error: WithdrawalTooLarge
        }
        
        
        let chain_id = 0u8; // Ethereum
        
        let deployed = self.deployed_amounts.get(&chain_id).unwrap_or(U512::zero());
        if deployed.is_zero() {
            self.reentrancy_guard.exit();
            return U512::zero(); // Error: InsufficientBalance
        }
        
        if amount > deployed {
            self.reentrancy_guard.exit();
            return U512::zero(); // Error: WithdrawalTooLarge
        }
        
        let bridged = self.bridged_amounts.get(&chain_id).unwrap_or(U512::zero());
        let new_deployed = deployed.checked_sub(amount).unwrap();
        let new_bridged = bridged.checked_sub(amount).unwrap();
        
        self.deployed_amounts.set(&chain_id, new_deployed);
        self.bridged_amounts.set(&chain_id, new_bridged);
        self.bridge_statuses.set(&chain_id, 4u8); // 4 = Withdrawing
        
        self.env().emit_event(WithdrawalInitiated {
            amount,
            target_chain: "Ethereum".to_string(),
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        amount
    }
    
    /// Harvest yields from cross-chain deployments
    /// 
    /// Process (MVP Simulation):
    /// 1. Query yields on target chains
    /// 2. Claim rewards
    /// 3. Bridge back or compound on target chain
    pub fn harvest(&mut self) -> U512 {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let current_time = self.env().get_block_time();
        let last_harvest = self.last_harvest.get_or_default();
        let min_interval = self.min_harvest_interval.get_or_default();
        
        if current_time < last_harvest + min_interval {
            self.reentrancy_guard.exit();
            return U512::zero(); // Error: Unauthorized
        }
        
        // This is complex as it requires cross-chain message passing
        
        let chain_id = 0u8; // Ethereum
        
        let deployed = self.deployed_amounts.get(&chain_id).unwrap_or(U512::zero());
        if deployed.is_zero() {
            self.reentrancy_guard.exit();
            return U512::zero();
        }
        
        let bridge_time = self.bridge_times.get(&chain_id).unwrap_or(0);
        let yields = self.yields_accrued.get(&chain_id).unwrap_or(U512::zero());
        
        let time_elapsed = current_time - bridge_time;
        let annual_apy_bps = 1800u64; // 18%
        let seconds_per_year = 31536000u64;
        
        let simulated_yield = deployed
            .checked_mul(U512::from(annual_apy_bps))
            .unwrap()
            .checked_mul(U512::from(time_elapsed))
            .unwrap()
            .checked_div(U512::from(seconds_per_year))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        let new_yield = if simulated_yield > yields {
            simulated_yield.checked_sub(yields).unwrap()
        } else {
            U512::zero()
        };
        
        self.yields_accrued.set(&chain_id, simulated_yield);
        self.bridge_statuses.set(&chain_id, 2u8); // 2 = Deployed
        
        let total = self.total_yields.get_or_default();
        self.total_yields.set(total.checked_add(new_yield).unwrap());
        self.last_harvest.set(current_time);
        
        self.env().emit_event(YieldHarvested {
            amount: new_yield,
            total_yields: simulated_yield,
            target_chain: "Ethereum".to_string(),
            timestamp: current_time,
        });
        
        self.reentrancy_guard.exit();
        new_yield
    }
    
    /// Get current balance across all chains
    pub fn get_balance(&self) -> U512 {
        let mut total = U512::zero();
        
        // Sum up deployed amounts and yields across all chains
        for chain in 0u8..4u8 { // 0=Ethereum, 1=Polygon, 2=Arbitrum, 3=Optimism
            let deployed = self.deployed_amounts.get(&chain).unwrap_or(U512::zero());
            let yields = self.yields_accrued.get(&chain).unwrap_or(U512::zero());
            total = total.checked_add(deployed).unwrap();
            total = total.checked_add(yields).unwrap();
        }
        
        total
    }
    
    /// Get current APY (higher than single-chain due to better opportunities)
    pub fn get_apy(&self) -> U256 {
        self.target_apy_bps.get_or_default()
    }
    
    /// Get risk level (High for cross-chain)
    pub fn get_risk_level(&self) -> u8 {
        2 // High risk (0=Low, 1=Medium, 2=High)
    }
    
    /// Get strategy name
    pub fn name(&self) -> String {
        "Cross-Chain Strategy".to_string()
    }
    
    /// Check if strategy is healthy
    pub fn is_healthy(&self) -> bool {
        if self.pausable.is_paused() {
            return false;
        }
        
        // - Bridge is operational
        // - Target chain protocols are healthy
        // - No pending failed transactions
        // - Bridge messages are being relayed
        
        true
    }
    
    /// Get max capacity
    pub fn max_capacity(&self) -> U512 {
        self.max_capacity.get_or_default()
    }
    
    
    pub fn set_max_capacity(&mut self, capacity: U512) {
        self.access_control.only_admin();
        self.max_capacity.set(capacity);
    }
    
    pub fn set_bridge_fee(&mut self, fee_bps: u32) {
        self.access_control.only_admin();
        
        // Max 2% bridge fee
        if fee_bps > 200 {
            self.env().revert(VaultError::Unauthorized);
        }
        
        self.bridge_fee_bps.set(fee_bps);
    }
    
    pub fn emergency_withdraw(&mut self) -> U512 {
        self.access_control.only_admin();
        
        let balance = self.get_balance();
        
        // from all target chains, potentially with losses
        
        self.withdraw(balance)
    }
    
    pub fn pause(&mut self) {
        self.access_control.only_guardian();
        self.pausable.pause();
    }
    
    pub fn unpause(&mut self) {
        self.access_control.only_admin();
        self.pausable.unpause();
    }
    
    
    pub fn get_position(&self, target_chain: u8) -> Option<(U512, U512, U512)> {
        let bridged = self.bridged_amounts.get(&target_chain);
        if bridged.is_some() {
            let deployed = self.deployed_amounts.get(&target_chain).unwrap_or(U512::zero());
            let yields = self.yields_accrued.get(&target_chain).unwrap_or(U512::zero());
            Some((bridged.unwrap(), deployed, yields))
        } else {
            None
        }
    }
    
    pub fn get_total_bridged(&self) -> U512 {
        self.total_bridged.get_or_default()
    }
    
    pub fn get_total_yields(&self) -> U512 {
        self.total_yields.get_or_default()
    }
    
    pub fn get_bridge_fee_bps(&self) -> u32 {
        self.bridge_fee_bps.get_or_default()
    }
}


#[derive(Event)]
struct BridgeInitiated {
    amount: U512,
    fee: U512,
    target_chain: String,
    bridge_tx: String,
    timestamp: u64,
}

#[derive(Event)]
struct WithdrawalInitiated {
    amount: U512,
    target_chain: String,
    timestamp: u64,
}

#[derive(Event)]
struct YieldHarvested {
    amount: U512,
    total_yields: U512,
    target_chain: String,
    timestamp: u64,
}
