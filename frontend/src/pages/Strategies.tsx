import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { motion } from 'framer-motion';
import {
  MdSpeed,
  MdSecurity,
  MdShowChart,
} from 'react-icons/md';
import { Card } from '@/components/common/Card';
import { Badge } from '@/components/common/Badge';
import { PerformanceSparkline } from '@/components/charts/PerformanceSparkline';
import { StrategyDetailModal } from '@/components/modals/StrategyDetailModal';
import { api } from '@/services/api';
import { formatCSPR, formatPercent } from '@/utils/format';

const riskColors = {
  LOW: 'success',
  MEDIUM: 'warning',
  HIGH: 'danger',
};

const riskIcons = {
  LOW: MdSecurity,
  MEDIUM: MdSpeed,
  HIGH: MdShowChart,
};

const container = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: {
      staggerChildren: 0.1,
    },
  },
};

const item = {
  hidden: { opacity: 0, y: 20 },
  show: { opacity: 1, y: 0 },
};

export const Strategies = () => {
  const [selectedStrategy, setSelectedStrategy] = useState<string | null>(null);
  
  const { data: strategies, isLoading } = useQuery({
    queryKey: ['strategies'],
    queryFn: () => api.getStrategies(),
  });
  
  return (
    <div className="space-y-8">
      {/* Header */}
      <div>
        <h1 className="text-4xl font-bold text-gray-900 mb-2">
          Yield Strategies
        </h1>
        <p className="text-gray-500">
          Automated strategies optimized for maximum returns
        </p>
      </div>
      
      {/* Strategy Cards Grid */}
      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {[1, 2, 3, 4].map((i) => (
            <Card key={i} className="h-[400px] animate-pulse">
              <div className="h-full bg-gray-100 rounded-xl" />
            </Card>
          ))}
        </div>
      ) : (
        <motion.div
          variants={container}
          initial="hidden"
          animate="show"
          className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
        >
          {strategies?.map((strategy: any) => {
            const RiskIcon = riskIcons[strategy.risk as keyof typeof riskIcons] || MdShowChart;
            const riskColor = riskColors[strategy.risk as keyof typeof riskColors] || 'default';
            
            return (
              <motion.div
                key={strategy.id}
                variants={item}
                whileHover={{ y: -5 }}
                transition={{ duration: 0.2 }}
                onClick={() => setSelectedStrategy(strategy.id)}
              >
                <Card className="h-full cursor-pointer" hover>
                  {/* Header */}
                  <div className="flex items-start justify-between mb-4">
                    <div className="flex-1">
                      <h3 className="text-xl font-bold text-gray-900 mb-1">
                        {strategy.displayName || strategy.name}
                      </h3>
                      <p className="text-sm text-gray-500">
                        {strategy.description || 'Yield strategy'}
                      </p>
                    </div>
                    <div className={`w-12 h-12 rounded-xl bg-${riskColor}-100 flex items-center justify-center flex-shrink-0 ml-3`}>
                      <RiskIcon className={`text-2xl text-${riskColor}-600`} />
                    </div>
                  </div>
                  
                  {/* APY */}
                  <div className="mb-4">
                    <p className="text-sm text-gray-500 mb-1">Current APY</p>
                    <p className="text-3xl font-bold text-success-600">
                      {formatPercent(strategy.apy)}
                    </p>
                  </div>
                  
                  {/* Stats */}
                  <div className="grid grid-cols-2 gap-3 mb-4">
                    <div className="p-3 rounded-xl bg-gray-50">
                      <p className="text-xs text-gray-500 mb-1">Allocated</p>
                      <p className="text-sm font-semibold text-gray-900">
                        {formatCSPR(strategy.allocated || '0')} CSPR
                      </p>
                    </div>
                    <div className="p-3 rounded-xl bg-gray-50">
                      <p className="text-xs text-gray-500 mb-1">Allocation</p>
                      <p className="text-sm font-semibold text-gray-900">
                        {strategy.allocation}%
                      </p>
                    </div>
                  </div>
                  
                  {/* Risk Badge */}
                  <Badge variant={riskColor as any}>
                    {strategy.risk} Risk
                  </Badge>
                  
                  {/* Performance Chart Mini */}
                  <div className="mt-4 pt-4 border-t border-gray-200">
                    <p className="text-xs text-gray-500 mb-2">30-day performance</p>
                    <PerformanceSparkline data={strategy.history} />
                  </div>
                </Card>
              </motion.div>
            );
          })}
        </motion.div>
      )}
      
      {/* Detailed View Modal */}
      {selectedStrategy && (
        <StrategyDetailModal
          strategy={strategies?.find((s: any) => s.id === selectedStrategy)}
          onClose={() => setSelectedStrategy(null)}
        />
      )}
    </div>
  );
};
