import { 
  CasperClient, 
  CLPublicKey, 
  DeployUtil, 
  RuntimeArgs, 
  CLValueBuilder,
  Contracts
} from 'casper-js-sdk';
import { CASPER_RPC_URL, CASPER_CHAIN_NAME, VAULT_CONTRACT_HASH } from '@/utils/constants';

// Contract entry points
const EP_DEPOSIT = 'deposit';
const EP_WITHDRAW = 'withdraw';

// Gas costs in motes (1 CSPR = 1,000,000,000 motes)
const DEPOSIT_GAS = '3000000000'; // 3 CSPR
const WITHDRAW_GAS = '3000000000'; // 3 CSPR
const MOTES_PER_CSPR = 1_000_000_000n;

// Helper to get wallet provider
function getWalletProvider() {
  if (window.casperlabsHelper) {
    return window.casperlabsHelper;
  }
  if (window.CasperWalletProvider) {
    return window.CasperWalletProvider();
  }
  if (window.csprclick) {
    return window.csprclick;
  }
  return null;
}

class ContractService {
  private casperClient: CasperClient;
  private contractClient: Contracts.Contract;

  constructor() {
    this.casperClient = new CasperClient(CASPER_RPC_URL);
    this.contractClient = new Contracts.Contract(this.casperClient);
    
    // Set contract hash if available
    if (VAULT_CONTRACT_HASH) {
      this.contractClient.setContractHash(VAULT_CONTRACT_HASH);
    }
  }

  /**
   * Convert CSPR to motes
   */
  private csprToMotes(cspr: string): bigint {
    const amount = parseFloat(cspr);
    return BigInt(Math.floor(amount * Number(MOTES_PER_CSPR)));
  }

  /**
   * Convert motes to CSPR
   */
  private motesToCspr(motes: bigint): string {
    return (Number(motes) / Number(MOTES_PER_CSPR)).toFixed(4);
  }

  /**
   * Create a deposit deploy
   */
  async createDepositDeploy(publicKeyHex: string, amountCspr: string): Promise<DeployUtil.Deploy> {
    const publicKey = CLPublicKey.fromHex(publicKeyHex);
    const amountMotes = this.csprToMotes(amountCspr);

    // Build runtime args with amount
    const args = RuntimeArgs.fromMap({
      amount: CLValueBuilder.u512(amountMotes.toString()),
    });

    // Create deploy
    const deploy = DeployUtil.makeDeploy(
      new DeployUtil.DeployParams(
        publicKey,
        CASPER_CHAIN_NAME,
        1, // gasPrice
        1800000 // ttl: 30 minutes
      ),
      DeployUtil.ExecutableDeployItem.newStoredContractByHash(
        Uint8Array.from(Buffer.from(VAULT_CONTRACT_HASH.replace('hash-', ''), 'hex')),
        EP_DEPOSIT,
        args
      ),
      DeployUtil.standardPayment(DEPOSIT_GAS)
    );

    return deploy;
  }

  /**
   * Create a withdraw deploy
   */
  async createWithdrawDeploy(publicKeyHex: string, amountCspr: string): Promise<DeployUtil.Deploy> {
    const publicKey = CLPublicKey.fromHex(publicKeyHex);
    const amountMotes = this.csprToMotes(amountCspr);

    const args = RuntimeArgs.fromMap({
      amount: CLValueBuilder.u512(amountMotes.toString()),
    });

    const deploy = DeployUtil.makeDeploy(
      new DeployUtil.DeployParams(
        publicKey,
        CASPER_CHAIN_NAME,
        1,
        1800000
      ),
      DeployUtil.ExecutableDeployItem.newStoredContractByHash(
        Uint8Array.from(Buffer.from(VAULT_CONTRACT_HASH.replace('hash-', ''), 'hex')),
        EP_WITHDRAW,
        args
      ),
      DeployUtil.standardPayment(WITHDRAW_GAS)
    );

    return deploy;
  }

  /**
   * Sign and submit a deploy via wallet
   */
  async signAndSubmitDeploy(deploy: DeployUtil.Deploy, publicKeyHex: string): Promise<string> {
    const walletProvider = getWalletProvider();
    
    if (!walletProvider) {
      throw new Error('Wallet not connected');
    }

    // Convert deploy to JSON for wallet signing
    const deployJSON = DeployUtil.deployToJson(deploy);
    
    // Request signature from wallet
    const signedDeployJSON = await walletProvider.sign(
      JSON.stringify(deployJSON),
      publicKeyHex
    );

    // Parse the signed deploy
    const signedDeploy = DeployUtil.deployFromJson(JSON.parse(signedDeployJSON));
    
    if (signedDeploy.err) {
      throw new Error('Failed to parse signed deploy');
    }

    // Submit to network
    const deployHash = await this.casperClient.putDeploy(signedDeploy.val);
    
    return deployHash;
  }

