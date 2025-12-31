// Wallet Types
export interface WalletInfo {
  address: string;
  publicKey: string;
  balance: string;
  isConnected: boolean;
}

export interface WalletState {
  wallet: WalletInfo | null;
  isConnecting: boolean;
  error: string | null;
}

export type WalletProvider = 'casper-signer' | 'casper-wallet' | 'ledger';

export interface SignMessageRequest {
  message: string;
  address: string;
}

export interface SignTransactionRequest {
  deploy: any;
  address: string;
}
