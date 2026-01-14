import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { walletService } from '@/services/wallet';
import toast from 'react-hot-toast';

interface WalletState {
  address: string | null;
  balance: string;
  isConnected: boolean;
  isConnecting: boolean;
  error: string | null;

  connect: () => Promise<void>;
  disconnect: () => Promise<void>;
  updateBalance: (balance?: string) => void;
  refreshBalance: () => Promise<void>;
  reconnect: () => Promise<void>;
}

export const useWalletStore = create<WalletState>()(
  persist(
    (set, get) => ({
      address: null,
      balance: '0',
      isConnected: false,
      isConnecting: false,
      error: null,

      connect: async () => {
        set({ isConnecting: true, error: null });
        
        try {
          // Check if wallet is installed (with retry mechanism for Brave/Chrome)
          const isInstalled = await walletService.isWalletInstalled();
          
          if (!isInstalled) {
            throw new Error(
              'Casper Wallet not detected. If you have it installed, please:\n' +
              '1. Refresh the page\n' +
              '2. Make sure the extension is enabled in your browser\n' +
              '3. In Brave: Check Settings → Extensions → Manage Extensions'
            );
          }

          // Connect wallet
          const { address, balance } = await walletService.connect();
          
          set({
            address,
            balance,
            isConnected: true,
            isConnecting: false,
            error: null,
          });

          toast.success('Wallet connected successfully!');
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : 'Failed to connect wallet';
          
          set({
            isConnecting: false,
            error: errorMessage,
          });

          toast.error(errorMessage);
          throw error;
        }
      },

      disconnect: async () => {
        try {
          await walletService.disconnect();
          
          set({
            address: null,
            balance: '0',
            isConnected: false,
            error: null,
          });

          toast.success('Wallet disconnected');
        } catch (error) {
          console.error('Disconnect error:', error);
          // Still clear state on error
          set({
            address: null,
            balance: '0',
            isConnected: false,
            error: null,
          });
        }
      },

      updateBalance: async (newBalance?: string) => {
        const { address } = get();
        
        if (!address) {
          return;
        }

        try {
          const balance = newBalance ?? await walletService.getBalance(address);
          set({ balance });
        } catch (error) {
          console.error('Failed to update balance:', error);
        }
      },

      refreshBalance: async () => {
        const { address } = get();
        
        if (!address) {
          return;
        }

        try {
          console.log('🔄 Refreshing wallet balance...');
          const balance = await walletService.getBalance(address);
          set({ balance });
          console.log('✅ Balance updated:', balance, 'CSPR');
        } catch (error) {
          console.error('Failed to refresh balance:', error);
        }
      },

      reconnect: async () => {
        try {
          const result = await walletService.reconnect();
          
          if (result) {
            set({
              address: result.address,
              balance: result.balance,
              isConnected: true,
              error: null,
            });
          } else {
            // Clear persisted state if reconnection failed
            set({
              address: null,
              balance: '0',
              isConnected: false,
            });
          }
        } catch (error) {
          console.error('Reconnection failed:', error);
          set({
            address: null,
            balance: '0',
            isConnected: false,
          });
        }
      },
    }),
    {
      name: 'wallet-storage',
      partialize: (state) => ({
        address: state.address,
        isConnected: state.isConnected,
      }),
    }
  )
);
