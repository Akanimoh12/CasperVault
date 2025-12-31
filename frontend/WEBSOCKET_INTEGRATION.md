# WebSocket Integration Guide

This guide explains the WebSocket implementation for real-time updates in CasperVault.

## Components Created

### 1. **WebSocket Service** (`services/websocket.ts`)
- Handles WebSocket connection lifecycle
- Automatic reconnection with exponential backoff (max 5 attempts)
- Event-based pub/sub system
- Connection state management

### 2. **useWebSocket Hook** (`hooks/useWebSocket.ts`)
- React hook for WebSocket integration
- Listens to multiple event types
- Cleanup on unmount
- Returns connection state and data

### 3. **RealtimeNotifications** (`components/common/RealtimeNotifications.tsx`)
- Displays toast notifications for WebSocket events
- Events handled:
  - `deposit` - New deposits
  - `withdraw` - Withdrawals
  - `compound` - Yield compounding
  - `rebalance` - Strategy rebalancing
  - `tvl_update` - TVL changes (silent)
  - `apy_update` - APY changes (silent)
  - `share_price_update` - Share price changes (silent)

### 4. **LiveStats** (`components/common/LiveStats.tsx`)
- Real-time display of TVL, APY, and share price
- Animated updates with Framer Motion
- Smooth transitions on value changes

### 5. **ConnectionStatus** (`components/common/ConnectionStatus.tsx`)
- Visual indicator of WebSocket connection state
- Animated pulsing dot when connected
- Shows: Connected, Connecting, Disconnected

## Integration Points

### Layout Component
- `RealtimeNotifications` added to display toast notifications globally

### Navbar Component  
- `ConnectionStatus` added to show connection state

### Dashboard Component
- Uses `useWebSocket` hook to listen for real-time updates
- Can trigger data refetch on specific events

## Testing Without Backend

The WebSocket service will attempt to connect to `ws://localhost:3002` by default. Without a backend:
- Connection will fail gracefully
- Reconnection attempts will be made (5 max)
- No errors will crash the app
- UI remains functional

## Testing With Mock WebSocket Server

Create a simple WebSocket test server:

\`\`\`javascript
// test-ws-server.js
const WebSocket = require('ws');
const wss = new WebSocket.Server({ port: 3002 });

wss.on('connection', (ws) => {
  console.log('Client connected');
  
  // Send mock events every few seconds
  const interval = setInterval(() => {
    // Mock deposit event
    ws.send(JSON.stringify({
      event: 'deposit',
      data: { amount: '1000', user: '0x123...' }
    }));
    
    // Mock TVL update
    ws.send(JSON.stringify({
      event: 'tvl_update',
      data: { tvl: '15000000' }
    }));
    
    // Mock APY update
    ws.send(JSON.stringify({
      event: 'apy_update',
      data: { apy: 12.5 }
    }));
  }, 5000);
  
  ws.on('close', () => {
    clearInterval(interval);
    console.log('Client disconnected');
  });
});

console.log('WebSocket server running on ws://localhost:3002');
\`\`\`

Run with: `node test-ws-server.js`

## Event Format

All WebSocket messages should follow this format:

\`\`\`typescript
{
  event: string,  // Event type (deposit, withdraw, etc.)
  data: any       // Event-specific data
}
\`\`\`

## Supported Events

| Event | Data Format | UI Response |
|-------|-------------|-------------|
| `deposit` | `{ amount: string, user: string }` | Toast notification |
| `withdraw` | `{ amount: string, user: string }` | Toast notification |
| `compound` | `{ amount: string }` | Toast notification |
| `rebalance` | `{ strategies: any }` | Toast notification |
| `tvl_update` | `{ tvl: string }` | Update LiveStats |
| `apy_update` | `{ apy: number }` | Update LiveStats |
| `share_price_update` | `{ price: string }` | Update LiveStats |

## Customization

### Change WebSocket URL
Modify the default URL in `useWebSocket.ts`:
\`\`\`typescript
export const useWebSocket = (url: string = 'ws://your-backend:port') => {
  // ...
}
\`\`\`

### Add New Event Types
1. Add event to the `events` array in `useWebSocket.ts`
2. Add handler in `RealtimeNotifications.tsx`

### Adjust Reconnection Behavior
Modify in `websocket.ts`:
\`\`\`typescript
private maxReconnectAttempts = 5;  // Change max attempts
const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
\`\`\`

## Production Deployment

1. Set WebSocket URL via environment variable:
\`\`\`bash
VITE_WS_URL=wss://api.caspervault.com/ws
\`\`\`

2. Update useWebSocket to use env variable:
\`\`\`typescript
const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:3002';
export const useWebSocket = (url: string = WS_URL) => {
  // ...
}
\`\`\`

3. Use secure WebSocket (wss://) in production

## Features Implemented

✅ WebSocket service with reconnection logic  
✅ React hook for WebSocket integration  
✅ Real-time toast notifications  
✅ Live stats component with animations  
✅ Connection status indicator  
✅ Event handling system  
✅ Auto-reconnect with exponential backoff  
✅ Error handling  
✅ TypeScript types  
✅ Integration with Layout and Navbar  

## Next Steps

- Implement backend WebSocket server
- Add authentication for WebSocket connections
- Add more event types as needed
- Implement message queuing for offline support
- Add WebSocket health checks
