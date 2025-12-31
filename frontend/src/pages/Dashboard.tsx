import { useState, useEffect } from 'react';
import { MdTrendingUp, MdAccountBalance, MdShowChart, MdPeople } from 'react-icons/md';
import { motion } from 'framer-motion';
import { useQuery } from '@tanstack/react-query';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { AllocationPieChart } from '@/components/charts/AllocationPieChart';
import { TVLChart } from '@/components/charts/TVLChart';
import { RecentActivity } from '@/components/dashboard/RecentActivity';
import { DepositModal, WithdrawModal } from '@/components/modals';
import { useWebSocket } from '@/hooks/useWebSocket';
import { api } from '@/services/api';
import { formatCSPR, formatPercent, formatNumber } from '@/utils/format';

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

export const Dashboard = () => {
  const [showDepositModal, setShowDepositModal] = useState(false);
  const [showWithdrawModal, setShowWithdrawModal] = useState(false);
  
  // Fetch dashboard data
  const { data: overview, isLoading: overviewLoading } = useQuery({
    queryKey: ['overview'],
    queryFn: () => api.getOverview(),
  });

  const { data: wsData } = useWebSocket();

  // Refetch overview when WebSocket receives updates
  useEffect(() => {
    if (wsData?.event === 'tvl_update' || wsData?.event === 'apy_update') {
      // Refetch data
    }
  }, [wsData]);

  // Mock cvCSPR balance - in production, fetch from contract
  const cvCSPRBalance = '0.00';

  const stats = [
    {
      icon: MdAccountBalance,
      label: 'Total Value Locked',
      value: overview ? formatCSPR(overview.tvl) : '...',
      subValue: overview ? `$${formatNumber(overview.tvlUSD)}` : '...',
      color: 'text-primary-600',
      bgColor: 'bg-primary-50',
    },
    {
      icon: MdShowChart,
      label: 'Current APY',
      value: overview ? formatPercent(overview.currentAPY) : '...',
      subValue: 'Average across strategies',
      color: 'text-accent-600',
      bgColor: 'bg-accent-50',
    },
    {
      icon: MdPeople,
      label: 'Total Users',
      value: overview ? formatNumber(overview.userCount) : '...',
      subValue: 'Active depositors',
      color: 'text-purple-600',
      bgColor: 'bg-purple-50',
    },
    {
      icon: MdTrendingUp,
      label: 'Net Inflow (24h)',
      value: overview ? formatCSPR(overview.netInflow) : '...',
      subValue: overview ? `$${formatNumber(overview.netInflowUSD)}` : '...',
      color: 'text-green-600',
      bgColor: 'bg-green-50',
    },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 font-display">Dashboard</h1>
          <p className="text-gray-500 mt-1">Overview of the CasperVault ecosystem</p>
        </div>
        <div className="flex gap-3">
          <Button
            variant="primary"
            onClick={() => setShowDepositModal(true)}
          >
            Deposit CSPR
          </Button>
          <Button
            variant="secondary"
            onClick={() => setShowWithdrawModal(true)}
          >
            Withdraw
          </Button>
        </div>
      </div>

      {/* Stats Grid */}
      <motion.div
        variants={container}
        initial="hidden"
        animate="show"
        className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4"
      >
        {stats.map((stat, index) => (
          <motion.div key={index} variants={item}>
            <Card className="hover:shadow-lg transition-shadow">
              <div className="flex items-start gap-4">
                <div className={`w-12 h-12 rounded-xl ${stat.bgColor} flex items-center justify-center flex-shrink-0`}>
                  <stat.icon className={`w-6 h-6 ${stat.color}`} />
                </div>
                <div className="flex-1 min-w-0">
                  <p className="text-sm text-gray-500 mb-1">{stat.label}</p>
                  {overviewLoading ? (
                    <div className="h-8 bg-gray-200 rounded animate-pulse" />
                  ) : (
                    <>
                      <p className="text-2xl font-bold text-gray-900 truncate">{stat.value}</p>
                      <p className="text-xs text-gray-500 mt-1">{stat.subValue}</p>
                    </>
                  )}
                </div>
              </div>
            </Card>
          </motion.div>
        ))}
      </motion.div>

      {/* Charts Row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* TVL Chart - 2/3 width */}
        <div className="lg:col-span-2">
          <Card title="Total Value Locked (30 Days)">
            <TVLChart />
          </Card>
        </div>

        {/* Allocation Pie Chart - 1/3 width */}
        <div>
          <Card title="Strategy Allocation">
            <AllocationPieChart />
          </Card>
        </div>
      </div>

      {/* Recent Activity */}
      <Card title="Recent Activity">
        <RecentActivity />
      </Card>

      {/* Modals */}
      <DepositModal
        isOpen={showDepositModal}
        onClose={() => setShowDepositModal(false)}
      />
      
      <WithdrawModal
        isOpen={showWithdrawModal}
        onClose={() => setShowWithdrawModal(false)}
        cvCSPRBalance={cvCSPRBalance}
      />
    </div>
  );
};
