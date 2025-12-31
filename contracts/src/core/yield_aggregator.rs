/// Yield Aggregator for CasperVault
/// 
/// Harvests yields from all sources (staking + strategies) and auto-compounds them
/// back into the vault to maximize returns.

use odra::prelude::*;
use odra::{Address, Event, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::*;
use crate::utils::{AccessControl, ReentrancyGuard, Pausable};
use crate::core::{LiquidStaking, StrategyRouter, VaultManager};

/// Yield report from all sources
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldReport {
    pub total_yield: U512,
    pub staking_yield: U512,
    pub dex_yield: U512,
    pub lending_yield: U512,
    pub crosschain_yield: U512,
    pub timestamp: u64,
    pub apy_snapshot: U256,
}

/// APY data point for historical tracking
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApyDataPoint {
    pub apy: U256,
    pub timestamp: u64,
    pub total_assets: U512,
}

#[odra::module]
pub struct YieldAggregator {
    /// Access control for admin/operator functions
    access_control: SubModule<AccessControl>,
    
    /// Reentrancy protection
    reentrancy_guard: SubModule<ReentrancyGuard>,
    
    /// Emergency pause
    pausable: SubModule<Pausable>,
    
    /// Reference to liquid staking contract
    liquid_staking: SubModule<LiquidStaking>,
    
    /// Reference to strategy router
    strategy_router: SubModule<StrategyRouter>,
    
    /// Reference to vault manager
    vault_manager: SubModule<VaultManager>,
    
    /// Last compound timestamp
    last_compound_time: Var<u64>,
    
    /// Minimum time between compounds (1 hour)
    min_compound_interval: Var<u64>,
    
    /// Minimum yield threshold for compounding (100 CSPR)
    min_yield_threshold: Var<U512>,
    
    /// Total yields harvested all-time
    total_yields_harvested: Var<U512>,
    
    /// Performance fee percentage (10% = 1000 basis points)
    performance_fee_bps: Var<u16>,
    
    /// Management fee percentage (2% annual = 200 basis points)
    management_fee_bps: Var<u16>,
    
    /// Accumulated fees pending withdrawal
    accumulated_fees: Var<U512>,
    
    /// Fee recipient address
    fee_recipient: Var<Address>,
    
    /// Historical yield reports
    yield_history: Mapping<u64, YieldReport>,
    
    /// Yield report counter
    report_count: Var<u64>,
    
    /// Historical APY data points
    apy_history: Mapping<u64, ApyDataPoint>,
    
    /// APY data point counter
    apy_count: Var<u64>,
    
    /// Share price history
    share_price_history: Mapping<u64, U256>,
}

#[odra::module]
impl YieldAggregator {
    /// Initialize the yield aggregator
    pub fn init(
        &mut self,
        fee_recipient: Address,
    ) {
        self.access_control.init();
        
        self.min_compound_interval.set(3600); // 1 hour
        self.min_yield_threshold.set(U512::from(100_000_000_000u64)); // 100 CSPR (9 decimals)
        self.performance_fee_bps.set(1000); // 10%
        self.management_fee_bps.set(200); // 2%
        self.fee_recipient.set(fee_recipient);
        self.last_compound_time.set(0);
        self.total_yields_harvested.set(U512::zero());
        self.accumulated_fees.set(U512::zero());
        self.report_count.set(0);
        self.apy_count.set(0);
    }
    
    /// Aggregate yields from all sources
    /// Callable by operator role or keeper
    pub fn aggregate_yields(&mut self) -> YieldReport {
        if !self.access_control.has_role(1, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        
        if self.pausable.is_paused() {
            self.env().revert(VaultError::ContractPaused);
        }
        
        let timestamp = self.env().get_block_time();
        
        // Harvest staking rewards
        let staking_yield = self.liquid_staking.compound_rewards();
        
        // Harvest from all strategies
        let strategy_yields = self.strategy_router.harvest_all();
        
        // For now, assume strategy_yields is total from all strategies
        // In full implementation, break down by strategy type
        let dex_yield = U512::zero();
        let lending_yield = U512::zero();
        let crosschain_yield = U512::zero();
        
        let total_yield = staking_yield + strategy_yields;
        
        // Get current blended APY
        let apy_snapshot = self.calculate_current_apy();
        
        // Create yield report
        let report = YieldReport {
            total_yield,
            staking_yield,
            dex_yield,
            lending_yield,
            crosschain_yield,
            timestamp,
            apy_snapshot,
        };
        
        let count = self.report_count.get_or_default();
        self.yield_history.set(&count, report.clone());
        self.report_count.set(count + 1);
        
        let total = self.total_yields_harvested.get_or_default();
        self.total_yields_harvested.set(total + total_yield);
        
        self.env().emit_event(YieldHarvested {
            total_yield,
            staking_yield,
            strategy_yield: strategy_yields,
            timestamp,
        });
        
        report
    }
    
    /// Compound yields back into the vault
    pub fn compound(&mut self, yield_amount: U512) {
        if !self.access_control.has_role(1, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        
        if self.pausable.is_paused() {
            self.env().revert(VaultError::ContractPaused);
        }
        
        let min_threshold = self.min_yield_threshold.get_or_default();
        if yield_amount < min_threshold {
            self.env().revert(VaultError::AmountTooLow);
        }
        
        let current_time = self.env().get_block_time();
        let last_time = self.last_compound_time.get_or_default();
        let min_interval = self.min_compound_interval.get_or_default();
        
        if current_time < last_time + min_interval {
            self.env().revert(VaultError::TooSoon);
        }
        
        let performance_fee = self.calculate_performance_fee(yield_amount);
        let net_yield = yield_amount - performance_fee;
        
        // Accumulate fees
        let current_fees = self.accumulated_fees.get_or_default();
        self.accumulated_fees.set(current_fees + performance_fee);
        
        // Deploy net yield to strategies
        // For MVP, we simulate by calling allocate
        self.strategy_router.allocate(net_yield);
        
        // The vault's total_assets will increase, raising share price
        self.update_share_price();
        
        self.last_compound_time.set(current_time);
        
        self.env().emit_event(YieldCompounded {
            gross_yield: yield_amount,
            net_yield,
            fees: performance_fee,
            timestamp: current_time,
        });
    }
    
    /// Auto-compound: harvest and compound in one transaction
    pub fn auto_compound(&mut self) -> U512 {
        if !self.should_compound() {
            self.env().revert(VaultError::ConditionsNotMet);
        }
        
        // Aggregate yields
        let report = self.aggregate_yields();
        
        // Compound the yields
        if report.total_yield > U512::zero() {
            self.compound(report.total_yield);
        }
        
        report.total_yield
    }
    
    /// Calculate performance fee (10% of profits)
    fn calculate_performance_fee(&self, profit: U512) -> U512 {
        let fee_bps = self.performance_fee_bps.get_or_default();
        let fee_u512 = U512::from(fee_bps);
        profit * fee_u512 / U512::from(10000u64)
    }
    
    /// Calculate management fee (2% annual, prorated)
    pub fn calculate_management_fee(&self, total_assets: U512, days_elapsed: u64) -> U512 {
        let fee_bps = self.management_fee_bps.get_or_default();
        let annual_fee = total_assets * U512::from(fee_bps) / U512::from(10000u64);
        let daily_fee = annual_fee / U512::from(365u64);
        daily_fee * U512::from(days_elapsed)
    }
    
    /// Update share price based on new total assets
    fn update_share_price(&mut self) {
        let timestamp = self.env().get_block_time();
        let share_price = self.vault_manager.get_share_price();
        
        self.share_price_history.set(&timestamp, share_price);
        
        let total_assets = self.vault_manager.total_assets();
        let apy = self.calculate_current_apy();
        
        let data_point = ApyDataPoint {
            apy,
            timestamp,
            total_assets,
        };
        
        let count = self.apy_count.get_or_default();
        self.apy_history.set(&count, data_point);
        self.apy_count.set(count + 1);
        
        self.env().emit_event(SharePriceUpdated {
            share_price,
            total_assets,
            timestamp,
        });
    }
    
    /// Calculate current APY from all sources
    fn calculate_current_apy(&self) -> U256 {
        // Get staking APY
        let staking_apy = self.liquid_staking.get_staking_apy();
        
        // Get blended strategy APY
        let strategy_apy = self.strategy_router.calculate_blended_apy();
        
        // Combine APYs (simplified: average for MVP)
        (U256::from(staking_apy) + strategy_apy) / U256::from(2u64)
    }
    
    /// Get historical APY over a period
    pub fn get_historical_apy(&self, period_seconds: u64) -> U256 {
        let current_time = self.env().get_block_time();
        let start_time = if current_time > period_seconds {
            current_time - period_seconds
        } else {
            0
        };
        
        let count = self.apy_count.get_or_default();
        if count == 0 {
            return U256::zero();
        }
        
        // Find data points in period
        let mut sum_apy = U256::zero();
        let mut data_points = 0u64;
        
        for i in 0..count {
            if let Some(data_point) = self.apy_history.get(&i) {
                if data_point.timestamp >= start_time {
                    sum_apy = sum_apy + data_point.apy;
                    data_points += 1;
                }
            }
        }
        
        if data_points > 0 {
            sum_apy / U256::from(data_points)
        } else {
            U256::zero()
        }
    }
    
    /// Check if compounding should be triggered
    pub fn should_compound(&self) -> bool {
        let current_time = self.env().get_block_time();
        let last_time = self.last_compound_time.get_or_default();
        let min_interval = self.min_compound_interval.get_or_default();
        
        if current_time < last_time + min_interval {
            return false;
        }
        
        // In full implementation, would check pending yields
        // For MVP, assume true if time condition met
        true
    }
    
    /// Get estimated gas cost for compound operation
    pub fn get_gas_estimate(&self) -> U512 {
        // Rough estimate: 500K gas units at 1 mote per unit
        U512::from(500_000_000_000u64) // 500K CSPR motes
    }
    
    /// Distribute accumulated fees to treasury
    pub fn distribute_fees(&mut self) {
        // Only admin can distribute fees (role_id = 0)
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        
        let fees = self.accumulated_fees.get_or_default();
        if fees == U512::zero() {
            self.env().revert(VaultError::NoFeesToDistribute);
        }
        
        let recipient = self.fee_recipient.get_or_default();
        
        // For MVP, just reset accumulated fees
        self.accumulated_fees.set(U512::zero());
        
        self.env().emit_event(FeesDistributed {
            amount: fees,
            recipient,
            timestamp: self.env().get_block_time(),
        });
    }
    
    /// Get total fees accumulated
    pub fn get_accumulated_fees(&self) -> U512 {
        self.accumulated_fees.get_or_default()
    }
    
    /// Get latest yield report
    pub fn get_latest_yield_report(&self) -> Option<YieldReport> {
        let count = self.report_count.get_or_default();
        if count == 0 {
            return None;
        }
        self.yield_history.get(&(count - 1))
    }
    
    /// Get yield report by index
    pub fn get_yield_report(&self, index: u64) -> Option<YieldReport> {
        self.yield_history.get(&index)
    }
    
    /// Get total number of yield reports
    pub fn get_report_count(&self) -> u64 {
        self.report_count.get_or_default()
    }
    
    /// Get share price at a specific timestamp
    pub fn get_historical_share_price(&self, timestamp: u64) -> Option<U256> {
        self.share_price_history.get(&timestamp)
    }
    
    /// Admin: Set minimum compound interval
    pub fn set_min_compound_interval(&mut self, interval: u64) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        self.min_compound_interval.set(interval);
    }
    
    /// Admin: Set minimum yield threshold
    pub fn set_min_yield_threshold(&mut self, threshold: U512) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        self.min_yield_threshold.set(threshold);
    }
    
    /// Admin: Set performance fee
    pub fn set_performance_fee(&mut self, fee_bps: u16) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        // Max 20% performance fee
        if fee_bps > 2000 {
            self.env().revert(VaultError::InvalidFee);
        }
        self.performance_fee_bps.set(fee_bps);
    }
    
    /// Admin: Set management fee
    pub fn set_management_fee(&mut self, fee_bps: u16) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        // Max 5% annual management fee
        if fee_bps > 500 {
            self.env().revert(VaultError::InvalidFee);
        }
        self.management_fee_bps.set(fee_bps);
    }
    
    /// Admin: Set fee recipient
    pub fn set_fee_recipient(&mut self, recipient: Address) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        self.fee_recipient.set(recipient);
    }
    
    /// Emergency: Pause compounding
    pub fn pause(&mut self) {
        if !self.access_control.has_role(2, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        self.pausable.pause();
    }
    
    /// Emergency: Unpause compounding
    pub fn unpause(&mut self) {
        if !self.access_control.has_role(0, self.env().caller()) {
            self.env().revert(VaultError::Unauthorized);
        }
        self.pausable.unpause();
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct YieldHarvested {
    pub total_yield: U512,
    pub staking_yield: U512,
    pub strategy_yield: U512,
    pub timestamp: u64,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct YieldCompounded {
    pub gross_yield: U512,
    pub net_yield: U512,
    pub fees: U512,
    pub timestamp: u64,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SharePriceUpdated {
    pub share_price: U256,
    pub total_assets: U512,
    pub timestamp: u64,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct FeesDistributed {
    pub amount: U512,
    pub recipient: Address,
    pub timestamp: u64,
}
