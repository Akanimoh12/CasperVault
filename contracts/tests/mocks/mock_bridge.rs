/// Mock Cross-Chain Bridge for Testing
/// 
/// Simulates a cross-chain bridge with configurable parameters
/// for testing the CrossChainStrategy without external dependencies.

use odra::prelude::*;
use odra::types::{Address, U512};

/// Target chain enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetChain {
    Ethereum,
    Polygon,
    Arbitrum,
    Optimism,
}

/// Bridge transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeStatus {
    Initiated,
    Confirmed,
    Completed,
    Failed,
}

/// Bridge transaction tracking
#[derive(Debug, Clone)]
struct BridgeTransaction {
    /// Source user
    user: Address,
    
    /// Amount bridged
    amount: U512,
    
    /// Target chain
    target_chain: TargetChain,
    
    /// Bridge timestamp
    bridge_time: u64,
    
    /// Status
    status: BridgeStatus,
    
    /// Transaction hash (simulated)
    tx_hash: String,
    
    /// Fee charged
    fee: U512,
}

/// Mock Bridge for testing
#[odra::module]
pub struct MockBridge {
    /// Bridge transactions (tx_hash -> transaction)
    transactions: Mapping<String, BridgeTransaction>,
    
    /// User's active bridges (user -> tx_hashes)
    user_bridges: Mapping<Address, Vec<String>>,
    
    /// Total bridged per chain
    total_bridged_ethereum: Var<U512>,
    total_bridged_polygon: Var<U512>,
    total_bridged_arbitrum: Var<U512>,
    total_bridged_optimism: Var<U512>,
    
    /// Configuration
    bridge_fee_bps: Var<u16>, // 50 = 0.5%
    min_bridge_amount: Var<U512>,
    confirmation_time: Var<u64>, // Seconds to confirm
    
    /// Target chain APYs (simulated yields)
    ethereum_apy_bps: Var<u16>, // 1800 = 18%
    polygon_apy_bps: Var<u16>, // 1600 = 16%
    arbitrum_apy_bps: Var<u16>, // 1700 = 17%
    optimism_apy_bps: Var<u16>, // 1650 = 16.5%
    
    /// Transaction counter for unique hashes
    tx_counter: Var<u64>,
}

#[odra::module]
impl MockBridge {
    /// Initialize the mock bridge
    pub fn init(&mut self, bridge_fee_bps: u16, min_bridge_amount: U512) {
        self.bridge_fee_bps.set(bridge_fee_bps);
        self.min_bridge_amount.set(min_bridge_amount);
        self.confirmation_time.set(3600); // 1 hour
        
        // Set target chain APYs
        self.ethereum_apy_bps.set(1800); // 18%
        self.polygon_apy_bps.set(1600); // 16%
        self.arbitrum_apy_bps.set(1700); // 17%
        self.optimism_apy_bps.set(1650); // 16.5%
        
        self.total_bridged_ethereum.set(U512::zero());
        self.total_bridged_polygon.set(U512::zero());
        self.total_bridged_arbitrum.set(U512::zero());
        self.total_bridged_optimism.set(U512::zero());
        self.tx_counter.set(0);
    }
    
