// API Response Types
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  hasMore: boolean;
}

// Dashboard Data
export interface DashboardData {
  overview: OverviewStats;
  strategies: Strategy[];
  recentActivity: Transaction[];
  tvlHistory: TVLDataPoint[];
}

export interface OverviewStats {
  totalValueLocked: string;
  currentAPY: number;
  totalUsers: number;
  totalRewards: string;
  change24h: number;
}

export interface TVLDataPoint {
  timestamp: number;
  total_assets: string;
  share_price: string;
}

// Analytics Data
export interface AnalyticsData {
  apyHistory: APYDataPoint[];
  strategyPerformance: StrategyPerformance[];
  userGrowth: UserGrowthPoint[];
  volumeData: VolumeDataPoint[];
}

export interface APYDataPoint {
  timestamp: number;
  apy: number;
  strategy?: string;
}

export interface StrategyPerformance {
  strategyId: string;
  strategyName: string;
  totalReturns: string;
  apy: number;
  tvl: string;
  transactions: number;
}

export interface UserGrowthPoint {
  timestamp: number;
  totalUsers: number;
  activeUsers: number;
}

export interface VolumeDataPoint {
  timestamp: number;
  deposits: string;
  withdrawals: string;
  volume: string;
}

// Portfolio Data
export interface PortfolioData {
  position: UserPosition;
  transactions: Transaction[];
  rewards: RewardsSummary;
  performance: PerformanceMetrics;
}

export interface RewardsSummary {
  totalEarned: string;
  pendingRewards: string;
  claimedRewards: string;
  apyEarned: number;
}

export interface PerformanceMetrics {
  totalDeposited: string;
  currentValue: string;
  totalReturns: string;
  returnsPercent: number;
  bestStrategy: string;
  timeInVault: number;
}

// WebSocket Message Types
export interface WebSocketMessage {
  event: string;
  data: any;
  timestamp: number;
}

export interface RealtimeUpdate {
  type: 'tvl_update' | 'apy_update' | 'transaction' | 'rebalance';
  data: any;
}

// Import types from contracts
import type { Strategy, Transaction, UserPosition } from './contracts';
