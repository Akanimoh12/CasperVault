// Contract Wrappers
export { BaseContract } from './BaseContract';
export { VaultContract } from './VaultContract';
export { StakingContract } from './StakingContract';
export { StrategyContract } from './StrategyContract';

// Event System
export { EventListener, EventType } from './EventListener';

// Transaction Management
export { TransactionManager } from './TransactionManager';
export type { Transaction, TxStatusCallback } from './TransactionManager';

// Account Management
export { AccountManager } from './AccountManager';
export type { AccountConfig } from './AccountManager';

// Re-export types
export type {
  TransactionResult,
  TransactionStatus,
  AccountRole,
  DepositEvent,
  WithdrawEvent,
  CompoundEvent,
  RebalanceEvent,
  BridgeEvent,
  ValidatorInfo,
  StrategyInfo,
} from '../types';
