import { createClient, SupabaseClient } from '@supabase/supabase-js';
import { config } from '../utils/config';
import { Logger } from '../utils/logger';
import { DatabaseError } from '../utils/errors';
import type {
  DbDeposit,
  DbWithdrawal,
  DbVaultSnapshot,
  DbStrategyPerformance,
  DepositEvent,
  WithdrawEvent,
} from '../types';

/**
 * Supabase database client
 */
class Database {
  private client: SupabaseClient;
  private isConnected: boolean = false;

  constructor() {
    if (!config.database.url || !process.env.SUPABASE_KEY) {
      throw new DatabaseError('Missing database configuration');
    }

    this.client = createClient(config.database.url, process.env.SUPABASE_KEY, {
      auth: {
        persistSession: false,
      },
    });

    Logger.info('Database client initialized');
  }

  /**
   * Test database connection
   */
  async connect(): Promise<void> {
    try {
      const { error } = await this.client.from('vault_snapshots').select('id').limit(1);

      if (error) {
        throw error;
      }

      this.isConnected = true;
      Logger.info('Database connection established');
    } catch (error) {
      Logger.error('Database connection failed', error);
      throw new DatabaseError('Failed to connect to database');
    }
  }

  /**
   * Check if database is connected
   */
  get connected(): boolean {
    return this.isConnected;
  }

  /**
   * Store deposit event
   */
  async storeDeposit(deposit: DepositEvent): Promise<void> {
    try {
      const { error } = await this.client.from('deposits').insert({
        wallet_address: deposit.user,
        amount: deposit.amount,
        cv_cspr_received: deposit.shares,
        transaction_hash: deposit.deployHash,
        timestamp: new Date(deposit.timestamp),
      });

      if (error) {
        throw error;
      }

      Logger.debug('Deposit stored', { deployHash: deposit.deployHash });
    } catch (error) {
      Logger.error('Failed to store deposit', error);
      throw new DatabaseError('Failed to store deposit', 'insert');
    }
  }

  /**
   * Store withdrawal event
   */
  async storeWithdrawal(withdrawal: WithdrawEvent): Promise<void> {
    try {
      const { error } = await this.client.from('withdrawals').insert({
        wallet_address: withdrawal.user,
        cv_cspr_amount: withdrawal.shares,
        cspr_received: withdrawal.amount,
        transaction_hash: withdrawal.deployHash,
        timestamp: new Date(withdrawal.timestamp),
      });

      if (error) {
        throw error;
      }

      Logger.debug('Withdrawal stored', { deployHash: withdrawal.deployHash });
    } catch (error) {
      Logger.error('Failed to store withdrawal', error);
      throw new DatabaseError('Failed to store withdrawal', 'insert');
    }
  }

  /**
   * Store vault snapshot
   */
  async storeVaultSnapshot(snapshot: {
    totalAssets: string;
    totalShares: string;
    sharePrice: string;
    apy: number;
  }): Promise<void> {
    try {
      const { error } = await this.client.from('vault_snapshots').insert({
        total_assets: snapshot.totalAssets,
        total_shares: snapshot.totalShares,
        share_price: snapshot.sharePrice,
        apy: snapshot.apy,
        timestamp: new Date(),
      });

      if (error) {
        throw error;
      }

      Logger.debug('Vault snapshot stored');
    } catch (error) {
      Logger.error('Failed to store vault snapshot', error);
      throw new DatabaseError('Failed to store vault snapshot', 'insert');
    }
  }

  /**
   * Store strategy performance
   */
  async storeStrategyPerformance(
    strategyName: string,
    apy: number,
    tvl: string
  ): Promise<void> {
    try {
      const { error } = await this.client.from('strategy_performance').insert({
        strategy_name: strategyName,
        apy,
        tvl,
        timestamp: new Date(),
      });

      if (error) {
        throw error;
      }

      Logger.debug('Strategy performance stored', { strategy: strategyName });
    } catch (error) {
      Logger.error('Failed to store strategy performance', error);
      throw new DatabaseError('Failed to store strategy performance', 'insert');
    }
  }

  /**
   * Get deposits for wallet
   */
  async getDeposits(walletAddress: string, limit: number = 50): Promise<DbDeposit[]> {
    try {
      const { data, error } = await this.client
        .from('deposits')
        .select('*')
        .eq('wallet_address', walletAddress)
        .order('timestamp', { ascending: false })
        .limit(limit);

      if (error) {
        throw error;
      }

      return data as DbDeposit[];
    } catch (error) {
      Logger.error('Failed to get deposits', error);
      throw new DatabaseError('Failed to get deposits', 'select');
    }
  }

  /**
   * Get vault snapshots for time period
   */
  async getVaultSnapshots(
    startDate: Date,
    endDate: Date = new Date()
  ): Promise<DbVaultSnapshot[]> {
    try {
      const { data, error } = await this.client
        .from('vault_snapshots')
        .select('*')
        .gte('timestamp', startDate.toISOString())
        .lte('timestamp', endDate.toISOString())
        .order('timestamp', { ascending: true });

      if (error) {
        throw error;
      }

      return data as DbVaultSnapshot[];
    } catch (error) {
      Logger.error('Failed to get vault snapshots', error);
      throw new DatabaseError('Failed to get vault snapshots', 'select');
    }
  }

  /**
   * Get strategy performance history
   */
  async getStrategyPerformance(
    strategyName: string,
    days: number = 30
  ): Promise<DbStrategyPerformance[]> {
    try {
      const startDate = new Date();
      startDate.setDate(startDate.getDate() - days);

      const { data, error } = await this.client
        .from('strategy_performance')
        .select('*')
        .eq('strategy_name', strategyName)
        .gte('timestamp', startDate.toISOString())
        .order('timestamp', { ascending: true });

      if (error) {
        throw error;
      }

      return data as DbStrategyPerformance[];
    } catch (error) {
      Logger.error('Failed to get strategy performance', error);
      throw new DatabaseError('Failed to get strategy performance', 'select');
    }
  }

  /**
   * Get unique wallet count
   */
  async getWalletCount(): Promise<number> {
    try {
      const { count, error } = await this.client
        .from('deposits')
        .select('wallet_address', { count: 'exact', head: true });

      if (error) {
        throw error;
      }

      return count || 0;
    } catch (error) {
      Logger.error('Failed to get wallet count', error);
      throw new DatabaseError('Failed to get wallet count', 'count');
    }
  }

  /**
   * Get total deposits sum
   */
  async getTotalDeposits(): Promise<number> {
    try {
      const { data, error } = await this.client
        .from('deposits')
        .select('amount')
        .then((res) => {
          if (res.error) throw res.error;
          return res.data.reduce((sum, d) => sum + parseFloat(d.amount), 0);
        });

      if (error) {
        throw error;
      }

      return data || 0;
    } catch (error) {
      Logger.error('Failed to get total deposits', error);
      throw new DatabaseError('Failed to get total deposits', 'aggregate');
    }
  }

  /**
   * Close database connection
   */
  async close(): Promise<void> {
    // Supabase client doesn't have explicit close method
    this.isConnected = false;
    Logger.info('Database connection closed');
  }
}

// Singleton instance
export const database = new Database();

export default database;
