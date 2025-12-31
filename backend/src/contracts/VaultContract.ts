import {
  CLValueBuilder,
  CLPublicKey,
  RuntimeArgs,
  Keys,
  CLU512,
  CLString,
  CLBool,
} from 'casper-js-sdk';
import { BaseContract, ContractConfig } from './BaseContract';
import { Logger } from '../utils/logger';
import { ValidationError, ContractError } from '../utils/errors';
import type { TransactionResult } from '../types';

/**
 * VaultContract wrapper for CasperVault main vault operations
 */
export class VaultContract extends BaseContract {
  constructor(contractHash: string) {
    super({
      contractHash,
      contractName: 'VaultContract',
    });
  }

  // ============================================
  // READ METHODS
  // ============================================

  /**
   * Get total assets in vault (CSPR)
   */
  async getTotalAssets(): Promise<string> {
    try {
      Logger.debug('Getting total assets');
      
      const result = await this.queryContract('total_assets');
      
      // Parse CLValue result
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return value.toString();
      }
      
      throw new ContractError('Invalid response format');
    } catch (error) {
      Logger.error('Failed to get total assets', error);
      throw error;
    }
  }

  /**
   * Get total shares minted (cvCSPR)
   */
  async getTotalShares(): Promise<string> {
    try {
      Logger.debug('Getting total shares');
      
      const result = await this.queryContract('total_shares');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return value.toString();
      }
      
      throw new ContractError('Invalid response format');
    } catch (error) {
      Logger.error('Failed to get total shares', error);
      throw error;
    }
  }

  /**
   * Get user's share balance
   */
  async getUserShares(address: string): Promise<string> {
    try {
      Logger.debug('Getting user shares', { address });
      
      // Validate address
      CLPublicKey.fromHex(address);
      
      const result = await this.queryContract(`balance_${address}`);
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return value.toString();
      }
      
      return '0';
    } catch (error) {
      Logger.error('Failed to get user shares', error);
      throw error;
    }
  }

  /**
   * Convert assets to shares
   */
  async convertToShares(assets: string): Promise<string> {
    try {
      Logger.debug('Converting assets to shares', { assets });
      
      const totalAssets = await this.getTotalAssets();
      const totalShares = await this.getTotalShares();
      
      const assetsNum = BigInt(assets);
      const totalAssetsNum = BigInt(totalAssets);
      const totalSharesNum = BigInt(totalShares);
      
      // If no shares exist yet, 1:1 ratio
      if (totalSharesNum === BigInt(0)) {
        return assets;
      }
      
      // shares = (assets * totalShares) / totalAssets
      const shares = (assetsNum * totalSharesNum) / totalAssetsNum;
      
      return shares.toString();
    } catch (error) {
      Logger.error('Failed to convert assets to shares', error);
      throw error;
    }
  }

  /**
   * Convert shares to assets
   */
  async convertToAssets(shares: string): Promise<string> {
    try {
      Logger.debug('Converting shares to assets', { shares });
      
      const totalAssets = await this.getTotalAssets();
      const totalShares = await this.getTotalShares();
      
      const sharesNum = BigInt(shares);
      const totalAssetsNum = BigInt(totalAssets);
      const totalSharesNum = BigInt(totalShares);
      
      // If no shares exist, return 0
      if (totalSharesNum === BigInt(0)) {
        return '0';
      }
      
      // assets = (shares * totalAssets) / totalShares
      const assets = (sharesNum * totalAssetsNum) / totalSharesNum;
      
      return assets.toString();
    } catch (error) {
      Logger.error('Failed to convert shares to assets', error);
      throw error;
    }
  }

  /**
   * Get current APY percentage
   */
  async getCurrentAPY(): Promise<number> {
    try {
      Logger.debug('Getting current APY');
      
      const result = await this.queryContract('current_apy');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return parseFloat(value.toString());
      }
      
      // Return default if not set
      return 0;
    } catch (error) {
      Logger.error('Failed to get current APY', error);
      throw error;
    }
  }

  /**
   * Check if vault is paused
   */
  async isPaused(): Promise<boolean> {
    try {
      Logger.debug('Checking if vault is paused');
      
      const result = await this.queryContract('is_paused');
      
      if (result && typeof result === 'object' && 'CLValue' in result) {
        const value = (result as any).CLValue;
        return Boolean(value);
      }
      
      return false;
    } catch (error) {
      Logger.error('Failed to check pause status', error);
      throw error;
    }
  }

  /**
   * Get share price (assets per 1 share)
   */
  async getSharePrice(): Promise<string> {
    try {
      Logger.debug('Getting share price');
      
      // 1 share = 10^9 motes
      const oneShare = (BigInt(10) ** BigInt(9)).toString();
      return await this.convertToAssets(oneShare);
    } catch (error) {
      Logger.error('Failed to get share price', error);
      throw error;
    }
  }

  // ============================================
  // WRITE METHODS
  // ============================================

  /**
   * Deposit CSPR into vault
   */
  async deposit(
    amount: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Depositing to vault', { amount });
      
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
      const paymentAmount = await this.estimateGas('deposit', args);
      
      // Call contract
      const result = await this.callEntrypoint(
        'deposit',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Deposit successful', {
        deployHash: result.deployHash,
        amount,
      });
      
      return result;
    } catch (error) {
      Logger.error('Deposit failed', error);
      throw error;
    }
  }

  /**
   * Withdraw shares from vault
   */
  async withdraw(
    shares: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Withdrawing from vault', { shares });
      
      // Validate shares
      const sharesNum = BigInt(shares);
      if (sharesNum <= BigInt(0)) {
        throw new ValidationError('Shares must be positive');
      }
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({
        shares: CLValueBuilder.u512(shares),
      });
      
      // Estimate gas
      const paymentAmount = await this.estimateGas('withdraw', args);
      
      // Call contract
      const result = await this.callEntrypoint(
        'withdraw',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Withdrawal successful', {
        deployHash: result.deployHash,
        shares,
      });
      
      return result;
    } catch (error) {
      Logger.error('Withdrawal failed', error);
      throw error;
    }
  }

  /**
   * Instant withdraw (bypass unstaking period)
   */
  async instantWithdraw(
    shares: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Instant withdrawing from vault', { shares });
      
      // Validate shares
      const sharesNum = BigInt(shares);
      if (sharesNum <= BigInt(0)) {
        throw new ValidationError('Shares must be positive');
      }
      
      // Build runtime args
      const args = RuntimeArgs.fromMap({
        shares: CLValueBuilder.u512(shares),
      });
      
      // Estimate gas (higher for instant withdraw)
      const paymentAmount = '7000000000'; // 7 CSPR
      
      // Call contract
      const result = await this.callEntrypoint(
        'instant_withdraw',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Instant withdrawal successful', {
        deployHash: result.deployHash,
        shares,
      });
      
      return result;
    } catch (error) {
      Logger.error('Instant withdrawal failed', error);
      throw error;
    }
  }

  // ============================================
  // ADMIN METHODS
  // ============================================

  /**
   * Pause vault (admin only)
   */
  async pause(signerKey: Keys.AsymmetricKey): Promise<TransactionResult> {
    try {
      Logger.info('Pausing vault');
      
      const args = RuntimeArgs.fromMap({});
      const paymentAmount = '3000000000'; // 3 CSPR
      
      const result = await this.callEntrypoint(
        'pause',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Vault paused', { deployHash: result.deployHash });
      
      return result;
    } catch (error) {
      Logger.error('Failed to pause vault', error);
      throw error;
    }
  }

  /**
   * Unpause vault (admin only)
   */
  async unpause(signerKey: Keys.AsymmetricKey): Promise<TransactionResult> {
    try {
      Logger.info('Unpausing vault');
      
      const args = RuntimeArgs.fromMap({});
      const paymentAmount = '3000000000'; // 3 CSPR
      
      const result = await this.callEntrypoint(
        'unpause',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Vault unpaused', { deployHash: result.deployHash });
      
      return result;
    } catch (error) {
      Logger.error('Failed to unpause vault', error);
      throw error;
    }
  }

  /**
   * Set vault fees (admin only)
   */
  async setFees(
    performanceFee: number,
    managementFee: number,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Setting vault fees', { performanceFee, managementFee });
      
      // Validate fees (max 20% each)
      if (performanceFee < 0 || performanceFee > 20) {
        throw new ValidationError('Performance fee must be between 0-20%');
      }
      if (managementFee < 0 || managementFee > 20) {
        throw new ValidationError('Management fee must be between 0-20%');
      }
      
      // Convert to basis points (1% = 100 bps)
      const performanceFeeBps = Math.floor(performanceFee * 100);
      const managementFeeBps = Math.floor(managementFee * 100);
      
      const args = RuntimeArgs.fromMap({
        performance_fee: CLValueBuilder.u32(performanceFeeBps),
        management_fee: CLValueBuilder.u32(managementFeeBps),
      });
      
      const paymentAmount = '3000000000'; // 3 CSPR
      
      const result = await this.callEntrypoint(
        'set_fees',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Fees updated', {
        deployHash: result.deployHash,
        performanceFee,
        managementFee,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to set fees', error);
      throw error;
    }
  }

  /**
   * Update APY (keeper only)
   */
  async updateAPY(
    apy: number,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Updating APY', { apy });
      
      if (apy < 0 || apy > 1000) {
        throw new ValidationError('APY must be between 0-1000%');
      }
      
      // Convert to basis points
      const apyBps = Math.floor(apy * 100);
      
      const args = RuntimeArgs.fromMap({
        apy: CLValueBuilder.u32(apyBps),
      });
      
      const paymentAmount = '3000000000'; // 3 CSPR
      
      const result = await this.callEntrypoint(
        'update_apy',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('APY updated', { deployHash: result.deployHash, apy });
      
      return result;
    } catch (error) {
      Logger.error('Failed to update APY', error);
      throw error;
    }
  }

  /**
   * Transfer fees to treasury (keeper only)
   */
  async transferFees(
    recipient: string,
    amount: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.info('Transferring fees', { recipient, amount });
      
      const recipientKey = CLPublicKey.fromHex(recipient);
      
      const args = RuntimeArgs.fromMap({
        recipient: recipientKey,
        amount: CLValueBuilder.u512(amount),
      });
      
      const paymentAmount = '3000000000'; // 3 CSPR
      
      const result = await this.callEntrypoint(
        'transfer_fees',
        args,
        paymentAmount,
        signerKey
      );
      
      Logger.info('Fees transferred', {
        deployHash: result.deployHash,
        recipient,
        amount,
      });
      
      return result;
    } catch (error) {
      Logger.error('Failed to transfer fees', error);
      throw error;
    }
  }
}

export default VaultContract;
