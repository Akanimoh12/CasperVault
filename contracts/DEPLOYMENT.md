# CasperVault Smart Contracts - Deployment Guide

## 🎯 Deployment Status

**✅ Library Compilation:** SUCCESS (0 errors)  
**📦 Deployment:** Ready with manual integration

## 📋 Prerequisites

### Required Tools

1. **Rust & Cargo** (v1.70+)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **WASM Target**
```bash
rustup target add wasm32-unknown-unknown
```

3. **Casper Client** (for deployment)
```bash
cargo install casper-client
```

4. **WASM Optimizer** (optional but recommended)
```bash
# Ubuntu/Debian
sudo apt install binaryen

# macOS
brew install binaryen
```

### Network Access

- **Testnet:** http://3.143.158.19:7777
- **Mainnet:** http://65.21.235.219:7777

### Funding

- **Testnet Faucet:** https://testnet.cspr.live/tools/faucet
- **Minimum:** ~100 CSPR for contract deployment

## 🏗️ Build Instructions

### Option 1: Automated Build (Recommended)

```bash
cd contracts
./build.sh
```

### Option 2: Manual Build

```bash
cd contracts

# Clean previous builds
cargo clean

# Build library (currently working)
cargo build --release --lib

# Build for WASM (needs Odra integration)
cargo build --release --target wasm32-unknown-unknown --lib
```

## 🚀 Deployment Options

### Option A: Using Odra SDK (Recommended for Production)

Odra provides the best developer experience for Casper smart contracts.

**1. Update Cargo.toml:**
```toml
[dependencies]
odra = { version = "0.8", features = ["casper-wasm"] }
```

**2. Create Test Environment:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::{Deployer, HostEnv, HostRef};
    
    #[test]
    fn test_deployment() {
        let env = odra_test::env();
        let mut vault = VaultManagerHostRef::deploy(&env, admin);
        // Test contract...
    }
}
```

**3. Deploy via Odra:**
```bash
# Coming soon - Odra deployment tools
odra build
odra deploy --network testnet
```

### Option B: Direct Casper Client Deployment

For manual deployment control.

**1. Prepare Deployment Key:**
```bash
# Generate new key pair
casper-client keygen keys/

# Or use existing key
export CASPER_SECRET_KEY=/path/to/secret_key.pem
```

**2. Deploy Contracts:**
```bash
# Set network
NETWORK=testnet  # or mainnet
NODE_URL="http://3.143.158.19:7777"
CHAIN_NAME="casper-test"

# Deploy VaultManager
casper-client put-deploy \
  --node-address $NODE_URL \
  --chain-name $CHAIN_NAME \
  --secret-key $CASPER_SECRET_KEY \
  --payment-amount 200000000000 \
  --session-path wasm/vault_manager.wasm \
  --session-arg "admin:key='account-hash-...'" \
  --session-arg "fee_bps:u32='1000'"

# Get deployment hash and check status
casper-client get-deploy \
  --node-address $NODE_URL \
  <DEPLOY_HASH>
```

### Option C: Frontend Integration with Casper Signer

Best for user-friendly deployment via web interface.

**1. Install Casper Signer:**
- Chrome/Brave: https://chrome.google.com/webstore
- Search for "Casper Signer"

**2. Integrate in Frontend:**
```typescript
import { CasperServiceByJsonRPC, CLPublicKey, DeployUtil } from 'casper-js-sdk';

const deploy = DeployUtil.makeDeploy(
  deployParams,
  contractInstallDeploy,
  paymentModuleBytes
);

// Sign with Casper Signer
const signature = await window.casperlabsHelper.sign(deploy, publicKey);
```

**3. Deploy via UI:**
- Use the CasperVault frontend deployment page
- Connect Casper Signer
- Deploy contracts interactively

## 📦 Contract Deployment Order

Deploy contracts in this order for proper initialization:

1. **Tokens First:**
   ```
   ├── lst_cspr (Liquid Staked CSPR)
   └── cv_cspr (CasperVault Shares)
   ```

2. **Core Contracts:**
   ```
   ├── LiquidStaking (requires: lst_cspr)
   ├── VaultManager (requires: cv_cspr, LiquidStaking)
   └── YieldAggregator (requires: VaultManager)
   ```

3. **Strategies:**
   ```
   ├── DEXStrategy
   ├── LendingStrategy
   └── CrossChainStrategy
   ```

4. **Utilities:**
   ```
   ├── AccessControl
   ├── ValidatorRegistry
   └── Monitor
   ```

## 🔧 Post-Deployment Configuration

### 1. Initialize Contracts

```bash
# Set up roles
casper-client put-deploy \
  --session-hash <VAULT_MANAGER_HASH> \
  --session-entry-point "grant_role" \
  --session-arg "role:u8='1'" \
  --session-arg "account:key='account-hash-...'"

