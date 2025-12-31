use odra::prelude::*;
use odra::{Address, Event, Mapping, Var};
use odra::casper_types::U512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertType {
    UnusualWithdrawal,
    SuddenApyChange,
    LargePriceMovement,
    FailedTransactionSpike,
    StrategyFailure,
    ValidatorIssue,
    BridgeAnomaly,
}

#[derive(Debug, PartialEq, Eq, odra::OdraType)]
pub struct AlertData {
    pub alert_type: u8,  // Changed from AlertType enum to u8
    pub severity: u8,
    pub description: Address,
    pub value: U512,
    pub timestamp: u64,
}

#[odra::module]
pub struct Monitor {
    alert_count: Var<u64>,
    alerts: Mapping<u64, AlertData>,
    last_tvl: Var<U512>,
    last_tvl_check: Var<u64>,
    failed_tx_count: Var<u64>,
    failed_tx_window_start: Var<u64>,
}

#[odra::module]
impl Monitor {
    pub fn init(&mut self) {
        self.alert_count.set(0);
        self.last_tvl.set(U512::zero());
        self.last_tvl_check.set(self.env().get_block_time());
        self.failed_tx_count.set(0);
        self.failed_tx_window_start.set(self.env().get_block_time());
    }
    
    pub fn emit_alert(&mut self, alert_type: u8, severity: u8, related_address: Address, value: U512) {
        let current_time = self.env().get_block_time();
        let alert = AlertData {
            alert_type,
            severity,
            description: related_address,
            value,
            timestamp: current_time,
        };
        
        let count = self.alert_count.get_or_default();
        self.alerts.set(&count, alert.clone());
        self.alert_count.set(count + 1);
        
        self.env().emit_event(AlertEmitted {
            alert_type,
            severity,
            related_address,
            value,
            timestamp: current_time,
        });
    }
    
    pub fn check_withdrawal_anomaly(&mut self, amount: U512, total_tvl: U512) -> bool {
        if total_tvl == U512::zero() {
            return false;
        }
        
        let percentage = (amount * U512::from(100u64)) / total_tvl;
        let ten_percent = U512::from(10u64);
        
        if percentage >= ten_percent {
            self.emit_alert(
                0, // AlertType::UnusualWithdrawal
                8,
                self.env().caller(),
                amount
            );
            return true;
        }
        
        false
    }
    
    pub fn check_tvl_rapid_decrease(&mut self, current_tvl: U512) -> bool {
        let current_time = self.env().get_block_time();
        let last_check = self.last_tvl_check.get_or_default();
        let last_tvl = self.last_tvl.get_or_default();
        
        if current_time < last_check + 3600 {
            return false;
        }
        
        if last_tvl == U512::zero() {
            self.last_tvl.set(current_tvl);
            self.last_tvl_check.set(current_time);
            return false;
        }
        
        if current_tvl < last_tvl {
            let decrease = last_tvl - current_tvl;
            let percentage = (decrease * U512::from(100u64)) / last_tvl;
            let twenty_percent = U512::from(20u64);
            
            if percentage >= twenty_percent {
                self.emit_alert(
                    0, // AlertType::UnusualWithdrawal
                    9,
                    self.env().caller(),
                    decrease
                );
                self.last_tvl.set(current_tvl);
                self.last_tvl_check.set(current_time);
                return true;
            }
        }
        
        self.last_tvl.set(current_tvl);
        self.last_tvl_check.set(current_time);
        false
    }
    
    pub fn record_failed_transaction(&mut self) {
        let current_time = self.env().get_block_time();
        let window_start = self.failed_tx_window_start.get_or_default();
        
        if current_time >= window_start + 3600 {
            self.failed_tx_count.set(1);
            self.failed_tx_window_start.set(current_time);
        } else {
            let count = self.failed_tx_count.get_or_default();
            self.failed_tx_count.set(count + 1);
            
            if count + 1 >= 10 {
                self.emit_alert(
                    3, // AlertType::FailedTransactionSpike
                    7,
                    self.env().caller(),
                    U512::from(count + 1)
                );
            }
        }
    }
    
    pub fn get_alert(&self, index: u64) -> Option<AlertData> {
        self.alerts.get(&index)
    }
    
    pub fn get_alert_count(&self) -> u64 {
        self.alert_count.get_or_default()
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct AlertEmitted {
    pub alert_type: u8,  // Changed from AlertType enum to u8
    pub severity: u8,
    pub related_address: Address,
    pub value: U512,
    pub timestamp: u64,
}
