import { useState, useEffect } from 'react';
import { formatDistanceToNow } from 'date-fns';
import { MdTrendingUp, MdTrendingDown, MdAutorenew, MdRefresh } from 'react-icons/md';
import { contractService } from '@/services/contractService';
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

const statusColors = {
  pending: 'text-yellow-600',
  success: 'text-green-600',
  failed: 'text-red-600',
  unknown: 'text-gray-500',
};

interface Transaction {
  id: string;
  type: 'Deposit' | 'Withdraw';
  amount: string;
  user: string;
  timestamp: number;
  txHash: string;
  status: string;
}

export const RecentActivity = () => {
  const [activities, setActivities] = useState<Transaction[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);

  // Load transactions from localStorage
  const loadTransactions = async () => {
    setIsLoading(true);
    try {
      // Update status of pending transactions
      await contractService.updateTransactionStatuses();
      
      // Reload after status update
      const updatedTransactions = contractService.getStoredTransactions();
      setActivities(updatedTransactions);
    } catch (error) {
      console.error('Failed to load transactions:', error);
    } finally {
      setIsLoading(false);
    }
  };

  // Initial load
  useEffect(() => {
    loadTransactions();
    
    // Poll for updates every 30 seconds
    const interval = setInterval(() => {
      loadTransactions();
    }, 30000);

    // Listen for storage changes (new transactions)
    const handleStorageChange = () => {
      loadTransactions();
    };
    window.addEventListener('storage', handleStorageChange);

    return () => {
      clearInterval(interval);
      window.removeEventListener('storage', handleStorageChange);
    };
  }, []);

  const handleRefresh = async () => {
    setRefreshing(true);
    await loadTransactions();
    setRefreshing(false);
  };

  if (isLoading) {
    return (
      <div className="space-y-4">
        {[...Array(3)].map((_, i) => (
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

  if (!activities || activities.length === 0) {
    return (
      <div className="text-center py-12">
        <div className="text-gray-400 mb-4">
          <MdTrendingUp className="text-5xl mx-auto opacity-50" />
        </div>
        <p className="text-gray-500 mb-2">No transactions yet</p>
        <p className="text-sm text-gray-400">
          Your deposit and withdrawal history will appear here
        </p>
      </div>
    );
  }

  return (
    <div>
      {/* Refresh button */}
      <div className="flex justify-end mb-4">
        <button
          onClick={handleRefresh}
          disabled={refreshing}
          className="flex items-center gap-2 px-3 py-1.5 text-sm text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50"
        >
          <MdRefresh className={`text-lg ${refreshing ? 'animate-spin' : ''}`} />
          {refreshing ? 'Refreshing...' : 'Refresh'}
        </button>
      </div>

      <div className="space-y-4">
        {activities.slice(0, 10).map((activity) => {
          const Icon = typeIcons[activity.type as keyof typeof typeIcons] || MdAutorenew;
          const badgeVariant = typeColors[activity.type as keyof typeof typeColors] || 'primary';
          const statusColor = statusColors[activity.status as keyof typeof statusColors] || statusColors.unknown;

          return (
            <div
              key={activity.id}
              className="flex items-center gap-4 p-4 rounded-xl hover:bg-gray-50 transition-colors border border-gray-100"
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
                  <span className={`text-xs font-medium ${statusColor}`}>
                    • {activity.status}
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
                className="text-sm text-primary-600 hover:text-primary-700 font-medium whitespace-nowrap"
              >
                View →
              </a>
            </div>
          );
        })}
      </div>
      
      {activities.length > 10 && (
        <div className="text-center mt-4 pt-4 border-t border-gray-100">
          <span className="text-sm text-gray-500">
            Showing 10 of {activities.length} transactions
          </span>
        </div>
      )}
    </div>
  );
};
