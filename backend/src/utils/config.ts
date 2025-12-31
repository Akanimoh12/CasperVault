import * as fs from 'fs';
import * as path from 'path';
import * as dotenv from 'dotenv';

// Load environment variables
dotenv.config();

/**
 * Configuration interface
 */
export interface Config {
  environment: string;
  server: {
    port: number;
    host: string;
    cors: {
      origin: string[];
      credentials: boolean;
    };
  };
  casper: {
    network: string;
    rpcUrl: string;
    eventStreamUrl: string;
    chainName: string;
  };
  contracts: {
    vaultManager: string;
    liquidStaking: string;
    strategyRouter: string;
    yieldAggregator: string;
    cvCsprToken: string;
    lstCsprToken: string;
  };
  services: {
    optimizer: {
      enabled: boolean;
      schedule: string;
      rebalanceThreshold: number;
      minGainMultiplier: number;
      maxRetries: number;
    };
    compounder: {
      enabled: boolean;
      schedule: string;
      minYieldThreshold: string;
      performanceFeeBps: number;
      managementFeeBps: number;
    };
    monitor: {
      enabled: boolean;
      healthCheckInterval: number;
      alertWebhook: string;
    };
  };
  database: {
    url: string;
    maxConnections: number;
  };
  redis: {
    host: string;
    port: number;
    password: string;
    db: number;
  };
  logging: {
    level: string;
    file: boolean;
    console: boolean;
    maxFiles: string;
    maxSize: string;
  };
  rateLimit: {
    windowMs: number;
    maxRequests: number;
  };
}

/**
 * Configuration loader
 */
class ConfigLoader {
  private config: Config | null = null;

  /**
   * Load configuration based on NODE_ENV
   */
  load(): Config {
    if (this.config) {
      return this.config;
    }

    const env = process.env.NODE_ENV || 'development';
    const configPath = path.join(__dirname, '../../config', `${env}.json`);

    if (!fs.existsSync(configPath)) {
      throw new Error(`Configuration file not found: ${configPath}`);
    }

    // Load base configuration from file
    const fileConfig = JSON.parse(fs.readFileSync(configPath, 'utf-8')) as Config;

    // Override with environment variables
    this.config = this.mergeWithEnvVars(fileConfig);

    return this.config;
  }

  /**
   * Merge configuration with environment variables
   */
  private mergeWithEnvVars(config: Config): Config {
    return {
      ...config,
      server: {
        ...config.server,
        port: parseInt(process.env.PORT || String(config.server.port), 10),
        host: process.env.HOST || config.server.host,
      },
      casper: {
        ...config.casper,
        rpcUrl: process.env.CASPER_RPC_URL || config.casper.rpcUrl,
        eventStreamUrl: process.env.CASPER_EVENT_STREAM_URL || config.casper.eventStreamUrl,
        network: process.env.CASPER_NETWORK || config.casper.network,
      },
      contracts: {
        vaultManager: process.env.VAULT_MANAGER_HASH || config.contracts.vaultManager,
        liquidStaking: process.env.LIQUID_STAKING_HASH || config.contracts.liquidStaking,
        strategyRouter: process.env.STRATEGY_ROUTER_HASH || config.contracts.strategyRouter,
        yieldAggregator: process.env.YIELD_AGGREGATOR_HASH || config.contracts.yieldAggregator,
        cvCsprToken: process.env.CV_CSPR_TOKEN_HASH || config.contracts.cvCsprToken,
        lstCsprToken: process.env.LST_CSPR_TOKEN_HASH || config.contracts.lstCsprToken,
      },
      database: {
        ...config.database,
        url: process.env.SUPABASE_URL || config.database.url,
      },
      redis: {
        ...config.redis,
        host: process.env.REDIS_HOST || config.redis.host,
        port: parseInt(process.env.REDIS_PORT || String(config.redis.port), 10),
        password: process.env.REDIS_PASSWORD || config.redis.password,
      },
      logging: {
        ...config.logging,
        level: process.env.LOG_LEVEL || config.logging.level,
      },
    };
  }

  /**
   * Get specific configuration value
   */
  get<K extends keyof Config>(key: K): Config[K] {
    if (!this.config) {
      this.load();
    }
    return this.config![key];
  }

  /**
   * Validate configuration
   */
  validate(): void {
    const config = this.load();

    const required = [
      config.casper.rpcUrl,
      config.database.url,
      config.redis.host,
    ];

    const missing = required.filter((val) => !val);
    if (missing.length > 0) {
      throw new Error('Missing required configuration values');
    }

    // Warn if contract addresses are not set
    if (!config.contracts.vaultManager) {
      console.warn('Warning: VAULT_MANAGER_HASH not configured');
    }
  }
}

// Singleton instance
export const configLoader = new ConfigLoader();

// Export loaded configuration
export const config = configLoader.load();

// Validate on startup
configLoader.validate();
