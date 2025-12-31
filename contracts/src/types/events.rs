use odra::prelude::*;
use odra::casper_types::{U256, U512};
use odra::{Address, Event};

/// Event emitted when a deposit is made
#[derive(Event, Debug, PartialEq, Eq)]
pub struct Deposit {
    pub user: Address,
    pub cspr_amount: U512,
    pub lst_cspr_amount: U512,
    pub shares_minted: U512,
    pub timestamp: u64,
}

/// Event emitted when a withdrawal is made
#[derive(Event, Debug, PartialEq, Eq)]
pub struct Withdraw {
    pub user: Address,
    pub shares_burned: U512,
    pub shares: U512,
    pub assets: U512,
    pub timestamp: u64,
}

/// Event emitted when an instant withdrawal is made
#[derive(Event, Debug, PartialEq, Eq)]
pub struct InstantWithdraw {
    pub user: Address,
    pub shares_burned: U512,
    pub cspr_amount: U512,
    pub fee_amount: U512,
    pub timestamp: u64,
}

/// Event emitted when a withdrawal request is created
#[derive(Event, Debug, PartialEq, Eq)]
pub struct WithdrawalRequested {
    pub request_id: U256,
    pub user: Address,
    pub shares: U512,
    pub assets_value: U512,
    pub unlock_time: u64,
}

/// Event emitted when a withdrawal request is completed
#[derive(Event, Debug, PartialEq, Eq)]
pub struct WithdrawalCompleted {
    pub request_id: U256,
    pub user: Address,
    pub assets: U512,
    pub shares: U512,
    pub cspr_amount: U512,
    pub timestamp: u64,
}

/// Event emitted when an instant withdrawal is processed
#[derive(Event, Debug, PartialEq, Eq)]
pub struct InstantWithdrawal {
    pub user: Address,
    pub shares_burned: U512,
    pub assets: U512,
    pub shares: U512,
    pub fee: U512,
    pub cspr_amount: U512,
    pub fee_amount: U512,
    pub timestamp: u64,
}

/// Event emitted when CSPR is staked
#[derive(Event, Debug, PartialEq, Eq)]
pub struct Stake {
    pub user: Address,
    pub cspr_amount: U512,
    pub lst_cspr_minted: U512,
    pub timestamp: u64,
}

/// Event emitted when lstCSPR is unstaked
#[derive(Event, Debug, PartialEq, Eq)]
pub struct Unstake {
    pub user: Address,
    pub lst_cspr_amount: U512,
    pub cspr_amount: U512,
    pub timestamp: u64,
}

/// Event emitted when rewards are compounded
#[derive(Event, Debug, PartialEq, Eq)]
pub struct CompoundRewards {
    pub total_rewards: U512,
    pub restaked_amount: U512,
    pub new_total_staked: U512,
    pub timestamp: u64,
}

/// Event emitted when a validator is added
#[derive(Event, Debug, PartialEq, Eq)]
pub struct ValidatorAdded {
    pub validator: Address,
    pub uptime_percentage: u8,
    pub commission_rate: u8,
}

/// Event emitted when a validator is removed
#[derive(Event, Debug, PartialEq, Eq)]
pub struct ValidatorRemoved {
    pub validator: Address,
    pub reason: String,
}

/// Event emitted when funds are allocated to strategies
#[derive(Event, Debug, PartialEq, Eq)]
pub struct AllocationUpdate {
    pub strategy_name: String,
    pub amount: U512,
    pub total_allocated: U512,
    pub timestamp: u64,
}

/// Event emitted when strategies are rebalanced
#[derive(Event, Debug, PartialEq, Eq)]
pub struct Rebalance {
    pub timestamp: u64,
}

/// Event emitted when yields are harvested
#[derive(Event, Debug, PartialEq, Eq)]
pub struct YieldHarvested {
    pub strategy_name: String,
    pub yield_amount: U512,
    pub timestamp: u64,
}

/// Event emitted when yields are compounded
#[derive(Event, Debug, PartialEq, Eq)]
pub struct YieldCompounded {
    pub total_yield: U512,
    pub performance_fee: U512,
    pub compounded_amount: U512,
    pub new_share_price: U256,
    pub timestamp: u64,
}

/// Event emitted when share price is updated
#[derive(Event, Debug, PartialEq, Eq)]
pub struct SharePriceUpdate {
    pub old_price: U256,
    pub new_price: U256,
    pub total_assets: U512,
    pub total_shares: U512,
    pub timestamp: u64,
}

/// Event emitted when a role is granted
#[derive(Event, Debug, PartialEq, Eq)]
pub struct RoleGranted {
    pub role: u8,
    pub account: Address,
    pub grantor: Address,
}

/// Event emitted when a role is revoked
#[derive(Event, Debug, PartialEq, Eq)]
pub struct RoleRevoked {
    pub role: u8,
    pub account: Address,
    pub revoker: Address,
}

/// Event emitted when a role is renounced
#[derive(Event, Debug, PartialEq, Eq)]
pub struct RoleRenounced {
    pub role: u8,
    pub account: Address,
}

/// Event emitted when contract is paused
#[derive(Event, Debug, PartialEq, Eq)]
pub struct Paused {
    pub by: Address,
    pub timestamp: u64,
}

/// Event emitted when contract is unpaused
#[derive(Event, Debug, PartialEq, Eq)]
pub struct Unpaused {
    pub by: Address,
    pub timestamp: u64,
}

/// Event emitted when fees are updated
#[derive(Event, Debug, PartialEq, Eq)]
pub struct FeesUpdated {
    pub performance_fee_bps: u32,
    pub management_fee_bps: u32,
    pub instant_withdrawal_fee_bps: u32,
    pub updated_by: Address,
}

/// Event emitted when fees are collected
#[derive(Event, Debug, PartialEq, Eq)]
pub struct FeesCollected {
    pub amount: U512,
    pub recipient: Address,
    pub timestamp: u64,
}

/// Event emitted when emergency actions are taken
#[derive(Event, Debug, PartialEq, Eq)]
pub struct EmergencyAction {
    pub action_type: String,
    pub by: Address,
    pub details: String,
    pub timestamp: u64,
}

/// Event emitted for cross-chain bridge operations
#[derive(Event, Debug, PartialEq, Eq)]
pub struct BridgeInitiated {
    pub source_chain: String,
    pub target_chain: String,
    pub amount: U512,
    pub transaction_id: String,
}

/// Event emitted when bridge operation completes
#[derive(Event, Debug, PartialEq, Eq)]
pub struct BridgeCompleted {
    pub transaction_id: String,
    pub amount: U512,
    pub timestamp: u64,
}

/// Event emitted when management fees are collected
#[derive(Event, Debug, PartialEq, Eq)]
pub struct ManagementFeesCollected {
    pub amount: U512,
    pub shares: U512,
    pub fee_recipient: Address,
    pub treasury: Address,
    pub timestamp: u64,
}

/// Event emitted when funds are rescued from contract
#[derive(Event, Debug, PartialEq, Eq)]
pub struct FundsRescued {
    pub token: Address,
    pub amount: U512,
    pub recipient: Address,
}
