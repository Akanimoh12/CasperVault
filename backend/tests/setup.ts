/**
 * Test setup file
 */

// Setup test environment
process.env.NODE_ENV = 'test';

// Mock environment variables for tests
process.env.SUPABASE_URL = 'https://test.supabase.co';
process.env.SUPABASE_KEY = 'test-key';
process.env.CASPER_RPC_URL = 'https://rpc.testnet.casperlabs.io/rpc';

// Add custom matchers if needed
expect.extend({
  // Custom matchers can be added here
});

// Global test timeout
jest.setTimeout(30000);
