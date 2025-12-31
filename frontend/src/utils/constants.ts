// API Configuration
export const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3001/api';
export const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:3001';

// Casper Network Configuration
export const CASPER_NETWORK = import.meta.env.VITE_CASPER_NETWORK || 'testnet';
export const CASPER_RPC_URL = import.meta.env.VITE_CASPER_RPC_URL || 'https://testnet.casper.network/rpc';
export const CASPER_CHAIN_NAME = import.meta.env.VITE_CASPER_CHAIN_NAME || 'casper-test';

// Contract Hashes (to be filled after deployment)
export const VAULT_CONTRACT_HASH = import.meta.env.VITE_VAULT_CONTRACT_HASH || '';
export const STAKING_CONTRACT_HASH = import.meta.env.VITE_STAKING_CONTRACT_HASH || '';
export const STRATEGY_CONTRACT_HASH = import.meta.env.VITE_STRATEGY_CONTRACT_HASH || '';

// Transaction Settings
export const DEFAULT_GAS_PRICE = 1; // 1 mote per gas
export const DEFAULT_TTL = 1800000; // 30 minutes in milliseconds

// UI Constants
export const TOAST_DURATION = 5000; // 5 seconds
export const REFRESH_INTERVAL = 30000; // 30 seconds
export const DEBOUNCE_DELAY = 300; // 300ms

// Validation
export const MIN_DEPOSIT_AMOUNT = 10; // 10 CSPR
export const MAX_DEPOSIT_AMOUNT = 1000000; // 1M CSPR
export const MIN_WITHDRAW_AMOUNT = 1; // 1 CSPR

// Fees
export const INSTANT_WITHDRAW_FEE = 0.5; // 0.5%
export const PERFORMANCE_FEE = 10; // 10%
export const MANAGEMENT_FEE = 2; // 2%

// Strategies
export const STRATEGIES = [
  {
    id: 'dex',
    name: 'DEX Liquidity',
    description: 'Provide liquidity on DEX platforms',
    apy: 25,
    risk: 'medium',
    color: '#0ea5e9',
  },
  {
    id: 'lending',
    name: 'Lending Protocol',
    description: 'Lend assets on lending platforms',
    apy: 15,
    risk: 'low',
    color: '#d946ef',
  },
  {
    id: 'cross_chain',
    name: 'Cross-Chain Bridge',
    description: 'Earn fees from cross-chain transfers',
    apy: 30,
    risk: 'high',
    color: '#10b981',
  },
  {
    id: 'staking',
    name: 'Native Staking',
    description: 'Stake CSPR with validators',
    apy: 12,
    risk: 'low',
    color: '#f59e0b',
  },
] as const;

// Risk Levels
export const RISK_LEVELS = {
  low: {
    label: 'Low Risk',
    color: 'success',
    description: 'Stable, predictable returns',
  },
  medium: {
    label: 'Medium Risk',
    color: 'warning',
    description: 'Balanced risk-reward profile',
  },
  high: {
    label: 'High Risk',
    color: 'danger',
    description: 'Higher returns, higher volatility',
  },
} as const;

// Chart Colors
export const CHART_COLORS = {
  primary: '#0ea5e9',
  accent: '#d946ef',
  success: '#10b981',
  warning: '#f59e0b',
  danger: '#ef4444',
  gray: '#6b7280',
};

// Animation Durations
export const ANIMATION = {
  fast: 150,
  normal: 300,
  slow: 500,
} as const;
