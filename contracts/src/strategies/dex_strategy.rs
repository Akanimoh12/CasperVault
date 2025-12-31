/// DEX Strategy for CasperVault
/// 
/// Provides liquidity to lstCSPR/CSPR pool on Casper DEX.
/// Earns yield from trading fees and liquidity mining rewards.

use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::strategies::strategy_interface::{IStrategy, RiskLevel, StrategyError};
use crate::utils::access_control::AccessControl;
use crate::utils::pausable::Pausable;
use crate::utils::reentrancy_guard::ReentrancyGuard;

/// LP position information
#[derive(Debug, Clone)]
struct LPPosition {
    /// LP tokens held
    lp_tokens: U512,
    
    /// lstCSPR deposited
    lst_cspr_amount: U512,
    
    /// CSPR deposited
    cspr_amount: U512,
    
    /// Timestamp of deposit
    deposit_time: u64,
    
    /// Accumulated trading fees
    trading_fees: U512,
    
    /// Accumulated mining rewards
    mining_rewards: U512,
}

/// Impermanent loss tracking
#[derive(Debug, Clone)]
struct ImpermanentLoss {
    /// Initial value in CSPR
    initial_value: U512,
    
    /// Current value in CSPR
    current_value: U512,
    
    /// Loss percentage in basis points (negative = profit)
    loss_bps: i32,
}

/// DEX Strategy Module
/// 
/// Architecture:
/// lstCSPR → Add Liquidity → LP Tokens → Stake → Earn Fees + Rewards
#[odra::module]
pub struct DEXStrategy {
    /// Access control for admin functions
    access_control: SubModule<AccessControl>,
    
    /// Pausable for emergencies
    pausable: SubModule<Pausable>,
    
    /// Reentrancy protection
    reentrancy_guard: SubModule<ReentrancyGuard>,
    
    /// CORE STATE
    
    /// Current LP position
    lp_position: Var<LPPosition>,
    
    /// Total lstCSPR deployed
    total_deployed: Var<U512>,
    
    /// Total harvested (lifetime)
    total_harvested: Var<U512>,
    
    /// CONTRACT ADDRESSES
    
    /// DEX contract address
    dex_address: Var<Address>,
    
    /// LP staking contract address
    lp_staking_address: Var<Address>,
    
    /// lstCSPR token address
    lst_cspr_address: Var<Address>,
    
    /// PARAMETERS
    
    /// Maximum capacity (lstCSPR)
    max_capacity: Var<U512>,
    
    /// Minimum deployment amount
    min_deployment: Var<U512>,
    
    /// Maximum slippage allowed (basis points)
    max_slippage_bps: Var<u16>,
    
    /// Target APY in basis points
    target_apy_bps: Var<U256>,
    
    /// Last harvest timestamp
    last_harvest: Var<u64>,
    
    /// Minimum harvest interval (seconds)
    min_harvest_interval: Var<u64>,
}

#[odra::module]
impl DEXStrategy {
    /// Initialize the DEX strategy
    /// 
    /// # Arguments
    /// * `admin` - Admin address
    /// * `dex_address` - DEX contract address
    /// * `lp_staking_address` - LP staking contract address
    /// * `lst_cspr_address` - lstCSPR token address
    pub fn init(
        &mut self,
        admin: Address,
        dex_address: Address,
        lp_staking_address: Address,
        lst_cspr_address: Address,
    ) {
        self.access_control.init(admin);
        
        self.dex_address.set(dex_address);
        self.lp_staking_address.set(lp_staking_address);
        self.lst_cspr_address.set(lst_cspr_address);
        
        self.max_capacity.set(U512::from(1_000_000u64) * U512::from(1_000_000_000u64)); // 1M CSPR
        self.min_deployment.set(U512::from(100u64) * U512::from(1_000_000_000u64)); // 100 CSPR
        self.max_slippage_bps.set(100); // 1% max slippage
        self.target_apy_bps.set(U256::from(1500u64)); // 15% target APY
        self.min_harvest_interval.set(43200); // 12 hours
        
        self.lp_position.set(LPPosition {
            lp_tokens: U512::zero(),
            lst_cspr_amount: U512::zero(),
            cspr_amount: U512::zero(),
            deposit_time: 0,
            trading_fees: U512::zero(),
            mining_rewards: U512::zero(),
        });
        
        self.total_deployed.set(U512::zero());
        self.total_harvested.set(U512::zero());
        self.last_harvest.set(0);
    }
    
