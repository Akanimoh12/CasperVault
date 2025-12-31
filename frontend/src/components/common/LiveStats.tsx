import { useEffect, useState } from 'react';
import { motion } from 'framer-motion';
import { useWebSocket } from '../../hooks/useWebSocket';
import { formatCSPR, formatPercent } from '../../utils/format';

export const LiveStats = () => {
  const { data } = useWebSocket();
  const [tvl, setTVL] = useState('0');
  const [apy, setAPY] = useState(0);
  const [sharePrice, setSharePrice] = useState('1.0');

  useEffect(() => {
    if (!data) return;

    if (data.event === 'tvl_update') {
      setTVL(data.data.tvl);
    } else if (data.event === 'apy_update') {
      setAPY(data.data.apy);
    } else if (data.event === 'share_price_update') {
      setSharePrice(data.data.price);
    }
  }, [data]);

  return (
    <div className="flex items-center gap-6">
      <motion.div
        key={tvl}
        initial={{ scale: 1.1, color: '#10b981' }}
        animate={{ scale: 1, color: '#111827' }}
        transition={{ duration: 0.3 }}
        className="text-center"
      >
        <p className="text-xs text-gray-500 mb-1">TVL</p>
        <p className="font-semibold text-gray-900">{formatCSPR(tvl)}</p>
      </motion.div>

      <div className="h-8 w-px bg-gray-200" />

      <motion.div
        key={apy}
        initial={{ scale: 1.1, color: '#10b981' }}
        animate={{ scale: 1, color: '#111827' }}
        transition={{ duration: 0.3 }}
        className="text-center"
      >
        <p className="text-xs text-gray-500 mb-1">APY</p>
        <p className="font-semibold text-success-600">{formatPercent(apy)}</p>
      </motion.div>

      <div className="h-8 w-px bg-gray-200" />

      <motion.div
        key={sharePrice}
        initial={{ scale: 1.1, color: '#10b981' }}
        animate={{ scale: 1, color: '#111827' }}
        transition={{ duration: 0.3 }}
        className="text-center"
      >
        <p className="text-xs text-gray-500 mb-1">Share Price</p>
        <p className="font-semibold text-gray-900">{sharePrice} CSPR</p>
      </motion.div>
    </div>
  );
};