  /**
   * Deposit CSPR into the vault
   */
  async deposit(publicKeyHex: string, amountCspr: string): Promise<{ deployHash: string; amount: string }> {
    console.log(`📥 Creating deposit for ${amountCspr} CSPR...`);
    
    const deploy = await this.createDepositDeploy(publicKeyHex, amountCspr);
    const deployHash = await this.signAndSubmitDeploy(deploy, publicKeyHex);
    
    console.log(`✅ Deposit submitted. Deploy hash: ${deployHash}`);
    
    // Store transaction in localStorage for activity tracking
    this.storeTransaction({
      type: 'Deposit',
      amount: amountCspr,
      deployHash,
      timestamp: Date.now(),
      user: publicKeyHex.slice(0, 8) + '...' + publicKeyHex.slice(-6),
      status: 'pending'
    });
    
    return { deployHash, amount: amountCspr };
  }

  /**
   * Withdraw CSPR from the vault
   */
  async withdraw(publicKeyHex: string, amountCspr: string): Promise<{ deployHash: string; amount: string }> {
    console.log(`📤 Creating withdrawal for ${amountCspr} CSPR...`);
    
    const deploy = await this.createWithdrawDeploy(publicKeyHex, amountCspr);
    const deployHash = await this.signAndSubmitDeploy(deploy, publicKeyHex);
    
    console.log(`✅ Withdrawal submitted. Deploy hash: ${deployHash}`);
    
    // Store transaction
    this.storeTransaction({
      type: 'Withdraw',
      amount: amountCspr,
      deployHash,
      timestamp: Date.now(),
      user: publicKeyHex.slice(0, 8) + '...' + publicKeyHex.slice(-6),
      status: 'pending'
    });
    
    return { deployHash, amount: amountCspr };
  }

  /**
   * Get vault balance (total deposited)
   */
  async getVaultBalance(): Promise<string> {
    try {
      if (!VAULT_CONTRACT_HASH) {
        return '0';
      }

      // Query the contract's named key for total_deposited
      const stateRootHash = await this.casperClient.nodeClient.getStateRootHash();
      
      const result = await this.casperClient.nodeClient.getBlockState(
        stateRootHash,
        VAULT_CONTRACT_HASH,
        ['total_deposited']
      );

      if (result && result.CLValue) {
        const motes = BigInt(result.CLValue.data.toString());
        return this.motesToCspr(motes);
      }
      
      return '0';
    } catch (error) {
      console.error('Failed to get vault balance:', error);
      return '0';
    }
  }

  /**
   * Get deploy status
   */
  async getDeployStatus(deployHash: string): Promise<{ status: string; cost?: string }> {
    try {
      const result = await this.casperClient.getDeploy(deployHash);
      
      if (result && result[1]) {
        const executionResults = result[1].execution_results;
        if (executionResults && executionResults.length > 0) {
          const execResult = executionResults[0].result;
          if ('Success' in execResult && execResult.Success) {
            return { status: 'success', cost: String(execResult.Success.cost) };
          } else if ('Failure' in execResult) {
            return { status: 'failed' };
          }
        }
      }
      
      return { status: 'pending' };
    } catch (error) {
      return { status: 'unknown' };
    }
  }

  /**
   * Store transaction in localStorage
   */
  private storeTransaction(tx: {
    type: string;
    amount: string;
    deployHash: string;
    timestamp: number;
    user: string;
    status: string;
  }) {
    const key = 'caspervault_transactions';
    const existing = localStorage.getItem(key);
    const transactions = existing ? JSON.parse(existing) : [];
    
    transactions.unshift({
      id: tx.deployHash,
      ...tx,
      txHash: tx.deployHash
    });
    
    // Keep only last 50 transactions
    localStorage.setItem(key, JSON.stringify(transactions.slice(0, 50)));
  }

  /**
   * Get stored transactions
   */
  getStoredTransactions(): Array<{
    id: string;
    type: 'Deposit' | 'Withdraw';
    amount: string;
    deployHash: string;
    txHash: string;
    timestamp: number;
    user: string;
    status: string;
  }> {
    const key = 'caspervault_transactions';
    const existing = localStorage.getItem(key);
    return existing ? JSON.parse(existing) : [];
  }

  /**
   * Update transaction status
   */
  async updateTransactionStatuses(): Promise<void> {
    const transactions = this.getStoredTransactions();
    let updated = false;

    for (const tx of transactions) {
      if (tx.status === 'pending') {
        const status = await this.getDeployStatus(tx.deployHash);
        if (status.status !== 'pending') {
          tx.status = status.status;
          updated = true;
        }
      }
    }

    if (updated) {
      localStorage.setItem('caspervault_transactions', JSON.stringify(transactions));
    }
  }

  /**
   * Get user's deposit history from blockchain
   */
  async getUserDeposits(publicKeyHex: string): Promise<Array<{
    deployHash: string;
    amount: string;
    timestamp: number;
    type: string;
  }>> {
    // For now, return stored transactions filtered by user
    const allTx = this.getStoredTransactions();
    const userShort = publicKeyHex.slice(0, 8) + '...' + publicKeyHex.slice(-6);
    return allTx.filter(tx => tx.user === userShort);
  }
}

export const contractService = new ContractService();
