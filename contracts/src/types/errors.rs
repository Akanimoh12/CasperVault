/// Custom error types for CasperVault contracts
#[derive(Debug, PartialEq, Eq)]
pub enum VaultError {
    /// User has insufficient balance
    InsufficientBalance = 1,
    /// Amount provided is zero
    ZeroAmount = 2,
    /// Total shares is zero (cannot calculate share price)
    ZeroTotalShares = 3,
    /// Operation would result in arithmetic overflow
    ArithmeticOverflow = 4,
    /// Operation would result in arithmetic underflow
    ArithmeticUnderflow = 5,
    /// Vault is paused
    Paused = 6,
    /// Vault is not paused
    NotPaused = 7,
    /// Caller is not authorized for this operation
    Unauthorized = 8,
    /// Reentrancy attempt detected
    ReentrancyGuard = 9,
    /// Rate limit exceeded
    RateLimitExceeded = 10,
    /// Insufficient liquidity in instant withdrawal pool
    InsufficientLiquidity = 11,
    /// Withdrawal request not found
    WithdrawalRequestNotFound = 12,
    /// Withdrawal timelock not expired
    TimelockNotExpired = 13,
    /// Invalid withdrawal request ID
    InvalidRequestId = 14,
    /// Timelock still active
    TimelockActive = 15,
    /// Invalid request
    InvalidRequest = 16,
    /// Contract is paused
    ContractPaused = 17,
    /// Amount too low for operation
    AmountTooLow = 18,
    /// Operation attempted too soon
    TooSoon = 19,
    /// Conditions not met for operation
    ConditionsNotMet = 20,
    /// No fees available to distribute
    NoFeesToDistribute = 21,
    /// Invalid fee percentage
    InvalidFee = 22,
    /// Slippage exceeded maximum allowed
    SlippageExceeded = 23,
}

/// Errors specific to liquid staking operations
#[derive(Debug, PartialEq, Eq)]
pub enum StakingError {
    /// Validator not found in registry
    ValidatorNotFound = 100,
    /// Validator does not meet minimum requirements
    ValidatorNotEligible = 101,
    /// Validator is at maximum capacity
    ValidatorAtCapacity = 102,
    /// No eligible validators available
    NoEligibleValidators = 103,
    /// Delegation failed
    DelegationFailed = 104,
    /// Undelegation failed
    UndelegationFailed = 105,
    /// Reward claim failed
    RewardClaimFailed = 106,
    /// Invalid validator address
    InvalidValidator = 107,
    /// Cannot unstake more than staked amount
    ExceedsStakedAmount = 108,
}

/// Errors related to strategy operations
#[derive(Debug, PartialEq, Eq)]
pub enum StrategyError {
    /// Strategy not found
    StrategyNotFound = 200,
    /// Strategy deployment failed
    DeploymentFailed = 201,
    /// Strategy withdrawal failed
    WithdrawalFailed = 202,
    /// Strategy harvest failed
    HarvestFailed = 203,
    /// Invalid allocation percentage
    InvalidAllocation = 204,
    /// Allocation exceeds maximum per strategy
    AllocationExceedsMax = 205,
    /// Cross-chain allocation exceeds maximum
    CrossChainExceedsMax = 206,
    /// Total allocation does not equal 100%
    InvalidTotalAllocation = 207,
    /// Strategy health check failed
    UnhealthyStrategy = 208,
    /// Insufficient balance in strategy
    InsufficientStrategyBalance = 209,
}

/// Errors related to access control
#[derive(Debug, PartialEq, Eq)]
pub enum AccessError {
    /// Caller does not have required role
    MissingRole = 300,
    /// Role does not exist
    InvalidRole = 301,
    /// Cannot renounce last admin
    CannotRenounceLastAdmin = 302,
    /// Multi-sig threshold not met
    InsufficientSignatures = 303,
    /// Timelock not expired
    TimelockActive = 304,
}

/// Errors related to token operations
#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    /// Insufficient token balance
    InsufficientTokenBalance = 400,
    /// Mint amount is zero
    ZeroMintAmount = 401,
    /// Burn amount is zero
    ZeroBurnAmount = 402,
    /// Transfer amount is zero
    ZeroTransferAmount = 403,
    /// Transfer to zero address
    TransferToZeroAddress = 404,
    /// Allowance exceeded
    AllowanceExceeded = 405,
}

/// Errors related to bridge operations
#[derive(Debug, PartialEq, Eq)]
pub enum BridgeError {
    /// Bridge operation failed
    BridgeOperationFailed = 500,
    /// Insufficient confirmations
    InsufficientConfirmations = 501,
    /// Invalid proof
    InvalidProof = 502,
    /// Bridge is paused
    BridgePaused = 503,
    /// Bridge rate limit exceeded
    BridgeRateLimitExceeded = 504,
}

// Implement From trait for all custom errors to convert to OdraError
impl From<VaultError> for odra::OdraError {
    fn from(error: VaultError) -> Self {
        odra::OdraError::ExecutionError(odra::ExecutionError::User(error as u16))
    }
}

impl From<StakingError> for odra::OdraError {
    fn from(error: StakingError) -> Self {
        odra::OdraError::ExecutionError(odra::ExecutionError::User(error as u16))
    }
}

impl From<StrategyError> for odra::OdraError {
    fn from(error: StrategyError) -> Self {
        odra::OdraError::ExecutionError(odra::ExecutionError::User(error as u16))
    }
}

impl From<AccessError> for odra::OdraError {
    fn from(error: AccessError) -> Self {
        odra::OdraError::ExecutionError(odra::ExecutionError::User(error as u16))
    }
}

impl From<TokenError> for odra::OdraError {
    fn from(error: TokenError) -> Self {
        odra::OdraError::ExecutionError(odra::ExecutionError::User(error as u16))
    }
}

impl From<BridgeError> for odra::OdraError {
    fn from(error: BridgeError) -> Self {
        odra::OdraError::ExecutionError(odra::ExecutionError::User(error as u16))
    }
}
