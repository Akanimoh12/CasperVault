import { config } from '../src/utils/config';
import { Logger } from '../src/utils/logger';
import database from '../src/database/client';

describe('Configuration', () => {
  it('should load configuration', () => {
    expect(config).toBeDefined();
    expect(config.environment).toBeDefined();
    expect(config.casper).toBeDefined();
  });

  it('should have required casper config', () => {
    expect(config.casper.rpcUrl).toBeDefined();
    expect(config.casper.network).toBeDefined();
  });
});

describe('Logger', () => {
  it('should log info message', () => {
    expect(() => Logger.info('Test message')).not.toThrow();
  });

  it('should log error message', () => {
    const error = new Error('Test error');
    expect(() => Logger.error('Test error', error)).not.toThrow();
  });
});

describe('Database', () => {
  it('should have database client', () => {
    expect(database).toBeDefined();
  });
});
