# CasperVault Deployment Guide

Complete guide for deploying CasperVault smart contracts to Casper testnet and mainnet.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Configuration](#configuration)
4. [Testnet Deployment](#testnet-deployment)
5. [Mainnet Deployment](#mainnet-deployment)
6. [Post-Deployment](#post-deployment)
7. [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Tools

- **Rust** >= 1.70.0
- **Cargo** latest stable
- **Odra CLI** >= 0.8.0
- **casper-client** >= 2.0.0
- **jq** (JSON processor)
- **Git**

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install Odra CLI
cargo install odra-casper-livenet-env

# Install casper-client
cargo install casper-client

# Install jq
sudo apt-get install jq  # Ubuntu/Debian
# or
brew install jq  # macOS
```

### Funding Requirements

**Testnet:**
- Request CSPR from [testnet faucet](https://testnet.cspr.live/tools/faucet)
- Minimum: 500 CSPR for deployment + 100 CSPR for testing

**Mainnet:**
- Production deployment: ~200 CSPR for gas
- Initial liquidity: 10,000+ CSPR recommended
- Emergency reserve: 1,000 CSPR

## Environment Setup

### 1. Clone Repository

```bash
git clone https://github.com/yourusername/CasperVault.git
cd CasperVault
```

### 2. Build Contracts

```bash
cd contracts
cargo build --release
```

Verify build artifacts:
```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```

Expected outputs:
- `vault_manager.wasm`
- `liquid_staking.wasm`
- `strategy_router.wasm`
- `yield_aggregator.wasm`
- `dex_strategy.wasm`
- `lending_strategy.wasm`
- `crosschain_strategy.wasm`
- `lst_cspr.wasm`
- `cv_cspr.wasm`

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test vault_unit_tests
cargo test --test integration
cargo test --test e2e

# Generate coverage report
cargo tarpaulin --out Html
```

Ensure **minimum 90% coverage** before deployment.

## Configuration

### Testnet Configuration

Edit `scripts/config/testnet.json`:

```json
{
  "network": "casper-test",
  "node_address": "http://95.216.67.162:7777",
  "chain_name": "casper-test",
  "initial_validators": [
    {
      "address": "01360af61b50cdcb7b92cffe2c99315d413d34ef77fadee0c105cc4f1d4120f986",
      "name": "validator-1",
      "commission": 5,
      "uptime_target": 98
    }
  ],
  "treasury_address": "YOUR_TREASURY_ADDRESS",
  "admin_keys": ["YOUR_ADMIN_KEY"],
  "fees": {
    "performance_fee_bps": 1000,
    "management_fee_bps": 200
  }
}
```

**Key Parameters:**
- `performance_fee_bps`: 1000 = 10% of profits
- `management_fee_bps`: 200 = 2% annual
- `instant_withdrawal_fee_bps`: 50 = 0.5%

### Mainnet Configuration

Edit `scripts/config/mainnet.json`:

**CRITICAL:** Replace all `REPLACE_BEFORE_MAINNET` placeholders with actual values.

```json
{
  "network": "casper",
  "node_address": "http://65.21.235.219:7777",
  "chain_name": "casper",
  "initial_validators": [
    // Use verified mainnet validators
  ],
  "multisig_config": {
    "required_signatures": 3,
    "total_signers": 5,
    "timelock_duration": 86400
  },
  "safety_checks": {
    "require_audit_report": true,
    "require_multisig_confirmation": true,
    "require_timelock": true
  }
}
```

### Validator Selection Criteria

All validators must meet:
- **Uptime**: > 95% (recommended: > 98%)
- **Commission**: < 10% (recommended: < 7%)
- **Verified**: Identity verified
- **Stake**: > 1M CSPR
- **History**: Active for > 6 months

## Testnet Deployment

### Step 1: Dry Run

Always perform dry run first:

```bash
DRY_RUN=true bash scripts/deploy/deploy-testnet.sh
```

Review the output carefully. No actual deployments will occur.

### Step 2: Actual Deployment

```bash
bash scripts/deploy/deploy-testnet.sh
```

**Deployment Sequence:**
1. Build contracts
2. Deploy token contracts (lstCSPR, cvCSPR)
3. Deploy core contracts (VaultManager, LiquidStaking, StrategyRouter)
4. Deploy strategies (DEX, Lending, CrossChain)
5. Initialize contracts
6. Set up roles and permissions
7. Verify deployment
8. Save contract addresses

**Expected Duration:** 10-15 minutes

### Step 3: Verify Deployment

```bash
bash scripts/verify/verify-deployment.sh testnet
```

Checks:
- ✓ All contracts deployed
- ✓ Addresses valid
- ✓ Permissions configured
- ✓ Validators whitelisted
- ✓ Fees configured
- ✓ Allocations correct

### Step 4: Test Full Flow

```bash
bash scripts/deploy/test-full-flow.sh
```

Simulates:
1. Get CSPR from faucet
2. Deposit to vault
3. Verify staking
4. Verify strategy deployment
5. Simulate yield accrual
6. Trigger compound
7. Withdraw with profit

### Step 5: Monitor Health

```bash
bash scripts/monitor/check-health.sh testnet
```

Monitor for 24-48 hours before considering stable.

## Mainnet Deployment

### Pre-Deployment Checklist

- [ ] Testnet deployment successful
- [ ] All tests passing (>90% coverage)
- [ ] Security audit completed
- [ ] Audit report reviewed
- [ ] Configuration verified (no placeholders)
- [ ] Multi-sig wallets configured
- [ ] Admin keys secured
- [ ] Emergency procedures documented
- [ ] Monitoring set up
- [ ] Team briefed

### Safety Checks

Mainnet deployment includes automatic safety checks:

1. **Configuration Verification**
   - No placeholder values
   - All addresses valid

2. **Audit Verification**
   - Audit reports present in `audits/`
   - Reports recent (< 30 days)

3. **Multi-sig Verification**
   - Required signatures configured (minimum 3/5)
   - Admin keys match multi-sig

4. **Testnet Verification**
   - Testnet deployment exists
   - Testnet has been tested

5. **Code Integrity**
   - No uncommitted changes
   - Commit hash recorded

6. **Dry Run Requirement**
   - Dry run must be completed first

### Step 1: Dry Run (Required)

```bash
DRY_RUN=true bash scripts/deploy/deploy-mainnet.sh
```

**This is REQUIRED** before actual deployment.

### Step 2: Actual Deployment

```bash
DRY_RUN=false bash scripts/deploy/deploy-mainnet.sh
```

**Confirmation Required:**
Type `DEPLOY TO MAINNET` to proceed.

**Deployment Process:**
- Multi-sig required for each contract
- Timelock enforced (24 hours minimum)
- Automatic backups created
- Detailed logging

**Expected Duration:** 2-3 days (including timelocks)

### Step 3: Verification

```bash
bash scripts/verify/verify-deployment.sh mainnet
```

### Step 4: Initial Testing

**Test with small amounts first:**

```bash
# Test deposit (10 CSPR)
# Manual process - use Casper wallet

# Wait 24 hours
# Monitor health continuously

# Test withdrawal
# Verify accounting
```

### Step 5: Gradual Rollout

**Week 1:**
- Announce to small group (10-20 users)
- Max deposit: 1,000 CSPR per user
- Monitor 24/7

**Week 2:**
- Increase to 100 users
- Max deposit: 5,000 CSPR per user
- Continue monitoring

**Week 3+:**
- Full public launch
- Remove deposit limits
- Ongoing monitoring

## Post-Deployment

### Monitoring Setup

**Continuous Health Checks:**
```bash
# Run every 5 minutes
*/5 * * * * /path/to/scripts/monitor/check-health.sh mainnet >> /var/log/caspervault/health.log 2>&1
```

**Event Monitoring:**
```bash
# Run continuously
bash scripts/monitor/monitor-events.sh mainnet
```

**Alerts:**
Configure alerts for:
- Pause events
- Strategy failures
- Large withdrawals (> 10% TVL)
- Validator issues
- Anomaly detection triggers

### Regular Maintenance

**Daily:**
- Review health logs
- Check APY performance
- Verify compound execution
- Monitor user activity

**Weekly:**
- Review validator performance
- Check strategy allocations
- Analyze yield sources
- Review gas costs

**Monthly:**
- Rebalance validators if needed
- Optimize strategy allocations
- Review fee structure
- Update documentation

### Upgrade Process

When upgrading contracts:

```bash
# 1. Test upgrade on testnet first
bash scripts/deploy/upgrade-contract.sh VaultManager ./new_vault.wasm testnet

# 2. Verify upgrade
bash scripts/verify/verify-upgrade.sh VaultManager testnet

# 3. If successful, upgrade mainnet
bash scripts/deploy/upgrade-contract.sh VaultManager ./new_vault.wasm mainnet
```

**Mainnet upgrades require:**
- Multi-sig approval (3/5)
- 48-hour timelock
- Automatic backup
- Rollback capability (7 days)

## Troubleshooting

### Common Issues

#### 1. Build Failures

**Problem:** `cargo build --release` fails

**Solution:**
```bash
# Clean build
cargo clean
cargo build --release

# Update dependencies
cargo update

# Check Rust version
rustup update stable
```

#### 2. Deployment Fails

**Problem:** Contract deployment transaction fails

**Causes:**
- Insufficient gas
- Network issues
- Contract size too large

**Solution:**
```bash
# Increase gas price
export GAS_PRICE=2  # Default is 1

# Use different node
export NODE_ADDRESS="http://alternative-node:7777"

# Check contract size
ls -lh target/wasm32-unknown-unknown/release/*.wasm
# Should be < 1MB each
```

#### 3. Verification Fails

**Problem:** `verify-deployment.sh` reports failures

**Solution:**
```bash
# Check addresses file
cat scripts/addresses.json

# Manually verify contract
casper-client query-global-state \
  --node-address http://95.216.67.162:7777 \
  --state-root-hash <hash> \
  --key <contract-address>

# Re-run verification with verbose output
bash -x scripts/verify/verify-deployment.sh
```

#### 4. Test Flow Fails

**Problem:** `test-full-flow.sh` fails at deposit

**Causes:**
- Contract not initialized
- Insufficient test funds
- Contract paused

**Solution:**
```bash
# Check contract status
# (query contract for pause status)

# Ensure faucet funds received
# Check balance before test

# Review logs
tail -f test-flow-*.log
```

#### 5. Mainnet Safety Check Fails

**Problem:** Deployment blocked by safety checks

**Solution:**
```bash
# Check specific failure
bash scripts/deploy/deploy-mainnet.sh 2>&1 | grep ERROR

# Common fixes:
# - Replace placeholders in mainnet.json
# - Add audit reports to audits/
# - Complete dry run first
# - Ensure testnet deployment exists
```

### Getting Help

**Documentation:**
- [Architecture Guide](ARCHITECTURE.md)
- [API Reference](CONTRACT_API.md)
- [Admin Guide](ADMIN_GUIDE.md)

**Support:**
- GitHub Issues: https://github.com/yourusername/CasperVault/issues
- Discord: [Your Discord Link]
- Email: support@caspervault.io

**Emergency:**
- Emergency pause: bash scripts/manage/emergency-pause.sh
- Contact admins immediately
- Review emergency runbook

## Best Practices

### Security

1. **Never commit private keys** to repository
2. **Use hardware wallets** for mainnet admin keys
3. **Test everything** on testnet first
4. **Keep backups** of all deployments
5. **Monitor continuously** after deployment
6. **Have rollback plan** ready

### Operations

1. **Document all changes** in deployment logs
2. **Communicate** with users about updates
3. **Gradual rollout** of new features
4. **Regular audits** of configurations
5. **Emergency procedures** practiced regularly

### Development

1. **Branch strategy**: main → develop → feature branches
2. **Code review**: Required for all changes
3. **Testing**: >90% coverage mandatory
4. **Audits**: Before major releases
5. **Documentation**: Keep up to date

## Appendix

### Gas Cost Estimates

| Operation | Testnet Gas | Mainnet Gas | CSPR Cost* |
|-----------|-------------|-------------|------------|
| Deploy VaultManager | 150M | 150M | 1.5 CSPR |
| Deploy Token | 100M | 100M | 1.0 CSPR |
| Deploy Strategy | 120M | 120M | 1.2 CSPR |
| Initialize | 50M | 50M | 0.5 CSPR |
| **Total Deployment** | **~2B** | **~2B** | **~20 CSPR** |

*At 1 CSPR = 100M motes gas

### Network Information

**Testnet:**
- Network: casper-test
- Node: http://95.216.67.162:7777
- Explorer: https://testnet.cspr.live
- Faucet: https://testnet.cspr.live/tools/faucet

**Mainnet:**
- Network: casper
- Node: http://65.21.235.219:7777
- Explorer: https://cspr.live
- RPC: Multiple available

### File Structure

```
CasperVault/
├── contracts/           # Smart contracts
├── scripts/
│   ├── deploy/         # Deployment scripts
│   ├── verify/         # Verification scripts
│   ├── manage/         # Management scripts
│   ├── monitor/        # Monitoring scripts
│   └── config/         # Network configurations
├── docs/               # Documentation
├── audits/             # Security audit reports
└── backups/            # Deployment backups
```

---

**Last Updated:** 2025-01-01  
**Version:** 1.0.0  
**Maintainer:** CasperVault Team