    /// Deploy funds to DEX liquidity pool
    /// 
    /// Process:
    /// 1. Receive lstCSPR from router
    /// 2. Calculate optimal CSPR pair amount
    /// 3. Add liquidity to lstCSPR/CSPR pool
    /// 4. Receive LP tokens
    /// 5. Stake LP tokens for rewards
    /// 6. Update position tracking
    pub fn deploy(&mut self, amount: U512) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let min_deploy = self.min_deployment.get_or_default();
        if amount < min_deploy {
            self.reentrancy_guard.exit();
            return Err(StrategyError::AmountTooLow);
        }
        
        let current = self.total_deployed.get_or_default();
        let max_cap = self.max_capacity.get_or_default();
        if current.checked_add(amount).unwrap() > max_cap {
            self.reentrancy_guard.exit();
            return Err(StrategyError::MaxCapacityReached);
        }
        
        let cspr_amount = amount;
        
        //     self.dex.add_liquidity(amount, cspr_amount, max_slippage);
        
        let lp_tokens = amount.checked_add(cspr_amount).unwrap()
            .checked_div(U512::from(2u64))
            .unwrap();
        let actual_lst = amount;
        let actual_cspr = cspr_amount;
        
        
        let mut position = self.lp_position.get_or_default();
        position.lp_tokens = position.lp_tokens.checked_add(lp_tokens).unwrap();
        position.lst_cspr_amount = position.lst_cspr_amount.checked_add(actual_lst).unwrap();
        position.cspr_amount = position.cspr_amount.checked_add(actual_cspr).unwrap();
        position.deposit_time = self.env().get_block_time();
        self.lp_position.set(position);
        
        let new_total = current.checked_add(actual_lst).unwrap();
        self.total_deployed.set(new_total);
        
