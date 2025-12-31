import { CasperClient, EventStream } from 'casper-js-sdk';
import { BaseContract } from './BaseContract';
import { Logger } from '../utils/logger';
import { config } from '../utils/config';
import type {
  DepositEvent,
  WithdrawEvent,
  CompoundEvent,
  RebalanceEvent,
  BridgeEvent,
  BaseEvent,
} from '../types';

/**
 * Event handler function type
 */
type EventHandler<T = BaseEvent> = (event: T) => void | Promise<void>;

/**
 * Event listener configuration
 */
interface EventListenerConfig {
  contracts: BaseContract[];
  pollInterval?: number; // milliseconds
  eventStreamUrl?: string;
}

/**
 * Event types enum
 */
enum EventType {
  DEPOSIT = 'Deposit',
  WITHDRAW = 'Withdraw',
  COMPOUND = 'Compound',
  REBALANCE = 'Rebalance',
  BRIDGE_INITIATED = 'BridgeInitiated',
  BRIDGE_COMPLETED = 'BridgeCompleted',
  ALLOCATION_UPDATED = 'AllocationUpdated',
  VALIDATOR_ADDED = 'ValidatorAdded',
  FEES_UPDATED = 'FeesUpdated',
  PAUSED = 'Paused',
  UNPAUSED = 'Unpaused',
}

/**
 * EventListener class for monitoring contract events
 */
export class EventListener {
  private contracts: BaseContract[];
  private eventHandlers: Map<string, Set<EventHandler>>;
  private casperClient: CasperClient;
  private pollInterval: number;
  private isRunning: boolean = false;
  private pollingTimer?: NodeJS.Timeout;
  private lastProcessedBlock: number = 0;

  constructor(config: EventListenerConfig) {
    this.contracts = config.contracts;
    this.eventHandlers = new Map();
    this.pollInterval = config.pollInterval || 10000; // Default 10 seconds
    this.casperClient = new CasperClient(config.eventStreamUrl || config.casper.rpcUrl);

    Logger.info('EventListener initialized', {
      contracts: this.contracts.map(c => c.name),
      pollInterval: this.pollInterval,
    });
  }

  // ============================================
  // EVENT REGISTRATION
  // ============================================

  /**
   * Register handler for deposit events
   */
  onDeposit(handler: EventHandler<DepositEvent>): void {
    this.registerHandler(EventType.DEPOSIT, handler);
  }

  /**
   * Register handler for withdraw events
   */
  onWithdraw(handler: EventHandler<WithdrawEvent>): void {
    this.registerHandler(EventType.WITHDRAW, handler);
  }

  /**
   * Register handler for compound events
   */
  onCompound(handler: EventHandler<CompoundEvent>): void {
    this.registerHandler(EventType.COMPOUND, handler);
  }

  /**
   * Register handler for rebalance events
   */
  onRebalance(handler: EventHandler<RebalanceEvent>): void {
    this.registerHandler(EventType.REBALANCE, handler);
  }

  /**
   * Register handler for bridge initiated events
   */
  onBridgeInitiated(handler: EventHandler<BridgeEvent>): void {
    this.registerHandler(EventType.BRIDGE_INITIATED, handler);
  }

  /**
   * Register handler for bridge completed events
   */
  onBridgeCompleted(handler: EventHandler<BridgeEvent>): void {
    this.registerHandler(EventType.BRIDGE_COMPLETED, handler);
  }

  /**
   * Register handler for allocation updated events
   */
  onAllocationUpdated(handler: EventHandler<BaseEvent>): void {
    this.registerHandler(EventType.ALLOCATION_UPDATED, handler);
  }

  /**
   * Register handler for any event type
   */
  on(eventType: string, handler: EventHandler): void {
    this.registerHandler(eventType, handler);
  }

  /**
   * Internal method to register handler
   */
  private registerHandler(eventType: string, handler: EventHandler): void {
    if (!this.eventHandlers.has(eventType)) {
      this.eventHandlers.set(eventType, new Set());
    }
    
    this.eventHandlers.get(eventType)!.add(handler);
    
    Logger.debug('Event handler registered', { eventType });
  }

  /**
   * Unregister handler
   */
  off(eventType: string, handler: EventHandler): void {
    const handlers = this.eventHandlers.get(eventType);
    if (handlers) {
      handlers.delete(handler);
      Logger.debug('Event handler unregistered', { eventType });
    }
  }

  // ============================================
  // LIFECYCLE
  // ============================================

  /**
   * Start listening for events
   */
  async start(): Promise<void> {
    if (this.isRunning) {
      Logger.warn('EventListener already running');
      return;
    }

    this.isRunning = true;
    Logger.info('EventListener started');

    // Get current block height
    try {
      const latestBlock = await this.casperClient.nodeClient.getLatestBlockInfo();
      this.lastProcessedBlock = latestBlock.block?.header.height || 0;
      Logger.info('Starting from block', { block: this.lastProcessedBlock });
    } catch (error) {
      Logger.error('Failed to get latest block', error);
      this.lastProcessedBlock = 0;
    }

    // Start polling
    this.poll();
  }

  /**
   * Stop listening for events
   */
  async stop(): Promise<void> {
    if (!this.isRunning) {
      Logger.warn('EventListener not running');
      return;
    }

    this.isRunning = false;

    if (this.pollingTimer) {
      clearTimeout(this.pollingTimer);
      this.pollingTimer = undefined;
    }

    Logger.info('EventListener stopped');
  }

  // ============================================
  // EVENT POLLING
  // ============================================

  /**
   * Poll for new events
   */
  private async poll(): Promise<void> {
    if (!this.isRunning) {
      return;
    }

    try {
      await this.checkForNewEvents();
    } catch (error) {
      Logger.error('Error polling for events', error);
    }

    // Schedule next poll
    this.pollingTimer = setTimeout(() => this.poll(), this.pollInterval);
  }

