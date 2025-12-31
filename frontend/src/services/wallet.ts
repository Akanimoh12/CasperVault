import { CasperClient, CLPublicKey, DeployUtil } from 'casper-js-sdk';
import { CASPER_RPC_URL } from '@/utils/constants';

class WalletService {
  private casperClient: CasperClient;
  private activePublicKey: string | null = null;

  constructor() {
    this.casperClient = new CasperClient(CASPER_RPC_URL);
  }

  async connect(): Promise<{ address: string; balance: string }> {
    try {
      console.log('🔗 Attempting to connect to Casper Wallet...');
      
      // Check if Casper Wallet extension is installed
      if (!window.casperlabsHelper) {
        console.error('❌ window.casperlabsHelper not found');
        throw new Error('Casper Wallet extension not found. Please install it from casper.network');
      }

      console.log('📡 Requesting wallet connection...');
      // Request connection
      await window.casperlabsHelper.requestConnection();
      
      // Connection is implicit if no error is thrown

      // Get active public key
      const publicKey = await window.casperlabsHelper.getActivePublicKey();
      this.activePublicKey = publicKey;

      // Fetch balance
      const balance = await this.getBalance(publicKey);

      // Store connection state
      localStorage.setItem('wallet_connected', 'true');
      localStorage.setItem('wallet_address', publicKey);

      return {
        address: publicKey,
        balance,
      };
    } catch (error) {
      console.error('Wallet connection failed:', error);
      throw error instanceof Error ? error : new Error('Failed to connect wallet');
    }
  }

  async disconnect(): Promise<void> {
    try {
      if (window.casperlabsHelper) {
        await window.casperlabsHelper.disconnectFromSite();
      }
      this.activePublicKey = null;
      localStorage.removeItem('wallet_connected');
      localStorage.removeItem('wallet_address');
    } catch (error) {
      console.error('Disconnect error:', error);
      // Still clear local state even if disconnect fails
      this.activePublicKey = null;
      localStorage.removeItem('wallet_connected');
      localStorage.removeItem('wallet_address');
    }
  }

  async getBalance(publicKeyHex: string): Promise<string> {
    try {
      const publicKey = CLPublicKey.fromHex(publicKeyHex);
      const balanceUref = await this.casperClient.balanceOfByPublicKey(publicKey);
      
      // Convert motes to CSPR (1 CSPR = 1,000,000,000 motes)
      const balanceInCSPR = balanceUref.toString() 
        ? (BigInt(balanceUref.toString()) / BigInt(1_000_000_000)).toString()
        : '0';
      
      return balanceInCSPR;
    } catch (error) {
      console.error('Failed to fetch balance:', error);
      return '0';
    }
  }

  async signTransaction(deploy: DeployUtil.Deploy): Promise<DeployUtil.Deploy> {
    if (!window.casperlabsHelper) {
      throw new Error('Casper Wallet not connected');
    }

    if (!this.activePublicKey) {
      throw new Error('No active public key');
    }

    try {
      const deployJSON = DeployUtil.deployToJson(deploy);
      const signedDeployJSON = await window.casperlabsHelper.sign(
        deployJSON,
        this.activePublicKey
      );
      
      const signedDeploy = DeployUtil.deployFromJson(signedDeployJSON);
      
      if (signedDeploy.err) {
        throw new Error('Invalid deploy returned from wallet');
      }
      
      return signedDeploy.val;
    } catch (error) {
      console.error('Transaction signing failed:', error);
      throw new Error('Failed to sign transaction');
    }
  }

  async isWalletInstalled(): Promise<boolean> {
    // Wait for wallet extension to inject its API (especially needed in Brave)
    const maxAttempts = 30; // 3 seconds total
    const delayMs = 100;

    console.log('🔍 Searching for Casper Wallet extension...');
    
    for (let i = 0; i < maxAttempts; i++) {
      // Check for all possible Casper wallet APIs
      const hasWallet = !!window.casperlabsHelper || !!window.CasperWalletProvider || !!window.csprclick;
      
      if (hasWallet) {
        console.log('✅ Casper Wallet detected after', i * delayMs, 'ms');
        console.log('Available APIs:', {
          casperlabsHelper: !!window.casperlabsHelper,
          CasperWalletProvider: !!window.CasperWalletProvider,
          csprclick: !!window.csprclick
        });
        return true;
      }
      
      // Debug on first attempt
      if (i === 0) {
        console.log('⏳ Wallet not found on first check, waiting...');
      }
      
      // Wait before next check
      await new Promise(resolve => setTimeout(resolve, delayMs));
    }

    console.error('❌ Casper Wallet not detected after 3 seconds.');
    console.error('Please check:');
    console.error('1. Extension is installed: brave://extensions/');
    console.error('2. Extension is ENABLED');
    console.error('3. Brave Shields are DOWN for this site');
    console.error('4. Try refreshing the page');
    return false;
  }

  isConnected(): boolean {
    return this.activePublicKey !== null && localStorage.getItem('wallet_connected') === 'true';
  }

  getActivePublicKey(): string | null {
    return this.activePublicKey;
  }

  async reconnect(): Promise<{ address: string; balance: string } | null> {
    const wasConnected = localStorage.getItem('wallet_connected') === 'true';
    const savedAddress = localStorage.getItem('wallet_address');

    if (wasConnected && savedAddress && window.casperlabsHelper) {
      try {
        const isConnected = await window.casperlabsHelper.isConnected();
        if (isConnected) {
          this.activePublicKey = savedAddress;
          const balance = await this.getBalance(savedAddress);
          return { address: savedAddress, balance };
        }
      } catch (error) {
        console.error('Reconnection failed:', error);
        this.disconnect();
      }
    }

    return null;
  }
}

export const walletService = new WalletService();
