import { useQuery } from '@tanstack/react-query';
import { formatDistanceToNow } from 'date-fns';
import { MdTrendingUp, MdTrendingDown, MdAutorenew } from 'react-icons/md';
import { api } from '@/services/api';
import { formatCSPR } from '@/utils/format';
import { Badge } from '@/components/common/Badge';

const typeIcons = {
  Deposit: MdTrendingUp,
  Withdraw: MdTrendingDown,
  Compound: MdAutorenew,
};

const typeColors = {
  Deposit: 'success',
  Withdraw: 'warning',
  Compound: 'primary',
};

export const RecentActivity = () => {
  const { data: activities, isLoading } = useQuery({
    queryKey: ['recent-activity'],
    queryFn: () => api.getRecentActivity(5),
  });

  if (isLoading) {
    return (
      <div className="space-y-4">
        {[...Array(5)].map((_, i) => (
          <div key={i} className="animate-pulse flex items-center gap-4">
            <div className="w-10 h-10 bg-gray-200 rounded-full" />
            <div className="flex-1 space-y-2">
              <div className="h-4 bg-gray-200 rounded w-1/4" />
              <div className="h-3 bg-gray-200 rounded w-1/3" />
            </div>
            <div className="h-4 bg-gray-200 rounded w-20" />
          </div>
        ))}
      </div>
    );
  }

  if (!activities || (activities as any[]).length === 0) {
    return (
      <div className="text-center py-12 text-gray-500">
        <p>No recent activity</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {(activities as any[]).map((activity: any) => {
        const Icon = typeIcons[activity.type as keyof typeof typeIcons];
        const badgeVariant = typeColors[activity.type as keyof typeof typeColors];

        return (
          <div
            key={activity.id}
            className="flex items-center gap-4 p-4 rounded-xl hover:bg-gray-50 transition-colors"
          >
            <div className={`w-10 h-10 rounded-full bg-${badgeVariant}-100 flex items-center justify-center`}>
              <Icon className={`text-xl text-${badgeVariant}-600`} />
            </div>

            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-1">
                <Badge variant={badgeVariant as any}>
                  {activity.type}
                </Badge>
                <span className="text-sm font-medium text-gray-900">
                  {formatCSPR(activity.amount)} CSPR
                </span>
              </div>
              <div className="flex items-center gap-2 text-xs text-gray-500">
                <span className="font-mono">{activity.user}</span>
                <span>•</span>
                <span>{formatDistanceToNow(new Date(activity.timestamp), { addSuffix: true })}</span>
              </div>
            </div>

            <a
              href={`https://testnet.cspr.live/deploy/${activity.txHash}`}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-primary-600 hover:text-primary-700 font-medium"
            >
              View →
            </a>
          </div>
        );
      })}
    </div>
  );
};