  /**
   * Check for new events since last processed block
   */
  private async checkForNewEvents(): Promise<void> {
    try {
      const latestBlock = await this.casperClient.nodeClient.getLatestBlockInfo();
      const currentBlock = latestBlock.block?.header.height || 0;

      if (currentBlock <= this.lastProcessedBlock) {
        // No new blocks
        return;
      }

      Logger.debug('Checking blocks for events', {
        from: this.lastProcessedBlock + 1,
        to: currentBlock,
      });

      // Process new blocks
      for (let height = this.lastProcessedBlock + 1; height <= currentBlock; height++) {
        await this.processBlock(height);
      }

      this.lastProcessedBlock = currentBlock;
    } catch (error) {
      Logger.error('Failed to check for new events', error);
    }
  }

  /**
   * Process a single block for events
   */
  private async processBlock(height: number): Promise<void> {
    try {
      const blockInfo = await this.casperClient.nodeClient.getBlockInfoByHeight(height);
      
      if (!blockInfo.block) {
        return;
      }

      const deployHashes = blockInfo.block.body.deploy_hashes;

      // Process each deploy in the block
      for (const deployHash of deployHashes) {
        await this.processDeploy(deployHash);
      }
    } catch (error) {
      Logger.error('Failed to process block', { height, error });
    }
  }

  /**
   * Process a single deploy for events
   */
  private async processDeployHash(deployHash: string): Promise<void> {
    try {
      const [deploy, raw] = await this.casperClient.nodeClient.getDeployInfo(deployHash);

      // Check if deploy is from one of our contracts
      const contractHash = this.getContractHashFromDeploy(deploy);
      if (!contractHash) {
        return;
      }

      const isOurContract = this.contracts.some(c => c.hash === contractHash);
      if (!isOurContract) {
        return;
      }

      // Extract events from execution results
      if (raw.execution_results && raw.execution_results.length > 0) {
        const result = raw.execution_results[0].result;
        
        if (result.Success) {
          const transforms = result.Success.effect.transforms;
          
          // Parse events from transforms
          const events = this.parseEventsFromTransforms(transforms, deployHash);
          
          // Emit events
          for (const event of events) {
            await this.emitEvent(event.type, event.data);
          }
        }
      }
    } catch (error) {
      Logger.error('Failed to process deploy', { deployHash, error });
    }
  }

  /**
   * Get contract hash from deploy
   */
  private getContractHashFromDeploy(deploy: any): string | null {
    try {
      if (deploy.session && deploy.session.StoredContractByHash) {
        return deploy.session.StoredContractByHash.hash;
      }
      return null;
    } catch (error) {
      return null;
    }
  }

  /**
   * Parse events from transforms
   */
  private parseEventsFromTransforms(transforms: any[], deployHash: string): Array<{ type: string; data: any }> {
    const events: Array<{ type: string; data: any }> = [];

    try {
      // Look for event dictionary writes
      for (const transform of transforms) {
        if (transform.key && transform.key.startsWith('dictionary-')) {
          // This is an event emission
          const eventData = this.parseEventData(transform);
          
          if (eventData) {
            events.push({
              type: eventData.eventType,
              data: {
                ...eventData,
                deployHash,
                timestamp: Date.now(),
              },
            });
          }
        }
      }
    } catch (error) {
      Logger.error('Failed to parse events from transforms', error);
    }

    return events;
  }

  /**
   * Parse event data from transform
   */
  private parseEventData(transform: any): any | null {
    try {
      // Parse based on event structure
      // This is simplified - actual parsing depends on contract event format
      
      const data = transform.transform?.WriteCLValue;
      if (!data) {
        return null;
      }

      // Extract event type and data
      return {
        eventType: data.event_type || 'Unknown',
        ...data,
      };
    } catch (error) {
      return null;
    }
  }

  /**
   * Emit event to registered handlers
   */
  private async emitEvent(eventType: string, data: any): Promise<void> {
    const handlers = this.eventHandlers.get(eventType);
    
    if (!handlers || handlers.size === 0) {
      return;
    }

    Logger.debug('Emitting event', { eventType, data });

    // Call all handlers
    const promises: Promise<void>[] = [];
    
    for (const handler of handlers) {
      try {
        const result = handler(data);
        if (result instanceof Promise) {
          promises.push(result);
        }
      } catch (error) {
        Logger.error('Event handler error', { eventType, error });
      }
    }

    // Wait for all async handlers
    if (promises.length > 0) {
      await Promise.allSettled(promises);
    }
  }

  // ============================================
  // UTILITY METHODS
  // ============================================

  /**
   * Get transaction events (for specific deploy hash)
   */
  async getTransactionEvents(deployHash: string): Promise<BaseEvent[]> {
    try {
      const [deploy, raw] = await this.casperClient.nodeClient.getDeployInfo(deployHash);

      if (!raw.execution_results || raw.execution_results.length === 0) {
        return [];
      }

      const result = raw.execution_results[0].result;
      
      if (!result.Success) {
        return [];
      }

      const transforms = result.Success.effect.transforms;
      const events = this.parseEventsFromTransforms(transforms, deployHash);
      
      return events.map(e => e.data);
    } catch (error) {
      Logger.error('Failed to get transaction events', { deployHash, error });
      return [];
    }
  }

  /**
   * Get event count by type
   */
  getHandlerCount(eventType: string): number {
    return this.eventHandlers.get(eventType)?.size || 0;
  }

  /**
   * Get all registered event types
   */
  getRegisteredEventTypes(): string[] {
    return Array.from(this.eventHandlers.keys());
  }
}

export default EventListener;
export { EventType };
