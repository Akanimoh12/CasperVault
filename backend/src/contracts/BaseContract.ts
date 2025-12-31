import {
  CasperClient,
  CLPublicKey,
  CLValueBuilder,
  DeployUtil,
  RuntimeArgs,
  Keys,
} from 'casper-js-sdk';
import * as fs from 'fs';
import { config } from '../utils/config';
import { Logger } from '../utils/logger';
import {
  ContractError,
  TransactionFailedError,
  NetworkError,
  RetryHandler,
} from '../utils/errors';
import type { TransactionResult, TransactionStatus } from '../types';

/**
 * Contract configuration
 */
export interface ContractConfig {
  contractHash: string;
  contractName: string;
}

/**
 * Base contract wrapper with common functionality
 */
export abstract class BaseContract {
  protected contractHash: string;
  protected contractName: string;
  protected casperClient: CasperClient;
  protected networkName: string;

  constructor(contractConfig: ContractConfig) {
    this.contractHash = contractConfig.contractHash;
    this.contractName = contractConfig.contractName;
    this.networkName = config.casper.network;

    // Initialize Casper client
    this.casperClient = new CasperClient(config.casper.rpcUrl);

    Logger.info(`${this.contractName} contract initialized`, {
      contractHash: this.contractHash,
      network: this.networkName,
    });
  }

  /**
   * Load private key from file
   */
  protected loadPrivateKey(keyPath: string): Keys.AsymmetricKey {
    try {
      const keyContent = fs.readFileSync(keyPath, 'utf-8');
      return Keys.Ed25519.parsePrivateKeyFile(keyContent);
    } catch (error) {
      Logger.error('Failed to load private key', error);
      throw new ContractError('Failed to load private key');
    }
  }

  /**
   * Call contract entry point
   */
  protected async callEntrypoint(
    entrypoint: string,
    args: RuntimeArgs,
    paymentAmount: string,
    signerKey: Keys.AsymmetricKey
  ): Promise<TransactionResult> {
    try {
      Logger.debug(`Calling ${this.contractName}.${entrypoint}`, {
        entrypoint,
        paymentAmount,
      });

      // Create deploy
      const deploy = DeployUtil.makeDeploy(
        new DeployUtil.DeployParams(
          CLPublicKey.fromHex(signerKey.publicKey.toHex()),
          this.networkName
        ),
        DeployUtil.ExecutableDeployItem.newStoredContractByHash(
          Uint8Array.from(Buffer.from(this.contractHash, 'hex')),
          entrypoint,
          args
        ),
        DeployUtil.standardPayment(paymentAmount)
      );

      // Sign deploy
      const signedDeploy = deploy.sign([signerKey]);

      // Submit deploy
      const deployHash = await this.casperClient.putDeploy(signedDeploy);

      Logger.transaction(
        `${this.contractName}.${entrypoint}`,
        deployHash,
        { entrypoint }
      );

      // Wait for result
      const result = await this.waitForTransaction(deployHash);

      if (!result.success) {
        throw new TransactionFailedError(
          `Transaction failed: ${result.error}`,
          deployHash
        );
      }

      return result;
    } catch (error) {
      Logger.error(`Failed to call ${entrypoint}`, error);
      throw new ContractError(`Failed to call ${entrypoint}: ${error}`);
    }
  }

  /**
   * Query contract state
   */
  protected async queryContract(key: string): Promise<unknown> {
    try {
      const stateRootHash = await this.casperClient.nodeClient.getStateRootHash();
      const blockState = await this.casperClient.nodeClient.getBlockState(
        stateRootHash,
        this.contractHash,
        [key]
      );

      return blockState;
    } catch (error) {
      Logger.error('Failed to query contract', error);
      throw new ContractError(`Failed to query contract: ${error}`);
    }
  }

  /**
   * Wait for transaction to complete
   */
  protected async waitForTransaction(
    deployHash: string,
    timeout: number = 180000
  ): Promise<TransactionResult> {
    try {
      Logger.debug('Waiting for transaction', { deployHash });

      const startTime = Date.now();

      // Poll for result
      while (Date.now() - startTime < timeout) {
        try {
          const [deploy, raw] = await this.casperClient.nodeClient.getDeployInfo(deployHash);

          if (raw.execution_results.length > 0) {
            const result = raw.execution_results[0].result;

            if (result.Success) {
              Logger.info('Transaction succeeded', { deployHash });

              return {
                success: true,
                deployHash,
                blockHash: result.Success.block_hash,
                timestamp: Date.now(),
              };
            } else if (result.Failure) {
              Logger.warn('Transaction failed', {
                deployHash,
                error: result.Failure.error_message,
              });

              return {
                success: false,
                deployHash,
                error: result.Failure.error_message,
                timestamp: Date.now(),
              };
            }
          }
        } catch (error) {
          // Deploy not found yet, continue polling
        }

        // Wait before next poll
        await new Promise((resolve) => setTimeout(resolve, 5000));
      }

      throw new Error('Transaction timeout');
    } catch (error) {
      Logger.error('Failed to wait for transaction', error);
      throw new NetworkError(`Failed to wait for transaction: ${error}`);
    }
  }

  /**
   * Estimate gas for transaction
   */
  protected async estimateGas(entrypoint: string, args: RuntimeArgs): Promise<string> {
    // For now, return fixed amount based on entrypoint
    // TODO: Implement actual gas estimation
    const gasMap: Record<string, string> = {
      deposit: '5000000000', // 5 CSPR
      withdraw: '5000000000',
      rebalance: '10000000000', // 10 CSPR
      compound: '10000000000',
      default: '3000000000', // 3 CSPR
    };

    return gasMap[entrypoint] || gasMap.default;
  }

  /**
   * Retry operation with exponential backoff
   */
  protected async retryOperation<T>(
    operation: () => Promise<T>,
    maxRetries: number = 3
  ): Promise<T> {
    return RetryHandler.retry(operation, maxRetries, 1000, (error) => {
      // Retry on network errors only
      return error instanceof NetworkError;
    });
  }

  /**
   * Get contract hash
   */
  get hash(): string {
    return this.contractHash;
  }

  /**
   * Get contract name
   */
  get name(): string {
    return this.contractName;
  }
}

export default BaseContract;
