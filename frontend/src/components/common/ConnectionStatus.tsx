import { useWebSocket } from '../../hooks/useWebSocket';
import { motion } from 'framer-motion';

export const ConnectionStatus = () => {
  const { connected, getConnectionState } = useWebSocket();
  const state = getConnectionState();

  const getStatusColor = () => {
    switch (state) {
      case 'CONNECTED':
        return 'bg-success-500';
      case 'CONNECTING':
        return 'bg-warning-500';
      case 'DISCONNECTED':
      case 'CLOSED':
        return 'bg-danger-500';
      default:
        return 'bg-gray-400';
    }
  };

  const getStatusText = () => {
    switch (state) {
      case 'CONNECTED':
        return 'Connected';
      case 'CONNECTING':
        return 'Connecting...';
      case 'DISCONNECTED':
      case 'CLOSED':
        return 'Disconnected';
      default:
        return 'Unknown';
    }
  };

  return (
    <div className="flex items-center gap-2 text-sm text-gray-600">
      <motion.div
        className={`w-2 h-2 rounded-full ${getStatusColor()}`}
        animate={{
          scale: connected ? [1, 1.2, 1] : 1,
          opacity: connected ? [1, 0.7, 1] : 0.5,
        }}
        transition={{
          duration: 2,
          repeat: connected ? Infinity : 0,
          ease: 'easeInOut',
        }}
      />
      <span className="text-xs">{getStatusText()}</span>
    </div>
  );
};
