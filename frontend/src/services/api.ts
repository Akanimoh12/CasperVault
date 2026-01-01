// import axios from 'axios';
// import { API_BASE_URL } from '@/utils/constants';

// API Types
export interface Overview {
  tvl: string;
  tvlUSD: number;
  currentAPY: number;
  userCount: number;
  netInflow: string;
  netInflowUSD: number;
}

export interface TVLDataPoint {
  timestamp: number;
  total_assets: string;
}

export interface Strategy {
  id: string;
  name: string;
  allocation: number;
  apy: number;
  displayName?: string;
  description?: string;
  risk?: 'LOW' | 'MEDIUM' | 'HIGH';
  allocated?: string;
  allocationPercent?: string;
  history?: Array<{ timestamp: number; apy: number }>;
  howItWorks?: string[];
  risks?: string[];
}

export interface Activity {
  id: string;
  type: 'Deposit' | 'Withdraw' | 'Compound';
  amount: string;
  user: string;
  timestamp: number;
  txHash: string;
}

// Mock data for testing until backend is ready
const MOCK_OVERVIEW: Overview = {
  tvl: '15234567',
  tvlUSD: 1523456,
  currentAPY: 12.5,
  userCount: 1234,
  netInflow: '234567',
  netInflowUSD: 23456,
};

const MOCK_TVL_HISTORY: TVLDataPoint[] = Array.from({ length: 30 }, (_, i) => ({
  timestamp: Date.now() - (29 - i) * 24 * 60 * 60 * 1000,
  total_assets: (10000000 + Math.random() * 5000000 + i * 100000).toString(),
}));

const MOCK_STRATEGIES: Strategy[] = [
  {
    id: '1',
    name: 'dex',
    displayName: 'DEX Liquidity',
    description: 'Provide liquidity to decentralized exchanges',
    allocation: 40,
    apy: 15.2,
    risk: 'MEDIUM',
    allocated: '6000000',
    allocationPercent: '40',
    history: Array.from({ length: 30 }, (_, i) => ({
      timestamp: Date.now() - (29 - i) * 24 * 60 * 60 * 1000,
      apy: 15.2 + (Math.random() - 0.5) * 2,
    })),
    howItWorks: [
      'Deposits are split across multiple DEX liquidity pools',
      'Automated rebalancing to maintain optimal allocations',
      'Harvest trading fees and LP rewards daily',
      'Compound rewards back into liquidity positions',
    ],
    risks: [
      'Impermanent loss from price divergence',
      'Smart contract vulnerabilities in DEX protocols',
      'Low liquidity during market volatility',
    ],
  },
  {
    id: '2',
    name: 'lending',
    displayName: 'Lending Protocol',
    description: 'Earn interest by lending CSPR',
    allocation: 30,
    apy: 10.5,
    risk: 'LOW',
    allocated: '4500000',
    allocationPercent: '30',
    history: Array.from({ length: 30 }, (_, i) => ({
      timestamp: Date.now() - (29 - i) * 24 * 60 * 60 * 1000,
      apy: 10.5 + (Math.random() - 0.5) * 1.5,
    })),
    howItWorks: [
      'Lend CSPR to verified lending protocols',
      'Earn stable interest rates from borrowers',
      'Automatic reinvestment of earned interest',
      'Withdraw anytime with minimal fees',
    ],
    risks: [
      'Borrower default risk (minimal with over-collateralization)',
      'Smart contract bugs in lending protocols',
      'Interest rate fluctuations during market changes',
    ],
  },
  {
    id: '3',
    name: 'cross_chain',
    displayName: 'Cross-chain Bridge',
    description: 'Facilitate cross-chain transfers',
    allocation: 20,
    apy: 18.3,
    risk: 'HIGH',
    allocated: '3000000',
    allocationPercent: '20',
    history: Array.from({ length: 30 }, (_, i) => ({
      timestamp: Date.now() - (29 - i) * 24 * 60 * 60 * 1000,
      apy: 18.3 + (Math.random() - 0.5) * 3,
    })),
    howItWorks: [
      'Provide liquidity to cross-chain bridges',
      'Earn fees from bridge transactions',
      'Support multiple blockchain networks',
      'Automated fee collection and compounding',
    ],
    risks: [
      'Bridge security vulnerabilities',
      'Cross-chain transaction failures',
      'Higher smart contract complexity',
      'Network congestion affecting transfers',
    ],
  },
  {
    id: '4',
    name: 'staking',
    displayName: 'Liquid Staking',
    description: 'Stake CSPR while maintaining liquidity',
    allocation: 10,
    apy: 8.7,
    risk: 'LOW',
    allocated: '1500000',
    allocationPercent: '10',
    history: Array.from({ length: 30 }, (_, i) => ({
      timestamp: Date.now() - (29 - i) * 24 * 60 * 60 * 1000,
      apy: 8.7 + (Math.random() - 0.5) * 1,
    })),
    howItWorks: [
      'Stake CSPR with trusted validators',
      'Receive liquid staking tokens (LST)',
      'Maintain liquidity while earning rewards',
      'Automatic reward distribution',
    ],
    risks: [
      'Validator performance affects returns',
      'Slashing penalties for validator misbehavior',
      'LST price depegging during market stress',
    ],
  },
];

