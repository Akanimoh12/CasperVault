import { useEffect, useState, useCallback } from 'react';
import { wsService } from '../services/websocket';

interface WebSocketData {
  event: string;
  data: any;
  timestamp: number;
}

export const useWebSocket = (url: string = 'ws://localhost:3002') => {
  const [data, setData] = useState<WebSocketData | null>(null);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    // Connect to WebSocket
    wsService.connect(url);
    setConnected(true);

    // Listen to all events
    const events = [
      'deposit',
      'withdraw',
      'compound',
      'rebalance',
      'tvl_update',
      'apy_update',
      'share_price_update',
    ];

    const callbacks = new Map<string, Function>();

    events.forEach((event) => {
      const callback = (eventData: any) => {
        setData({ event, data: eventData, timestamp: Date.now() });
      };
      callbacks.set(event, callback);
      wsService.on(event, callback);
    });

    // Cleanup function
    return () => {
      events.forEach((event) => {
        const callback = callbacks.get(event);
        if (callback) {
          wsService.off(event, callback);
        }
      });
      wsService.disconnect();
      setConnected(false);
    };
  }, [url]);

  const send = useCallback((event: string, data: any) => {
    wsService.send(event, data);
  }, []);

  const isConnected = useCallback(() => {
    return wsService.isConnected();
  }, []);

  const getConnectionState = useCallback(() => {
    return wsService.getConnectionState();
  }, []);

  return { data, connected, send, isConnected, getConnectionState };
};

