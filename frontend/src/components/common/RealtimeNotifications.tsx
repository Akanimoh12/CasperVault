import { useEffect } from 'react';
import toast from 'react-hot-toast';
import { MdTrendingUp, MdSwapHoriz, MdRefresh, MdTrendingDown } from 'react-icons/md';
import { useWebSocket } from '../../hooks/useWebSocket';
import { formatCSPR } from '../../utils/format';

export const RealtimeNotifications = () => {
  const { data } = useWebSocket();

  useEffect(() => {
    if (!data) return;

    switch (data.event) {
      case 'deposit':
        toast.success(
          <div className="flex items-center gap-2">
            <MdTrendingUp className="text-success-500" />
            <div>
              <p className="font-semibold">New Deposit</p>
              <p className="text-sm">{formatCSPR(data.data.amount)} CSPR</p>
            </div>
          </div>,
          {
            duration: 4000,
            position: 'bottom-right',
          }
        );
        break;

      case 'withdraw':
        toast.success(
          <div className="flex items-center gap-2">
            <MdTrendingDown className="text-warning-500" />
            <div>
              <p className="font-semibold">Withdrawal</p>
              <p className="text-sm">{formatCSPR(data.data.amount)} CSPR</p>
            </div>
          </div>,
          {
            duration: 4000,
            position: 'bottom-right',
          }
        );
        break;

      case 'compound':
        toast.success(
          <div className="flex items-center gap-2">
            <MdRefresh className="text-primary-500" />
            <div>
              <p className="font-semibold">Yields Compounded</p>
              <p className="text-sm">{formatCSPR(data.data.amount)} CSPR</p>
            </div>
          </div>,
          {
            duration: 4000,
            position: 'bottom-right',
          }
        );
        break;

      case 'rebalance':
        toast(
          <div className="flex items-center gap-2">
            <MdSwapHoriz className="text-accent-500" />
            <div>
              <p className="font-semibold">Strategies Rebalanced</p>
              <p className="text-sm">Allocation optimized</p>
            </div>
          </div>,
          {
            duration: 4000,
            position: 'bottom-right',
            icon: '⚖️',
          }
        );
        break;

      case 'tvl_update':
        // Silent update - no toast notification
        console.log('TVL updated:', data.data.tvl);
        break;

      case 'apy_update':
        // Silent update - no toast notification
        console.log('APY updated:', data.data.apy);
        break;

      case 'share_price_update':
        // Silent update - no toast notification
        console.log('Share price updated:', data.data.price);
        break;

      default:
        console.log('Unknown WebSocket event:', data.event);
    }
  }, [data]);

  return null;
};
