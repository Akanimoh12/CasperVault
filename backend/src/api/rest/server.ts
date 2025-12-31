/**
 * REST API Server
 * 
 * Provides HTTP endpoints for frontend
 */

import express from 'express';
import { Logger } from '../../utils/logger';
import { config } from '../../utils/config';

export class RestApiServer {
  private app: express.Application;

  constructor() {
    this.app = express();
    this.setupMiddleware();
    this.setupRoutes();
    
    Logger.info('REST API Server initialized');
  }

  private setupMiddleware(): void {
    // Implementation in Prompt 5
    // TODO: Setup CORS
    // TODO: Setup body parser
    // TODO: Setup rate limiting
    // TODO: Setup error handling
  }

  private setupRoutes(): void {
    // Implementation in Prompt 5
    // TODO: Add portfolio routes
    // TODO: Add strategies routes
    // TODO: Add analytics routes
    // TODO: Add transaction routes
  }

  start(): void {
    const port = config.server.port;
    this.app.listen(port, () => {
      Logger.info(`REST API Server listening on port ${port}`);
    });
  }
}

export default RestApiServer;
