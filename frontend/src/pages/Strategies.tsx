import { motion } from 'framer-motion';
import { MdShowChart } from 'react-icons/md';
import { Card } from '@/components/common';

export const Strategies = () => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
      className="space-y-6"
    >
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-primary-500 to-accent-500 flex items-center justify-center">
          <MdShowChart className="w-6 h-6 text-white" />
        </div>
        <div>
          <h1 className="text-3xl font-bold text-gray-900 font-display">Strategies</h1>
          <p className="text-gray-500">Explore and invest in yield strategies</p>
        </div>
      </div>

      <Card title="Available Strategies">
        <div className="space-y-2">
          <p className="text-4xl font-bold text-gray-900">Coming Soon</p>
          <p className="text-sm text-gray-500">Strategy cards and filtering in PROMPT 5</p>
        </div>
      </Card>
    </motion.div>
  );
};
