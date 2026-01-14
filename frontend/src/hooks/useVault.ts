import { useState, useCallback } from 'react';
import { useWalletStore } from '@/store/walletStore';
import { contractService } from '@/services/contractService';
import { VAULT_CONTRACT_HASH } from '@/utils/constants';

// Exchange rate - in real implementation, fetch from contract
const EXCHANGE_RATE = 1.0; // 1 CSPR = 1 cvCSPR initially

interface VaultHook {
  deposit: (amount: string) => Promise<any>;
  withdraw: (shares: string) => Promise<any>;
  instantWithdraw: (shares: string) => Promise<any>;
  estimateShares: (amount: string) => string;
  estimateCSPR: (shares: string) => string;
  loading: boolean;
  isContractReady: boolean;
}

export const useVault = (): VaultHook => {
  const [loading, setLoading] = useState(false);
  const { address, refreshBalance } = useWalletStore();
  
  // Check if contract is configured
  const isContractReady = Boolean(VAULT_CONTRACT_HASH);

  /**
   * Deposit CSPR into the vault - REAL CONTRACT CALL
   */
  const deposit = useCallback(async (amount: string): Promise<any> => {
    if (!address) {
      throw new Error('Wallet not connected');
    }

    if (!isContractReady) {
      throw new Error('Contract not configured. Please set VITE_VAULT_CONTRACT_HASH');
    }

    setLoading(true);
    try {
      console.log('📥 Initiating deposit to CasperVault contract...');
      
      // Real contract call
      const result = await contractService.deposit(address, amount);
      
      console.log('✅ Deposit transaction submitted:', result);
      
      // Refresh wallet balance after a short delay
      setTimeout(() => {
        refreshBalance?.();
      }, 5000);
      
      return {
        deployHash: result.deployHash,
        amount: result.amount,
        shares: estimateShares(amount),
      };
    } catch (error) {
      console.error('❌ Deposit failed:', error);
      throw error;
    } finally {
      setLoading(false);
    }
  }, [address, isContractReady, refreshBalance]);

  /**
   * Withdraw CSPR from the vault - REAL CONTRACT CALL
   */
  const withdraw = useCallback(async (shares: string): Promise<any> => {
    if (!address) {
      throw new Error('Wallet not connected');
    }

    if (!isContractReady) {
      throw new Error('Contract not configured');
    }

    setLoading(true);
    try {
      console.log('📤 Initiating withdrawal from CasperVault contract...');
      
      const cspr = estimateCSPR(shares);
      
      // Real contract call
      const result = await contractService.withdraw(address, cspr);
      
      console.log('✅ Withdrawal transaction submitted:', result);
      
      // Refresh balance after delay
      setTimeout(() => {
        refreshBalance?.();
      }, 5000);
      
      return {
        deployHash: result.deployHash,
        shares,
        cspr: result.amount,
        unlockTime: Date.now() + 7 * 24 * 60 * 60 * 1000, // 7 days for standard withdraw
      };
    } catch (error) {
      console.error('❌ Withdrawal failed:', error);
      throw error;
    } finally {
      setLoading(false);
    }
  }, [address, isContractReady, refreshBalance]);

  /**
   * Instant withdraw with 0.5% fee - REAL CONTRACT CALL
   */
  const instantWithdraw = useCallback(async (shares: string): Promise<any> => {
    if (!address) {
      throw new Error('Wallet not connected');
    }

    if (!isContractReady) {
      throw new Error('Contract not configured');
    }

    setLoading(true);
    try {
      console.log('⚡ Initiating instant withdrawal...');
      
      const cspr = parseFloat(estimateCSPR(shares));
      const fee = cspr * 0.005; // 0.5% fee
      const finalAmount = cspr - fee;
      
      // Real contract call (using withdraw for now, fee handled in contract)
      const result = await contractService.withdraw(address, finalAmount.toString());
      
      console.log('✅ Instant withdrawal submitted:', result);
      
      setTimeout(() => {
        refreshBalance?.();
      }, 5000);
      
      return {
        deployHash: result.deployHash,
        shares,
        cspr: finalAmount.toString(),
        fee: fee.toString(),
      };
    } catch (error) {
      console.error('❌ Instant withdrawal failed:', error);
      throw error;
    } finally {
      setLoading(false);
    }
  }, [address, isContractReady, refreshBalance]);

  /**
   * Estimate cvCSPR shares from CSPR amount
   */
  const estimateShares = useCallback((amount: string): string => {
    try {
      const cspr = parseFloat(amount);
      if (isNaN(cspr) || cspr <= 0) return '0';
      
      const shares = cspr * EXCHANGE_RATE;
      return shares.toFixed(4);
    } catch {
      return '0';
    }
  }, []);

  /**
   * Estimate CSPR amount from cvCSPR shares
   */
  const estimateCSPR = useCallback((shares: string): string => {
    try {
      const cvCSPR = parseFloat(shares);
      if (isNaN(cvCSPR) || cvCSPR <= 0) return '0';
      
      const cspr = cvCSPR / EXCHANGE_RATE;
      return cspr.toFixed(4);
    } catch {
      return '0';
    }
  }, []);

  return {
    deposit,
    withdraw,
    instantWithdraw,
    estimateShares,
    estimateCSPR,
    loading,
    isContractReady,
  };
};
