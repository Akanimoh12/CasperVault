import { useState } from 'react';
import { Dialog, Transition } from '@headlessui/react';
import { Fragment } from 'react';
import { MdClose, MdWarning, MdInfo } from 'react-icons/md';
import { motion } from 'framer-motion';
import toast from 'react-hot-toast';
import { Button } from '../common/Button';
import { useVault } from '@/hooks/useVault';
import { formatCSPR } from '@/utils/format';

interface WithdrawModalProps {
  isOpen: boolean;
  onClose: () => void;
  cvCSPRBalance: string;
}

export const WithdrawModal = ({ isOpen, onClose, cvCSPRBalance }: WithdrawModalProps) => {
  const [amount, setAmount] = useState('');
  const [instant, setInstant] = useState(false);
  const [loading, setLoading] = useState(false);
  const [txHash, setTxHash] = useState<string | null>(null);
  const { withdraw, instantWithdraw, estimateCSPR, isContractReady } = useVault();
  
  const handleWithdraw = async () => {
    if (!amount || parseFloat(amount) <= 0) {
      toast.error('Please enter a valid amount');
      return;
    }
    
    if (parseFloat(amount) > parseFloat(cvCSPRBalance)) {
      toast.error('Insufficient cvCSPR balance');
      return;
    }
    
    setLoading(true);
    setTxHash(null);
    try {
      let result;
      if (instant) {
        result = await instantWithdraw(amount);
        toast.success(
          <div>
            <p className="font-semibold">Instant withdrawal submitted!</p>
            <p className="text-sm">Funds will be available shortly.</p>
          </div>
        );
      } else {
        result = await withdraw(amount);
        toast.success(
          <div>
            <p className="font-semibold">Withdrawal initiated!</p>
            <p className="text-sm">Unlocking in 7 days.</p>
          </div>
        );
      }
      setTxHash(result.deployHash);
      setAmount('');
    } catch (error: any) {
      const errorMsg = error?.message || 'Withdrawal failed';
      toast.error(errorMsg);
      console.error(error);
    } finally {
      setLoading(false);
    }
  };
  
  const estimatedCSPR = parseFloat(estimateCSPR(amount || '0'));
  const instantFee = instant ? estimatedCSPR * 0.005 : 0; // 0.5% fee
  const finalAmount = estimatedCSPR - instantFee;
  
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
                    Withdraw CSPR
                  </Dialog.Title>
                  <button
                    onClick={onClose}
                    className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                  >
                    <MdClose className="text-xl text-gray-500" />
                  </button>
                </div>
                
                {/* cvCSPR Balance */}
                <div className="p-4 bg-gray-50 rounded-xl mb-6">
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-gray-600">Your cvCSPR Balance</span>
                    <span className="font-semibold text-gray-900">
                      {formatCSPR(cvCSPRBalance)} cvCSPR
                    </span>
                  </div>
                </div>
                
                {/* Withdrawal Type Selector */}
                <div className="grid grid-cols-2 gap-3 mb-6">
                  <motion.button
                    whileTap={{ scale: 0.98 }}
                    onClick={() => setInstant(false)}
                    className={`p-4 rounded-xl border-2 transition-all ${
                      !instant
                        ? 'border-primary-500 bg-primary-50'
                        : 'border-gray-200 bg-white hover:border-gray-300'
                    }`}
                  >
                    <p className="font-semibold text-gray-900 mb-1">Standard</p>
                    <p className="text-xs text-gray-500">7-day unlock</p>
                    <p className="text-sm font-semibold text-success-600 mt-2">No fee</p>
                  </motion.button>
                  
                  <motion.button
                    whileTap={{ scale: 0.98 }}
                    onClick={() => setInstant(true)}
                    className={`p-4 rounded-xl border-2 transition-all ${
                      instant
                        ? 'border-primary-500 bg-primary-50'
                        : 'border-gray-200 bg-white hover:border-gray-300'
                    }`}
                  >
                    <p className="font-semibold text-gray-900 mb-1">Instant</p>
                    <p className="text-xs text-gray-500">Immediate</p>
                    <p className="text-sm font-semibold text-warning-600 mt-2">0.5% fee</p>
                  </motion.button>
                </div>
                
                {/* Amount Input */}
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Amount to Withdraw
                  </label>
                  <div className="relative">
                    <input
                      type="number"
                      value={amount}
                      onChange={(e) => setAmount(e.target.value)}
                      placeholder="0.00"
                      className="w-full px-4 py-4 text-2xl font-bold rounded-xl border-2 border-gray-200 focus:border-primary-500 focus:outline-none transition-colors pr-32"
                    />
                    <div className="absolute right-4 top-1/2 -translate-y-1/2 flex items-center gap-2">
                      <span className="text-gray-500 font-medium">cvCSPR</span>
                      <button
                        onClick={() => setAmount(cvCSPRBalance)}
                        className="px-3 py-1 rounded-lg bg-primary-100 text-primary-700 text-sm font-semibold hover:bg-primary-200 transition-colors"
                      >
                        MAX
                      </button>
                    </div>
                  </div>
                </div>
                
                {/* Calculation Breakdown */}
                <div className="space-y-3 mb-6">
                  <div className="flex items-center justify-between p-3 rounded-xl bg-gray-50">
                    <span className="text-sm text-gray-600">Estimated CSPR</span>
                    <span className="font-semibold text-gray-900">
                      {formatCSPR(estimatedCSPR)} CSPR
                    </span>
                  </div>
                  
                  {instant && (
                    <div className="flex items-center justify-between p-3 rounded-xl bg-warning-50">
                      <span className="text-sm text-gray-600">Instant fee (0.5%)</span>
                      <span className="font-semibold text-warning-700">
                        -{formatCSPR(instantFee)} CSPR
                      </span>
                    </div>
                  )}
                  
                  <div className="flex items-center justify-between p-3 rounded-xl bg-primary-50 border-2 border-primary-200">
                    <span className="text-sm font-semibold text-gray-900">You will receive</span>
                    <span className="font-bold text-gray-900">
                      {formatCSPR(finalAmount)} CSPR
                    </span>
                  </div>
                </div>
                
                {/* Warning for standard withdrawal */}
                {!instant && !txHash && (
                  <div className="flex gap-3 p-4 rounded-xl bg-warning-50 border border-warning-200 mb-6">
                    <MdWarning className="text-xl text-warning-500 flex-shrink-0 mt-0.5" />
                    <div className="text-sm text-warning-900">
                      <p className="font-semibold mb-1">7-day unlocking period</p>
                      <p>
                        Your funds will be available for claim after 7 days. You won't earn
                        yields during this period.
                      </p>
                    </div>
                  </div>
                )}

                {/* Transaction Success */}
                {txHash && (
                  <div className="flex gap-3 p-4 rounded-xl bg-green-50 border border-green-200 mb-6">
                    <MdInfo className="text-xl text-green-500 flex-shrink-0 mt-0.5" />
                    <div className="text-sm text-green-900">
                      <p className="font-semibold mb-1">✅ Withdrawal Submitted!</p>
                      <a
                        href={`https://testnet.cspr.live/deploy/${txHash}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-green-700 underline hover:text-green-800"
                      >
                        View on Explorer →
                      </a>
                    </div>
                  </div>
                )}

                {/* Contract Not Ready Warning */}
                {!isContractReady && !txHash && (
                  <div className="flex gap-3 p-4 rounded-xl bg-yellow-50 border border-yellow-200 mb-6">
                    <MdWarning className="text-xl text-yellow-500 flex-shrink-0 mt-0.5" />
                    <div className="text-sm text-yellow-900">
                      <p className="font-semibold mb-1">Contract Not Deployed</p>
                      <p>
                        Transactions will be simulated for demo purposes.
                      </p>
                    </div>
                  </div>
                )}
                
                {/* Action Buttons */}
                <div className="flex gap-3">
                  <Button
                    variant="secondary"
                    onClick={() => { setTxHash(null); onClose(); }}
                    className="flex-1"
                  >
                    {txHash ? 'Close' : 'Cancel'}
                  </Button>
                  {!txHash && (
                    <Button
                      variant="primary"
                      onClick={handleWithdraw}
                      loading={loading}
                      disabled={!amount || parseFloat(amount) <= 0}
                      className="flex-1"
                    >
                      {instant ? 'Withdraw Now' : 'Start Unlock'}
                    </Button>
                  )}
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition>
  );
};
