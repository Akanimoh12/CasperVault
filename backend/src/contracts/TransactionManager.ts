import { Deploy, DeployUtil, Keys } from 'casper-js-sdk';
import { BaseContract } from './BaseContract';
import { Logger } from '../utils/logger';
import { RetryHandler, NetworkError, TransactionFailedError } from '../utils/errors';
import type { TransactionResult, TransactionStatus } from '../types';
import Bull, { Queue, Job } from 'bull';
import { config } from '../utils/config';

/**
 * Transaction configuration
 */
export interface Transaction {
  id: string;
  contract: BaseContract;
  entrypoint: string;
  args: any;
  paymentAmount: string;
  signerKey: Keys.AsymmetricKey;
  priority?: number;
  retries?: number;
  metadata?: Record<string, any>;
}

/**
 * Transaction status callback
 */
export type TxStatusCallback = (status: TransactionStatus, data?: any) => void;

/**
 * Transaction queue options
 */
interface QueueOptions {
  concurrency?: number;
  maxRetries?: number;
  retryDelay?: number;
}

/**
 * TransactionManager for queuing and managing contract transactions
 */
export class TransactionManager {
  private queue: Queue;
  private activeTransactions: Map<string, Transaction>;
  private transactionCallbacks: Map<string, TxStatusCallback[]>;
  private maxRetries: number;
  private retryDelay: number;

  constructor(options: QueueOptions = {}) {
    this.activeTransactions = new Map();
    this.transactionCallbacks = new Map();
    this.maxRetries = options.maxRetries || 3;
    this.retryDelay = options.retryDelay || 5000;

    // Initialize Bull queue with Redis
    this.queue = new Bull('transactions', {
      redis: {
        host: config.redis.host,
        port: config.redis.port,
        password: config.redis.password || undefined,
      },
      defaultJobOptions: {
        attempts: this.maxRetries,
        backoff: {
          type: 'exponential',
          delay: this.retryDelay,
        },
        removeOnComplete: true,
        removeOnFail: false,
      },
    });

    // Setup queue processors
    this.setupProcessors(options.concurrency || 1);

    Logger.info('TransactionManager initialized', {
      concurrency: options.concurrency || 1,
      maxRetries: this.maxRetries,
    });
  }

  // ============================================
  // QUEUE MANAGEMENT
  // ============================================

  /**
   * Queue a transaction for execution
   */
  async queueTransaction(tx: Transaction): Promise<string> {
    try {
      Logger.info('Queueing transaction', {
        id: tx.id,
        contract: tx.contract.name,
        entrypoint: tx.entrypoint,
      });

      // Add to Bull queue
      const job = await this.queue.add(tx, {
        priority: tx.priority || 0,
        jobId: tx.id,
      });

      // Store transaction
      this.activeTransactions.set(tx.id, tx);

      Logger.info('Transaction queued', {
        id: tx.id,
        jobId: job.id,
      });

      return tx.id;
    } catch (error) {
      Logger.error('Failed to queue transaction', error);
      throw error;
    }
  }

  /**
   * Setup queue processors
   */
  private setupProcessors(concurrency: number): void {
    this.queue.process(concurrency, async (job: Job<Transaction>) => {
      const tx = job.data;
      
      Logger.info('Processing transaction', {
        id: tx.id,
        attempt: job.attemptsMade + 1,
        maxAttempts: job.opts.attempts,
      });

      try {
        // Update status
        await this.updateStatus(tx.id, 'PENDING', {
          attempt: job.attemptsMade + 1,
        });

        // Execute transaction
        const result = await this.executeTransaction(tx);

        // Update status
        await this.updateStatus(tx.id, 'SUCCESS', {
          deployHash: result.deployHash,
          blockHash: result.blockHash,
        });

        // Remove from active transactions
        this.activeTransactions.delete(tx.id);

        return result;
      } catch (error) {
        Logger.error('Transaction execution failed', {
          id: tx.id,
          error: error instanceof Error ? error.message : String(error),
        });

        // Update status
        await this.updateStatus(tx.id, 'FAILED', {
          error: error instanceof Error ? error.message : String(error),
          attempt: job.attemptsMade + 1,
        });

        throw error;
      }
    });

    // Setup event handlers
    this.queue.on('completed', (job, result) => {
      Logger.info('Transaction completed', {
        id: job.data.id,
        deployHash: result.deployHash,
      });
    });

    this.queue.on('failed', (job, error) => {
      Logger.error('Transaction failed', {
        id: job?.data.id,
        error: error.message,
        attempts: job?.attemptsMade,
      });
    });

    this.queue.on('stalled', (job) => {
      Logger.warn('Transaction stalled', {
        id: job.data.id,
      });
    });
  }

  /**
   * Execute transaction
   */
  private async executeTransaction(tx: Transaction): Promise<TransactionResult> {
    try {
      // Call contract
      const result = await tx.contract['callEntrypoint'](
        tx.entrypoint,
        tx.args,
        tx.paymentAmount,
        tx.signerKey
      );

      if (!result.success) {
        throw new TransactionFailedError(
          `Transaction failed: ${result.error}`,
          result.deployHash
        );
      }

      return result;
    } catch (error) {
      Logger.error('Transaction execution error', error);
      throw error;
    }
  }

  /**
   * Update transaction status
   */
  private async updateStatus(
    txId: string,
    status: TransactionStatus,
    data?: any
  ): Promise<void> {
    const callbacks = this.transactionCallbacks.get(txId);
    
    if (callbacks) {
      for (const callback of callbacks) {
        try {
          callback(status, data);
        } catch (error) {
          Logger.error('Status callback error', { txId, error });
        }
      }
    }
  }

