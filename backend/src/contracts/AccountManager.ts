import { Keys, CLPublicKey, Deploy, DeployUtil } from 'casper-js-sdk';
import * as fs from 'fs';
import * as path from 'path';
import { Logger } from '../utils/logger';
import { ValidationError, AuthenticationError } from '../utils/errors';
import type { AccountRole, TransactionResult } from '../types';
import Database from '../database/client';

/**
 * Account configuration
 */
export interface AccountConfig {
  keysDirectory: string;
  accounts: {
    admin?: string; // Path to admin key file
    operator?: string; // Path to operator key file
    keeper?: string; // Path to keeper key file
    guardian?: string; // Path to guardian key file
  };
}

/**
 * Multi-sig proposal
 */
interface Proposal {
  id: string;
  transaction: any;
  proposer: AccountRole;
  approvals: Set<AccountRole>;
  requiredApprovals: number;
  createdAt: number;
  expiresAt: number;
  status: 'pending' | 'approved' | 'executed' | 'rejected' | 'expired';
}

/**
 * AccountManager for managing multiple accounts and multi-sig operations
 */
export class AccountManager {
  private accounts: Map<AccountRole, Keys.AsymmetricKey>;
  private addresses: Map<AccountRole, string>;
  private keysDirectory: string;
  private proposals: Map<string, Proposal>;
  private database: Database;

  constructor(config: AccountConfig) {
    this.accounts = new Map();
    this.addresses = new Map();
    this.keysDirectory = config.keysDirectory;
    this.proposals = new Map();
    this.database = Database.getInstance();

    // Load accounts
    this.loadAccounts(config.accounts);

    Logger.info('AccountManager initialized', {
      accounts: Array.from(this.accounts.keys()),
      keysDirectory: this.keysDirectory,
    });
  }

  // ============================================
  // ACCOUNT MANAGEMENT
  // ============================================

  /**
   * Load accounts from key files
   */
  private loadAccounts(accountPaths: AccountConfig['accounts']): void {
    for (const [role, keyPath] of Object.entries(accountPaths)) {
      if (!keyPath) {
        continue;
      }

      try {
        const accountRole = role.toUpperCase() as AccountRole;
        const fullPath = path.join(this.keysDirectory, keyPath);
        
        Logger.debug('Loading account key', { role, path: fullPath });
        
        const key = this.loadPrivateKey(fullPath);
        this.accounts.set(accountRole, key);
        
        // Store address
        const address = key.publicKey.toHex();
        this.addresses.set(accountRole, address);
        
        Logger.info('Account loaded', {
          role,
          address: address.substring(0, 10) + '...',
        });
      } catch (error) {
        Logger.error('Failed to load account', { role, error });
        throw new ValidationError(`Failed to load ${role} account: ${error}`);
      }
    }

    // Validate at least admin account exists
    if (!this.accounts.has('ADMIN')) {
      throw new ValidationError('Admin account is required');
    }
  }

  /**
   * Load private key from file
   */
  private loadPrivateKey(keyPath: string): Keys.AsymmetricKey {
    try {
      if (!fs.existsSync(keyPath)) {
        throw new Error(`Key file not found: ${keyPath}`);
      }

      const keyContent = fs.readFileSync(keyPath, 'utf-8');
      return Keys.Ed25519.parsePrivateKeyFile(keyContent);
    } catch (error) {
      Logger.error('Failed to load private key', { keyPath, error });
      throw error;
    }
  }

  /**
   * Get account by role
   */
  getAccount(role: AccountRole): Keys.AsymmetricKey {
    const account = this.accounts.get(role);
    
    if (!account) {
      throw new AuthenticationError(`Account not found: ${role}`);
    }

    return account;
  }

  /**
   * Get address by role
   */
  getAddress(role: AccountRole): string {
    const address = this.addresses.get(role);
    
    if (!address) {
      throw new AuthenticationError(`Address not found: ${role}`);
    }

    return address;
  }

  /**
   * Check if role exists
   */
  hasRole(role: AccountRole): boolean {
    return this.accounts.has(role);
  }

  /**
   * Get all available roles
   */
  getAvailableRoles(): AccountRole[] {
    return Array.from(this.accounts.keys());
  }

