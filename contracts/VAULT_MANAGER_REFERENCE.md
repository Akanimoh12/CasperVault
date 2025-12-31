# VaultManager Quick Reference

## 🎯 Core Functions

### User Operations

**deposit() → U512**
- Deposits CSPR, receives cvCSPR shares
- Rate limited: 10K per tx, 50K per day
- Charges 2% annual management fee
- Returns: shares minted

**withdraw(shares: U512) → U512**
- Burns shares, receives CSPR
- Uses instant pool first, then strategies
- Charges performance fee on profits
- Returns: CSPR amount (after fees)

**request_withdrawal(shares: U512) → u64**
- Creates 7-day time-locked request
- No instant fee (saves 0.5%)
- Returns: request_id

**complete_withdrawal(request_id: u64) → U512**
- Completes after 7 days
- Only performance fee charged
- Returns: CSPR amount

**instant_withdraw(shares: U512) → U512**
- Immediate withdrawal from pool
- Charges 0.5% instant fee + performance fee
- Limited by pool liquidity
- Returns: CSPR amount

### View Functions

**get_user_shares(user) → U512**
- User's share balance

**get_user_assets(user) → U512**
- User's CSPR value

**get_share_price() → U512**
- Current share price (scaled 1e9)

**convert_to_shares(assets) → U512**
- Calculate shares for CSPR amount

**convert_to_assets(shares) → U512**
- Calculate CSPR for shares

**max_deposit(user) → U512**
- Remaining daily deposit limit

**max_withdraw(user) → U512**
- User's withdrawable amount

**get_instant_pool_balance() → U512**
- Available instant liquidity

## 💰 Fee Structure

| Fee Type | Default | Range | Charged On |
|----------|---------|-------|-----------|
| Performance | 10% | 0-50% | Profits only |
| Management | 2% annual | 0-10% | Time-based |
| Instant Withdrawal | 0.5% | 0-5% | Instant withdrawals |

## 🎲 Share Calculation (ERC-4626)

```rust
// First deposit
shares = assets  // 1:1 ratio

// Subsequent deposits
shares = (assets * totalShares) / totalAssets

// Withdrawal
assets = (shares * totalAssets) / totalShares
```

## 🔒 Limits & Security

| Parameter | Default | Range |
|-----------|---------|-------|
| Max deposit per tx | 10,000 CSPR | Configurable |
| Max deposit per day | 50,000 CSPR | Configurable |
| Withdrawal timelock | 7 days | 1-30 days |
| Min shares | 0.01 CSPR | Anti-dust |
| Instant pool target | 5% | 0-50% |

## 🔑 Admin Functions

**set_fees(perf, mgmt, instant)**
- Update fee rates

**update_deposit_limits(max_tx, max_day)**
- Change rate limits

**set_withdrawal_timelock(seconds)**
- Adjust timelock period

**set_instant_pool_target(bps)**
- Change pool target %

**pause() / unpause()**
- Emergency controls

**rescue_funds(token, amount, recipient)**
- Recover stuck assets

**collect_fees(recipient)**
- Transfer accumulated fees

**collect_management_fees()**
- Mint fee shares (keeper only)

## 📊 State Variables

**total_shares: U512**
- Total cvCSPR shares outstanding

**user_shares: Mapping<Address, U512>**
- User share balances

**user_deposits: Mapping<Address, UserDeposit>**
- Cost basis tracking

**instant_withdrawal_pool: U512**
- Liquidity pool balance

**fees_collected: U512**
- Accumulated fees

**withdrawal_requests: Mapping<u64, WithdrawalRequest>**
- Pending withdrawal requests

## 🔄 Flow Diagrams

### Deposit Flow
```
User CSPR
    ↓
Rate Limit Check
    ↓
Management Fee Collection
    ↓
Stake → lstCSPR (LiquidStaking)
    ↓
Calculate Shares (ERC-4626)
    ↓
Update User Tracking
    ↓
Mint cvCSPR (Token)
    ↓
Deploy to Strategies (5% to pool, 95% to strategies)
    ↓
Emit Deposit Event
```

### Withdrawal Flow (Regular)
```
User Shares
    ↓
Validate Balance
    ↓
Calculate Assets (ERC-4626)
    ↓
Check Instant Pool
    ↓
[If insufficient] Withdraw from Strategies
    ↓
[If needed] Unstake lstCSPR → CSPR
    ↓
Calculate Performance Fee
    ↓
Burn Shares
    ↓
Transfer CSPR to User
    ↓
Emit Withdraw Event
```

