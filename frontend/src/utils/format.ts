import numeral from 'numeral';
import { format as formatDate } from 'date-fns';

/**
 * Format CSPR amount with commas and decimals
 */
export const formatCSPR = (amount: string | number, decimals = 2): string => {
  const value = typeof amount === 'string' ? parseFloat(amount) : amount;
  if (isNaN(value)) return '0.00';
  return numeral(value).format(`0,0.${'0'.repeat(decimals)}`);
};

/**
 * Format percentage value
 */
export const formatPercent = (value: number | string, decimals = 2): string => {
  const num = typeof value === 'string' ? parseFloat(value) : value;
  if (isNaN(num)) return '0.00%';
  return `${num.toFixed(decimals)}%`;
};

/**
 * Format large numbers with K, M, B suffixes
 */
export const formatNumber = (value: number | string): string => {
  const num = typeof value === 'string' ? parseFloat(value) : value;
  if (isNaN(num)) return '0';
  
  if (num >= 1e9) return numeral(num).format('0.00a').toUpperCase();
  if (num >= 1e6) return numeral(num).format('0.00a').toUpperCase();
  if (num >= 1e3) return numeral(num).format('0.00a').toUpperCase();
  
  return numeral(num).format('0,0.00');
};

/**
 * Format USD currency
 */
export const formatUSD = (value: number | string): string => {
  const num = typeof value === 'string' ? parseFloat(value) : value;
  if (isNaN(num)) return '$0.00';
  return numeral(num).format('$0,0.00');
};

/**
 * Format wallet address (shorten)
 */
export const formatAddress = (address: string, startChars = 8, endChars = 6): string => {
  if (!address || address.length < startChars + endChars) return address;
  return `${address.slice(0, startChars)}...${address.slice(-endChars)}`;
};

/**
 * Parse CSPR amount to motes (1 CSPR = 1e9 motes)
 */
export const parseCSPR = (amount: string | number): bigint => {
  const value = typeof amount === 'string' ? parseFloat(amount) : amount;
  if (isNaN(value)) return BigInt(0);
  return BigInt(Math.floor(value * 1e9));
};

/**
 * Format motes to CSPR
 */
export const motesToCSPR = (motes: bigint | string): string => {
  const value = typeof motes === 'string' ? BigInt(motes) : motes;
  return (Number(value) / 1e9).toFixed(2);
};

/**
 * Format timestamp to human-readable date
 */
export const formatTimestamp = (timestamp: number | string, formatString = 'MMM dd, yyyy HH:mm'): string => {
  const date = typeof timestamp === 'string' ? new Date(timestamp) : new Date(timestamp);
  return formatDate(date, formatString);
};

/**
 * Format duration in seconds to human-readable string
 */
export const formatDuration = (seconds: number): string => {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h`;
  return `${Math.floor(seconds / 86400)}d`;
};

/**
 * Calculate percentage change
 */
export const calculateChange = (current: number, previous: number): number => {
  if (previous === 0) return 0;
  return ((current - previous) / previous) * 100;
};

/**
 * Format APY with color coding
 */
export const formatAPY = (apy: number): { value: string; color: string } => {
  const formatted = formatPercent(apy);
  
  let color = 'text-gray-900';
  if (apy >= 50) color = 'text-success-600';
  else if (apy >= 20) color = 'text-primary-600';
  else if (apy >= 10) color = 'text-gray-700';
  else color = 'text-gray-500';
  
  return { value: formatted, color };
};
