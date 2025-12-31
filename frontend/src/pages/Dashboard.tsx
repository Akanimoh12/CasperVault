import { motion } from 'framer-motion';
import { MdDashboard } from 'react-icons/md';
import { Card } from '@/components/common';

export const Dashboard = () => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
      className="space-y-6"
    >
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-primary-500 to-accent-500 flex items-center justify-center">
          <MdDashboard className="w-6 h-6 text-white" />
        </div>
        <div>
          <h1 className="text-3xl font-bold text-gray-900 font-display">Dashboard</h1>
          <p className="text-gray-500">Overview of your DeFi portfolio</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <Card title="Total Value Locked">
          <div className="space-y-2">
            <p className="text-4xl font-bold text-gray-900">Coming Soon</p>
            <p className="text-sm text-gray-500">Full implementation in PROMPT 5</p>
          </div>
        </Card>

        <Card title="Active Strategies">
          <div className="space-y-2">
            <p className="text-4xl font-bold text-gray-900">Coming Soon</p>
            <p className="text-sm text-gray-500">Full implementation in PROMPT 5</p>
          </div>
        </Card>

        <Card title="Total Rewards">
          <div className="space-y-2">
            <p className="text-4xl font-bold text-gray-900">Coming Soon</p>
            <p className="text-sm text-gray-500">Full implementation in PROMPT 5</p>
          </div>
        </Card>
      </div>
    </motion.div>
  );
};
