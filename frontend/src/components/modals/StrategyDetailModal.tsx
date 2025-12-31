import { Dialog, Transition } from '@headlessui/react';
import { Fragment } from 'react';
import { MdClose, MdTrendingUp } from 'react-icons/md';
import { Card } from '../common/Card';
import { Badge } from '../common/Badge';
import { PerformanceSparkline } from '../charts/PerformanceSparkline';
import { formatCSPR, formatPercent } from '@/utils/format';

interface StrategyDetailModalProps {
  strategy: any;
  onClose: () => void;
}

const riskColors = {
  LOW: 'success',
  MEDIUM: 'warning',
  HIGH: 'danger',
};

export const StrategyDetailModal = ({ strategy, onClose }: StrategyDetailModalProps) => {
  if (!strategy) return null;

  return (
    <Transition appear show={true} as={Fragment}>
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
              <Dialog.Panel className="w-full max-w-4xl transform overflow-hidden rounded-2xl bg-white p-8 shadow-xl transition-all">
                {/* Header */}
                <div className="flex items-center justify-between mb-6">
                  <div>
                    <Dialog.Title className="text-3xl font-bold text-gray-900">
                      {strategy.displayName}
                    </Dialog.Title>
                    <p className="text-gray-500 mt-1">{strategy.description}</p>
                  </div>
                  <button
                    onClick={onClose}
                    className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                  >
                    <MdClose className="text-2xl text-gray-500" />
                  </button>
                </div>
                
                {/* Key Metrics */}
                <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
                  <Card>
                    <p className="text-xs text-gray-500 mb-1">Current APY</p>
                    <p className="text-2xl font-bold text-success-600">
                      {formatPercent(strategy.apy)}
                    </p>
                  </Card>
                  <Card>
                    <p className="text-xs text-gray-500 mb-1">Allocated</p>
                    <p className="text-2xl font-bold text-gray-900">
                      {formatCSPR(strategy.allocated || '0')} CSPR
                    </p>
                  </Card>
                  <Card>
                    <p className="text-xs text-gray-500 mb-1">Risk Level</p>
                    <Badge variant={riskColors[strategy.risk as keyof typeof riskColors] as any}>
                      {strategy.risk}
                    </Badge>
                  </Card>
                  <Card>
                    <p className="text-xs text-gray-500 mb-1">7-day Return</p>
                    <div className="flex items-center gap-1 text-success-600">
                      <MdTrendingUp className="text-xl" />
                      <span className="text-2xl font-bold">+2.4%</span>
                    </div>
                  </Card>
                </div>
                
                {/* Performance Chart */}
                <Card title="Historical Performance" subtitle="APY over time (30 days)" className="mb-6">
                  <div className="h-32">
                    <PerformanceSparkline data={strategy.history} />
                  </div>
                </Card>
                
                {/* Strategy Details */}
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <Card title="How it Works">
                    <ul className="space-y-3">
                      {strategy.howItWorks?.map((item: string, i: number) => (
                        <li key={i} className="flex items-start gap-3">
                          <span className="flex-shrink-0 w-6 h-6 rounded-full bg-primary-100 text-primary-600 flex items-center justify-center text-sm font-semibold">
                            {i + 1}
                          </span>
                          <span className="text-sm text-gray-600 flex-1">{item}</span>
                        </li>
                      ))}
                    </ul>
                  </Card>
                  
                  <Card title="Risk Factors">
                    <ul className="space-y-3">
                      {strategy.risks?.map((risk: string, i: number) => (
                        <li key={i} className="flex items-start gap-3">
                          <span className="flex-shrink-0 text-warning-500 text-xl">⚠</span>
                          <span className="text-sm text-gray-600 flex-1">{risk}</span>
                        </li>
                      ))}
                    </ul>
                  </Card>
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition>
  );
};
