import { useState } from 'react';
import { motion } from 'framer-motion';
import { MdAccountBalanceWallet } from 'react-icons/md';
import { useWalletStore } from '@/store/walletStore';
import { WalletModal } from './WalletModal.tsx';
import { formatAddress, formatCSPR } from '@/utils/format';

export const WalletButton = () => {
  const [showModal, setShowModal] = useState(false);
  const { address, balance, isConnected, isConnecting, connect, disconnect } = useWalletStore();

  const handleConnect = async () => {
    try {
      await connect();
    } catch (error) {
      console.error('Connection failed:', error);
    }
  };

  if (isConnected && address) {
    return (
      <>
        <motion.button
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.98 }}
          onClick={() => setShowModal(true)}
          className="flex items-center gap-3 px-4 py-2 rounded-xl bg-primary-50 border-2 border-primary-500 hover:bg-primary-100 transition-all"
        >
          <div className="flex flex-col items-end">
            <span className="text-sm font-semibold text-gray-900">
              {formatCSPR(balance)} CSPR
            </span>
            <span className="text-xs text-gray-500">
              {formatAddress(address)}
            </span>
          </div>
          <div className="w-8 h-8 rounded-full bg-primary-500 flex items-center justify-center">
            <MdAccountBalanceWallet className="text-white" />
          </div>
        </motion.button>

        <WalletModal
          isOpen={showModal}
          onClose={() => setShowModal(false)}
          address={address}
          balance={balance}
          onDisconnect={disconnect}
        />
      </>
    );
  }

  return (
    <motion.button
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      onClick={handleConnect}
      disabled={isConnecting}
      className="flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-primary-500 to-accent-500 text-white rounded-xl font-medium shadow-md hover:shadow-lg transition-shadow disabled:opacity-50 disabled:cursor-not-allowed"
    >
      {isConnecting ? (
        <>
          <svg className="animate-spin h-5 w-5" viewBox="0 0 24 24">
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
              fill="none"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
          <span>Connecting...</span>
        </>
      ) : (
        <>
          <MdAccountBalanceWallet className="w-5 h-5" />
          <span>Connect Wallet</span>
        </>
      )}
    </motion.button>
  );
};

