/// Cross-Chain Strategy for CasperVault
/// 
/// Bridge lstCSPR to other chains (Ethereum/Polygon) and deploy to protocols like
/// Aave, Compound, or Curve for additional yield opportunities.

use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::strategies::strategy_interface::{IStrategy, RiskLevel, StrategyError};
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
    
    /// Current cross-chain positions by chain
    positions: Mapping<u8, CrossChainPosition>, // u8 = TargetChain as discriminant
    
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
    pub fn deploy(&mut self, amount: U512) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let min = self.min_bridge_amount.get_or_default();
        if amount < min {
            self.reentrancy_guard.exit();
            return Err(StrategyError::AmountTooLow);
        }
        
        let current_total = self.get_balance();
        let max_cap = self.max_capacity.get_or_default();
        if current_total.checked_add(amount).unwrap() > max_cap {
            self.reentrancy_guard.exit();
            return Err(StrategyError::MaxCapacityReached);
        }
        
        let fee_bps = self.bridge_fee_bps.get_or_default();
        let bridge_fee = amount
            .checked_mul(U512::from(fee_bps))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        let amount_after_fee = amount.checked_sub(bridge_fee).unwrap();
        
        //     amount_after_fee,
        //     target_chain,
        //     target_protocol
        // );
        
        let target_chain = TargetChain::Ethereum;
        let chain_id = target_chain as u8;
        
        let position = CrossChainPosition {
            bridged_amount: amount_after_fee,
            target_chain,
            bridge_time: self.env().get_block_time(),
            deployed_amount: amount_after_fee,
            yields_accrued: U512::zero(),
            bridge_tx_hash: format!("0xsimulated{}", self.env().get_block_time()),
            status: BridgeStatus::Deployed,
        };
        
        let existing = self.positions.get(&chain_id);
        if let Some(mut existing_pos) = existing {
            existing_pos.bridged_amount = existing_pos.bridged_amount.checked_add(amount_after_fee).unwrap();
            existing_pos.deployed_amount = existing_pos.deployed_amount.checked_add(amount_after_fee).unwrap();
            self.positions.set(&chain_id, existing_pos);
        } else {
            self.positions.set(&chain_id, position);
        }
        
        let total = self.total_bridged.get_or_default();
        self.total_bridged.set(total.checked_add(amount_after_fee).unwrap());
        
        self.env().emit_event(BridgeInitiated {
            amount: amount_after_fee,
            fee: bridge_fee,
            target_chain: format!("{:?}", target_chain),
            bridge_tx: format!("0xsimulated{}", self.env().get_block_time()),
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        Ok(amount_after_fee)
    }
    
    /// Withdraw funds from cross-chain strategy
    /// 
    /// Process (MVP Simulation):
    /// 1. Initiate withdrawal on target chain
    /// 2. Wait for bridge confirmation
    /// 3. Receive lstCSPR back
    pub fn withdraw(&mut self, amount: U512) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let total_balance = self.get_balance();
        
        if amount > total_balance {
            self.reentrancy_guard.exit();
            return Err(StrategyError::WithdrawalTooLarge);
        }
        
        
        let chain_id = TargetChain::Ethereum as u8;
        let mut position = self.positions.get(&chain_id)
            .ok_or(StrategyError::InsufficientBalance)?;
        
        if amount > position.deployed_amount {
            self.reentrancy_guard.exit();
            return Err(StrategyError::WithdrawalTooLarge);
        }
        
        position.deployed_amount = position.deployed_amount.checked_sub(amount).unwrap();
        position.bridged_amount = position.bridged_amount.checked_sub(amount).unwrap();
        position.status = BridgeStatus::Withdrawing;
        self.positions.set(&chain_id, position);
        
        self.env().emit_event(WithdrawalInitiated {
            amount,
            target_chain: "Ethereum".to_string(),
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        Ok(amount)
    }
    
    /// Harvest yields from cross-chain deployments
    /// 
    /// Process (MVP Simulation):
    /// 1. Query yields on target chains
    /// 2. Claim rewards
    /// 3. Bridge back or compound on target chain
    pub fn harvest(&mut self) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let current_time = self.env().get_block_time();
        let last_harvest = self.last_harvest.get_or_default();
        let min_interval = self.min_harvest_interval.get_or_default();
        
        if current_time < last_harvest + min_interval {
            self.reentrancy_guard.exit();
            return Err(StrategyError::Unauthorized);
        }
        
        // This is complex as it requires cross-chain message passing
        
        let chain_id = TargetChain::Ethereum as u8;
        let position_opt = self.positions.get(&chain_id);
        
        if position_opt.is_none() {
            self.reentrancy_guard.exit();
            return Ok(U512::zero());
        }
        
        let position = position_opt.unwrap();
        
        let time_elapsed = current_time - position.bridge_time;
        let annual_apy_bps = 1800u64; // 18%
        let seconds_per_year = 31536000u64;
        
        let simulated_yield = position.deployed_amount
            .checked_mul(U512::from(annual_apy_bps))
            .unwrap()
            .checked_mul(U512::from(time_elapsed))
            .unwrap()
            .checked_div(U512::from(seconds_per_year))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        let new_yield = if simulated_yield > position.yields_accrued {
            simulated_yield.checked_sub(position.yields_accrued).unwrap()
        } else {
            U512::zero()
        };
        
        let mut new_position = position;
        new_position.yields_accrued = simulated_yield;
        new_position.status = BridgeStatus::Deployed;
        self.positions.set(&chain_id, new_position);
        
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
        Ok(new_yield)
    }
    
    /// Get current balance across all chains
    pub fn get_balance(&self) -> U512 {
        let mut total = U512::zero();
        
        // Sum across all chains
        for chain in [TargetChain::Ethereum, TargetChain::Polygon] {
            if let Some(position) = self.positions.get(&(chain as u8)) {
                total = total.checked_add(position.deployed_amount).unwrap();
                total = total.checked_add(position.yields_accrued).unwrap();
            }
        }
        
        total
    }
    
    /// Get current APY (higher than single-chain due to better opportunities)
    pub fn get_apy(&self) -> U256 {
        self.target_apy_bps.get_or_default()
    }
    
    /// Get risk level (High for cross-chain)
    pub fn get_risk_level(&self) -> RiskLevel {
        RiskLevel::High
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
            self.env().revert(StrategyError::Unauthorized);
        }
        
        self.bridge_fee_bps.set(fee_bps);
    }
    
    pub fn emergency_withdraw(&mut self) -> U512 {
        self.access_control.only_admin();
        
        let balance = self.get_balance();
        
        // from all target chains, potentially with losses
        
        match self.withdraw(balance) {
            Ok(amount) => amount,
            Err(_) => U512::zero(),
        }
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
        self.positions.get(&target_chain).map(|pos| {
            (pos.bridged_amount, pos.deployed_amount, pos.yields_accrued)
        })
    }
    
    pub fn get_total_bridged(&self) -> U512 {
        self.total_bridged.get_or_default()
    }
    
    pub fn get_total_yields(&self) -> U512 {
        self.total_yields.get_or_default()
    }
    
    pub fn get_bridge_fee_bps(&self) -> u16 {
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
