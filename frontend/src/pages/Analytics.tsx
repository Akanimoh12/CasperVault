import { motion } from 'framer-motion';
import { MdAnalytics } from 'react-icons/md';
import { Card } from '@/components/common';

export const Analytics = () => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
      className="space-y-6"
    >
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-primary-500 to-accent-500 flex items-center justify-center">
          <MdAnalytics className="w-6 h-6 text-white" />
        </div>
        <div>
          <h1 className="text-3xl font-bold text-gray-900 font-display">Analytics</h1>
          <p className="text-gray-500">Performance metrics and insights</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card title="Performance Charts">
          <div className="space-y-2">
            <p className="text-4xl font-bold text-gray-900">Coming Soon</p>
            <p className="text-sm text-gray-500">Charts and visualizations in PROMPT 5</p>
          </div>
        </Card>

        <Card title="Historical Data">
          <div className="space-y-2">
            <p className="text-4xl font-bold text-gray-900">Coming Soon</p>
            <p className="text-sm text-gray-500">Time series analysis in PROMPT 5</p>
          </div>
        </Card>
      </div>
    </motion.div>
  );
};