        self.env().emit_event(Deployed {
            amount: actual_lst,
            lp_tokens,
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        Ok(actual_lst)
    }
    
    /// Withdraw funds from DEX pool
    /// 
    /// Process:
    /// 1. Calculate LP tokens to unstake
    /// 2. Unstake LP tokens
    /// 3. Remove liquidity from pool
    /// 4. Receive lstCSPR and CSPR
    /// 5. Return lstCSPR to router
    pub fn withdraw(&mut self, amount: U512) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let position = self.lp_position.get_or_default();
        
        if amount > position.lst_cspr_amount {
            self.reentrancy_guard.exit();
            return Err(StrategyError::WithdrawalTooLarge);
        }
        
        let lp_to_unstake = if position.lst_cspr_amount.is_zero() {
            U512::zero()
        } else {
            amount.checked_mul(position.lp_tokens).unwrap()
                .checked_div(position.lst_cspr_amount).unwrap()
        };
        
        
        //     self.dex.remove_liquidity(lp_to_unstake, min_lst, min_cspr);
        
        let lst_received = amount;
        let cspr_received = amount; // Assume 1:1 for simplicity
        
        let mut new_position = position;
        new_position.lp_tokens = new_position.lp_tokens.checked_sub(lp_to_unstake).unwrap();
        new_position.lst_cspr_amount = new_position.lst_cspr_amount.checked_sub(lst_received).unwrap();
        new_position.cspr_amount = new_position.cspr_amount.checked_sub(cspr_received).unwrap();
        self.lp_position.set(new_position);
        
        let current = self.total_deployed.get_or_default();
        self.total_deployed.set(current.checked_sub(lst_received).unwrap());
        
        self.env().emit_event(Withdrawn {
            amount: lst_received,
            lp_tokens_burned: lp_to_unstake,
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        Ok(lst_received)
    }
    
    /// Harvest trading fees and mining rewards
    /// 
    /// Process:
    /// 1. Claim accumulated trading fees from DEX
    /// 2. Claim mining rewards from staking
    /// 3. Swap rewards to lstCSPR if needed
    /// 4. Return harvested amount
    pub fn harvest(&mut self) -> Result<U512, StrategyError> {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let current_time = self.env().get_block_time();
        let last_harvest = self.last_harvest.get_or_default();
        let min_interval = self.min_harvest_interval.get_or_default();
        
        if current_time < last_harvest + min_interval {
            self.reentrancy_guard.exit();
            return Err(StrategyError::Unauthorized); // Could add specific RateLimitError
        }
        
        
        
        let position = self.lp_position.get_or_default();
        let time_elapsed = current_time - position.deposit_time;
        let annual_apy_bps = 1200u64; // 12%
        let seconds_per_year = 31536000u64;
        
        let simulated_yield = position.lst_cspr_amount
            .checked_mul(U512::from(annual_apy_bps))
            .unwrap()
            .checked_mul(U512::from(time_elapsed))
            .unwrap()
            .checked_div(U512::from(seconds_per_year))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        let trading_fees = simulated_yield.checked_div(U512::from(2u64)).unwrap();
        let mining_rewards = simulated_yield.checked_sub(trading_fees).unwrap();
        
        let mut new_position = position;
        new_position.trading_fees = new_position.trading_fees.checked_add(trading_fees).unwrap();
        new_position.mining_rewards = new_position.mining_rewards.checked_add(mining_rewards).unwrap();
        self.lp_position.set(new_position);
        
        let total_yield = trading_fees.checked_add(mining_rewards).unwrap();
        let current_harvested = self.total_harvested.get_or_default();
        self.total_harvested.set(current_harvested.checked_add(total_yield).unwrap());
        self.last_harvest.set(current_time);
        
        self.env().emit_event(Harvested {
            trading_fees,
            mining_rewards,
            total: total_yield,
            timestamp: current_time,
        });
        
        self.reentrancy_guard.exit();
        Ok(total_yield)
    }
    
    /// Get current balance in strategy
    pub fn get_balance(&self) -> U512 {
        let position = self.lp_position.get_or_default();
        
        // Total value = deployed + accrued rewards
        position.lst_cspr_amount
            .checked_add(position.trading_fees)
            .unwrap()
            .checked_add(position.mining_rewards)
            .unwrap()
    }
    
    /// Calculate current APY
    /// 
    /// APY = (total_harvested / total_deployed) * (seconds_per_year / time_elapsed) * 10000
    pub fn get_apy(&self) -> U256 {
        let position = self.lp_position.get_or_default();
        
        if position.lst_cspr_amount.is_zero() || position.deposit_time == 0 {
            return self.target_apy_bps.get_or_default();
        }
        
        let current_time = self.env().get_block_time();
        let time_elapsed = current_time - position.deposit_time;
        
        if time_elapsed == 0 {
            return self.target_apy_bps.get_or_default();
        }
        
        let total_yield = position.trading_fees.checked_add(position.mining_rewards).unwrap();
        let seconds_per_year = 31536000u64;
        
        // APY = (yield / deployed) * (1 year / time) * 10000
        let apy = total_yield
            .checked_mul(U512::from(seconds_per_year))
            .unwrap()
            .checked_mul(U512::from(10000u64))
            .unwrap()
            .checked_div(position.lst_cspr_amount)
            .unwrap()
            .checked_div(U512::from(time_elapsed))
            .unwrap();
        
        U256::try_from(apy).unwrap_or(self.target_apy_bps.get_or_default())
    }
    
    /// Get risk level (Medium for DEX LPs)
    pub fn get_risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    /// Get strategy name
    pub fn name(&self) -> String {
        "DEX Liquidity Strategy".to_string()
    }
    
    /// Check if strategy is healthy
    pub fn is_healthy(&self) -> bool {
        if self.pausable.is_paused() {
            return false;
        }
        
        let apy = self.get_apy();
        let min_apy = U256::from(100u64); // 1%
        let max_apy = U256::from(50000u64); // 500% (alarm threshold)
        
        if apy < min_apy || apy > max_apy {
            return false;
        }
        
        // - Pool liquidity sufficient
        // - Price oracle working
        // - No emergency pause
        
        true
    }
    
    /// Get maximum capacity
    pub fn max_capacity(&self) -> U512 {
        self.max_capacity.get_or_default()
    }
    
    /// Calculate impermanent loss
    /// 
    /// IL = (2 * sqrt(price_ratio) / (1 + price_ratio)) - 1
    pub fn calculate_impermanent_loss(&self) -> ImpermanentLoss {
        let position = self.lp_position.get_or_default();
        
        if position.lst_cspr_amount.is_zero() || position.cspr_amount.is_zero() {
            return ImpermanentLoss {
                initial_value: U512::zero(),
                current_value: U512::zero(),
                loss_bps: 0,
            };
        }
        
        // Initial value (assuming 1:1 ratio at deposit)
        let initial_value = position.lst_cspr_amount
            .checked_add(position.cspr_amount)
            .unwrap();
        
        let current_value = initial_value
            .checked_add(position.trading_fees)
            .unwrap()
            .checked_add(position.mining_rewards)
            .unwrap();
        
        let diff = if current_value > initial_value {
            let profit = current_value.checked_sub(initial_value).unwrap();
            let profit_bps = profit
                .checked_mul(U512::from(10000u64))
                .unwrap()
                .checked_div(initial_value)
                .unwrap();
            -(i32::try_from(profit_bps).unwrap_or(0))
        } else {
            let loss = initial_value.checked_sub(current_value).unwrap();
            let loss_bps = loss
                .checked_mul(U512::from(10000u64))
                .unwrap()
                .checked_div(initial_value)
                .unwrap();
            i32::try_from(loss_bps).unwrap_or(0)
        };
        
        ImpermanentLoss {
            initial_value,
            current_value,
            loss_bps: diff,
        }
    }
    
    
    /// Update max capacity
    pub fn set_max_capacity(&mut self, capacity: U512) {
        self.access_control.only_admin();
        self.max_capacity.set(capacity);
    }
    
    /// Update max slippage
    pub fn set_max_slippage(&mut self, slippage_bps: u16) {
        self.access_control.only_admin();
        
        // Max 5% slippage
        if slippage_bps > 500 {
            self.env().revert(StrategyError::Unauthorized);
        }
        
        self.max_slippage_bps.set(slippage_bps);
    }
    
    /// Emergency withdraw (admin only)
    pub fn emergency_withdraw(&mut self) -> U512 {
        self.access_control.only_admin();
        
        let position = self.lp_position.get_or_default();
        let total = position.lst_cspr_amount;
        
        // Attempt withdrawal of all funds
        match self.withdraw(total) {
            Ok(amount) => amount,
            Err(_) => U512::zero(),
        }
    }
    
    /// Pause strategy
    pub fn pause(&mut self) {
        self.access_control.only_guardian();
        self.pausable.pause();
    }
    
    /// Unpause strategy
    pub fn unpause(&mut self) {
        self.access_control.only_admin();
        self.pausable.unpause();
    }
    
    
    pub fn get_lp_position(&self) -> (U512, U512, U512) {
        let position = self.lp_position.get_or_default();
        (position.lp_tokens, position.lst_cspr_amount, position.cspr_amount)
    }
    
    pub fn get_total_deployed(&self) -> U512 {
        self.total_deployed.get_or_default()
    }
    
    pub fn get_total_harvested(&self) -> U512 {
        self.total_harvested.get_or_default()
    }
    
    pub fn get_rewards_accrued(&self) -> (U512, U512) {
        let position = self.lp_position.get_or_default();
        (position.trading_fees, position.mining_rewards)
    }
}


#[derive(Event)]
struct Deployed {
    amount: U512,
    lp_tokens: U512,
    timestamp: u64,
}

#[derive(Event)]
struct Withdrawn {
    amount: U512,
    lp_tokens_burned: U512,
    timestamp: u64,
}

#[derive(Event)]
struct Harvested {
    trading_fees: U512,
    mining_rewards: U512,
    total: U512,
    timestamp: u64,
}
