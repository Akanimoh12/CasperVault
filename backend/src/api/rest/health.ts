import { Request, Response } from 'express';
import database from '../../database/client';
import { config } from '../../utils/config';

/**
 * Health check endpoint
 * Returns system health status
 */
export async function healthCheck(_req: Request, res: Response): Promise<void> {
  try {
    const health = {
      status: 'healthy',
      timestamp: new Date().toISOString(),
      version: '1.0.0',
      environment: config.environment,
      services: {
        api: true,
        database: database.connected,
        casper: !!config.casper.rpcUrl,
      },
    };

    res.status(200).json(health);
  } catch (error) {
    res.status(503).json({
      status: 'unhealthy',
      timestamp: new Date().toISOString(),
      error: error instanceof Error ? error.message : 'Unknown error',
    });
  }
}

export default healthCheck;