### Time-Locked Withdrawal Flow
```
request_withdrawal(shares)
    ↓
Create Request (7-day lock)
    ↓
Lock User Shares
    ↓
[Wait 7 days]
    ↓
complete_withdrawal(request_id)
    ↓
Validate Timelock
    ↓
Process Withdrawal (no instant fee)
    ↓
Transfer CSPR
```

## 📈 Example Scenarios

### Scenario 1: First Deposit
```
User deposits: 1,000 CSPR
Total shares: 0
Total assets: 0

Shares received: 1,000 (1:1 ratio)
Share price: 1.0
```

### Scenario 2: Deposit After Yield
```
Total shares: 1,000
Total assets: 1,200 CSPR (20% yield)
Share price: 1.2

User deposits: 1,200 CSPR
Shares received: (1,200 * 1,000) / 1,200 = 1,000
New share price: 1.2 (unchanged)
```

### Scenario 3: Withdrawal With Profit
```
User: 1,000 shares at cost basis 1,000 CSPR
Current share price: 1.2
Withdrawal value: 1,200 CSPR

Profit: 200 CSPR
Performance fee (10%): 20 CSPR
User receives: 1,180 CSPR
```

### Scenario 4: Fee Comparison
```
Position: 1,000 shares worth 1,100 CSPR
Profit: 100 CSPR
Performance fee: 10 CSPR

Instant withdrawal:
- Instant fee: 5.5 CSPR (0.5%)
- Total fees: 15.5 CSPR
- Received: 1,084.5 CSPR

Time-locked withdrawal:
- Instant fee: 0 CSPR
- Total fees: 10 CSPR
- Received: 1,090 CSPR
- Savings: 5.5 CSPR
```

## 🧮 Key Formulas

### Share Price
```
share_price = total_assets / total_shares
```

### Performance Fee
```
profit = withdrawal_amount - cost_basis
if profit > 0:
    fee = profit * performance_fee_bps / 10000
else:
    fee = 0
```

### Management Fee (Annual)
```
time_fraction = time_elapsed / 31536000  // 1 year in seconds
fee_shares = total_shares * (mgmt_fee_bps / 10000) * time_fraction
```

### Strategy Deployment
```
target_pool = total_assets * pool_target_bps / 10000
pool_deficit = target_pool - current_pool

if deposit_amount <= pool_deficit:
    deploy_amount = 0  // Fill pool only
else:
    deploy_amount = deposit_amount - pool_deficit
```

## 🔍 Integration TODOs

1. **Line ~170**: `liquid_staking.stake(amount)`
2. **Line ~210**: `cv_cspr_token.mint(caller, shares)`
3. **Line ~220**: `strategy_router.allocate(amount)`
4. **Line ~619**: `strategy_router.withdraw(amount)`
5. **Line ~621**: `liquid_staking.unstake(amount)`
6. **Line ~645**: `cv_cspr_token.burn(caller, shares)`
7. **Line ~760**: `cv_cspr_token.burn(caller, shares)`
8. **Line ~843**: `cv_cspr_token.burn(caller, shares)`
9. **Line ~928**: `liquid_staking.balance_of()` and `convert_to_assets()`

## 📚 Files to Review

- `src/core/vault_manager.rs` (1,276 lines) - Main implementation
- `tests/unit/vault_manager_tests.rs` (516 lines) - Unit tests
- `tests/integration/vault_integration_tests.rs` (698 lines) - Integration tests
- `PROMPT_3_COMPLETE.md` (890 lines) - Detailed documentation
- `PROJECT_STATUS.md` (775 lines) - Project overview

## 🎯 Testing Checklist

- [ ] Execute 51 unit tests in Odra environment
- [ ] Execute 29 integration scenarios
- [ ] Test all withdrawal mechanisms
- [ ] Verify fee calculations
- [ ] Test rate limiting
- [ ] Test access control
- [ ] Test pause functionality
- [ ] Test edge cases
- [ ] Gas optimization analysis
- [ ] Security audit

## 🚀 Deployment Checklist

- [ ] Deploy LiquidStaking contract
- [ ] Deploy cvCSPR token
- [ ] Deploy StrategyRouter
- [ ] Deploy VaultManager
- [ ] Link contracts (set addresses)
- [ ] Grant roles (admin, keeper, guardian)
- [ ] Set initial parameters
- [ ] Fund instant pool
- [ ] Verify on block explorer
- [ ] Open for deposits

---

*Quick Reference - VaultManager v1.0*  
*For detailed documentation, see PROMPT_3_COMPLETE.md*
