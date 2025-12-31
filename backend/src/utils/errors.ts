/**
 * Base custom error class
 */
export class AppError extends Error {
  constructor(
    message: string,
    public statusCode: number = 500,
    public isOperational: boolean = true
  ) {
    super(message);
    this.name = this.constructor.name;
    Error.captureStackTrace(this, this.constructor);
  }
}

/**
 * Validation error (400)
 */
export class ValidationError extends AppError {
  constructor(message: string, public details?: Record<string, unknown>) {
    super(message, 400);
  }
}

/**
 * Authentication error (401)
 */
export class AuthenticationError extends AppError {
  constructor(message: string = 'Authentication required') {
    super(message, 401);
  }
}

/**
 * Authorization error (403)
 */
export class AuthorizationError extends AppError {
  constructor(message: string = 'Insufficient permissions') {
    super(message, 403);
  }
}

/**
 * Not found error (404)
 */
export class NotFoundError extends AppError {
  constructor(resource: string) {
    super(`${resource} not found`, 404);
  }
}

/**
 * Conflict error (409)
 */
export class ConflictError extends AppError {
  constructor(message: string) {
    super(message, 409);
  }
}

/**
 * Rate limit error (429)
 */
export class RateLimitError extends AppError {
  constructor(message: string = 'Too many requests') {
    super(message, 429);
  }
}

/**
 * Contract interaction errors
 */
export class ContractError extends AppError {
  constructor(message: string, public contractName?: string, public deployHash?: string) {
    super(message, 500, false);
  }
}

export class TransactionFailedError extends ContractError {
  constructor(message: string, deployHash: string) {
    super(message, undefined, deployHash);
    this.name = 'TransactionFailedError';
  }
}

export class InsufficientGasError extends ContractError {
  constructor(required: string, available: string) {
    super(`Insufficient gas: required ${required}, available ${available}`);
    this.name = 'InsufficientGasError';
  }
}

export class ContractNotFoundError extends ContractError {
  constructor(contractName: string) {
    super(`Contract not found: ${contractName}`, contractName);
    this.name = 'ContractNotFoundError';
  }
}

/**
 * Network errors
 */
export class NetworkError extends AppError {
  constructor(message: string, public retryable: boolean = true) {
    super(message, 503, false);
    this.name = 'NetworkError';
  }
}

/**
 * Database errors
 */
export class DatabaseError extends AppError {
  constructor(message: string, public operation?: string) {
    super(message, 500, false);
    this.name = 'DatabaseError';
  }
}

/**
 * External service errors
 */
export class ExternalServiceError extends AppError {
  constructor(service: string, message: string) {
    super(`${service} error: ${message}`, 503, false);
    this.name = 'ExternalServiceError';
  }
}

/**
 * Error handler utility
 */
export class ErrorHandler {
  /**
   * Check if error is operational
   */
  static isOperationalError(error: Error): boolean {
    if (error instanceof AppError) {
      return error.isOperational;
    }
    return false;
  }

  /**
   * Check if error is retryable
   */
  static isRetryable(error: Error): boolean {
    if (error instanceof NetworkError) {
      return error.retryable;
    }
    if (error instanceof ContractError) {
      return false; // Contract errors usually aren't retryable
    }
    return false;
  }

  /**
   * Extract error message
   */
  static extractMessage(error: unknown): string {
    if (error instanceof Error) {
      return error.message;
    }
    if (typeof error === 'string') {
      return error;
    }
    return 'Unknown error occurred';
  }

  /**
   * Format error for API response
   */
  static formatApiError(error: Error): {
    message: string;
    statusCode: number;
    error?: string;
    details?: Record<string, unknown>;
  } {
    if (error instanceof AppError) {
      return {
        message: error.message,
        statusCode: error.statusCode,
        error: error.name,
        details: error instanceof ValidationError ? error.details : undefined,
      };
    }

    return {
      message: 'Internal server error',
      statusCode: 500,
      error: 'InternalServerError',
    };
  }
}

/**
 * Retry utility for error handling
 */
export class RetryHandler {
  /**
   * Retry async operation with exponential backoff
   */
  static async retry<T>(
    operation: () => Promise<T>,
    maxRetries: number = 3,
    baseDelay: number = 1000,
    shouldRetry?: (error: Error) => boolean
  ): Promise<T> {
    let lastError: Error;

    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      try {
        return await operation();
      } catch (error) {
        lastError = error instanceof Error ? error : new Error(String(error));

        // Check if we should retry
        if (shouldRetry && !shouldRetry(lastError)) {
          throw lastError;
        }

        // Last attempt, throw error
        if (attempt === maxRetries) {
          throw lastError;
        }

        // Calculate delay with exponential backoff
        const delay = baseDelay * Math.pow(2, attempt);
        await new Promise((resolve) => setTimeout(resolve, delay));
      }
    }

    throw lastError!;
  }
}
