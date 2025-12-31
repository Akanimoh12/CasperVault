import {
  VaultContract,
  StakingContract,
  StrategyContract,
  EventListener,
  TransactionManager,
  AccountManager,
} from './src/contracts';
import { config } from './src/utils/config';
import { Logger } from './src/utils/logger';

/**
 * Main application class integrating all contract wrappers
 */
class CasperVaultBackend {
  // Contract wrappers
  private vaultContract: VaultContract;
  private stakingContract: StakingContract;
  private strategyContract: StrategyContract;
  
  // Management systems
  private eventListener: EventListener;
  private txManager: TransactionManager;
  private accountManager: AccountManager;

  constructor() {
    Logger.info('Initializing CasperVault Backend...');

    // Initialize contract wrappers
    this.vaultContract = new VaultContract(config.contracts.vaultManager);
    this.stakingContract = new StakingContract(config.contracts.stakingManager);
    this.strategyContract = new StrategyContract(config.contracts.strategyRouter);

    // Initialize account manager
    this.accountManager = new AccountManager({
      keysDirectory: process.env.KEYS_DIR || './keys',
      accounts: {
        admin: process.env.ADMIN_KEY || 'admin.pem',
        operator: process.env.OPERATOR_KEY || 'operator.pem',
        keeper: process.env.KEEPER_KEY || 'keeper.pem',
        guardian: process.env.GUARDIAN_KEY,
      },
    });

    // Initialize transaction manager
    this.txManager = new TransactionManager({
      concurrency: 2,
      maxRetries: 3,
      retryDelay: 5000,
    });

    // Initialize event listener
    this.eventListener = new EventListener({
      contracts: [this.vaultContract, this.stakingContract, this.strategyContract],
      pollInterval: 10000, // 10 seconds
    });

    Logger.info('CasperVault Backend initialized successfully');
  }

  /**
   * Start all services
   */
  async start(): Promise<void> {
    try {
      Logger.info('Starting services...');

      // Setup event handlers
      this.setupEventHandlers();

      // Start event listener
      await this.eventListener.start();

      // Start periodic tasks
      this.startPeriodicTasks();

      Logger.info('All services started successfully');
    } catch (error) {
      Logger.error('Failed to start services', error);
      throw error;
    }
  }

  /**
   * Setup event handlers for real-time monitoring
   */
  private setupEventHandlers(): void {
    // Handle deposits
    this.eventListener.onDeposit(async (event) => {
      Logger.info('Deposit detected', {
        user: event.user,
        amount: event.amount,
        shares: event.shares,
      });

      // Update database
      // await this.database.storeDeposit(event);

      // Broadcast to WebSocket clients
      // this.wsServer.broadcast('deposit', event);
    });

    // Handle withdrawals
    this.eventListener.onWithdraw(async (event) => {
      Logger.info('Withdrawal detected', {
        user: event.user,
        shares: event.shares,
        amount: event.amount,
      });

      // Update database
      // await this.database.storeWithdrawal(event);

      // Broadcast to WebSocket clients
      // this.wsServer.broadcast('withdraw', event);
    });

    // Handle compounds
    this.eventListener.onCompound(async (event) => {
      Logger.info('Compound executed', {
        totalYield: event.totalYield,
        feeCollected: event.feeCollected,
      });

      // Update vault statistics
      await this.updateVaultStats();
    });

    // Handle rebalances
    this.eventListener.onRebalance(async (event) => {
      Logger.info('Rebalance executed', {
        oldAllocation: event.oldAllocation,
        newAllocation: event.newAllocation,
      });

      // Update strategy performance
      // await this.database.storeStrategyPerformance(...);
    });
  }

  /**
   * Start periodic background tasks
   */
  private startPeriodicTasks(): void {
    // Daily compound (midnight UTC)
    this.scheduleTask('0 0 * * *', () => this.runDailyCompound());

    // Yield optimization (every 12 hours)
    this.scheduleTask('0 */12 * * *', () => this.runYieldOptimization());

    // Health check (every 5 minutes)
    this.scheduleTask('*/5 * * * *', () => this.runHealthCheck());

    // Vault statistics update (every hour)
    this.scheduleTask('0 * * * *', () => this.updateVaultStats());
  }

  /**
   * Daily compound operation
   */
  private async runDailyCompound(): Promise<void> {
    try {
      Logger.info('Running daily compound...');

      const keeperKey = this.accountManager.getAccount('KEEPER');

      // 1. Check pending rewards
      const pendingRewards = await this.stakingContract.getPendingRewards();
      
      if (BigInt(pendingRewards) === BigInt(0)) {
        Logger.info('No rewards to compound');
        return;
      }

      Logger.info('Pending rewards:', pendingRewards);

      // 2. Harvest from all strategies
      const harvestTx = {
        id: `harvest-${Date.now()}`,
        contract: this.strategyContract,
        entrypoint: 'harvestAll',
        args: {},
        paymentAmount: '15000000000',
        signerKey: keeperKey,
        priority: 1,
      };

      await this.txManager.queueTransaction(harvestTx);

      // 3. Compound staking rewards
      const compoundTx = {
        id: `compound-${Date.now()}`,
        contract: this.stakingContract,
        entrypoint: 'compoundRewards',
        args: {},
        paymentAmount: '10000000000',
        signerKey: keeperKey,
        priority: 1,
      };

      await this.txManager.queueTransaction(compoundTx);

      // 4. Update APY
      const blendedAPY = await this.strategyContract.getBlendedAPY();
      
      const updateAPYTx = {
        id: `update-apy-${Date.now()}`,
        contract: this.vaultContract,
        entrypoint: 'updateAPY',
        args: { apy: Math.floor(blendedAPY * 100) },
        paymentAmount: '3000000000',
        signerKey: keeperKey,
        priority: 1,
      };

      await this.txManager.queueTransaction(updateAPYTx);

      Logger.info('Daily compound queued successfully');
    } catch (error) {
      Logger.error('Daily compound failed', error);
    }
  }

