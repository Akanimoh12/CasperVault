/// Strategy modules for CasperVault
/// 
/// This module contains all strategy implementations for yield generation.

pub mod strategy_interface;
pub mod dex_strategy;
pub mod lending_strategy;
pub mod crosschain_strategy;

// Re-export key types
pub use strategy_interface::{IStrategy, RiskLevel, StrategyError, StrategyMetadata, AllocationConfig};
pub use dex_strategy::DEXStrategy;
pub use lending_strategy::LendingStrategy;
pub use crosschain_strategy::CrossChainStrategy;