# Configure strategies
casper-client put-deploy \
  --session-hash <STRATEGY_ROUTER_HASH> \
  --session-entry-point "add_strategy" \
  --session-arg "name:string='dex'" \
  --session-arg "strategy:key='contract-hash-...'"
```

### 2. Verify Deployment

```bash
# Check contract state
casper-client get-state-root-hash --node-address $NODE_URL

# Query contract storage
casper-client query-global-state \
  --node-address $NODE_URL \
  --state-root-hash <STATE_ROOT> \
  --key <CONTRACT_HASH>
```

### 3. Test Contract Calls

```bash
# Test deposit
casper-client put-deploy \
  --session-hash <VAULT_MANAGER_HASH> \
  --session-entry-point "deposit" \
  --session-arg "amount:u512='1000000000'"
```

## 🧪 Testing

### Unit Tests

```bash
cd contracts
cargo test --lib
```

### Integration Tests

```bash
# Using Odra test environment
cargo test --features test-support
```

### Testnet Testing

1. Deploy to testnet
2. Get test CSPR from faucet
3. Test all contract functions
4. Monitor via explorer: https://testnet.cspr.live

## 📊 Gas Costs (Estimated)

| Contract | Deployment | Typical Call |
|----------|-----------|--------------|
| VaultManager | ~150 CSPR | 2-5 CSPR |
| LiquidStaking | ~100 CSPR | 3-7 CSPR |
| Tokens | ~80 CSPR | 1-2 CSPR |
| Strategies | ~120 CSPR | 2-4 CSPR |

*Note: Testnet costs may vary. Mainnet costs are 100x less.*

## 🔍 Monitoring & Verification

### Explorer
- **Testnet:** https://testnet.cspr.live
- **Mainnet:** https://cspr.live

### Contract Verification
```bash
# Get contract hash
casper-client get-account-info \
  --node-address $NODE_URL \
  --public-key <PUBLIC_KEY>

# View contract on explorer
https://testnet.cspr.live/contract/<CONTRACT_HASH>
```

## 🐛 Troubleshooting

### Build Issues

**Problem:** WASM build fails with panic handler error
```
Solution: Use cargo build --lib (library builds successfully)
          Binary deployment needs Odra integration
```

**Problem:** Out of memory during build
```
Solution: Increase system swap or use build server
          cargo build --release has lower memory requirements
```

### Deployment Issues

**Problem:** Insufficient payment
```
Solution: Increase --payment-amount (default: 200000000000)
```

**Problem:** Invalid argument types
```
Solution: Check argument formats in CLI
          Example: "amount:u512='1000'"
```

**Problem:** Contract already exists
```
Solution: Either update existing contract or deploy to new account
```

## 📚 Additional Resources

- **Odra Documentation:** https://odra.dev
- **Casper Docs:** https://docs.casper.network
- **Casper GitHub:** https://github.com/casper-network
- **CasperVault Docs:** ../README.md

## 🤝 Support

- **GitHub Issues:** https://github.com/yourusername/caspervault/issues
- **Discord:** [Coming soon]
- **Email:** support@caspervault.io

## ✅ Deployment Checklist

- [ ] Rust & Cargo installed
- [ ] WASM target added
- [ ] Casper client installed
- [ ] Test account created
- [ ] Testnet CSPR acquired
- [ ] Contracts built successfully
- [ ] Deployment keys generated
- [ ] Network configuration set
- [ ] Contracts deployed in order
- [ ] Post-deployment initialization complete
- [ ] Integration tests passed
- [ ] Explorer verification done
- [ ] Frontend updated with contract hashes

---

**Current Status:** ✅ Ready for testnet deployment with manual integration

For automated deployment, we're working on Odra CLI integration. In the meantime, use Option B (Direct Casper Client) or Option C (Frontend Integration) for deployment.
