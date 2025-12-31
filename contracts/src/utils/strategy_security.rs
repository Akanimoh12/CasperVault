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
    strategy_health: Mapping<Address, StrategyHealth>,
    risk_scores: Mapping<Address, RiskScore>,
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
    
    pub fn health_check(&mut self, strategy: Address) -> HealthStatus {
        let current_time = self.env().get_block_time();
        let mut health = self.strategy_health.get(&strategy).unwrap_or(StrategyHealth {
            last_check: 0,
            status: HealthStatus::Healthy,
            reported_balance: U512::zero(),
            actual_balance: U512::zero(),
            apy: 0,
            consecutive_failures: 0,
        });
        
        let status = if health.consecutive_failures >= 3 {
            HealthStatus::Critical
        } else if health.consecutive_failures >= 2 {
            HealthStatus::Warning
        } else if self.check_apy_in_range(health.apy) {
            HealthStatus::Healthy
        } else {
            HealthStatus::Warning
        };
        
        health.status = status;
        health.last_check = current_time;
        self.strategy_health.set(&strategy, health);
        
        self.env().emit_event(HealthCheckPerformed {
            strategy,
            status,
            timestamp: current_time,
        });
        
        status
    }
    
    pub fn risk_assessment(&mut self, strategy: Address) -> RiskScore {
        let health = self.strategy_health.get(&strategy).unwrap_or(StrategyHealth {
            last_check: 0,
            status: HealthStatus::Healthy,
            reported_balance: U512::zero(),
            actual_balance: U512::zero(),
            apy: 800,
            consecutive_failures: 0,
        });
        
        let tvl_score = self.calculate_tvl_score(health.actual_balance);
        let audit_score = 20;
        let age_score = 15;
        let performance_score = if health.consecutive_failures == 0 { 25 } else { 0 };
        
        let total_score = tvl_score + audit_score + age_score + performance_score;
        
        let risk_score = RiskScore {
            score: total_score,
            tvl_score,
            audit_score,
            age_score,
            performance_score,
        };
        
        self.risk_scores.set(&strategy, risk_score);
        risk_score
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
        if let Some(risk_score) = self.risk_scores.get(&strategy) {
            let threshold = self.max_risk_threshold.get_or_default();
            if risk_score.score > threshold {
                return true;
            }
        }
        
        if let Some(health) = self.strategy_health.get(&strategy) {
            if matches!(health.status, HealthStatus::Critical) {
                return true;
            }
        }
        
        false
    }
    
    pub fn record_failure(&mut self, strategy: Address) {
        let mut health = self.strategy_health.get(&strategy).unwrap_or(StrategyHealth {
            last_check: self.env().get_block_time(),
            status: HealthStatus::Healthy,
            reported_balance: U512::zero(),
            actual_balance: U512::zero(),
            apy: 0,
            consecutive_failures: 0,
        });
        
        health.consecutive_failures += 1;
        self.strategy_health.set(&strategy, health);
    }
    
    pub fn record_success(&mut self, strategy: Address) {
        let mut health = self.strategy_health.get(&strategy).unwrap_or(StrategyHealth {
            last_check: self.env().get_block_time(),
            status: HealthStatus::Healthy,
            reported_balance: U512::zero(),
            actual_balance: U512::zero(),
            apy: 0,
            consecutive_failures: 0,
        });
        
        health.consecutive_failures = 0;
        self.strategy_health.set(&strategy, health);
    }
    
    pub fn update_balance(&mut self, strategy: Address, reported: U512, actual: U512) {
        let mut health = self.strategy_health.get(&strategy).unwrap_or(StrategyHealth {
            last_check: self.env().get_block_time(),
            status: HealthStatus::Healthy,
            reported_balance: U512::zero(),
            actual_balance: U512::zero(),
            apy: 0,
            consecutive_failures: 0,
        });
        
        health.reported_balance = reported;
        health.actual_balance = actual;
        self.strategy_health.set(&strategy, health);
    }
    
    pub fn set_max_risk_threshold(&mut self, threshold: u8) {
        self.max_risk_threshold.set(threshold);
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct HealthCheckPerformed {
    pub strategy: Address,
    pub status: HealthStatus,
    pub timestamp: u64,
}
