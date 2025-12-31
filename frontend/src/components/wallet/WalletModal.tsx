import { Dialog, Transition } from '@headlessui/react';
import { Fragment } from 'react';
import { MdClose, MdContentCopy, MdExitToApp, MdRefresh } from 'react-icons/md';
import { motion } from 'framer-motion';
import toast from 'react-hot-toast';
import { formatCSPR } from '@/utils/format';
import { useWalletStore } from '@/store/walletStore';

interface WalletModalProps {
  isOpen: boolean;
  onClose: () => void;
  address: string;
  balance: string;
  onDisconnect: () => void;
}

export const WalletModal = ({
  isOpen,
  onClose,
  address,
  balance,
  onDisconnect,
}: WalletModalProps) => {
  const { updateBalance } = useWalletStore();

  const copyAddress = () => {
    navigator.clipboard.writeText(address);
    toast.success('Address copied to clipboard!');
  };

  const handleDisconnect = () => {
    onDisconnect();
    onClose();
  };

  const handleRefreshBalance = () => {
    // Call updateBalance with empty string to trigger refresh
    updateBalance('');
    toast.success('Balance refreshing...');
  };

  return (
    <Transition appear show={isOpen} as={Fragment}>
      <Dialog as="div" className="relative z-50" onClose={onClose}>
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-black/30 backdrop-blur-sm" />
        </Transition.Child>

        <div className="fixed inset-0 overflow-y-auto">
          <div className="flex min-h-full items-center justify-center p-4">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 scale-95"
              enterTo="opacity-100 scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 scale-100"
              leaveTo="opacity-0 scale-95"
            >
              <Dialog.Panel className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 shadow-xl transition-all">
                {/* Header */}
                <div className="flex items-center justify-between mb-6">
                  <Dialog.Title className="text-xl font-bold text-gray-900">
                    Your Wallet
                  </Dialog.Title>
                  <button
                    onClick={onClose}
                    className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                  >
                    <MdClose className="text-xl text-gray-500" />
                  </button>
                </div>

                {/* Balance Card */}
                <div className="card bg-gradient-to-br from-primary-50 to-accent-50 border-2 border-primary-200 mb-4">
                  <div className="flex items-start justify-between mb-2">
                    <p className="text-sm text-gray-600">Balance</p>
                    <motion.button
                      whileHover={{ scale: 1.1, rotate: 180 }}
                      whileTap={{ scale: 0.9 }}
                      onClick={handleRefreshBalance}
                      className="p-1 rounded-lg hover:bg-primary-100 transition-colors"
                      title="Refresh balance"
                    >
                      <MdRefresh className="text-lg text-primary-600" />
                    </motion.button>
                  </div>
                  <p className="text-3xl font-bold text-gray-900">
                    {formatCSPR(balance)} <span className="text-xl">CSPR</span>
                  </p>
                </div>

                {/* Address Card */}
                <div className="card bg-gray-50 mb-6">
                  <p className="text-sm text-gray-600 mb-2">Address</p>
                  <div className="flex items-center gap-2">
                    <code className="flex-1 text-sm font-mono text-gray-900 truncate">
                      {address}
                    </code>
                    <motion.button
                      whileHover={{ scale: 1.1 }}
                      whileTap={{ scale: 0.9 }}
                      onClick={copyAddress}
                      className="p-2 rounded-lg bg-white hover:bg-gray-200 transition-colors"
                      title="Copy address"
                    >
                      <MdContentCopy className="text-primary-500" />
                    </motion.button>
                  </div>
                </div>

                {/* Network Info */}
                <div className="flex items-center justify-between mb-6 px-4 py-3 bg-gray-50 rounded-xl">
                  <span className="text-sm text-gray-600">Network</span>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-success-500 animate-pulse" />
                    <span className="text-sm font-medium text-gray-900">Casper Testnet</span>
                  </div>
                </div>

                {/* Disconnect Button */}
                <motion.button
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                  onClick={handleDisconnect}
                  className="w-full flex items-center justify-center gap-2 px-6 py-3 bg-danger-500 text-white rounded-xl font-medium hover:bg-danger-600 transition-colors"
                >
                  <MdExitToApp className="text-xl" />
                  <span>Disconnect Wallet</span>
                </motion.button>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition>
  );
};
