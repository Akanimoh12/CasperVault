import { config } from './utils/config';
import { Logger } from './utils/logger';
import database from './database/client';

/**
 * Main application entry point
 */
class Application {
  /**
   * Initialize application
   */
  async initialize(): Promise<void> {
    try {
      Logger.info('Initializing CasperVault Backend...', {
        environment: config.environment,
        network: config.casper.network,
      });

      // Connect to database
      Logger.info('Connecting to database...');
      await database.connect();
      Logger.info('Database connected successfully');

      // Log contract addresses
      Logger.info('Contract configuration:', {
        vaultManager: config.contracts.vaultManager || 'NOT_SET',
        liquidStaking: config.contracts.liquidStaking || 'NOT_SET',
        strategyRouter: config.contracts.strategyRouter || 'NOT_SET',
      });

      Logger.info('CasperVault Backend initialized successfully');
    } catch (error) {
      Logger.error('Failed to initialize application', error);
      throw error;
    }
  }

  /**
   * Start application services
   */
  async start(): Promise<void> {
    try {
      await this.initialize();

      Logger.info('Starting services...');

      // Services will be started in later prompts:
      // - REST API server
      // - WebSocket server
      // - Optimizer bot
      // - Compounder bot
      // - Monitor service

      Logger.info('All services started successfully');

      // Setup graceful shutdown
      this.setupGracefulShutdown();
    } catch (error) {
      Logger.error('Failed to start application', error);
      process.exit(1);
    }
  }

  /**
   * Shutdown application gracefully
   */
  async shutdown(): Promise<void> {
    try {
      Logger.info('Shutting down application...');

      // Close database connection
      await database.close();

      // Stop services
      // TODO: Add service cleanup

      Logger.info('Application shut down successfully');
      process.exit(0);
    } catch (error) {
      Logger.error('Error during shutdown', error);
      process.exit(1);
    }
  }

  /**
   * Setup graceful shutdown handlers
   */
  private setupGracefulShutdown(): void {
    process.on('SIGTERM', () => {
      Logger.info('SIGTERM received, starting graceful shutdown');
      void this.shutdown();
    });

    process.on('SIGINT', () => {
      Logger.info('SIGINT received, starting graceful shutdown');
      void this.shutdown();
    });

    process.on('uncaughtException', (error: Error) => {
      Logger.error('Uncaught exception', error);
      void this.shutdown();
    });

    process.on('unhandledRejection', (reason: unknown) => {
      Logger.error('Unhandled rejection', reason instanceof Error ? reason : new Error(String(reason)));
      void this.shutdown();
    });
  }
}

// Create and start application
const app = new Application();

// Start if running directly
if (require.main === module) {
  void app.start();
}

export default app;
