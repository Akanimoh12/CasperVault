import { PieChart, Pie, Cell, ResponsiveContainer, Legend, Tooltip } from 'recharts';
import { useQuery } from '@tanstack/react-query';
import { api } from '@/services/api';

const COLORS: Record<string, string> = {
  dex: '#0ea5e9',
  lending: '#d946ef',
  cross_chain: '#10b981',
  staking: '#f59e0b',
};

export const AllocationPieChart = () => {
  const { data: strategies } = useQuery({
    queryKey: ['strategies'],
    queryFn: () => api.getStrategies(),
  });
  
  if (!strategies) {
    return (
      <div className="flex items-center justify-center h-[300px]">
        <div className="animate-pulse text-gray-400">Loading...</div>
      </div>
    );
  }
  
  const data = (strategies as any[]).map((s: any) => ({
    name: s.name.charAt(0).toUpperCase() + s.name.slice(1).replace('_', ' '),
    value: parseFloat(s.allocationPercent),
    color: COLORS[s.name as keyof typeof COLORS] || '#6b7280',
  }));
  
  return (
    <ResponsiveContainer width="100%" height={300}>
      <PieChart>
        <Pie
          data={data}
          cx="50%"
          cy="50%"
          labelLine={false}
          label={({ name, percent }) => `${name}: ${(percent * 100).toFixed(0)}%`}
          outerRadius={80}
          fill="#8884d8"
          dataKey="value"
        >
          {data.map((entry, index) => (
            <Cell key={`cell-${index}`} fill={entry.color} />
          ))}
        </Pie>
        <Tooltip />
        <Legend />
      </PieChart>
    </ResponsiveContainer>
  );
};
