import { useState } from 'react';
import { motion } from 'framer-motion';
import { Card } from '../components/common/Card';
import { TVLChart } from '../components/charts/TVLChart';
import { APYChart } from '../components/charts/APYChart';
import { YieldDistributionChart } from '../components/charts/YieldDistributionChart';
import { UserGrowthChart } from '../components/charts/UserGrowthChart';
import { StrategyComparisonChart } from '../components/charts/StrategyComparisonChart';

export const Analytics = () => {
  const [period, setPeriod] = useState('30d');

  const periods = [
    { label: '7D', value: '7d' },
    { label: '30D', value: '30d' },
    { label: '90D', value: '90d' },
    { label: 'All', value: 'all' },
  ];

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-4xl font-bold text-gray-900 mb-2">
            Analytics
          </h1>
          <p className="text-gray-500">
            Deep insights into vault performance
          </p>
        </div>

        {/* Period Selector */}
        <div className="flex gap-2 p-1 rounded-xl bg-gray-100">
          {periods.map((p) => (
            <motion.button
              key={p.value}
              whileTap={{ scale: 0.95 }}
              onClick={() => setPeriod(p.value)}
              className={`px-4 py-2 rounded-lg font-medium transition-all ${
                period === p.value
                  ? 'bg-white text-primary-600 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              {p.label}
            </motion.button>
          ))}
        </div>
      </div>

      {/* TVL & APY Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card title="Total Value Locked" subtitle={`Last ${period}`}>
          <TVLChart period={period} />
        </Card>

        <Card title="APY History" subtitle={`Last ${period}`}>
          <APYChart period={period} />
        </Card>
      </div>

      {/* Yield Distribution */}
      <Card title="Yield Distribution by Strategy" subtitle="Where yields come from">
        <YieldDistributionChart period={period} />
      </Card>

      {/* User Growth & Strategy Comparison */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card title="User Growth" subtitle="New users over time">
          <UserGrowthChart period={period} />
        </Card>

        <Card title="Strategy Comparison" subtitle="APY comparison">
          <StrategyComparisonChart period={period} />
        </Card>
      </div>
    </div>
  );
};
