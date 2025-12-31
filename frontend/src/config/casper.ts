import { CasperClient, CLPublicKey } from 'casper-js-sdk';
import {
  CASPER_RPC_URL,
  CASPER_CHAIN_NAME,
  VAULT_CONTRACT_HASH,
  STAKING_CONTRACT_HASH,
  STRATEGY_CONTRACT_HASH,
} from '../utils/constants';

// Casper Client
export const casperClient = new CasperClient(CASPER_RPC_URL);

// Contract Configuration
export const contracts = {
  vault: {
    hash: VAULT_CONTRACT_HASH,
    packageHash: '',
  },
  staking: {
    hash: STAKING_CONTRACT_HASH,
    packageHash: '',
  },
  strategy: {
    hash: STRATEGY_CONTRACT_HASH,
    packageHash: '',
  },
};

// Network Configuration
export const networkConfig = {
  chainName: CASPER_CHAIN_NAME,
  rpcUrl: CASPER_RPC_URL,
};

// Helper: Create CLPublicKey from hex string
export const createPublicKey = (hexString: string): CLPublicKey => {
  return CLPublicKey.fromHex(hexString);
};

// Helper: Get contract hash bytes
export const getContractHashBytes = (hash: string): Uint8Array => {
  // Remove 'hash-' prefix if present
  const cleanHash = hash.replace('hash-', '');
  return Uint8Array.from(Buffer.from(cleanHash, 'hex'));
};

// Deploy Status Polling
export const pollDeployStatus = async (
  deployHash: string,
  maxAttempts = 30,
  interval = 5000
): Promise<any> => {
  for (let i = 0; i < maxAttempts; i++) {
    try {
      const [deploy, raw] = await casperClient.getDeploy(deployHash);
      
      if (raw.execution_results.length > 0) {
        const result = raw.execution_results[0].result;
        
        if (result.Success) {
          return { success: true, deploy, result };
        } else if (result.Failure) {
          return { success: false, deploy, error: result.Failure };
        }
      }
      
      // Wait before next attempt
      await new Promise((resolve) => setTimeout(resolve, interval));
    } catch (error) {
      console.error(`Attempt ${i + 1} failed:`, error);
      
      if (i === maxAttempts - 1) {
        throw new Error('Max polling attempts reached');
      }
      
      await new Promise((resolve) => setTimeout(resolve, interval));
    }
  }
  
  throw new Error('Deploy status could not be determined');
};

// Gas Price Estimation
export const estimateGasPrice = async (): Promise<number> => {
  // For testnet, use fixed gas price
  // For mainnet, implement dynamic gas price estimation
  return 1; // 1 mote per gas
};

// Validate Deploy
export const validateDeploy = (deploy: any): boolean => {
  try {
    // Basic validation
    if (!deploy.header) return false;
    if (!deploy.payment) return false;
    if (!deploy.session) return false;
    if (!deploy.approvals || deploy.approvals.length === 0) return false;
    
    return true;
  } catch {
    return false;
  }
};

export default {
  client: casperClient,
  contracts,
  networkConfig,
  createPublicKey,
  getContractHashBytes,
  pollDeployStatus,
  estimateGasPrice,
  validateDeploy,
};