    /// Initiate bridge to target chain
    /// 
    /// Returns: (tx_hash, amount_after_fee, fee)
    pub fn bridge_to(&mut self, amount: U512, target_chain: TargetChain) -> (String, U512, U512) {
        let caller = self.env().caller();
        
        // Validate minimum amount
        let min_amount = self.min_bridge_amount.get_or_default();
        if amount < min_amount {
            return (String::new(), U512::zero(), U512::zero());
        }
        
        // Calculate fee
        let fee_bps = self.bridge_fee_bps.get_or_default();
        let fee = amount
            .checked_mul(U512::from(fee_bps))
            .and_then(|v| v.checked_div(U512::from(10000u64)))
            .unwrap_or(U512::zero());
        
        let amount_after_fee = amount.checked_sub(fee).unwrap_or(U512::zero());
        
        // Generate unique tx hash
        let counter = self.tx_counter.get_or_default();
        let tx_hash = format!("0xmock{:016x}", counter);
        self.tx_counter.set(counter + 1);
        
        // Create transaction
        let transaction = BridgeTransaction {
            user: caller,
            amount: amount_after_fee,
            target_chain,
            bridge_time: self.env().get_block_time(),
            status: BridgeStatus::Initiated,
            tx_hash: tx_hash.clone(),
            fee,
        };
        
        self.transactions.set(&tx_hash, transaction);
        
        // Track user's bridges
        let mut user_txs = self.user_bridges.get(&caller).unwrap_or_default();
        user_txs.push(tx_hash.clone());
        self.user_bridges.set(&caller, user_txs);
        
        // Update total bridged
        match target_chain {
            TargetChain::Ethereum => {
                let total = self.total_bridged_ethereum.get_or_default();
                self.total_bridged_ethereum.set(total.checked_add(amount_after_fee).unwrap());
            }
            TargetChain::Polygon => {
                let total = self.total_bridged_polygon.get_or_default();
                self.total_bridged_polygon.set(total.checked_add(amount_after_fee).unwrap());
            }
            TargetChain::Arbitrum => {
                let total = self.total_bridged_arbitrum.get_or_default();
                self.total_bridged_arbitrum.set(total.checked_add(amount_after_fee).unwrap());
            }
            TargetChain::Optimism => {
                let total = self.total_bridged_optimism.get_or_default();
                self.total_bridged_optimism.set(total.checked_add(amount_after_fee).unwrap());
            }
        }
        
        // Emit event
        self.env().emit_event(BridgeInitiated {
            user: caller,
            amount: amount_after_fee,
            fee,
            target_chain: self.chain_to_string(target_chain),
            tx_hash: tx_hash.clone(),
            timestamp: self.env().get_block_time(),
        });
        
        (tx_hash, amount_after_fee, fee)
    }
    
    /// Confirm bridge transaction (simulates confirmation from target chain)
    pub fn confirm_bridge(&mut self, tx_hash: String) {
        if let Some(mut transaction) = self.transactions.get(&tx_hash) {
            transaction.status = BridgeStatus::Confirmed;
            self.transactions.set(&tx_hash, transaction.clone());
            
            self.env().emit_event(BridgeConfirmed {
                tx_hash,
                target_chain: self.chain_to_string(transaction.target_chain),
                timestamp: self.env().get_block_time(),
            });
        }
    }
    
    /// Complete bridge transaction (simulates deployment on target chain)
    pub fn complete_bridge(&mut self, tx_hash: String) {
        if let Some(mut transaction) = self.transactions.get(&tx_hash) {
            transaction.status = BridgeStatus::Completed;
            self.transactions.set(&tx_hash, transaction.clone());
            
            self.env().emit_event(BridgeCompleted {
                tx_hash,
                amount: transaction.amount,
                target_chain: self.chain_to_string(transaction.target_chain),
                timestamp: self.env().get_block_time(),
            });
        }
    }
    
    /// Calculate yields earned on target chain
    /// 
    /// Returns: Yields earned based on time elapsed and APY
    pub fn calculate_yield(&self, tx_hash: String) -> U512 {
        if let Some(transaction) = self.transactions.get(&tx_hash) {
            if transaction.status != BridgeStatus::Completed {
                return U512::zero();
            }
            
            let current_time = self.env().get_block_time();
            let time_elapsed = current_time.saturating_sub(transaction.bridge_time);
            
            // Get APY for target chain
            let apy_bps = match transaction.target_chain {
                TargetChain::Ethereum => self.ethereum_apy_bps.get_or_default(),
                TargetChain::Polygon => self.polygon_apy_bps.get_or_default(),
                TargetChain::Arbitrum => self.arbitrum_apy_bps.get_or_default(),
                TargetChain::Optimism => self.optimism_apy_bps.get_or_default(),
            };
            
            // Calculate yield
            let seconds_per_year = 31_536_000u64;
            transaction.amount
                .checked_mul(U512::from(apy_bps))
                .and_then(|v| v.checked_mul(U512::from(time_elapsed)))
                .and_then(|v| v.checked_div(U512::from(10000u64)))
                .and_then(|v| v.checked_div(U512::from(seconds_per_year)))
                .unwrap_or(U512::zero())
        } else {
            U512::zero()
        }
    }
    
