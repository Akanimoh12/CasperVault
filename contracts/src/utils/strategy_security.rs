use odra::prelude::*;
use odra::{Address, Event, Mapping, Var};
use odra::casper_types::U512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unresponsive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RiskScore {
    pub score: u8,
    pub tvl_score: u8,
    pub audit_score: u8,
    pub age_score: u8,
    pub performance_score: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyHealth {
    pub last_check: u64,
    pub status: HealthStatus,
    pub reported_balance: U512,
    pub actual_balance: U512,
    pub apy: u32,
    pub consecutive_failures: u8,
}

#[odra::module]
pub struct StrategySecurity {
    // StrategyHealth fields - flattened
    strategy_last_check: Mapping<Address, u64>,
    strategy_status: Mapping<Address, u8>, // 0=Healthy, 1=Warning, 2=Critical, 3=Unresponsive
    strategy_reported_balance: Mapping<Address, U512>,
    strategy_actual_balance: Mapping<Address, U512>,
    strategy_apy: Mapping<Address, u32>,
    strategy_failures: Mapping<Address, u8>,
    
    // RiskScore fields - flattened
    risk_score: Mapping<Address, u8>,
    risk_tvl_score: Mapping<Address, u8>,
    risk_audit_score: Mapping<Address, u8>,
    risk_age_score: Mapping<Address, u8>,
    risk_performance_score: Mapping<Address, u8>,
    
    max_risk_threshold: Var<u8>,
    health_check_interval: Var<u64>,
    max_apy_threshold: Var<u32>,
    min_apy_threshold: Var<u32>,
}

#[odra::module]
impl StrategySecurity {
    pub fn init(&mut self) {
        self.max_risk_threshold.set(75);
        self.health_check_interval.set(3600);
        self.max_apy_threshold.set(10000);
        self.min_apy_threshold.set(100);
    }
    
    pub fn health_check(&mut self, strategy: Address) -> u8 {
        let current_time = self.env().get_block_time();
        let last_check = self.strategy_last_check.get(&strategy).unwrap_or(0);
        let consecutive_failures = self.strategy_failures.get(&strategy).unwrap_or(0);
        let apy = self.strategy_apy.get(&strategy).unwrap_or(0);
        
        let status = if consecutive_failures >= 3 {
            2u8 // Critical
        } else if consecutive_failures >= 2 {
            1u8 // Warning
        } else if self.check_apy_in_range(apy) {
            0u8 // Healthy
        } else {
            1u8 // Warning
        };
        
        self.strategy_status.set(&strategy, status);
        self.strategy_last_check.set(&strategy, current_time);
        
        self.env().emit_event(HealthCheckPerformed {
            strategy,
            status,
            timestamp: current_time,
        });
        
        status
    }
    
    pub fn risk_assessment(&mut self, strategy: Address) -> u8 {
        let actual_balance = self.strategy_actual_balance.get(&strategy).unwrap_or(U512::zero());
        let consecutive_failures = self.strategy_failures.get(&strategy).unwrap_or(0);
        
        let tvl_score = self.calculate_tvl_score(actual_balance);
        let audit_score = 20;
        let age_score = 15;
        let performance_score = if consecutive_failures == 0 { 25 } else { 0 };
        
        let total_score = tvl_score + audit_score + age_score + performance_score;
        
        self.risk_score.set(&strategy, total_score);
        self.risk_tvl_score.set(&strategy, tvl_score);
        self.risk_audit_score.set(&strategy, audit_score);
        self.risk_age_score.set(&strategy, age_score);
        self.risk_performance_score.set(&strategy, performance_score);
        
        total_score
    }
    
    fn calculate_tvl_score(&self, tvl: U512) -> u8 {
        let million = U512::from(1_000_000_000_000_000u64);
        if tvl >= million * U512::from(10u64) {
            40
        } else if tvl >= million * U512::from(5u64) {
            30
        } else if tvl >= million {
            20
        } else {
            10
        }
    }
    
    fn check_apy_in_range(&self, apy: u32) -> bool {
        let max_apy = self.max_apy_threshold.get_or_default();
        let min_apy = self.min_apy_threshold.get_or_default();
        apy >= min_apy && apy <= max_apy
    }
    
    pub fn should_withdraw(&self, strategy: Address) -> bool {
        if let Some(risk_score_value) = self.risk_score.get(&strategy) {
            let threshold = self.max_risk_threshold.get_or_default();
            if risk_score_value > threshold {
                return true;
            }
        }
        
        let status = self.strategy_status.get(&strategy).unwrap_or(0);
        if status == 2 { // Critical
            return true;
        }
        
        false
    }
    
    pub fn record_failure(&mut self, strategy: Address) {
        let failures = self.strategy_failures.get(&strategy).unwrap_or(0);
        self.strategy_failures.set(&strategy, failures + 1);
    }
    
    pub fn record_success(&mut self, strategy: Address) {
        self.strategy_failures.set(&strategy, 0);
        self.strategy_status.set(&strategy, 0); // Healthy
    }
    
    pub fn update_balance(&mut self, strategy: Address, reported: U512, actual: U512) {
        self.strategy_reported_balance.set(&strategy, reported);
        self.strategy_actual_balance.set(&strategy, actual);
    }
    
    pub fn set_max_risk_threshold(&mut self, threshold: u8) {
        self.max_risk_threshold.set(threshold);
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct HealthCheckPerformed {
    pub strategy: Address,
    pub status: u8, // 0=Healthy, 1=Warning, 2=Critical, 3=Unresponsive
    pub timestamp: u64,
}
