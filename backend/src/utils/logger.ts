import winston from 'winston';
import DailyRotateFile from 'winston-daily-rotate-file';
import * as path from 'path';
import { config } from './config';

/**
 * Custom log format
 */
const logFormat = winston.format.combine(
  winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
  winston.format.errors({ stack: true }),
  winston.format.splat(),
  winston.format.json()
);

/**
 * Console log format (pretty print for development)
 */
const consoleFormat = winston.format.combine(
  winston.format.colorize(),
  winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
  winston.format.printf(({ timestamp, level, message, ...meta }) => {
    const metaStr = Object.keys(meta).length ? `\n${JSON.stringify(meta, null, 2)}` : '';
    return `${timestamp} [${level}]: ${message}${metaStr}`;
  })
);

/**
 * Create transports based on configuration
 */
const createTransports = (): winston.transport[] => {
  const transports: winston.transport[] = [];

  // Console transport
  if (config.logging.console) {
    transports.push(
      new winston.transports.Console({
        format: consoleFormat,
      })
    );
  }

  // File transports
  if (config.logging.file) {
    // Error log
    transports.push(
      new winston.transports.File({
        filename: path.join(__dirname, '../../logs/error.log'),
        level: 'error',
        format: logFormat,
      })
    );

    // Combined log with rotation
    transports.push(
      new DailyRotateFile({
        filename: path.join(__dirname, '../../logs/app-%DATE%.log'),
        datePattern: 'YYYY-MM-DD',
        maxSize: config.logging.maxSize,
        maxFiles: config.logging.maxFiles,
        format: logFormat,
      })
    );
  }

  return transports;
};

/**
 * Logger instance
 */
export const logger = winston.createLogger({
  level: config.logging.level,
  format: logFormat,
  transports: createTransports(),
  exitOnError: false,
});

/**
 * Logger utility methods
 */
export class Logger {
  /**
   * Log info message
   */
  static info(message: string, meta?: Record<string, unknown>): void {
    logger.info(message, meta);
  }

  /**
   * Log warning message
   */
  static warn(message: string, meta?: Record<string, unknown>): void {
    logger.warn(message, meta);
  }

  /**
   * Log error message
   */
  static error(message: string, error?: Error | unknown, meta?: Record<string, unknown>): void {
    if (error instanceof Error) {
      logger.error(message, {
        error: {
          message: error.message,
          stack: error.stack,
          name: error.name,
        },
        ...meta,
      });
    } else {
      logger.error(message, { error, ...meta });
    }
  }

  /**
   * Log debug message
   */
  static debug(message: string, meta?: Record<string, unknown>): void {
    logger.debug(message, meta);
  }

  /**
   * Log transaction
   */
  static transaction(
    action: string,
    deployHash: string,
    meta?: Record<string, unknown>
  ): void {
    logger.info(`Transaction: ${action}`, {
      deployHash,
      action,
      ...meta,
    });
  }

  /**
   * Log bot activity
   */
  static bot(botName: string, action: string, meta?: Record<string, unknown>): void {
    logger.info(`[${botName}] ${action}`, {
      bot: botName,
      action,
      ...meta,
    });
  }

  /**
   * Log API request
   */
  static api(method: string, path: string, statusCode: number, duration: number): void {
    logger.info('API Request', {
      method,
      path,
      statusCode,
      duration: `${duration}ms`,
    });
  }
}

// Handle uncaught exceptions and rejections
logger.exceptions.handle(
  new winston.transports.File({
    filename: path.join(__dirname, '../../logs/exceptions.log'),
  })
);

logger.rejections.handle(
  new winston.transports.File({
    filename: path.join(__dirname, '../../logs/rejections.log'),
  })
);

export default logger;
