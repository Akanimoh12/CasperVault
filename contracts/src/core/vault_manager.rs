use odra::prelude::*;
use odra::{Address, Event, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};
use crate::types::*;
use crate::utils::{AccessControl, ReentrancyGuard, Pausable};
use crate::tokens::{CvCspr, LstCspr};

/// Withdrawal request structure for time-locked withdrawals
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawalRequest {
    pub user: Address,
    pub shares: U512,
    pub assets_value: U512,  // lstCSPR value at request time
    pub unlock_time: u64,
    pub is_completed: bool,
}

/// User deposit tracking for performance fee calculation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserDeposit {
    pub total_deposited: U512,  // Total CSPR deposited
    pub total_shares: U512,       // Total shares owned
    pub cost_basis: U512,         // Average cost per share
    pub last_deposit_time: u64,
}

/// VaultManager - Main vault contract (ERC-4626 compliant)
/// 
/// This contract manages user deposits, withdrawals, and vault shares following
/// the ERC-4626 Tokenized Vault Standard for maximum composability.
/// 
/// **Architecture:**
/// User CSPR → LiquidStaking (→lstCSPR) → VaultManager (→cvCSPR) → Strategies
/// 
/// **Share Price Appreciation:**
/// - Initial: 1 cvCSPR = 1 lstCSPR (1:1 ratio)
/// - As yields accrue: 1 cvCSPR = 1.1 lstCSPR (10% appreciation)
/// - Formula: sharePrice = totalAssets / totalShares
/// 
/// **Key Responsibilities:**
/// - Accept CSPR deposits and mint cvCSPR shares
/// - Calculate fair share prices (ERC-4626 compliant)
/// - Handle two withdrawal types:
///   * Time-locked (7 days, no fee)
///   * Instant (from pool, 0.5% fee)
/// - Deploy assets to yield strategies
/// - Collect and distribute fees
/// - Emergency pause/unpause
#[odra::module]
pub struct VaultManager {
    /// Access control module
    access_control: SubModule<AccessControl>,
    /// Reentrancy guard for security
    reentrancy_guard: SubModule<ReentrancyGuard>,
    /// Pausable for emergencies
    pausable: SubModule<Pausable>,
    
    
    /// Total assets under management (in lstCSPR)
    /// Includes: vault balance + strategy deployments + accrued yields
    total_assets: Var<U512>,
    
    /// Total vault shares issued (cvCSPR)
    total_shares: Var<U512>,
    
    /// User shares mapping (user -> shares balance)
    user_shares: Mapping<Address, U512>,
    
    /// User deposit tracking (for performance fee calculation)
    user_deposits: Mapping<Address, UserDeposit>,
    
    
    /// cvCSPR token contract address
    cv_cspr_token: Var<Address>,
    
    /// lstCSPR token contract address
    lst_cspr_token: Var<Address>,
    
    /// LiquidStaking contract address
    liquid_staking_contract: Var<Address>,
    
    /// StrategyRouter contract address
    strategy_router_contract: Var<Address>,
    
    
    /// Withdrawal requests mapping (request_id -> WithdrawalRequest)
    withdrawal_requests: Mapping<U256, WithdrawalRequest>,
    
    /// Next withdrawal request ID
    next_request_id: Var<U256>,
    
    /// Timelock for standard withdrawals (in seconds)
    withdrawal_timelock: Var<u64>,  // Default: 7 days
    
    /// Instant withdrawal pool liquidity (lstCSPR)
    instant_withdrawal_pool: Var<U512>,
    
    /// Target instant withdrawal pool percentage (basis points)
    instant_pool_target_bps: Var<u16>,  // Default: 500 (5%)
    
    
    /// Performance fee (basis points, 10000 = 100%)
    performance_fee_bps: Var<u16>,  // Default: 1000 (10%)
    
    /// Management fee annual rate (basis points)
    management_fee_bps: Var<u16>,  // Default: 200 (2%)
    
    /// Instant withdrawal fee (basis points)
    instant_withdrawal_fee_bps: Var<u16>,  // Default: 50 (0.5%)
    
    /// Accumulated fees (in lstCSPR)
    fees_collected: Var<U512>,
    
    /// Last management fee collection timestamp
    last_fee_collection: Var<u64>,
    
    /// Protocol treasury address
    treasury: Var<Address>,
    
    
    /// Maximum deposit per transaction (rate limiting)
    max_deposit: Var<U512>,  // Default: 10,000 CSPR
    
