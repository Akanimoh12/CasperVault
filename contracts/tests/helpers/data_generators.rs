use odra::{casper_types::U512, Address};

pub struct UserDepositData {
    pub user: Address,
    pub amount: U512,
    pub timestamp: u64,
}

pub struct YieldScenario {
    pub initial_tvl: U512,
    pub dex_apy: u16,
    pub lending_apy: u16,
    pub staking_apy: u16,
    pub duration_days: u64,
    pub compound_frequency: u32,
}

pub struct StressTestConfig {
    pub num_users: usize,
    pub min_deposit: U512,
    pub max_deposit: U512,
    pub num_deposits: usize,
    pub num_withdrawals: usize,
    pub num_compounds: usize,
}

impl Default for StressTestConfig {
    fn default() -> Self {
        Self {
            num_users: 100,
            min_deposit: U512::from(100u64),
            max_deposit: U512::from(100_000u64),
            num_deposits: 150,
            num_withdrawals: 50,
            num_compounds: 10,
        }
    }
}

pub fn generate_user_deposits(count: usize, base_amount: U512) -> Vec<UserDepositData> {
    let mut deposits = Vec::new();
    
    for i in 0..count {
        let mut address_bytes = [0u8; 32];
        address_bytes[0] = (i as u8);
        
        let multiplier = ((i % 10) + 1) as u64;
        let amount = base_amount * U512::from(multiplier);
        
        deposits.push(UserDepositData {
            user: Address::from(address_bytes),
            amount,
            timestamp: (i as u64) * 3600,
        });
    }
    
    deposits
}

pub fn generate_yield_scenarios() -> Vec<YieldScenario> {
    vec![
        YieldScenario {
            initial_tvl: U512::from(1_000_000u64),
            dex_apy: 800,
            lending_apy: 500,
            staking_apy: 1000,
            duration_days: 30,
            compound_frequency: 10,
        },
        YieldScenario {
            initial_tvl: U512::from(5_000_000u64),
            dex_apy: 1200,
            lending_apy: 700,
            staking_apy: 900,
            duration_days: 90,
            compound_frequency: 30,
        },
        YieldScenario {
            initial_tvl: U512::from(10_000_000u64),
            dex_apy: 600,
            lending_apy: 400,
            staking_apy: 800,
            duration_days: 365,
            compound_frequency: 365,
        },
    ]
}

pub fn generate_random_amounts(count: usize, min: U512, max: U512, seed: u64) -> Vec<U512> {
    let mut amounts = Vec::new();
    let range = (max - min).as_u64();
    
    for i in 0..count {
        let random = ((seed + i as u64) * 48271) % 2147483647;
        let amount = min + U512::from(random % range);
        amounts.push(amount);
    }
    
    amounts
}

pub fn generate_validator_set(count: usize) -> Vec<(Address, u8, u8)> {
    let mut validators = Vec::new();
    
    for i in 0..count {
        let mut address_bytes = [0u8; 32];
        address_bytes[0] = 200 + (i as u8);
        
        let uptime = 95 + ((i % 5) as u8);
        let commission = 5 + ((i % 3) as u8);
        
        validators.push((Address::from(address_bytes), uptime, commission));
    }
    
    validators
}

pub fn generate_allocation_strategies() -> Vec<(String, u8)> {
    vec![
        ("Conservative".to_string(), 60),
        ("Balanced".to_string(), 25),
        ("Aggressive".to_string(), 15),
    ]
}

pub fn generate_edge_case_amounts() -> Vec<U512> {
    vec![
        U512::zero(),
        U512::from(1u64),
        U512::from(u64::MAX),
        U512::from(1_000_000u64),
        U512::from(1_000_000_000u64),
    ]
}

pub fn generate_time_series(start: u64, intervals: usize, interval_seconds: u64) -> Vec<u64> {
    (0..intervals)
        .map(|i| start + (i as u64 * interval_seconds))
        .collect()
}

pub fn generate_apy_variations(base_apy: u16, variations: usize) -> Vec<u16> {
    let mut apys = Vec::new();
    
    for i in 0..variations {
        let variation = ((i as i32 - variations as i32 / 2) * 100) as i16;
        let apy = (base_apy as i32 + variation as i32).max(0) as u16;
        apys.push(apy);
    }
    
    apys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_user_deposits() {
        let deposits = generate_user_deposits(10, U512::from(1000u64));
        assert_eq!(deposits.len(), 10);
        
        for deposit in &deposits {
            assert!(deposit.amount >= U512::from(1000u64));
        }
    }

    #[test]
    fn test_generate_yield_scenarios() {
        let scenarios = generate_yield_scenarios();
        assert_eq!(scenarios.len(), 3);
        
        for scenario in &scenarios {
            assert!(scenario.dex_apy > 0);
            assert!(scenario.duration_days > 0);
        }
    }

    #[test]
    fn test_generate_random_amounts() {
        let amounts = generate_random_amounts(
            5,
            U512::from(100u64),
            U512::from(1000u64),
            12345
        );
        
        assert_eq!(amounts.len(), 5);
        
        for amount in &amounts {
            assert!(*amount >= U512::from(100u64));
            assert!(*amount <= U512::from(1000u64));
        }
    }

    #[test]
    fn test_generate_validator_set() {
        let validators = generate_validator_set(5);
        assert_eq!(validators.len(), 5);
        
        for (_, uptime, commission) in &validators {
            assert!(*uptime >= 95);
            assert!(*commission < 10);
        }
    }
}
