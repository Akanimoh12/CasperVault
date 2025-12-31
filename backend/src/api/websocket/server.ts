/**
 * WebSocket Server
 * 
 * Provides real-time updates to frontend
 */

import WebSocket from 'ws';
import { Logger } from '../../utils/logger';

export class WebSocketServer {
  private wss: WebSocket.Server | null = null;
  private clients: Set<WebSocket> = new Set();

  constructor() {
    Logger.info('WebSocket Server initialized');
  }

  start(port: number): void {
    this.wss = new WebSocket.Server({ port });
    
    this.wss.on('connection', (ws: WebSocket) => {
      Logger.debug('WebSocket client connected');
      this.clients.add(ws);
      
      ws.on('close', () => {
        Logger.debug('WebSocket client disconnected');
        this.clients.delete(ws);
      });
    });
    
    Logger.info(`WebSocket Server listening on port ${port}`);
  }

  broadcast(event: string, data: unknown): void {
    const message = JSON.stringify({ event, data, timestamp: Date.now() });
    
    this.clients.forEach((client) => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(message);
      }
    });
  }

  stop(): void {
    this.wss?.close();
    Logger.info('WebSocket Server stopped');
  }
}

export default WebSocketServer;