    /// Maximum deposit per user per day
    max_deposit_per_day: Var<U512>,  // Default: 50,000 CSPR
    
    /// User daily deposit tracking (user -> (day, amount))
    daily_deposits: Mapping<Address, (u64, U512)>,
    
    /// Minimum shares to mint (prevent dust)
    min_shares: Var<U512>,  // Default: 1000 (0.000001 shares)
}

#[odra::module]
impl VaultManager {
    /// Initialize the VaultManager
    pub fn init(
        &mut self,
        admin: Address,
        treasury: Address,
        cv_cspr_token: Address,
        lst_cspr_token: Address,
        liquid_staking_contract: Address,
    ) {
        // Initialize modules
        self.access_control.init(admin);
        self.reentrancy_guard.init();
        self.pausable.init();
        
        self.treasury.set(treasury);
        self.cv_cspr_token.set(cv_cspr_token);
        self.lst_cspr_token.set(lst_cspr_token);
        self.liquid_staking_contract.set(liquid_staking_contract);
        
        // Initialize core state
        self.total_assets.set(U512::zero());
        self.total_shares.set(U512::zero());
        self.next_request_id.set(U256::zero());
        
        // Set default fees (in basis points)
        self.performance_fee_bps.set(1000);      // 10%
        self.management_fee_bps.set(200);        // 2% annual
        self.instant_withdrawal_fee_bps.set(50); // 0.5%
        
        // Set withdrawal timelock (7 days)
        self.withdrawal_timelock.set(7 * 24 * 60 * 60);
        
        // Set instant pool target (5% of total assets)
        self.instant_pool_target_bps.set(500);
        
        // Set deposit limits
        self.max_deposit.set(U512::from(10_000_000_000_000u64)); // 10,000 CSPR
        self.max_deposit_per_day.set(U512::from(50_000_000_000_000u64)); // 50,000 CSPR
        
        // Set minimum shares (prevent dust attacks)
        self.min_shares.set(U512::from(1000u64));
        
        // Initialize fees and pool
        self.fees_collected.set(U512::zero());
        self.instant_withdrawal_pool.set(U512::zero());
        self.last_fee_collection.set(self.env().get_block_time());
    }