const MOCK_ACTIVITIES: Activity[] = [
  {
    id: '1',
    type: 'Deposit' as const,
    amount: '1000',
    user: '01a3b4...c5d6',
    timestamp: Date.now() - 1000 * 60 * 5,
    txHash: '0x123...abc',
  },
  {
    id: '2',
    type: 'Withdraw' as const,
    amount: '500',
    user: '02b4c5...d6e7',
    timestamp: Date.now() - 1000 * 60 * 15,
    txHash: '0x234...bcd',
  },
  {
    id: '3',
    type: 'Compound' as const,
    amount: '250',
    user: '03c5d6...e7f8',
    timestamp: Date.now() - 1000 * 60 * 30,
    txHash: '0x345...cde',
  },
  {
    id: '4',
    type: 'Deposit' as const,
    amount: '2500',
    user: '04d6e7...f8g9',
    timestamp: Date.now() - 1000 * 60 * 45,
    txHash: '0x456...def',
  },
  {
    id: '5',
    type: 'Withdraw' as const,
    amount: '750',
    user: '05e7f8...g9h0',
    timestamp: Date.now() - 1000 * 60 * 60,
    txHash: '0x567...efg',
  },
];

class ApiService {
  async getOverview(): Promise<Overview> {
    try {
      // TODO: Replace with real API call when backend is ready
      // const response = await axios.get(`${API_BASE_URL}/overview`);
      // return response.data;
      
      // Return mock data for now
      return new Promise((resolve) => {
        setTimeout(() => resolve(MOCK_OVERVIEW), 500);
      });
    } catch (error) {
      console.error('Failed to fetch overview:', error);
      return MOCK_OVERVIEW;
    }
  }

  async getTVLHistory(_period?: string): Promise<TVLDataPoint[]> {
    try {
      // TODO: Replace with real API call
      // const response = await axios.get(`${API_BASE_URL}/tvl-history?period=${period}`);
      // return response.data;
      
      return new Promise((resolve) => {
        setTimeout(() => resolve(MOCK_TVL_HISTORY), 500);
      });
    } catch (error) {
      console.error('Failed to fetch TVL history:', error);
      return MOCK_TVL_HISTORY;
    }
  }

  async getStrategies(): Promise<Strategy[]> {
    try {
      // TODO: Replace with real API call
      // const response = await axios.get(`${API_BASE_URL}/strategies`);
      // return response.data;
      
      return new Promise((resolve) => {
        setTimeout(() => resolve(MOCK_STRATEGIES), 500);
      });
    } catch (error) {
      console.error('Failed to fetch strategies:', error);
      return MOCK_STRATEGIES;
    }
  }

  async getRecentActivity(_limit = 10): Promise<Activity[]> {
    try {
      // TODO: Replace with real API call
      // const response = await axios.get(`${API_BASE_URL}/activity?limit=${limit}`);
      // return response.data;
      
      return new Promise((resolve) => {
        setTimeout(() => resolve(MOCK_ACTIVITIES), 500);
      });
    } catch (error) {
      console.error('Failed to fetch recent activity:', error);
      return MOCK_ACTIVITIES;
    }
  }

  // Analytics data methods
  async getYieldDistribution(period: string): Promise<any[]> {
    // Generate mock yield distribution data
    const days = period === '7d' ? 7 : period === '30d' ? 30 : period === '90d' ? 90 : 365;
    const data = Array.from({ length: days }, (_, i) => {
      const date = new Date();
      date.setDate(date.getDate() - (days - i));
      
      return {
        date: date.toISOString(),
        'DEX Liquidity': 3500000 + Math.random() * 500000,
        'Lending Protocol': 2800000 + Math.random() * 400000,
        'Cross-chain Bridge': 2200000 + Math.random() * 300000,
        'Liquid Staking': 1000000 + Math.random() * 200000,
      };
    });

    return new Promise((resolve) => {
      setTimeout(() => resolve(data), 500);
    });
  }

  async getUserGrowth(period: string): Promise<any[]> {
    // Generate mock user growth data
    const days = period === '7d' ? 7 : period === '30d' ? 30 : period === '90d' ? 90 : 365;
    const data = Array.from({ length: days }, (_, i) => {
      const date = new Date();
      date.setDate(date.getDate() - (days - i));
      
      return {
        date: date.toISOString(),
        users: Math.floor(10 + i * 2 + Math.random() * 5),
      };
    });

    return new Promise((resolve) => {
      setTimeout(() => resolve(data), 500);
    });
  }

  async getStrategyComparison(_period: string): Promise<any[]> {
    // Generate mock strategy comparison data
    return new Promise((resolve) => {
      setTimeout(() => resolve([
        { name: 'DEX Liquidity', apy: 15.2, allocation: 40, risk: 'MEDIUM' },
        { name: 'Lending Protocol', apy: 10.5, allocation: 30, risk: 'LOW' },
        { name: 'Cross-chain Bridge', apy: 18.3, allocation: 20, risk: 'HIGH' },
        { name: 'Liquid Staking', apy: 8.7, allocation: 10, risk: 'LOW' },
      ]), 500);
    });
  }

  async getAPYHistory(period: string): Promise<any[]> {
    // Generate mock APY history data
    const days = period === '7d' ? 7 : period === '30d' ? 30 : period === '90d' ? 90 : 365;
    const data = Array.from({ length: days }, (_, i) => {
      const date = new Date();
      date.setDate(date.getDate() - (days - i));
      
      return {
        timestamp: date.getTime(),
        apy: 12 + Math.sin(i / 10) * 2 + Math.random() * 1.5,
      };
    });

    return new Promise((resolve) => {
      setTimeout(() => resolve(data), 500);
    });
  }
}

export const api = new ApiService();
