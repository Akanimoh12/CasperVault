/**
 * Common TypeScript types and interfaces
 */

import { CLPublicKey } from 'casper-js-sdk';

/**
 * Transaction result
 */
export interface TransactionResult {
  success: boolean;
  deployHash: string;
  blockHash?: string;
  timestamp?: number;
  error?: string;
}

/**
 * Transaction status
 */
export enum TransactionStatus {
  PENDING = 'pending',
  SUCCESS = 'success',
  FAILED = 'failed',
}

/**
 * Account role for multi-sig operations
 */
export enum AccountRole {
  ADMIN = 'admin',
  OPERATOR = 'operator',
  KEEPER = 'keeper',
  GUARDIAN = 'guardian',
}

/**
 * Contract event types
 */
export interface BaseEvent {
  contractHash: string;
  blockHash: string;
  deployHash: string;
  timestamp: number;
}

export interface DepositEvent extends BaseEvent {
  user: string;
  amount: string;
  shares: string;
}

export interface WithdrawEvent extends BaseEvent {
  user: string;
  shares: string;
  amount: string;
}

export interface CompoundEvent extends BaseEvent {
  totalYield: string;
  feesCollected: string;
  newSharePrice: string;
}

export interface RebalanceEvent extends BaseEvent {
  oldAllocations: Record<string, string>;
  newAllocations: Record<string, string>;
}

export interface BridgeEvent extends BaseEvent {
  sourceChain: string;
  targetChain: string;
  amount: string;
  transactionId: string;
}

/**
 * Strategy allocation map
 */
export type AllocationMap = Map<string, number>;

/**
 * APY data
 */
export interface APYData {
  timestamp: number;
  strategies: Array<{
    strategy: string;
    apy: number;
    success: boolean;
  }>;
  blendedAPY: number;
}

/**
 * Risk levels
 */
export enum RiskLevel {
  LOW = 'LOW',
  MEDIUM = 'MEDIUM',
  HIGH = 'HIGH',
}

/**
 * Optimization result
 */
export interface OptimizationResult {
  success: boolean;
  timestamp?: number;
  apys?: APYData;
  currentAllocation?: AllocationMap;
  optimalAllocation?: AllocationMap;
  deviation?: number;
  rebalanced?: boolean;
  error?: string;
}

/**
 * Compound result
 */
export interface CompoundResult {
  success: boolean;
  skipped?: boolean;
  yieldsHarvested?: string;
  netCompounded?: string;
  feesCollected?: string;
  error?: string;
}

/**
 * Harvest result
 */
export interface HarvestResult {
  stakingRewards: string;
  dexYields: string;
  lendingYields: string;
  crossChainYields: string;
  total: string;
}

/**
 * Fee breakdown
 */
export interface FeeBreakdown {
  performanceFee: string;
  managementFee: string;
  total: string;
}

/**
 * Health check result
 */
export interface HealthReport {
  status: 'healthy' | 'unhealthy';
  checks: Array<{
    name: string;
    healthy: boolean;
    message?: string;
    timestamp: number;
  }>;
  timestamp: number;
}

/**
 * Anomaly detection
 */
export interface Anomaly {
  type: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  data: unknown;
  timestamp?: number;
}

/**
 * Alert
 */
export interface Alert {
  severity: 'low' | 'medium' | 'high' | 'critical';
  title: string;
  message: string;
  data?: Record<string, unknown>;
  timestamp: number;
}

/**
 * API response wrapper
 */
export interface ApiResponse<T = unknown> {
  success: boolean;
  data?: T;
  error?: {
    message: string;
    code?: string;
    details?: Record<string, unknown>;
  };
  timestamp: number;
}

/**
 * Pagination
 */
export interface PaginationParams {
  page: number;
  limit: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
  };
}

/**
 * Validator info
 */
export interface ValidatorInfo {
  address: string;
  name?: string;
  apy: number;
  stakedAmount: string;
  capacity: string;
  isActive: boolean;
  lastUpdated: number;
}

/**
 * Strategy info
 */
export interface StrategyInfo {
  name: string;
  type: 'dex' | 'lending' | 'crosschain';
  allocated: string;
  allocationPercent: number;
  apy: number;
  risk: RiskLevel;
  isActive: boolean;
}

/**
 * Portfolio data
 */
export interface PortfolioData {
  address: string;
  shares: string;
  currentValue: number;
  totalDeposited: number;
  profit: number;
  profitPercent: number;
  deposits: DepositRecord[];
  performance: PerformanceData[];
}

export interface DepositRecord {
  amount: number;
  shares: string;
  transactionHash: string;
  timestamp: Date;
}

export interface PerformanceData {
  date: string;
  value: number;
  apy: number;
}

/**
 * Vault stats
 */
export interface VaultStats {
  tvl: string;
  tvlUSD: number;
  currentAPY: number;
  userCount: number;
  totalDeposits: number;
  totalWithdrawals: number;
  netInflow: number;
  sharePrice: string;
}

/**
 * WebSocket message
 */
export interface WebSocketMessage {
  event: string;
  data: unknown;
  timestamp: number;
}

/**
 * Database models
 */
export interface DbDeposit {
  id: string;
  wallet_address: string;
  amount: string;
  cv_cspr_received: string;
  transaction_hash: string;
  timestamp: Date;
}

export interface DbWithdrawal {
  id: string;
  wallet_address: string;
  cv_cspr_amount: string;
  cspr_received: string;
  transaction_hash: string;
  timestamp: Date;
}

export interface DbVaultSnapshot {
  id: string;
  total_assets: string;
  total_shares: string;
  share_price: string;
  apy: number;
  timestamp: Date;
}

export interface DbStrategyPerformance {
  id: string;
  strategy_name: string;
  apy: number;
  tvl: string;
  timestamp: Date;
}
