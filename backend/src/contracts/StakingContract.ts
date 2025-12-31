import {
  CLValueBuilder,
  CLPublicKey,
  RuntimeArgs,
  Keys,
  CLList,
  CLString,
} from 'casper-js-sdk';
import { BaseContract, ContractConfig } from './BaseContract';
import { Logger } from '../utils/logger';
import { ValidationError, ContractError } from '../utils/errors';
import type { TransactionResult, ValidatorInfo } from '../types';

/**
 * StakingContract wrapper for managing staking operations
 */
export class StakingContract extends BaseContract {
  constructor(contractHash: string) {
    super({
      contractHash,
      contractName: 'StakingContract',
    });
  }

  // ============================================
  // READ METHODS
  // ============================================

  /**
   * Get total amount staked
   */
  async getTotalStaked(): Promise<string> {
    try {
      Logger.debug('Getting total staked amount');
      
      const result = await this.queryContract('total_staked');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return value.toString();
      }
      
      throw new ContractError('Invalid response format');
    } catch (error) {
      Logger.error('Failed to get total staked', error);
      throw error;
    }
  }

  /**
   * Get list of active validators
   */
  async getValidators(): Promise<ValidatorInfo[]> {
    try {
      Logger.debug('Getting validators');
      
      const result = await this.queryContract('validators');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const validatorsData = (result as any).CLValue;
        
        // Parse validator list
        const validators: ValidatorInfo[] = [];
        
        // Assume format: [[address, stake_amount, commission], ...]
        if (Array.isArray(validatorsData)) {
          for (const validator of validatorsData) {
            validators.push({
              address: validator[0].toString(),
              stakeAmount: validator[1].toString(),
              commission: parseFloat(validator[2].toString()),
              active: true,
            });
          }
        }
        
        return validators;
      }
      
      return [];
    } catch (error) {
      Logger.error('Failed to get validators', error);
      throw error;
    }
  }

  /**
   * Get exchange rate (lstCSPR to CSPR)
   */
  async getExchangeRate(): Promise<string> {
    try {
      Logger.debug('Getting exchange rate');
      
      const result = await this.queryContract('exchange_rate');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return value.toString();
      }
      
      // Default 1:1 if not set
      return (BigInt(10) ** BigInt(9)).toString();
    } catch (error) {
      Logger.error('Failed to get exchange rate', error);
      throw error;
    }
  }

  /**
   * Get staked amount for specific validator
   */
  async getValidatorStake(validatorAddress: string): Promise<string> {
    try {
      Logger.debug('Getting validator stake', { validatorAddress });
      
      const result = await this.queryContract(`validator_stake_${validatorAddress}`);
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return value.toString();
      }
      
      return '0';
    } catch (error) {
      Logger.error('Failed to get validator stake', error);
      throw error;
    }
  }

  /**
   * Get pending rewards
   */
  async getPendingRewards(): Promise<string> {
    try {
      Logger.debug('Getting pending rewards');
      
      const result = await this.queryContract('pending_rewards');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return value.toString();
      }
      
      return '0';
    } catch (error) {
      Logger.error('Failed to get pending rewards', error);
      throw error;
    }
  }

  /**
   * Get total rewards earned
   */
  async getTotalRewards(): Promise<string> {
    try {
      Logger.debug('Getting total rewards');
      
      const result = await this.queryContract('total_rewards');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return value.toString();
      }
      
      return '0';
    } catch (error) {
      Logger.error('Failed to get total rewards', error);
      throw error;
    }
  }

  // ============================================
  // WRITE METHODS
  // ============================================

  /**
   * Stake CSPR to validators
   */
  async stake(
    amount: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Staking CSPR', { amount });
      
      // Validate amount
      const amountNum = BigInt(amount);
      if (amountNum <= BigInt(0)) {
        throw new ValidationError('Amount must be positive');
      }
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({
        amount: CLValueBuilder.u512(amount),
      });
      
      // Estimate gas
      const paymentAmount = await this.estimateGas('stake', args);
      
      // Call contract
      const result = await this.callEntrypoint(
        'stake',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Stake successful', {
        deployHash: result.deployHash,
        amount,
      });
      
      return result;
    } catch (error) {
      Logger.error('Stake failed', error);
      throw error;
    }
  }

  /**
   * Unstake from validators
   */
  async unstake(
    amount: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Unstaking', { amount });
      
      // Validate amount
      const amountNum = BigInt(amount);
      if (amountNum <= BigInt(0)) {
        throw new ValidationError('Amount must be positive');
      }
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({
        amount: CLValueBuilder.u512(amount),
      });
      
      // Estimate gas
      const paymentAmount = await this.estimateGas('unstake', args);
      
      // Call contract
      const result = await this.callEntrypoint(
        'unstake',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Unstake initiated', {
        deployHash: result.deployHash,
        amount,
      });
      
      return result;
    } catch (error) {
      Logger.error('Unstake failed', error);
      throw error;
    }
  }

  /**
   * Compound rewards back into staking
   */
  async compoundRewards(
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Compounding rewards');
      
      // Check pending rewards
      const pendingRewards = await this.getPendingRewards();
      
      if (BigInt(pendingRewards) === BigInt(0)) {
        Logger.info('No rewards to compound');
        throw new ValidationError('No rewards available to compound');
      }
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({});
      
      // Estimate gas (higher for compound operation)
      const paymentAmount = '10000000000'; // 10 CSPR
      
      // Call contract
      const result = await this.callEntrypoint(
        'compound_rewards',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Compound successful', {
        deployHash: result.deployHash,
        rewardsCompounded: pendingRewards,
      });
      
      return result;
    } catch (error) {
      Logger.error('Compound failed', error);
      throw error;
    }
  }

  /**
   * Claim rewards (transfer to vault)
   */
  async claimRewards(
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Claiming rewards');
      
      const args = RuntimeArgs.fromMap({});
      const paymentAmount = '5000000000'; // 5 CSPR
      
      const result = await this.callEntrypoint(
        'claim_rewards',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Rewards claimed', { deployHash: result.deployHash });
      
      return result;
    } catch (error) {
      Logger.error('Claim rewards failed', error);
      throw error;
    }
  }

  // ============================================
  // ADMIN METHODS
  // ============================================

  /**
   * Add validator to whitelist (admin only)
   */
  async addValidator(
    validatorAddress: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Adding validator', { validatorAddress });
      
      // Validate address
      const validatorKey = CLPublicKey.fromHex(validatorAddress);
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({
        validator: validatorKey,
      });
      
      const paymentAmount = '3000000000'; // 3 CSPR
      
      // Call contract
      const result = await this.callEntrypoint(
        'add_validator',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Validator added', {
        deployHash: result.deployHash,
        validatorAddress,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to add validator', error);
      throw error;
    }
  }

  /**
   * Remove validator from whitelist (admin only)
   */
  async removeValidator(
    validatorAddress: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Removing validator', { validatorAddress });
      
      const validatorKey = CLPublicKey.fromHex(validatorAddress);
      
      const args = RuntimeArgs.fromMap({
        validator: validatorKey,
      });
      
      const paymentAmount = '3000000000'; // 3 CSPR
      
      const result = await this.callEntrypoint(
        'remove_validator',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Validator removed', {
        deployHash: result.deployHash,
        validatorAddress,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to remove validator', error);
      throw error;
    }
  }

  /**
   * Update validator allocation (operator only)
   */
  async updateValidatorAllocation(
    allocations: Map<string, string>,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Updating validator allocations', {
        count: allocations.size,
      });
      
      // Build validator list
      const validators: CLPublicKey[] = [];
      const amounts: string[] = [];
      
      for (const [address, amount] of allocations) {
        validators.push(CLPublicKey.fromHex(address));
        amounts.push(amount);
      }
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({
        validators: CLValueBuilder.list(validators),
        amounts: CLValueBuilder.list(amounts.map(a => CLValueBuilder.u512(a))),
      });
      
      const paymentAmount = '10000000000'; // 10 CSPR
      
      const result = await this.callEntrypoint(
        'update_allocations',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Allocations updated', {
        deployHash: result.deployHash,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to update allocations', error);
      throw error;
    }
  }

  /**
   * Rebalance stake across validators (operator only)
   */
  async rebalance(
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Rebalancing validators');
      
      const args = RuntimeArgs.fromMap({});
      const paymentAmount = '15000000000'; // 15 CSPR
      
      const result = await this.callEntrypoint(
        'rebalance',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Rebalance completed', {
        deployHash: result.deployHash,
      });
      
      return result;
    } catch (error) {
      Logger.error('Rebalance failed', error);
      throw error;
    }
  }

  /**
   * Set minimum stake amount (admin only)
   */
  async setMinStakeAmount(
    amount: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Setting min stake amount', { amount });
      
      const args = RuntimeArgs.fromMap({
        amount: CLValueBuilder.u512(amount),
      });
      
      const paymentAmount = '3000000000'; // 3 CSPR
      
      const result = await this.callEntrypoint(
        'set_min_stake',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Min stake amount set', {
        deployHash: result.deployHash,
        amount,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to set min stake', error);
      throw error;
    }
  }
}

export default StakingContract;
