// Contract Types
export interface VaultInfo {
  totalAssets: string;
  totalShares: string;
  sharePrice: string;
  currentAPY: number;
  isPaused: boolean;
  performanceFee: number;
  managementFee: number;
}

export interface UserPosition {
  shares: string;
  assets: string;
  depositedAmount: string;
  rewards: string;
  apy: number;
  lastDepositTime: number;
}

export interface Strategy {
  id: string;
  name: string;
  description: string;
  contractHash: string;
  allocation: number;
  targetAllocation: number;
  currentAPY: number;
  tvl: string;
  risk: 'low' | 'medium' | 'high';
  isActive: boolean;
}

export interface Validator {
  address: string;
  name: string;
  commission: number;
  delegators: number;
  totalStake: string;
  isActive: boolean;
  apy: number;
}

export interface Transaction {
  id: string;
  type: 'deposit' | 'withdraw' | 'compound' | 'rebalance' | 'harvest';
  amount: string;
  shares?: string;
  timestamp: number;
  deployHash: string;
  status: 'pending' | 'completed' | 'failed';
  from: string;
  to?: string;
  fee: string;
}

export interface ContractEvent {
  type: 'deposit' | 'withdraw' | 'compound' | 'rebalance' | 'allocationUpdate' | 'bridgeInitiated' | 'bridgeCompleted';
  timestamp: number;
  blockHeight: number;
  deployHash: string;
  data: Record<string, any>;
}

// Deploy Configuration
export interface DeployConfig {
  chainName: string;
  gasPrice: number;
  ttl: number;
  dependencies: string[];
}

// Transaction Request
export interface TransactionRequest {
  type: 'deposit' | 'withdraw' | 'instantWithdraw' | 'compound';
  amount?: string;
  shares?: string;
  gasLimit?: number;
}