  // ============================================
  // TRANSACTION SUBMISSION
  // ============================================

  /**
   * Submit transaction with retries
   */
  async submitWithRetry(
    tx: Transaction,
    maxRetries?: number
  ): Promise<TransactionResult> {
    const retries = maxRetries || this.maxRetries;
    
    Logger.info('Submitting transaction with retries', {
      id: tx.id,
      maxRetries: retries,
    });

    return RetryHandler.retry(
      async () => {
        const result = await this.executeTransaction(tx);
        
        if (!result.success) {
          throw new TransactionFailedError(
            `Transaction failed: ${result.error}`,
            result.deployHash
          );
        }
        
        return result;
      },
      retries,
      this.retryDelay,
      (error) => {
        // Retry on network errors only
        return error instanceof NetworkError;
      }
    );
  }

  /**
   * Submit batch of transactions
   */
  async batchSubmit(txs: Transaction[]): Promise<TransactionResult[]> {
    Logger.info('Submitting batch transactions', { count: txs.length });

    const promises = txs.map(tx => this.queueTransaction(tx));
    const txIds = await Promise.all(promises);

    // Wait for all transactions to complete
    const results: TransactionResult[] = [];
    
    for (const txId of txIds) {
      const result = await this.waitForTransaction(txId);
      results.push(result);
    }

    return results;
  }

  /**
   * Wait for transaction to complete
   */
  private async waitForTransaction(txId: string, timeout: number = 300000): Promise<TransactionResult> {
    return new Promise((resolve, reject) => {
      const startTime = Date.now();

      const checkInterval = setInterval(async () => {
        // Check timeout
        if (Date.now() - startTime > timeout) {
          clearInterval(checkInterval);
          reject(new Error('Transaction timeout'));
          return;
        }

        // Check if transaction completed
        const job = await this.queue.getJob(txId);
        
        if (!job) {
          // Job not found or completed
          clearInterval(checkInterval);
          reject(new Error('Transaction not found'));
          return;
        }

        const state = await job.getState();
        
        if (state === 'completed') {
          clearInterval(checkInterval);
          const result = await job.finished();
          resolve(result);
        } else if (state === 'failed') {
          clearInterval(checkInterval);
          reject(new Error('Transaction failed'));
        }
      }, 1000);
    });
  }

  // ============================================
  // MONITORING
  // ============================================

  /**
   * Monitor transaction status
   */
  async monitorTransaction(
    deployHash: string,
    callback: TxStatusCallback
  ): Promise<void> {
    Logger.debug('Monitoring transaction', { deployHash });

    // Find transaction by deploy hash
    let txId: string | undefined;
    
    for (const [id, tx] of this.activeTransactions) {
      if (tx.metadata?.deployHash === deployHash) {
        txId = id;
        break;
      }
    }

    if (!txId) {
      Logger.warn('Transaction not found for monitoring', { deployHash });
      return;
    }

    // Add callback
    if (!this.transactionCallbacks.has(txId)) {
      this.transactionCallbacks.set(txId, []);
    }
    
    this.transactionCallbacks.get(txId)!.push(callback);
  }

  /**
   * Get transaction status
   */
  async getTransactionStatus(txId: string): Promise<string | null> {
    const job = await this.queue.getJob(txId);
    
    if (!job) {
      return null;
    }

    return await job.getState();
  }

  /**
   * Get queue statistics
   */
  async getQueueStats(): Promise<{
    waiting: number;
    active: number;
    completed: number;
    failed: number;
    delayed: number;
  }> {
    const [waiting, active, completed, failed, delayed] = await Promise.all([
      this.queue.getWaitingCount(),
      this.queue.getActiveCount(),
      this.queue.getCompletedCount(),
      this.queue.getFailedCount(),
      this.queue.getDelayedCount(),
    ]);

    return { waiting, active, completed, failed, delayed };
  }

  // ============================================
  // GAS ESTIMATION
  // ============================================

  /**
   * Estimate gas for transaction
   */
  async estimateGas(tx: Transaction): Promise<string> {
    try {
      Logger.debug('Estimating gas', {
        contract: tx.contract.name,
        entrypoint: tx.entrypoint,
      });

      // Use contract's gas estimation
      const estimate = await tx.contract['estimateGas'](tx.entrypoint, tx.args);

      Logger.debug('Gas estimated', {
        contract: tx.contract.name,
        entrypoint: tx.entrypoint,
        estimate,
      });

      return estimate;
    } catch (error) {
      Logger.error('Gas estimation failed', error);
      // Return default
      return '5000000000'; // 5 CSPR
    }
  }

  // ============================================
  // CLEANUP
  // ============================================

  /**
   * Cancel pending transaction
   */
  async cancelTransaction(txId: string): Promise<boolean> {
    try {
      const job = await this.queue.getJob(txId);
      
      if (!job) {
        return false;
      }

      const state = await job.getState();
      
      if (state === 'waiting' || state === 'delayed') {
        await job.remove();
        this.activeTransactions.delete(txId);
        
        Logger.info('Transaction cancelled', { txId });
        return true;
      }

      return false;
    } catch (error) {
      Logger.error('Failed to cancel transaction', { txId, error });
      return false;
    }
  }

  /**
   * Clean failed transactions
   */
  async cleanFailedTransactions(): Promise<number> {
    const failed = await this.queue.getFailed();
    
    for (const job of failed) {
      await job.remove();
    }

    Logger.info('Cleaned failed transactions', { count: failed.length });
    
    return failed.length;
  }

  /**
   * Close transaction manager
   */
  async close(): Promise<void> {
    await this.queue.close();
    Logger.info('TransactionManager closed');
  }
}

export default TransactionManager;