  /**
   * Yield optimization operation
   */
  private async runYieldOptimization(): Promise<void> {
    try {
      Logger.info('Running yield optimization...');

      const operatorKey = this.accountManager.getAccount('OPERATOR');

      // 1. Fetch current APYs and allocations
      const strategies = await this.strategyContract.getActiveStrategies();
      const currentAllocations = await this.strategyContract.getAllocations();

      Logger.info('Current strategies:', strategies);

      // 2. Calculate optimal allocation (simplified)
      const optimalAllocation = new Map<string, number>();
      
      // Sort by APY
      const sorted = strategies.sort((a, b) => b.apy - a.apy);
      
      // Allocate more to higher APY strategies
      if (sorted.length >= 3) {
        optimalAllocation.set(sorted[0].name, 40); // Highest APY
        optimalAllocation.set(sorted[1].name, 35); // Second highest
        optimalAllocation.set(sorted[2].name, 25); // Third highest
      }

      Logger.info('Optimal allocation:', optimalAllocation);

      // 3. Check if rebalance needed (deviation > 5%)
      // Simplified deviation check
      let needsRebalance = false;
      // ... deviation calculation logic ...

      if (needsRebalance) {
        Logger.info('Rebalance needed, queuing transactions...');

        // Set target allocation
        const setTargetTx = {
          id: `set-target-${Date.now()}`,
          contract: this.strategyContract,
          entrypoint: 'setTargetAllocation',
          args: { allocations: optimalAllocation },
          paymentAmount: '5000000000',
          signerKey: operatorKey,
          priority: 2,
        };

        await this.txManager.queueTransaction(setTargetTx);

        // Execute rebalance
        const rebalanceTx = {
          id: `rebalance-${Date.now()}`,
          contract: this.strategyContract,
          entrypoint: 'rebalance',
          args: {},
          paymentAmount: '20000000000',
          signerKey: operatorKey,
          priority: 2,
        };

        await this.txManager.queueTransaction(rebalanceTx);

        Logger.info('Rebalance queued successfully');
      } else {
        Logger.info('No rebalance needed');
      }
    } catch (error) {
      Logger.error('Yield optimization failed', error);
    }
  }

  /**
   * Health check operation
   */
  private async runHealthCheck(): Promise<void> {
    try {
      // Check vault health
      const isPaused = await this.vaultContract.isPaused();
      const totalAssets = await this.vaultContract.getTotalAssets();
      
      if (isPaused) {
        Logger.warn('ALERT: Vault is paused!');
        // Send alert to admins
      }

      // Check transaction queue
      const queueStats = await this.txManager.getQueueStats();
      
      if (queueStats.failed > 10) {
        Logger.warn('ALERT: High number of failed transactions', queueStats);
      }

      Logger.debug('Health check passed', {
        isPaused,
        totalAssets,
        queueStats,
      });
    } catch (error) {
      Logger.error('Health check failed', error);
    }
  }

  /**
   * Update vault statistics
   */
  private async updateVaultStats(): Promise<void> {
    try {
      const totalAssets = await this.vaultContract.getTotalAssets();
      const totalShares = await this.vaultContract.getTotalShares();
      const sharePrice = await this.vaultContract.getSharePrice();
      const currentAPY = await this.vaultContract.getCurrentAPY();

      Logger.info('Vault stats updated', {
        totalAssets,
        totalShares,
        sharePrice,
        apy: currentAPY / 100,
      });

      // Store in database
      // await this.database.storeVaultSnapshot(...);
    } catch (error) {
      Logger.error('Failed to update vault stats', error);
    }
  }

  /**
   * Schedule periodic task
   */
  private scheduleTask(cronExpression: string, task: () => Promise<void>): void {
    // Use node-cron or similar
    // cron.schedule(cronExpression, task);
    Logger.info('Scheduled task', { cronExpression });
  }

  /**
   * Stop all services
   */
  async stop(): Promise<void> {
    Logger.info('Stopping services...');

    await this.eventListener.stop();
    await this.txManager.close();

    Logger.info('All services stopped');
  }
}

// Example usage
async function main() {
  const backend = new CasperVaultBackend();

  try {
    await backend.start();
    
    Logger.info('CasperVault Backend running...');

    // Handle graceful shutdown
    process.on('SIGINT', async () => {
      Logger.info('Shutting down...');
      await backend.stop();
      process.exit(0);
    });

    process.on('SIGTERM', async () => {
      Logger.info('Shutting down...');
      await backend.stop();
      process.exit(0);
    });
  } catch (error) {
    Logger.error('Failed to start backend', error);
    process.exit(1);
  }
}

// Run if main module
if (require.main === module) {
  main().catch((error) => {
    Logger.error('Fatal error', error);
    process.exit(1);
  });
}

export default CasperVaultBackend;
