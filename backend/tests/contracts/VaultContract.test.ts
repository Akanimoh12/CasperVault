import { describe, it, expect, beforeAll, jest } from '@jest/globals';
import { VaultContract } from '../../src/contracts/VaultContract';
import { Keys } from 'casper-js-sdk';

// Mock configuration
jest.mock('../../src/utils/config', () => ({
  config: {
    casper: {
      network: 'casper-test',
      rpcUrl: 'http://localhost:7777/rpc',
    },
  },
}));

describe('VaultContract', () => {
  let vaultContract: VaultContract;
  let testKey: Keys.AsymmetricKey;

  beforeAll(() => {
    // Initialize contract
    vaultContract = new VaultContract(
      '0000000000000000000000000000000000000000000000000000000000000000'
    );

    // Generate test key
    testKey = Keys.Ed25519.new();
  });

  describe('Read Methods', () => {
    it('should get total assets', async () => {
      // Mock the queryContract method
      jest.spyOn(vaultContract as any, 'queryContract').mockResolvedValue({
        CLValue: '1000000000000',
      });

      const totalAssets = await vaultContract.getTotalAssets();
      expect(totalAssets).toBe('1000000000000');
    });

    it('should get total shares', async () => {
      jest.spyOn(vaultContract as any, 'queryContract').mockResolvedValue({
        CLValue: '1000000000000',
      });

      const totalShares = await vaultContract.getTotalShares();
      expect(totalShares).toBe('1000000000000');
    });

    it('should get user shares', async () => {
      jest.spyOn(vaultContract as any, 'queryContract').mockResolvedValue({
        CLValue: '500000000000',
      });

      const userShares = await vaultContract.getUserShares(
        testKey.publicKey.toHex()
      );
      expect(userShares).toBe('500000000000');
    });

    it('should convert assets to shares', async () => {
      // Mock total assets and shares
      jest.spyOn(vaultContract as any, 'queryContract')
        .mockResolvedValueOnce({ CLValue: '1000000000000' }) // totalAssets
        .mockResolvedValueOnce({ CLValue: '1000000000000' }); // totalShares

      const shares = await vaultContract.convertToShares('500000000000');
      expect(shares).toBe('500000000000');
    });

    it('should convert shares to assets', async () => {
      jest.spyOn(vaultContract as any, 'queryContract')
        .mockResolvedValueOnce({ CLValue: '1000000000000' }) // totalAssets
        .mockResolvedValueOnce({ CLValue: '1000000000000' }); // totalShares

      const assets = await vaultContract.convertToAssets('500000000000');
      expect(assets).toBe('500000000000');
    });

    it('should get current APY', async () => {
      jest.spyOn(vaultContract as any, 'queryContract').mockResolvedValue({
        CLValue: '1200', // 12% in basis points
      });

      const apy = await vaultContract.getCurrentAPY();
      expect(apy).toBe(1200);
    });

    it('should check if paused', async () => {
      jest.spyOn(vaultContract as any, 'queryContract').mockResolvedValue({
        CLValue: false,
      });

      const isPaused = await vaultContract.isPaused();
      expect(isPaused).toBe(false);
    });
  });

  describe('Write Methods', () => {
    it('should deposit to vault', async () => {
      // Mock callEntrypoint
      jest.spyOn(vaultContract as any, 'callEntrypoint').mockResolvedValue({
        success: true,
        deployHash: 'test-deploy-hash',
        timestamp: Date.now(),
      });

      const result = await vaultContract.deposit('1000000000', testKey);
      
      expect(result.success).toBe(true);
      expect(result.deployHash).toBe('test-deploy-hash');
    });

    it('should reject invalid deposit amount', async () => {
      await expect(vaultContract.deposit('0', testKey)).rejects.toThrow(
        'Amount must be positive'
      );
    });

    it('should withdraw from vault', async () => {
      jest.spyOn(vaultContract as any, 'callEntrypoint').mockResolvedValue({
        success: true,
        deployHash: 'test-deploy-hash',
        timestamp: Date.now(),
      });

      const result = await vaultContract.withdraw('500000000', testKey);
      
      expect(result.success).toBe(true);
      expect(result.deployHash).toBe('test-deploy-hash');
    });

    it('should instant withdraw', async () => {
      jest.spyOn(vaultContract as any, 'callEntrypoint').mockResolvedValue({
        success: true,
        deployHash: 'test-deploy-hash',
        timestamp: Date.now(),
      });

      const result = await vaultContract.instantWithdraw('500000000', testKey);
      
      expect(result.success).toBe(true);
    });
  });

  describe('Admin Methods', () => {
    it('should pause vault', async () => {
      jest.spyOn(vaultContract as any, 'callEntrypoint').mockResolvedValue({
        success: true,
        deployHash: 'test-deploy-hash',
        timestamp: Date.now(),
      });

      const result = await vaultContract.pause(testKey);
      
      expect(result.success).toBe(true);
    });

    it('should unpause vault', async () => {
      jest.spyOn(vaultContract as any, 'callEntrypoint').mockResolvedValue({
        success: true,
        deployHash: 'test-deploy-hash',
        timestamp: Date.now(),
      });

      const result = await vaultContract.unpause(testKey);
      
      expect(result.success).toBe(true);
    });

    it('should set fees', async () => {
      jest.spyOn(vaultContract as any, 'callEntrypoint').mockResolvedValue({
        success: true,
        deployHash: 'test-deploy-hash',
        timestamp: Date.now(),
      });

      const result = await vaultContract.setFees(10, 2, testKey);
      
      expect(result.success).toBe(true);
    });

    it('should reject invalid fees', async () => {
      await expect(vaultContract.setFees(25, 2, testKey)).rejects.toThrow(
        'Performance fee must be between 0-20%'
      );

      await expect(vaultContract.setFees(10, 25, testKey)).rejects.toThrow(
        'Management fee must be between 0-20%'
      );
    });

    it('should update APY', async () => {
      jest.spyOn(vaultContract as any, 'callEntrypoint').mockResolvedValue({
        success: true,
        deployHash: 'test-deploy-hash',
        timestamp: Date.now(),
      });

      const result = await vaultContract.updateAPY(15, testKey);
      
      expect(result.success).toBe(true);
    });
  });

  describe('Error Handling', () => {
    it('should handle network errors', async () => {
      jest
        .spyOn(vaultContract as any, 'queryContract')
        .mockRejectedValue(new Error('Network error'));

      await expect(vaultContract.getTotalAssets()).rejects.toThrow();
    });

    it('should handle contract errors', async () => {
      jest
        .spyOn(vaultContract as any, 'callEntrypoint')
        .mockRejectedValue(new Error('Contract error'));

      await expect(vaultContract.deposit('1000000000', testKey)).rejects.toThrow();
    });
  });
});
