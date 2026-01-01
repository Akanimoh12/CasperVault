import { useState } from 'react';
import { Dialog, Transition } from '@headlessui/react';
import { Fragment } from 'react';
import { MdClose, MdInfo } from 'react-icons/md';
import toast from 'react-hot-toast';
import { Button } from '../common/Button';
import { useWalletStore } from '@/store/walletStore';
import { useVault } from '@/hooks/useVault';
import { formatCSPR } from '@/utils/format';

interface DepositModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const DepositModal = ({ isOpen, onClose }: DepositModalProps) => {
  const [amount, setAmount] = useState('');
  const [loading, setLoading] = useState(false);
  const { balance } = useWalletStore();
  const { deposit, estimateShares } = useVault();
  
  const handleDeposit = async () => {
    if (!amount || parseFloat(amount) <= 0) {
      toast.error('Please enter a valid amount');
      return;
    }
    
    if (parseFloat(amount) > parseFloat(balance)) {
      toast.error('Insufficient balance');
      return;
    }
    
    setLoading(true);
    try {
      await deposit(amount);
      toast.success('Deposit successful!');
      onClose();
      setAmount('');
    } catch (error) {
      toast.error('Deposit failed');
      console.error(error);
    } finally {
      setLoading(false);
    }
  };
  
  const maxAmount = () => {
    // Leave 1 CSPR for gas
    const max = Math.max(0, parseFloat(balance) - 1);
    setAmount(max.toString());
  };
  
  const estimatedShares = estimateShares(amount || '0');
  
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
                  <Dialog.Title className="text-2xl font-bold text-gray-900">
                    Deposit CSPR
                  </Dialog.Title>
                  <button
                    onClick={onClose}
                    className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                  >
                    <MdClose className="text-xl text-gray-500" />
                  </button>
                </div>
                
                {/* Balance */}
                <div className="p-4 bg-gray-50 rounded-xl mb-6">
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-gray-600">Your Balance</span>
                    <span className="font-semibold text-gray-900">
                      {formatCSPR(balance)} CSPR
                    </span>
                  </div>
                </div>
                
                {/* Amount Input */}
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Amount to Deposit
                  </label>
                  <div className="relative">
                    <input
                      type="number"
                      value={amount}
                      onChange={(e) => setAmount(e.target.value)}
                      placeholder="0.00"
                      className="w-full px-4 py-4 text-2xl font-bold rounded-xl border-2 border-gray-200 focus:border-primary-500 focus:outline-none transition-colors pr-24"
                    />
                    <div className="absolute right-4 top-1/2 -translate-y-1/2 flex items-center gap-2">
                      <span className="text-gray-500 font-medium">CSPR</span>
                      <button
                        onClick={maxAmount}
                        className="px-3 py-1 rounded-lg bg-primary-100 text-primary-700 text-sm font-semibold hover:bg-primary-200 transition-colors"
                      >
                        MAX
                      </button>
                    </div>
                  </div>
                </div>
                
                {/* Info Cards */}
                <div className="space-y-3 mb-6">
                  <div className="flex items-center justify-between p-3 rounded-xl bg-primary-50">
                    <span className="text-sm text-gray-600">You will receive</span>
                    <span className="font-semibold text-gray-900">
                      {formatCSPR(estimatedShares)} cvCSPR
                    </span>
                  </div>
                  
                  <div className="flex items-center justify-between p-3 rounded-xl bg-success-50">
                    <span className="text-sm text-gray-600">Current APY</span>
                    <span className="font-semibold text-success-700">12.5%</span>
                  </div>
                  
                  <div className="flex items-center justify-between p-3 rounded-xl bg-gray-50">
                    <span className="text-sm text-gray-600">Est. Gas Fee</span>
                    <span className="font-semibold text-gray-900">~0.5 CSPR</span>
                  </div>
                </div>
                
                {/* Info Banner */}
                <div className="flex gap-3 p-4 rounded-xl bg-blue-50 border border-blue-200 mb-6">
                  <MdInfo className="text-xl text-blue-500 flex-shrink-0 mt-0.5" />
                  <div className="text-sm text-blue-900">
                    <p className="font-semibold mb-1">How it works</p>
                    <p>
                      Your CSPR is automatically allocated across multiple yield strategies
                      and liquid staking. Earn compound interest daily!
                    </p>
                  </div>
                </div>
                
                {/* Action Buttons */}
                <div className="flex gap-3">
                  <Button
                    variant="secondary"
                    onClick={onClose}
                    className="flex-1"
                  >
                    Cancel
                  </Button>
                  <Button
                    variant="primary"
                    onClick={handleDeposit}
                    loading={loading}
                    disabled={!amount || parseFloat(amount) <= 0}
                    className="flex-1"
                  >
                    Deposit
                  </Button>
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition>
  );
};
