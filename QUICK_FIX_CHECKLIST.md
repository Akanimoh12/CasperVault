# ⚡ Quick Fix Checklist - 416 Errors

## 🎯 Use This for Fast Execution

Copy and paste these prompts in order to an AI coding assistant.

---

## 🔴 PHASE 1: Critical Contract Fixes (3 hours)

### ✅ Step 1: Fix Event Definitions
```
```

### ✅ Step 2: Fix UserDeposit Struct
```
In contracts/src/core/vault_manager.rs, find UserDeposit struct and:
1. Add field: total_shares: U512,
2. Add derives if missing: #[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
```

### ✅ Step 3: Fix WithdrawalRequest Struct
```
In contracts/src/core/vault_manager.rs, find WithdrawalRequest struct and:
1. Add field: unlock_time: u64,
2. Add derives: #[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
```

### ✅ Step 4: Add Missing VaultManager Fields
```
In contracts/src/core/vault_manager.rs, add to VaultManager struct:

next_withdrawal_id: Var<U256>,
last_management_fee_collection: Var<u64>,
liquid_staking_address: Var<Address>,
strategy_router_address: Var<Address>,
cv_cspr_token_address: Var<Address>,
max_deposit_per_tx: Var<U512>,
```

**Run**: `cargo check` (should see ~200 fewer errors)

---

## 🟡 PHASE 2: Event Field Fixes (30 min)

### ✅ Step 5: Fix Withdraw Event
```
In contracts/src/types/events.rs, change Withdraw struct:
- Rename 'shares_burned' to 'shares' (or add shares as alias)
```

### ✅ Step 6: Fix WithdrawalCompleted Event
```
In contracts/src/types/events.rs, update WithdrawalCompleted:

#[derive(Event, Debug, PartialEq, Eq)]
pub struct WithdrawalCompleted {
    pub request_id: U256,
    pub user: Address,
    pub cspr_amount: U512,
    pub assets: U512,
    pub shares: U512,
    pub timestamp: u64,
}
```

---

## 🟢 PHASE 3: Type & Method Fixes (1.5 hours)

### ✅ Step 7: Fix daily_deposits Type
```
In contracts/src/core/vault_manager.rs:
1. Find daily_deposits field declaration
2. If it's Mapping<Address, (u64, U512)>, change to Mapping<Address, U512>
3. Update all usages at lines 630, 782, 787, 799 to just use U512
```

### ✅ Step 8: Fix Mapping remove() Calls
```
In contracts/src/core/vault_manager.rs at lines 330, 331, 521, 522:
Replace:
  self.user_shares.remove(&caller);
With:
  self.user_shares.set(&caller, U512::zero());

Do same for user_deposits with default UserDeposit.
```

### ✅ Step 9: Fix Address References
```
In contracts/src/core/vault_manager.rs:
Line 218: Change check_daily_deposit_limit(caller, amount) to check_daily_deposit_limit(&caller, amount)
Line 247: Change update_user_deposit_tracking(caller, ...) to update_user_deposit_tracking(&caller, ...)
```

### ✅ Step 10: Fix Type Conversions
```
Line 466: Change request_id to U256::from(request_id)
Line 420: Remove unreachable!() and properly handle the None case
```

**Run**: `cargo check` (should see <20 errors remaining)

---

## 🔵 PHASE 4: Cleanup (30 min)

### ✅ Step 11: Remove Unused Imports
```
contracts/src/core/vault_manager.rs - Remove:
- InstantWithdraw (line 4)
- CvCspr, LstCspr (line 7)

frontend/src/services/api.ts - Remove:
- axios (if unused)
- API_BASE_URL (if unused)

frontend/src/hooks/useVault.ts - Remove:
- MOCK_APY

frontend/src/components/modals/DepositModal.tsx - Remove:
- parseCSPR (if unused)
- address variable (if unused)
- tx variable (if unused)
```

---

## 🎯 PHASE 5: Frontend Fix (15 min)

### ✅ Step 12: Verify Wallet Service
```
Check frontend/src/services/wallet.ts has at bottom:

export class WalletService { ... }
export const walletService = new WalletService();

If missing, add exports.
If present, run: rm -rf node_modules/.vite && npm run dev
```

---

## ✅ FINAL VALIDATION

```bash
# Contracts
cd contracts
cargo clean
cargo check --all-features
cargo test

# Frontend  
cd ../frontend
npm run build
npm run lint

# Expected: ✅ 0 errors
```

---

## 🎉 ONE-LINER VERSION (If You're Brave)

Run this prompt to fix everything at once:

```
Fix all CasperVault errors:

CONTRACTS (contracts/src/):
1. types/events.rs - Add InstantWithdrawal, ManagementFeesCollected, FundsRescued events
2. core/vault_manager.rs - Add to UserDeposit: total_shares:U512
3. core/vault_manager.rs - Add to WithdrawalRequest: unlock_time:u64  
4. core/vault_manager.rs - Add to VaultManager: next_withdrawal_id, last_management_fee_collection, liquid_staking_address, strategy_router_address, cv_cspr_token_address, max_deposit_per_tx
5. types/events.rs - Withdraw: rename shares_burned to shares
6. types/events.rs - WithdrawalCompleted: add assets, shares, timestamp fields
7. core/vault_manager.rs - Change daily_deposits to Mapping<Address,U512>
8. core/vault_manager.rs - Replace .remove() with .set(&key, default)
9. core/vault_manager.rs - Lines 218,247: pass &caller not caller
10. core/vault_manager.rs - Line 466: U256::from(request_id)
11. Remove unused imports everywhere

FRONTEND (frontend/src/):
1. Verify wallet.ts exports walletService properly
2. Remove unused imports from api.ts, useVault.ts, DepositModal.tsx

Add CLTyped, ToBytes, FromBytes derives to UserDeposit and WithdrawalRequest structs.
```

---

## 📊 Progress Tracker

Track which phase you've completed:

- [ ] Phase 1: Critical Fixes (Steps 1-4) → ~200 errors fixed
- [ ] Phase 2: Events (Steps 5-6) → ~10 errors fixed  
- [ ] Phase 3: Types (Steps 7-10) → ~150 errors fixed
- [ ] Phase 4: Cleanup (Step 11) → ~10 warnings fixed
- [ ] Phase 5: Frontend (Step 12) → ~36 errors fixed
- [ ] Final Validation → 0 errors

**Total**: 416 errors → 0 errors ✅

---

## 🆘 If Stuck

**Contracts not compiling?**
```bash
cd contracts
cargo update -p odra_macros
cargo clean && cargo build
```

**Frontend module errors?**
```bash
cd frontend
rm -rf node_modules .vite dist
npm install
npm run dev
```

**Still have errors?**
- Check Odra version: `cargo tree | grep odra`
- Check TypeScript: `npx tsc --noEmit`
- Read the full guide: `FIX_ALL_ERRORS.md`

---

## ⏱️ Time Estimate

- **If following phases**: 5-6 hours
- **If using one-liner**: 3-4 hours (higher risk)
- **With AI assistant**: 2-3 hours (recommended)

---

## 💡 Pro Tip

Start with Phase 1, run `cargo check` after each step. This way you'll see progress and catch issues early!

---

**You got this! 🚀**