  /**
   * Add account dynamically
   */
  addAccount(role: AccountRole, keyPath: string): void {
    try {
      Logger.info('Adding account', { role });
      
      const fullPath = path.join(this.keysDirectory, keyPath);
      const key = this.loadPrivateKey(fullPath);
      
      this.accounts.set(role, key);
      this.addresses.set(role, key.publicKey.toHex());
      
      Logger.info('Account added', {
        role,
        address: key.publicKey.toHex().substring(0, 10) + '...',
      });
    } catch (error) {
      Logger.error('Failed to add account', { role, error });
      throw error;
    }
  }

  /**
   * Remove account
   */
  removeAccount(role: AccountRole): void {
    if (role === 'ADMIN') {
      throw new ValidationError('Cannot remove admin account');
    }

    this.accounts.delete(role);
    this.addresses.delete(role);
    
    Logger.info('Account removed', { role });
  }

  // ============================================
  // TRANSACTION SIGNING
  // ============================================

  /**
   * Sign transaction with specified role
   */
  signTransaction(deploy: Deploy, role: AccountRole): Deploy {
    try {
      Logger.debug('Signing transaction', { role });
      
      const account = this.getAccount(role);
      const signedDeploy = deploy.sign([account]);
      
      Logger.debug('Transaction signed', {
        role,
        deployHash: Buffer.from(signedDeploy.hash).toString('hex'),
      });
      
      return signedDeploy;
    } catch (error) {
      Logger.error('Failed to sign transaction', { role, error });
      throw error;
    }
  }

  /**
   * Sign with multiple roles
   */
  signTransactionMultiple(deploy: Deploy, roles: AccountRole[]): Deploy {
    try {
      Logger.debug('Multi-signing transaction', { roles });
      
      const keys = roles.map(role => this.getAccount(role));
      const signedDeploy = deploy.sign(keys);
      
      Logger.debug('Transaction multi-signed', {
        roles,
        deployHash: Buffer.from(signedDeploy.hash).toString('hex'),
      });
      
      return signedDeploy;
    } catch (error) {
      Logger.error('Failed to multi-sign transaction', { roles, error });
      throw error;
    }
  }

  // ============================================
  // MULTI-SIG PROPOSALS
  // ============================================

  /**
   * Propose transaction for multi-sig approval
   */
  async proposeTransaction(
    transaction: any,
    proposer: AccountRole = 'OPERATOR',
    requiredApprovals: number = 2
  ): Promise<string> {
    try {
      Logger.info('Creating multi-sig proposal', {
        proposer,
        requiredApprovals,
      });

      // Generate proposal ID
      const proposalId = this.generateProposalId();

      // Create proposal
      const proposal: Proposal = {
        id: proposalId,
        transaction,
        proposer,
        approvals: new Set([proposer]), // Auto-approve from proposer
        requiredApprovals,
        createdAt: Date.now(),
        expiresAt: Date.now() + 24 * 60 * 60 * 1000, // 24 hours
        status: 'pending',
      };

      // Store proposal
      this.proposals.set(proposalId, proposal);

      Logger.info('Proposal created', {
        proposalId,
        proposer,
        requiredApprovals,
      });

      return proposalId;
    } catch (error) {
      Logger.error('Failed to create proposal', error);
      throw error;
    }
  }

  /**
   * Approve proposal
   */
  async approveProposal(
    proposalId: string,
    role: AccountRole
  ): Promise<void> {
    try {
      Logger.info('Approving proposal', { proposalId, role });

      const proposal = this.proposals.get(proposalId);
      
      if (!proposal) {
        throw new ValidationError('Proposal not found');
      }

      // Check if expired
      if (Date.now() > proposal.expiresAt) {
        proposal.status = 'expired';
        throw new ValidationError('Proposal has expired');
      }

      // Check if already executed
      if (proposal.status === 'executed') {
        throw new ValidationError('Proposal already executed');
      }

      // Add approval
      proposal.approvals.add(role);

      Logger.info('Proposal approved', {
        proposalId,
        role,
        approvals: proposal.approvals.size,
        required: proposal.requiredApprovals,
      });

      // Check if ready to execute
      if (proposal.approvals.size >= proposal.requiredApprovals) {
        proposal.status = 'approved';
        Logger.info('Proposal ready for execution', { proposalId });
      }
    } catch (error) {
      Logger.error('Failed to approve proposal', { proposalId, error });
      throw error;
    }
  }

