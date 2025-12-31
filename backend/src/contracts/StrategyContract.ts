import {
  CLValueBuilder,
  CLPublicKey,
  RuntimeArgs,
  Keys,
  CLMap,
  CLString,
  CLU512,
} from 'casper-js-sdk';
import { BaseContract, ContractConfig } from './BaseContract';
import { Logger } from '../utils/logger';
import { ValidationError, ContractError } from '../utils/errors';
import type { TransactionResult, StrategyInfo, AllocationMap } from '../types';

/**
 * StrategyContract wrapper for managing yield strategies
 */
export class StrategyContract extends BaseContract {
  constructor(contractHash: string) {
    super({
      contractHash,
      contractName: 'StrategyContract',
    });
  }

  // ============================================
  // READ METHODS
  // ============================================

  /**
   * Get current allocations across strategies
   */
  async getAllocations(): Promise<Map<string, string>> {
    try {
      Logger.debug('Getting strategy allocations');
      
      const result = await this.queryContract('allocations');
      
      const allocations = new Map<string, string>();
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const data = (result as any).CLValue;
        
        // Parse map of strategy_name -> amount
        if (typeof data === 'object') {
          for (const [strategy, amount] of Object.entries(data)) {
            allocations.set(strategy, amount as string);
          }
        }
      }
      
      return allocations;
    } catch (error) {
      Logger.error('Failed to get allocations', error);
      throw error;
    }
  }

  /**
   * Get APY for specific strategy
   */
  async getStrategyAPY(strategyName: string): Promise<number> {
    try {
      Logger.debug('Getting strategy APY', { strategyName });
      
      const result = await this.queryContract(`apy_${strategyName}`);
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return parseFloat(value.toString()) / 100; // Convert from basis points
      }
      
      return 0;
    } catch (error) {
      Logger.error('Failed to get strategy APY', error);
      throw error;
    }
  }

  /**
   * Get blended APY across all strategies
   */
  async getBlendedAPY(): Promise<number> {
    try {
      Logger.debug('Getting blended APY');
      
      const result = await this.queryContract('blended_apy');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return parseFloat(value.toString()) / 100;
      }
      
      // Calculate manually if not stored
      const allocations = await this.getAllocations();
      let totalValue = BigInt(0);
      let weightedAPY = 0;
      
      for (const [strategy, amount] of allocations) {
        const apy = await this.getStrategyAPY(strategy);
        const amountNum = BigInt(amount);
        
        weightedAPY += apy * Number(amountNum);
        totalValue += amountNum;
      }
      
      if (totalValue > BigInt(0)) {
        return weightedAPY / Number(totalValue);
      }
      
      return 0;
    } catch (error) {
      Logger.error('Failed to get blended APY', error);
      throw error;
    }
  }

  /**
   * Get detailed info for a strategy
   */
  async getStrategyInfo(strategyName: string): Promise<StrategyInfo> {
    try {
      Logger.debug('Getting strategy info', { strategyName });
      
      const allocations = await this.getAllocations();
      const apy = await this.getStrategyAPY(strategyName);
      const tvl = allocations.get(strategyName) || '0';
      
      return {
        name: strategyName,
        apy,
        tvl,
        active: true,
        riskLevel: this.getRiskLevel(strategyName),
      };
    } catch (error) {
      Logger.error('Failed to get strategy info', error);
      throw error;
    }
  }

  /**
   * Get all active strategies
   */
  async getActiveStrategies(): Promise<StrategyInfo[]> {
    try {
      Logger.debug('Getting active strategies');
      
      const allocations = await this.getAllocations();
      const strategies: StrategyInfo[] = [];
      
      for (const [name] of allocations) {
        const info = await this.getStrategyInfo(name);
        strategies.push(info);
      }
      
      return strategies;
    } catch (error) {
      Logger.error('Failed to get active strategies', error);
      throw error;
    }
  }

  /**
   * Get target allocation percentages
   */
  async getTargetAllocation(): Promise<Map<string, number>> {
    try {
      Logger.debug('Getting target allocation');
      
      const result = await this.queryContract('target_allocation');
      
      const targets = new Map<string, number>();
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const data = (result as any).CLValue;
        
        if (typeof data === 'object') {
          for (const [strategy, percentage] of Object.entries(data)) {
            targets.set(strategy, Number(percentage));
          }
        }
      }
      
      return targets;
    } catch (error) {
      Logger.error('Failed to get target allocation', error);
      throw error;
    }
  }

  // ============================================
  // WRITE METHODS
  // ============================================

  /**
   * Allocate funds to a specific strategy
   */
  async allocate(
    amount: string,
    strategyName: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Allocating to strategy', { amount, strategyName });
      
      // Validate amount
      const amountNum = BigInt(amount);
      if (amountNum <= BigInt(0)) {
        throw new ValidationError('Amount must be positive');
      }
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({
        amount: CLValueBuilder.u512(amount),
        strategy: CLValueBuilder.string(strategyName),
      });
      
      // Estimate gas
      const paymentAmount = '8000000000'; // 8 CSPR
      
      // Call contract
      const result = await this.callEntrypoint(
        'allocate',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Allocation successful', {
        deployHash: result.deployHash,
        amount,
        strategyName,
      });
      
      return result;
    } catch (error) {
      Logger.error('Allocation failed', error);
      throw error;
    }
  }

  /**
   * Rebalance strategies to match target allocation
   */
  async rebalance(
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Rebalancing strategies');
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({});
      
      // Higher gas for rebalancing (may involve multiple swaps)
      const paymentAmount = '20000000000'; // 20 CSPR
      
      // Call contract
      const result = await this.callEntrypoint(
        'rebalance',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Rebalance successful', {
        deployHash: result.deployHash,
      });
      
      return result;
    } catch (error) {
      Logger.error('Rebalance failed', error);
      throw error;
    }
  }

  /**
   * Harvest yields from all strategies
   */
  async harvestAll(
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Harvesting all strategies');
      
      const args = RuntimeArgs.fromMap({});
      
      // Higher gas for harvesting multiple strategies
      const paymentAmount = '15000000000'; // 15 CSPR
      
      const result = await this.callEntrypoint(
        'harvest_all',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Harvest successful', {
        deployHash: result.deployHash,
      });
      
      return result;
    } catch (error) {
      Logger.error('Harvest failed', error);
      throw error;
    }
  }

  /**
   * Harvest yield from specific strategy
   */
  async harvestStrategy(
    strategyName: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Harvesting strategy', { strategyName });
      
      const args = RuntimeArgs.fromMap({
        strategy: CLValueBuilder.string(strategyName),
      });
      
      const paymentAmount = '8000000000'; // 8 CSPR
      
      const result = await this.callEntrypoint(
        'harvest_strategy',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Strategy harvest successful', {
        deployHash: result.deployHash,
        strategyName,
      });
      
      return result;
    } catch (error) {
      Logger.error('Strategy harvest failed', error);
      throw error;
    }
  }

  // ============================================
  // ADMIN METHODS
  // ============================================

  /**
   * Set target allocation percentages (operator only)
   */
  async setTargetAllocation(
    allocations: Map<string, number>,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Setting target allocation', {
        strategies: Array.from(allocations.keys()),
      });
      
      // Validate allocations sum to 100%
      let total = 0;
      for (const percentage of allocations.values()) {
        if (percentage < 0 || percentage > 100) {
          throw new ValidationError('Percentage must be between 0-100');
        }
        total += percentage;
      }
      
      if (Math.abs(total - 100) > 0.01) {
        throw new ValidationError('Allocations must sum to 100%');
      }
      
      // Convert to arrays for CLMap
      const strategies: string[] = [];
      const percentages: number[] = [];
      
      for (const [strategy, percentage] of allocations) {
        strategies.push(strategy);
        percentages.push(Math.floor(percentage * 100)); // Convert to basis points
      }
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({
        strategies: CLValueBuilder.list(strategies.map(s => CLValueBuilder.string(s))),
        percentages: CLValueBuilder.list(percentages.map(p => CLValueBuilder.u32(p))),
      });
      
      const paymentAmount = '5000000000'; // 5 CSPR
      
      const result = await this.callEntrypoint(
        'set_target_allocation',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Target allocation set', {
        deployHash: result.deployHash,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to set target allocation', error);
      throw error;
    }
  }

  /**
   * Add new strategy (admin only)
   */
  async addStrategy(
    strategyName: string,
    strategyAddress: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Adding strategy', { strategyName, strategyAddress });
      
      const args = RuntimeArgs.fromMap({
        name: CLValueBuilder.string(strategyName),
        address: CLValueBuilder.string(strategyAddress),
      });
      
      const paymentAmount = '5000000000'; // 5 CSPR
      
      const result = await this.callEntrypoint(
        'add_strategy',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Strategy added', {
        deployHash: result.deployHash,
        strategyName,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to add strategy', error);
      throw error;
    }
  }

  /**
   * Remove strategy (admin only)
   */
  async removeStrategy(
    strategyName: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Removing strategy', { strategyName });
      
      const args = RuntimeArgs.fromMap({
        strategy: CLValueBuilder.string(strategyName),
      });
      
      const paymentAmount = '5000000000'; // 5 CSPR
      
      const result = await this.callEntrypoint(
        'remove_strategy',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Strategy removed', {
        deployHash: result.deployHash,
        strategyName,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to remove strategy', error);
      throw error;
    }
  }

  /**
   * Update strategy APY (keeper only)
   */
  async updateStrategyAPY(
    strategyName: string,
    apy: number,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Updating strategy APY', { strategyName, apy });
      
      if (apy < 0 || apy > 1000) {
        throw new ValidationError('APY must be between 0-1000%');
      }
      
      // Convert to basis points
      const apyBps = Math.floor(apy * 100);
      
      const args = RuntimeArgs.fromMap({
        strategy: CLValueBuilder.string(strategyName),
        apy: CLValueBuilder.u32(apyBps),
      });
      
      const paymentAmount = '3000000000'; // 3 CSPR
      
      const result = await this.callEntrypoint(
        'update_strategy_apy',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Strategy APY updated', {
        deployHash: result.deployHash,
        strategyName,
        apy,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to update strategy APY', error);
      throw error;
    }
  }

  /**
   * Emergency withdraw from strategy (admin only)
   */
  async emergencyWithdraw(
    strategyName: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.warn('Emergency withdraw initiated', { strategyName });
      
      const args = RuntimeArgs.fromMap({
        strategy: CLValueBuilder.string(strategyName),
      });
      
      const paymentAmount = '10000000000'; // 10 CSPR
      
      const result = await this.callEntrypoint(
        'emergency_withdraw',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.warn('Emergency withdraw completed', {
        deployHash: result.deployHash,
        strategyName,
      });
      
      return result;
    } catch (error) {
      Logger.error('Emergency withdraw failed', error);
      throw error;
    }
  }

  // ============================================
  // HELPER METHODS
  // ============================================

  /**
   * Get risk level for strategy (hardcoded for now)
   */
  private getRiskLevel(strategyName: string): 'LOW' | 'MEDIUM' | 'HIGH' {
    const riskMap: Record<string, 'LOW' | 'MEDIUM' | 'HIGH'> = {
      dex: 'LOW',
      lending: 'LOW',
      cross_chain: 'MEDIUM',
      liquid_staking: 'LOW',
      yield_farming: 'MEDIUM',
      leveraged: 'HIGH',
    };
    
    return riskMap[strategyName.toLowerCase()] || 'MEDIUM';
  }
}

export default StrategyContract;
