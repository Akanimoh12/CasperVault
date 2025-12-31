extern crate alloc;

pub mod core;
pub mod tokens;
pub mod utils;
pub mod types;
pub mod strategies;
pub mod mocks;

pub use core::{VaultManager, LiquidStaking, StrategyRouter, YieldAggregator};
pub use tokens::{LstCspr, CvCspr};
pub use utils::{AccessControl, ReentrancyGuard, Pausable, Role};
pub use types::*;
pub use strategies::{
    IStrategy, RiskLevel, StrategyError, StrategyMetadata, AllocationConfig,
    DEXStrategy, LendingStrategy, CrossChainStrategy
};
pub use mocks::*;

use odra::prelude::*;
use odra::Event;
use odra::{Address, Mapping, SubModule, Var};
use odra::casper_types::{U256, U512};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert_eq!(1 + 1, 2);
    }
}