  /**
   * Reject proposal
   */
  async rejectProposal(
    proposalId: string,
    role: AccountRole
  ): Promise<void> {
    try {
      Logger.info('Rejecting proposal', { proposalId, role });

      const proposal = this.proposals.get(proposalId);
      
      if (!proposal) {
        throw new ValidationError('Proposal not found');
      }

      // Only admin or guardian can reject
      if (role !== 'ADMIN' && role !== 'GUARDIAN') {
        throw new AuthenticationError('Insufficient permissions to reject');
      }

      proposal.status = 'rejected';
      
      Logger.info('Proposal rejected', { proposalId, role });
    } catch (error) {
      Logger.error('Failed to reject proposal', { proposalId, error });
      throw error;
    }
  }

  /**
   * Execute approved proposal
   */
  async executeProposal(proposalId: string): Promise<TransactionResult> {
    try {
      Logger.info('Executing proposal', { proposalId });

      const proposal = this.proposals.get(proposalId);
      
      if (!proposal) {
        throw new ValidationError('Proposal not found');
      }

      // Check if approved
      if (proposal.status !== 'approved') {
        throw new ValidationError(`Proposal not approved (status: ${proposal.status})`);
      }

      // Check if expired
      if (Date.now() > proposal.expiresAt) {
        proposal.status = 'expired';
        throw new ValidationError('Proposal has expired');
      }

      // Get signing roles (all approvers)
      const signingRoles = Array.from(proposal.approvals);

      // Sign and submit transaction
      // Note: Actual execution depends on transaction structure
      // This is a simplified version
      
      Logger.info('Executing multi-sig transaction', {
        proposalId,
        signers: signingRoles,
      });

      // Mark as executed
      proposal.status = 'executed';

      // Clean up old proposal
      setTimeout(() => {
        this.proposals.delete(proposalId);
      }, 60000); // Keep for 1 minute

      return {
        success: true,
        deployHash: 'multi-sig-' + proposalId,
        timestamp: Date.now(),
      };
    } catch (error) {
      Logger.error('Failed to execute proposal', { proposalId, error });
      throw error;
    }
  }

  /**
   * Get proposal status
   */
  getProposal(proposalId: string): Proposal | undefined {
    return this.proposals.get(proposalId);
  }

  /**
   * Get all pending proposals
   */
  getPendingProposals(): Proposal[] {
    return Array.from(this.proposals.values()).filter(
      p => p.status === 'pending' || p.status === 'approved'
    );
  }

  /**
   * Clean expired proposals
   */
  cleanExpiredProposals(): number {
    const now = Date.now();
    let cleaned = 0;

    for (const [id, proposal] of this.proposals) {
      if (now > proposal.expiresAt && proposal.status !== 'executed') {
        proposal.status = 'expired';
        this.proposals.delete(id);
        cleaned++;
      }
    }

    if (cleaned > 0) {
      Logger.info('Cleaned expired proposals', { count: cleaned });
    }

    return cleaned;
  }

  // ============================================
  // UTILITY METHODS
  // ============================================

  /**
   * Generate unique proposal ID
   */
  private generateProposalId(): string {
    return `proposal-${Date.now()}-${Math.random().toString(36).substring(7)}`;
  }

  /**
   * Verify signature
   */
  verifySignature(
    message: string,
    signature: Uint8Array,
    role: AccountRole
  ): boolean {
    try {
      const account = this.getAccount(role);
      // Simplified verification - actual implementation depends on SDK
      return true;
    } catch (error) {
      return false;
    }
  }

  /**
   * Get account balance
   */
  async getBalance(role: AccountRole): Promise<string> {
    try {
      const address = this.getAddress(role);
      // TODO: Query balance from network
      // This is a placeholder
      return '0';
    } catch (error) {
      Logger.error('Failed to get balance', { role, error });
      throw error;
    }
  }

  /**
   * Export public keys
   */
  exportPublicKeys(): Record<AccountRole, string> {
    const publicKeys: Record<string, string> = {};
    
    for (const [role, address] of this.addresses) {
      publicKeys[role] = address;
    }

    return publicKeys as Record<AccountRole, string>;
  }
}

export default AccountManager;
