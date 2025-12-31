import { describe, it, expect, beforeAll, jest } from '@jest/globals';
import { AccountManager } from '../../src/contracts/AccountManager';
import * as fs from 'fs';
import * as path from 'path';

// Mock fs module
jest.mock('fs');
jest.mock('../../src/utils/config');
jest.mock('../../src/database/client');

describe('AccountManager', () => {
  const mockKeyContent = `-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIFakeKeyContentHereForTestingPurposesOnly
-----END PRIVATE KEY-----`;

  beforeAll(() => {
    // Mock file system
    (fs.existsSync as jest.Mock).mockReturnValue(true);
    (fs.readFileSync as jest.Mock).mockReturnValue(mockKeyContent);
  });

  describe('Account Loading', () => {
    it('should load accounts from config', () => {
      const config = {
        keysDirectory: '/test/keys',
        accounts: {
          admin: 'admin.pem',
          operator: 'operator.pem',
        },
      };

      // Note: This will fail without actual key files
      // In real tests, use mock keys
      expect(() => new AccountManager(config)).not.toThrow();
    });

    it('should throw if admin account missing', () => {
      const config = {
        keysDirectory: '/test/keys',
        accounts: {
          operator: 'operator.pem',
        },
      };

      expect(() => new AccountManager(config)).toThrow('Admin account is required');
    });
  });

  describe('Account Access', () => {
    let accountManager: AccountManager;

    beforeAll(() => {
      accountManager = new AccountManager({
        keysDirectory: '/test/keys',
        accounts: {
          admin: 'admin.pem',
          operator: 'operator.pem',
        },
      });
    });

    it('should get account by role', () => {
      expect(() => accountManager.getAccount('ADMIN')).not.toThrow();
    });

    it('should get address by role', () => {
      const address = accountManager.getAddress('ADMIN');
      expect(address).toBeTruthy();
    });

    it('should check if role exists', () => {
      expect(accountManager.hasRole('ADMIN')).toBe(true);
      expect(accountManager.hasRole('KEEPER')).toBe(false);
    });

    it('should get available roles', () => {
      const roles = accountManager.getAvailableRoles();
      expect(roles).toContain('ADMIN');
      expect(roles).toContain('OPERATOR');
    });
  });

  describe('Multi-sig Proposals', () => {
    let accountManager: AccountManager;

    beforeAll(() => {
      accountManager = new AccountManager({
        keysDirectory: '/test/keys',
        accounts: {
          admin: 'admin.pem',
          operator: 'operator.pem',
          keeper: 'keeper.pem',
        },
      });
    });

    it('should create proposal', async () => {
      const transaction = { entrypoint: 'test', args: {} };
      const proposalId = await accountManager.proposeTransaction(
        transaction,
        'OPERATOR',
        2
      );

      expect(proposalId).toBeTruthy();
      expect(proposalId).toMatch(/^proposal-/);
    });

    it('should approve proposal', async () => {
      const transaction = { entrypoint: 'test', args: {} };
      const proposalId = await accountManager.proposeTransaction(
        transaction,
        'OPERATOR',
        2
      );

      await expect(
        accountManager.approveProposal(proposalId, 'ADMIN')
      ).resolves.not.toThrow();

      const proposal = accountManager.getProposal(proposalId);
      expect(proposal?.approvals.size).toBe(2);
      expect(proposal?.status).toBe('approved');
    });

    it('should reject proposal', async () => {
      const transaction = { entrypoint: 'test', args: {} };
      const proposalId = await accountManager.proposeTransaction(
        transaction,
        'OPERATOR',
        2
      );

      await accountManager.rejectProposal(proposalId, 'ADMIN');

      const proposal = accountManager.getProposal(proposalId);
      expect(proposal?.status).toBe('rejected');
    });

    it('should execute approved proposal', async () => {
      const transaction = { entrypoint: 'test', args: {} };
      const proposalId = await accountManager.proposeTransaction(
        transaction,
        'OPERATOR',
        2
      );

      await accountManager.approveProposal(proposalId, 'ADMIN');

      const result = await accountManager.executeProposal(proposalId);
      expect(result.success).toBe(true);
    });

    it('should not execute unapproved proposal', async () => {
      const transaction = { entrypoint: 'test', args: {} };
      const proposalId = await accountManager.proposeTransaction(
        transaction,
        'OPERATOR',
        3 // Requires 3 approvals
      );

      await expect(
        accountManager.executeProposal(proposalId)
      ).rejects.toThrow('Proposal not approved');
    });

    it('should get pending proposals', async () => {
      const transaction = { entrypoint: 'test', args: {} };
      
      await accountManager.proposeTransaction(transaction, 'OPERATOR', 2);
      await accountManager.proposeTransaction(transaction, 'KEEPER', 2);

      const pending = accountManager.getPendingProposals();
      expect(pending.length).toBeGreaterThan(0);
    });
  });

  describe('Error Handling', () => {
    it('should throw on invalid key file', () => {
      (fs.existsSync as jest.Mock).mockReturnValue(false);

      const config = {
        keysDirectory: '/test/keys',
        accounts: {
          admin: 'invalid.pem',
        },
      };

      expect(() => new AccountManager(config)).toThrow();
    });

    it('should throw on missing account', () => {
      const accountManager = new AccountManager({
        keysDirectory: '/test/keys',
        accounts: {
          admin: 'admin.pem',
        },
      });

      expect(() => accountManager.getAccount('KEEPER')).toThrow(
        'Account not found: KEEPER'
      );
    });
  });
});
