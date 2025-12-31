import { LineChart, Line, ResponsiveContainer } from 'recharts';

interface PerformanceSparklineProps {
  data?: Array<{ timestamp: number; apy: number }>;
}

export const PerformanceSparkline = ({ data }: PerformanceSparklineProps) => {
  if (!data || data.length === 0) {
    return (
      <div className="w-full h-[40px] flex items-center justify-center">
        <span className="text-xs text-gray-400">No data</span>
      </div>
    );
  }
  
  return (
    <ResponsiveContainer width="100%" height={40}>
      <LineChart data={data}>
        <Line
          type="monotone"
          dataKey="apy"
          stroke="#10b981"
          strokeWidth={2}
          dot={false}
        />
      </LineChart>
    </ResponsiveContainer>
  );
};