    /// Deposit CSPR and receive cvCSPR vault shares
    /// 
    /// **Process Flow:**
    /// 1. Validate deposit amount and rate limits
    /// 2. Receive CSPR from user (via attached_value)
    /// 3. Collect management fees (time-based)
    /// 4. Stake CSPR → get lstCSPR (via LiquidStaking)
    /// 5. Calculate cvCSPR shares to mint (ERC-4626 formula)
    /// 6. Mint cvCSPR shares to user
    /// 7. Deploy lstCSPR to strategies (via StrategyRouter)
    /// 8. Replenish instant withdrawal pool if needed
    /// 9. Update user tracking for performance fees
    /// 10. Emit Deposit event
    /// 
    /// **Returns:** Amount of cvCSPR shares minted
    pub fn deposit(&mut self) -> U512 {
        // Security checks
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let amount = self.env().attached_value();
        let caller = self.env().caller();
        
        if amount.is_zero() {
            self.env().revert(VaultError::ZeroAmount);
        }
        
        let max_deposit = self.max_deposit.get_or_default();
        if amount > max_deposit {
            self.env().revert(VaultError::RateLimitExceeded);
        }
        
        self.check_daily_deposit_limit(caller, amount);
        
        // Collect any pending management fees
        self.collect_management_fees();
        
        // Step 1: Stake CSPR to get lstCSPR
        // For now, assume 1:1 (will get actual lstCSPR amount from staking)
        let lst_cspr_received = amount;
        
        // Step 2: Calculate shares to mint (ERC-4626)
        let shares_to_mint = self.convert_to_shares(lst_cspr_received);
        
        // Validate minimum shares
        if shares_to_mint < self.min_shares.get_or_default() {
            self.env().revert(VaultError::InsufficientBalance);
        }
        
        // Step 3: Update total assets and shares
        let current_assets = self.total_assets.get_or_default();
        self.total_assets.set(current_assets + lst_cspr_received);
        
        let current_shares = self.total_shares.get_or_default();
        self.total_shares.set(current_shares + shares_to_mint);
        
        // Step 4: Update user shares
        let user_current_shares = self.user_shares.get(&caller).unwrap_or(U512::zero());
        self.user_shares.set(&caller, user_current_shares + shares_to_mint);
        
        // Step 5: Update user deposit tracking (for performance fees)
        self.update_user_deposit_tracking(caller, amount, shares_to_mint);
        
        // Step 6: Mint cvCSPR shares to user
        
        // Step 7: Deploy to strategies
        let amount_to_deploy = self.calculate_strategy_deployment(lst_cspr_received);
        if amount_to_deploy > U512::zero() {
        }
        
        // Step 8: Replenish instant withdrawal pool
        let pool_amount = lst_cspr_received - amount_to_deploy;
        if pool_amount > U512::zero() {
            let current_pool = self.instant_withdrawal_pool.get_or_default();
            self.instant_withdrawal_pool.set(current_pool + pool_amount);
        }
        
        self.env().emit_event(Deposit {
            user: caller,
            cspr_amount: amount,
            lst_cspr_amount: lst_cspr_received,
            shares_minted: shares_to_mint,
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        shares_to_mint
    }


    /// Withdraw assets by burning shares
    /// 
    /// Flow:
    /// 1. Burn caller's cvCSPR shares
    /// 2. Calculate assets using ERC-4626 formula
    /// 3. Withdraw lstCSPR from strategies if needed
    /// 4. Unstake lstCSPR → CSPR via LiquidStaking
    /// 5. Calculate and collect performance fee
    /// 6. Transfer CSPR to user
    /// 
    /// Returns: Amount of CSPR transferred to user (after fees)
    pub fn withdraw(&mut self, shares: U512) -> U512 {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let caller = self.env().caller();
        
        // Step 1: Validate user has enough shares
        let user_shares = self.user_shares.get(&caller).unwrap_or_default();
        if shares > user_shares || shares.is_zero() {
            self.reentrancy_guard.exit();
            self.env().revert(VaultError::InsufficientBalance);
        }
        
        // Step 2: Calculate assets using ERC-4626
        let total_assets_value = self.convert_to_assets(shares);
        
        // Step 3: Check instant withdrawal pool availability
        let instant_pool = self.instant_withdrawal_pool.get_or_default();
        
        // If pool has enough liquidity, use instant path (saves gas)
        let assets_after_fee = if total_assets_value <= instant_pool {
            let new_pool = instant_pool.checked_sub(total_assets_value).unwrap();
            self.instant_withdrawal_pool.set(new_pool);
            
            let fee_amount = self.calculate_performance_fee(&caller, total_assets_value);
            total_assets_value.checked_sub(fee_amount).unwrap()
        } else {
            // Need to withdraw from strategies
            let amount_from_pool = instant_pool;
            let amount_from_strategies = total_assets_value.checked_sub(instant_pool).unwrap();
            
            // Empty the pool
            self.instant_withdrawal_pool.set(U512::zero());
            
            
            
            let fee_amount = self.calculate_performance_fee(&caller, total_assets_value);
            total_assets_value.checked_sub(fee_amount).unwrap()
        };
        
        // Step 4: Burn user shares
        let new_user_shares = user_shares.checked_sub(shares).unwrap();
        if new_user_shares.is_zero() {
            self.user_shares.remove(&caller);
            self.user_deposits.remove(&caller);
        } else {
            self.user_shares.set(&caller, new_user_shares);
        }
        
        let total = self.total_shares.get_or_default();
        self.total_shares.set(total.checked_sub(shares).unwrap());
        
        // Step 5: TODO: Burn cvCSPR tokens
        
        // Step 6: TODO: Transfer CSPR to user
        
        self.env().emit_event(Withdraw {
            user: caller,
            assets: assets_after_fee,
            shares,
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        assets_after_fee
    }

    /// Request a time-locked withdrawal (no instant fee)
    /// 
    /// Benefits:
    /// - No instant withdrawal fee (saves 0.5%)
    /// - Can withdraw any amount (not limited by pool)
    /// 
    /// Tradeoff: Must wait timelock period (default 7 days)
    pub fn request_withdrawal(&mut self, shares: U512) -> u64 {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let caller = self.env().caller();
        
        let user_shares = self.user_shares.get(&caller).unwrap_or_default();
        if shares > user_shares || shares.is_zero() {
            self.reentrancy_guard.exit();
            self.env().revert(VaultError::InsufficientBalance);
        }
        
        let assets_value = self.convert_to_assets(shares);
        
        // Create withdrawal request
        let request_id = self.next_withdrawal_id.get_or_default();
        let unlock_time = self.env().get_block_time() + self.withdrawal_timelock.get_or_default();
        
        let request = WithdrawalRequest {
            user: caller,
            shares,
            assets_value,
            request_time: self.env().get_block_time(),
            unlock_time,
            completed: false,
        };
        
        self.withdrawal_requests.set(&request_id, request);
        self.next_withdrawal_id.set(request_id + 1);
        
        // Lock user shares (don't burn yet)
        // User can't withdraw or transfer these shares until request is completed
        let new_user_shares = user_shares.checked_sub(shares).unwrap();
        self.user_shares.set(&caller, new_user_shares);
        
        self.env().emit_event(WithdrawalRequested {
            user: caller,
            request_id,
            shares,
            assets_value,
            unlock_time,
        });
        
        self.reentrancy_guard.exit();
        request_id
    }

    /// Complete a time-locked withdrawal after timelock expires
    pub fn complete_withdrawal(&mut self, request_id: u64) -> U512 {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let caller = self.env().caller();
        
        // Get request
        let mut request = self.withdrawal_requests.get(&request_id)
            .unwrap_or_else(|| {
                self.reentrancy_guard.exit();
                self.env().revert(VaultError::InvalidRequest);
                unreachable!()
            });
        
        // Validate request
        if request.user != caller {
            self.reentrancy_guard.exit();
            self.env().revert(VaultError::Unauthorized);
        }
        
        if request.completed {
            self.reentrancy_guard.exit();
            self.env().revert(VaultError::InvalidRequest);
        }
        
        if self.env().get_block_time() < request.unlock_time {
            self.reentrancy_guard.exit();
            self.env().revert(VaultError::TimelockActive);
        }
        
        request.completed = true;
        self.withdrawal_requests.set(&request_id, request);
        
        // Withdraw from strategies if needed
        let instant_pool = self.instant_withdrawal_pool.get_or_default();
        
        if request.assets_value > instant_pool {
            let amount_from_strategies = request.assets_value.checked_sub(instant_pool).unwrap();
            
            
            
            self.instant_withdrawal_pool.set(U512::zero());
        } else {
            let new_pool = instant_pool.checked_sub(request.assets_value).unwrap();
            self.instant_withdrawal_pool.set(new_pool);
        }
        
        let fee_amount = self.calculate_performance_fee(&caller, request.assets_value);
        let assets_after_fee = request.assets_value.checked_sub(fee_amount).unwrap();
        
        let total = self.total_shares.get_or_default();
        self.total_shares.set(total.checked_sub(request.shares).unwrap());
        
        
        
        self.env().emit_event(WithdrawalCompleted {
            user: caller,
            request_id,
            assets: assets_after_fee,
            shares: request.shares,
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        assets_after_fee
    }

    /// Instant withdrawal with fee (uses liquidity pool)
    /// 
    /// Charges instant_withdrawal_fee (default 0.5%) for immediate liquidity
    /// Limited by instant withdrawal pool size
    pub fn instant_withdraw(&mut self, shares: U512) -> U512 {
        self.pausable.when_not_paused();
        self.reentrancy_guard.enter();
        
        let caller = self.env().caller();
        
        let user_shares = self.user_shares.get(&caller).unwrap_or_default();
        if shares > user_shares || shares.is_zero() {
            self.reentrancy_guard.exit();
            self.env().revert(VaultError::InsufficientBalance);
        }
        
        let assets_value = self.convert_to_assets(shares);
        
        let instant_pool = self.instant_withdrawal_pool.get_or_default();
        if assets_value > instant_pool {
            self.reentrancy_guard.exit();
            self.env().revert(VaultError::InsufficientLiquidity);
        }
        
        let instant_fee_bps = self.instant_withdrawal_fee_bps.get_or_default();
        let instant_fee = assets_value.checked_mul(U512::from(instant_fee_bps))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        let performance_fee = self.calculate_performance_fee(&caller, assets_value);
        
        // Total fees
        let total_fees = instant_fee.checked_add(performance_fee).unwrap();
        let assets_after_fee = assets_value.checked_sub(total_fees).unwrap();
        
        let new_pool = instant_pool.checked_sub(assets_value).unwrap();
        self.instant_withdrawal_pool.set(new_pool);
        
        let current_fees = self.fees_collected.get_or_default();
        self.fees_collected.set(current_fees.checked_add(total_fees).unwrap());
        
        // Burn user shares
        let new_user_shares = user_shares.checked_sub(shares).unwrap();
        if new_user_shares.is_zero() {
            self.user_shares.remove(&caller);
            self.user_deposits.remove(&caller);
        } else {
            self.user_shares.set(&caller, new_user_shares);
        }
        
        let total = self.total_shares.get_or_default();
        self.total_shares.set(total.checked_sub(shares).unwrap());
        
        
        
        self.env().emit_event(InstantWithdrawal {
            user: caller,
            assets: assets_after_fee,
            shares,
            fee: total_fees,
            timestamp: self.env().get_block_time(),
        });
        
        self.reentrancy_guard.exit();
        assets_after_fee
    }

    // ERC-4626 STANDARD FUNCTIONS

    /// Convert assets (CSPR) to shares (cvCSPR) using ERC-4626 formula
    /// 
    /// Formula:
    /// - If totalShares == 0: shares = assets (1:1 initial ratio)
    /// - Else: shares = (assets * totalShares) / totalAssets
    /// 
    /// This ensures fair share price for all users
    pub fn convert_to_shares(&self, assets: U512) -> U512 {
        let total_shares = self.total_shares.get_or_default();
        
        if total_shares.is_zero() {
            // First deposit: 1:1 ratio
            return assets;
        }
        
        let total_assets = self.total_assets();
        
        if total_assets.is_zero() {
            // Edge case: vault has shares but no assets (shouldn't happen)
            return assets;
        }
        
        // shares = (assets * totalShares) / totalAssets
        assets.checked_mul(total_shares)
            .unwrap()
            .checked_div(total_assets)
            .unwrap()
    }

    /// Convert shares (cvCSPR) to assets (CSPR) using ERC-4626 formula
    /// 
    /// Formula: assets = (shares * totalAssets) / totalShares
    /// 
    /// As yields accrue, totalAssets grows faster than totalShares,
    /// making each share worth more CSPR over time
    pub fn convert_to_assets(&self, shares: U512) -> U512 {
        let total_shares = self.total_shares.get_or_default();
        
        if total_shares.is_zero() {
            // No shares exist
            return U512::zero();
        }
        
        let total_assets = self.total_assets();
        
        // assets = (shares * totalAssets) / totalShares
        shares.checked_mul(total_assets)
            .unwrap()
            .checked_div(total_shares)
            .unwrap()
    }

    /// Calculate total assets under management
    /// 
    /// Includes:
    /// - lstCSPR staked in LiquidStaking contract
    /// - Assets deployed in strategy contracts
    /// - Instant withdrawal pool
    /// - Accrued but uncollected rewards
    pub fn total_assets(&self) -> U512 {
        // Start with instant pool
        let mut total = self.instant_withdrawal_pool.get_or_default();
        
        
        
        total
    }

    /// Maximum deposit allowed for a user (for rate limiting)
    pub fn max_deposit(&self, user: Address) -> U512 {
        let deposit_data = self.user_deposits.get(&user);
        
        match deposit_data {
            Some(data) => {
                let current_time = self.env().get_block_time();
                let time_window = 86400u64; // 24 hours
                
                // Reset if outside window
                if current_time > data.last_deposit_time + time_window {
                    return self.max_deposit_per_day.get_or_default();
                }
                
                let max_daily = self.max_deposit_per_day.get_or_default();
                let used = self.daily_deposits.get(&user).unwrap_or_default();
                max_daily.checked_sub(used).unwrap_or(U512::zero())
            },
            None => self.max_deposit_per_day.get_or_default(),
        }
    }

    /// Maximum withdrawal allowed for a user
    pub fn max_withdraw(&self, user: Address) -> U512 {
        let shares = self.user_shares.get(&user).unwrap_or_default();
        self.convert_to_assets(shares)
    }

    // FEE CALCULATION HELPERS

    /// Calculate performance fee for a user's withdrawal
    /// 
    /// Performance fee is charged on PROFITS only, not principal
    /// Tracks user's cost basis to determine profit
    fn calculate_performance_fee(&mut self, user: &Address, withdrawal_amount: U512) -> U512 {
        let deposit_data = self.user_deposits.get(user);
        
        match deposit_data {
            Some(data) => {
                let cost_basis = data.cost_basis;
                
                if withdrawal_amount <= cost_basis {
                    // No profit, no fee
                    return U512::zero();
                }
                
                let profit = withdrawal_amount.checked_sub(cost_basis).unwrap();
                
                // Apply performance fee to profit only
                let fee_bps = self.performance_fee_bps.get_or_default();
                let fee = profit.checked_mul(U512::from(fee_bps))
                    .unwrap()
                    .checked_div(U512::from(10000u64))
                    .unwrap();
                
                let current_fees = self.fees_collected.get_or_default();
                self.fees_collected.set(current_fees.checked_add(fee).unwrap());
                
                fee
            },
            None => {
                // No deposit data, treat entire withdrawal as profit (edge case)
                let fee_bps = self.performance_fee_bps.get_or_default();
                let fee = withdrawal_amount.checked_mul(U512::from(fee_bps))
                    .unwrap()
                    .checked_div(U512::from(10000u64))
                    .unwrap();
                
                let current_fees = self.fees_collected.get_or_default();
                self.fees_collected.set(current_fees.checked_add(fee).unwrap());
                
                fee
            }
        }
    }

    /// Collect management fees (time-based, called by keeper)
    /// 
    /// Management fee accrues continuously at annual rate (default 2%)
    /// Collected by minting new shares to treasury
    pub fn collect_management_fees(&mut self) {
        self.access_control.only_keeper();
        
        let current_time = self.env().get_block_time();
        let last_collection = self.last_management_fee_collection.get_or_default();
        
        // Require at least 1 hour between collections
        if current_time < last_collection + 3600 {
            self.env().revert(VaultError::RateLimitExceeded);
        }
        
        let time_elapsed = current_time - last_collection;
        
        let total_shares = self.total_shares.get_or_default();
        let fee_bps = self.management_fee_bps.get_or_default();
        let seconds_per_year = 31536000u64; // 365 days
        
        let fee_shares = total_shares
            .checked_mul(U512::from(fee_bps))
            .unwrap()
            .checked_mul(U512::from(time_elapsed))
            .unwrap()
            .checked_div(U512::from(seconds_per_year))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        if fee_shares.is_zero() {
            return;
        }
        
        self.total_shares.set(total_shares.checked_add(fee_shares).unwrap());
        self.last_management_fee_collection.set(current_time);
        
        let treasury = self.treasury.get().unwrap();
        let treasury_shares = self.user_shares.get(&treasury).unwrap_or_default();
        self.user_shares.set(&treasury, treasury_shares.checked_add(fee_shares).unwrap());
        
        
        self.env().emit_event(ManagementFeesCollected {
            shares: fee_shares,
            treasury,
            timestamp: current_time,
        });
    }

    /// Calculate optimal amount to deploy to strategies vs keep in pool
    fn calculate_strategy_deployment(&self, deposit_amount: U512) -> U512 {
        // Get target instant pool percentage (default 5%)
        let target_bps = self.instant_pool_target_bps.get_or_default();
        
        let total_assets = self.total_assets();
        let target_pool_size = total_assets
            .checked_mul(U512::from(target_bps))
            .unwrap()
            .checked_div(U512::from(10000u64))
            .unwrap();
        
        let current_pool = self.instant_withdrawal_pool.get_or_default();
        
        if current_pool >= target_pool_size {
            // Pool is at target, deploy entire amount
            return deposit_amount;
        }
        
        // Pool needs replenishment
        let pool_deficit = target_pool_size.checked_sub(current_pool).unwrap();
        
        if deposit_amount <= pool_deficit {
            // Entire deposit goes to pool
            return U512::zero();
        }
        
        // Split: fill pool deficit, deploy remainder
        deposit_amount.checked_sub(pool_deficit).unwrap()
    }

    /// Check and update daily deposit limit for user
    fn check_daily_deposit_limit(&mut self, user: &Address, amount: U512) -> bool {
        let current_time = self.env().get_block_time();
        let time_window = 86400u64; // 24 hours
        
        let deposit_data = self.user_deposits.get(user);
        
        match deposit_data {
            Some(data) => {
                if current_time > data.last_deposit_time + time_window {
                    // Reset daily limit
                    self.daily_deposits.set(user, amount);
                    return true;
                }
                
                let current_daily = self.daily_deposits.get(user).unwrap_or_default();
                let new_daily = current_daily.checked_add(amount).unwrap();
                let max_daily = self.max_deposit_per_day.get_or_default();
                
                if new_daily > max_daily {
                    return false;
                }
                
                self.daily_deposits.set(user, new_daily);
                true
            },
            None => {
                // First deposit
                self.daily_deposits.set(user, amount);
                true
            }
        }
    }

    /// Update user deposit tracking for fee calculations
    fn update_user_deposit_tracking(&mut self, user: &Address, amount: U512, shares: U512) {
        let current_time = self.env().get_block_time();
        
        let mut deposit_data = self.user_deposits.get(user).unwrap_or(UserDeposit {
            cost_basis: U512::zero(),
            total_deposited: U512::zero(),
            last_deposit_time: 0,
        });
        
        deposit_data.cost_basis = deposit_data.cost_basis.checked_add(amount).unwrap();
        deposit_data.total_deposited = deposit_data.total_deposited.checked_add(amount).unwrap();
        deposit_data.last_deposit_time = current_time;
        
        self.user_deposits.set(user, deposit_data);
    }


    /// Update contract addresses (admin only)
    pub fn set_liquid_staking(&mut self, address: Address) {
        self.access_control.only_admin();
        self.liquid_staking_address.set(address);
    }

    pub fn set_strategy_router(&mut self, address: Address) {
        self.access_control.only_admin();
        self.strategy_router_address.set(address);
    }

    pub fn set_cv_cspr_token(&mut self, address: Address) {
        self.access_control.only_admin();
        self.cv_cspr_token_address.set(address);
    }

    /// Update instant pool target (admin only)
    pub fn set_instant_pool_target(&mut self, target_bps: u16) {
        self.access_control.only_admin();
        
        // Validate: max 50% (5000 bps)
        if target_bps > 5000 {
            self.env().revert(VaultError::Unauthorized);
        }
        
        self.instant_pool_target_bps.set(target_bps);
    }

    /// Update deposit limits (admin only)
    pub fn update_deposit_limits(&mut self, max_per_tx: U512, max_per_day: U512) {
        self.access_control.only_admin();
        
        self.max_deposit_per_tx.set(max_per_tx);
        self.max_deposit_per_day.set(max_per_day);
    }

    /// Update withdrawal timelock (admin only)
    pub fn set_withdrawal_timelock(&mut self, timelock: u64) {
        self.access_control.only_admin();
        
        // Minimum 1 day, maximum 30 days
        if timelock < 86400 || timelock > 2592000 {
            self.env().revert(VaultError::Unauthorized);
        }
        
        self.withdrawal_timelock.set(timelock);
    }

    /// Rescue stuck funds (admin only, emergency use)
    pub fn rescue_funds(&mut self, token: Address, amount: U512, recipient: Address) {
        self.access_control.only_admin();
        
        
        self.env().emit_event(FundsRescued {
            token,
            amount,
            recipient,
            timestamp: self.env().get_block_time(),
        });
    }


    pub fn get_user_shares(&self, user: Address) -> U512 {
        self.user_shares.get(&user).unwrap_or_default()
    }

    pub fn get_user_assets(&self, user: Address) -> U512 {
        let shares = self.get_user_shares(user);
        self.convert_to_assets(shares)
    }

    pub fn get_withdrawal_request(&self, request_id: u64) -> Option<WithdrawalRequest> {
        self.withdrawal_requests.get(&request_id)
    }

    pub fn get_instant_pool_balance(&self) -> U512 {
        self.instant_withdrawal_pool.get_or_default()
    }

    pub fn get_fees_collected(&self) -> U512 {
        self.fees_collected.get_or_default()
    }

    pub fn get_share_price(&self) -> U512 {
        // Price of 1 share in CSPR (scaled by 1e9)
        let one_share = U512::from(1_000_000_000u64); // 1.0 with 9 decimals
        self.convert_to_assets(one_share)
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Deposit {
    pub user: Address,
    pub amount: U512,
    pub shares: U512,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Withdraw {
    pub user: Address,
    pub amount: U512,
    pub shares: U512,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct WithdrawalRequested {
    pub user: Address,
    pub request_id: u64,
    pub shares: U512,
    pub unlock_time: u64,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct WithdrawalCompleted {
    pub user: Address,
    pub request_id: u64,
    pub amount: U512,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct InstantWithdrawal {
    pub user: Address,
    pub amount: U512,
    pub fee: U512,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ManagementFeesCollected {
    pub amount: U512,
    pub fee_recipient: Address,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct FundsRescued {
    pub token: Address,
    pub amount: U512,
    pub recipient: Address,
}
