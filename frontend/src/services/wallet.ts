import { CasperClient, CLPublicKey, DeployUtil, CasperServiceByJsonRPC } from 'casper-js-sdk';
import { CASPER_RPC_URL } from '@/utils/constants';

// Helper to get the wallet provider
function getWalletProvider() {
  if (window.casperlabsHelper) {
    return window.casperlabsHelper;
  }
  if (window.CasperWalletProvider) {
    // CasperWalletProvider is a function that returns the provider object
    return window.CasperWalletProvider();
  }
  if (window.csprclick) {
    return window.csprclick;
  }
  return null;
}

class WalletService {
  private casperClient: CasperClient;
  private rpcService: CasperServiceByJsonRPC;
  private activePublicKey: string | null = null;
  private walletProvider: any = null;

  constructor() {
    console.log('🔧 Initializing Casper Wallet Service with RPC:', CASPER_RPC_URL);
    this.casperClient = new CasperClient(CASPER_RPC_URL);
    this.rpcService = new CasperServiceByJsonRPC(CASPER_RPC_URL);
  }

  async connect(): Promise<{ address: string; balance: string }> {
    try {
      console.log('🔗 Attempting to connect to Casper Wallet...');
      
      // Get the wallet provider
      this.walletProvider = getWalletProvider();
      
      if (!this.walletProvider) {
        console.error('❌ No wallet provider found');
        throw new Error('Casper Wallet extension not found. Please install it from casper.network');
      }

      console.log('📡 Requesting wallet connection...');
      // Request connection
      await this.walletProvider.requestConnection();
      
      console.log('✅ Connection accepted, getting active key...');

      // Get active public key
      const publicKey = await this.walletProvider.getActivePublicKey();
      this.activePublicKey = publicKey;
      
      console.log('🔑 Public Key:', publicKey);

      // Fetch balance
      const balance = await this.getBalance(publicKey);
      console.log('💰 Balance:', balance, 'CSPR');

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
      if (this.walletProvider && this.walletProvider.disconnectFromSite) {
        await this.walletProvider.disconnectFromSite();
      }
      this.activePublicKey = null;
      this.walletProvider = null;
      localStorage.removeItem('wallet_connected');
      localStorage.removeItem('wallet_address');
    } catch (error) {
      console.error('Disconnect error:', error);
      // Still clear local state even if disconnect fails
      this.activePublicKey = null;
      this.walletProvider = null;
      localStorage.removeItem('wallet_connected');
      localStorage.removeItem('wallet_address');
    }
  }

  async getBalance(publicKeyHex: string): Promise<string> {
    console.log('🔍 Fetching balance for:', publicKeyHex);
    console.log('🌐 Using RPC URL:', CASPER_RPC_URL);
    
    try {
      const publicKey = CLPublicKey.fromHex(publicKeyHex);
      console.log('✅ Parsed public key successfully');
      
      // Method 1: Try using CasperServiceByJsonRPC directly for state root hash
      const latestBlock = await this.rpcService.getLatestBlockInfo();
      console.log('📦 Latest block info:', latestBlock?.block?.header?.height);
      
      if (!latestBlock?.block?.header?.state_root_hash) {
        console.error('❌ Could not get state root hash from latest block');
        return '0';
      }
      
      const stateRootHash = latestBlock.block.header.state_root_hash;
      console.log('🔑 State root hash:', stateRootHash);
      
      // Get account hash
      const accountHash = publicKey.toAccountHashStr();
      console.log('👤 Account hash:', accountHash);
      
      // Query the balance using getAccountBalance
      try {
        const balanceUref = await this.rpcService.getAccountBalanceUrefByPublicKey(
          stateRootHash,
          publicKey
        );
        console.log('💰 Balance URef:', balanceUref);
        
        if (balanceUref) {
          const balance = await this.rpcService.getAccountBalance(stateRootHash, balanceUref);
          console.log('💵 Raw balance (motes):', balance?.toString());
          
          if (balance) {
            // Convert motes to CSPR (1 CSPR = 1,000,000,000 motes)
            const balanceInCSPR = (BigInt(balance.toString()) / BigInt(1_000_000_000)).toString();
            console.log('✅ Balance in CSPR:', balanceInCSPR);
            return balanceInCSPR;
          }
        }
      } catch (balanceError) {
        console.warn('⚠️ Could not get balance via URef method:', balanceError);
      }
      
      // Fallback: Try direct method
      try {
        const balance = await this.casperClient.balanceOfByPublicKey(publicKey);
        console.log('💵 Fallback balance (motes):', balance?.toString());
        
        if (balance) {
          const balanceInCSPR = (BigInt(balance.toString()) / BigInt(1_000_000_000)).toString();
          console.log('✅ Fallback balance in CSPR:', balanceInCSPR);
          return balanceInCSPR;
        }
      } catch (fallbackError) {
        console.warn('⚠️ Fallback balance method failed:', fallbackError);
      }
      
      return '0';
    } catch (error) {
      console.error('❌ Failed to fetch balance:', error);
      return '0';
    }
  }

  async signTransaction(deploy: DeployUtil.Deploy): Promise<DeployUtil.Deploy> {
    if (!this.walletProvider) {
      throw new Error('Wallet not connected');
    }

    if (!this.activePublicKey) {
      throw new Error('No active public key');
    }

    try {
      const deployJSON = DeployUtil.deployToJson(deploy);
      const signedDeployJSON = await this.walletProvider.sign(
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
        
        // Log the actual objects for debugging
        if (window.casperlabsHelper) {
          console.log('casperlabsHelper methods:', Object.keys(window.casperlabsHelper));
        }
        if (window.CasperWalletProvider) {
          console.log('CasperWalletProvider:', window.CasperWalletProvider);
        }
        if (window.csprclick) {
          console.log('csprclick:', window.csprclick);
        }
        
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

    if (wasConnected && savedAddress) {
      this.walletProvider = getWalletProvider();
      
      if (this.walletProvider) {
        try {
          const isConnected = await this.walletProvider.isConnected();
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
    }

    return null;
  }
}

export const walletService = new WalletService();