    /// Withdraw from target chain (initiate return bridge)
    /// 
    /// Returns: (Amount to return, Bridge fee)
    pub fn withdraw_from_target(&mut self, tx_hash: String, amount: U512) -> (U512, U512) {
        if let Some(transaction) = self.transactions.get(&tx_hash) {
            if transaction.status != BridgeStatus::Completed {
                return (U512::zero(), U512::zero());
            }
            
            // Calculate fee for return bridge
            let fee_bps = self.bridge_fee_bps.get_or_default();
            let fee = amount
                .checked_mul(U512::from(fee_bps))
                .and_then(|v| v.checked_div(U512::from(10000u64)))
                .unwrap_or(U512::zero());
            
            let amount_after_fee = amount.checked_sub(fee).unwrap_or(U512::zero());
            
            // Update total bridged (reduce)
            match transaction.target_chain {
                TargetChain::Ethereum => {
                    let total = self.total_bridged_ethereum.get_or_default();
                    self.total_bridged_ethereum.set(total.checked_sub(amount).unwrap_or(U512::zero()));
                }
                TargetChain::Polygon => {
                    let total = self.total_bridged_polygon.get_or_default();
                    self.total_bridged_polygon.set(total.checked_sub(amount).unwrap_or(U512::zero()));
                }
                TargetChain::Arbitrum => {
                    let total = self.total_bridged_arbitrum.get_or_default();
                    self.total_bridged_arbitrum.set(total.checked_sub(amount).unwrap_or(U512::zero()));
                }
                TargetChain::Optimism => {
                    let total = self.total_bridged_optimism.get_or_default();
                    self.total_bridged_optimism.set(total.checked_sub(amount).unwrap_or(U512::zero()));
                }
            }
            
            // Emit event
            self.env().emit_event(WithdrawalInitiated {
                tx_hash: tx_hash.clone(),
                amount: amount_after_fee,
                fee,
                target_chain: self.chain_to_string(transaction.target_chain),
                timestamp: self.env().get_block_time(),
            });
            
            (amount_after_fee, fee)
        } else {
            (U512::zero(), U512::zero())
        }
    }
    
    /// Get bridge transaction details
    pub fn get_transaction(&self, tx_hash: String) -> Option<(Address, U512, u8, BridgeStatus)> {
        self.transactions.get(&tx_hash).map(|tx| {
            let chain_id = match tx.target_chain {
                TargetChain::Ethereum => 1u8,
                TargetChain::Polygon => 2u8,
                TargetChain::Arbitrum => 3u8,
                TargetChain::Optimism => 4u8,
            };
            (tx.user, tx.amount, chain_id, tx.status)
        })
    }
    
    /// Get user's bridge transactions
    pub fn get_user_bridges(&self, user: Address) -> Vec<String> {
        self.user_bridges.get(&user).unwrap_or_default()
    }
    
    /// Get total bridged to a specific chain
    pub fn get_total_bridged(&self, target_chain: TargetChain) -> U512 {
        match target_chain {
            TargetChain::Ethereum => self.total_bridged_ethereum.get_or_default(),
            TargetChain::Polygon => self.total_bridged_polygon.get_or_default(),
            TargetChain::Arbitrum => self.total_bridged_arbitrum.get_or_default(),
            TargetChain::Optimism => self.total_bridged_optimism.get_or_default(),
        }
    }
    
    /// Get APY for a specific target chain
    pub fn get_target_apy(&self, target_chain: TargetChain) -> u16 {
        match target_chain {
            TargetChain::Ethereum => self.ethereum_apy_bps.get_or_default(),
            TargetChain::Polygon => self.polygon_apy_bps.get_or_default(),
            TargetChain::Arbitrum => self.arbitrum_apy_bps.get_or_default(),
            TargetChain::Optimism => self.optimism_apy_bps.get_or_default(),
        }
    }
    
    /// Helper: Convert chain enum to string
    fn chain_to_string(&self, chain: TargetChain) -> String {
        match chain {
            TargetChain::Ethereum => "Ethereum".to_string(),
            TargetChain::Polygon => "Polygon".to_string(),
            TargetChain::Arbitrum => "Arbitrum".to_string(),
            TargetChain::Optimism => "Optimism".to_string(),
        }
    }
}

// ============================================
// EVENTS
// ============================================

#[odra::event]
struct BridgeInitiated {
    user: Address,
    amount: U512,
    fee: U512,
    target_chain: String,
    tx_hash: String,
    timestamp: u64,
}

#[odra::event]
struct BridgeConfirmed {
    tx_hash: String,
    target_chain: String,
    timestamp: u64,
}

#[odra::event]
struct BridgeCompleted {
    tx_hash: String,
    amount: U512,
    target_chain: String,
    timestamp: u64,
}

#[odra::event]
struct WithdrawalInitiated {
    tx_hash: String,
    amount: U512,
    fee: U512,
    target_chain: String,
    timestamp: u64,
}
