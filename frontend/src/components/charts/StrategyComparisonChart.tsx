import { useQuery } from '@tanstack/react-query';
import {
  ComposedChart,
  Bar,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import { api } from '../../services/api';

interface StrategyComparisonChartProps {
  period: string;
}

export const StrategyComparisonChart = ({ period }: StrategyComparisonChartProps) => {
  const { data } = useQuery({
    queryKey: ['strategy-comparison', period],
    queryFn: () => api.getStrategyComparison(period),
  });

  if (!data) {
    return (
      <div className="w-full h-[300px] flex items-center justify-center">
        <div className="text-gray-400">Loading...</div>
      </div>
    );
  }

  return (
    <ResponsiveContainer width="100%" height={300}>
      <ComposedChart data={data} margin={{ top: 20, right: 30, left: 0, bottom: 0 }}>
        <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
        <XAxis
          dataKey="name"
          tick={{ fill: '#6b7280', fontSize: 12 }}
          axisLine={false}
        />
        <YAxis
          yAxisId="left"
          tick={{ fill: '#6b7280', fontSize: 12 }}
          axisLine={false}
          tickFormatter={(value) => `${value}%`}
          label={{ value: 'Allocation %', angle: -90, position: 'insideLeft', fill: '#6b7280' }}
        />
        <YAxis
          yAxisId="right"
          orientation="right"
          tick={{ fill: '#6b7280', fontSize: 12 }}
          axisLine={false}
          tickFormatter={(value) => `${value}%`}
          label={{ value: 'APY %', angle: 90, position: 'insideRight', fill: '#6b7280' }}
        />
        <Tooltip
          contentStyle={{
            backgroundColor: '#fff',
            border: '1px solid #e5e7eb',
            borderRadius: '8px',
            boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1)',
          }}
        />
        <Legend />
        <Bar
          yAxisId="left"
          dataKey="allocation"
          fill="#0ea5e9"
          radius={[8, 8, 0, 0]}
          name="Allocation %"
        />
        <Line
          yAxisId="right"
          type="monotone"
          dataKey="apy"
          stroke="#10b981"
          strokeWidth={3}
          dot={{ fill: '#10b981', r: 5 }}
          name="APY %"
        />
      </ComposedChart>
    </ResponsiveContainer>
  );
};
