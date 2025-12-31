# 🔧 CasperVault Error Fix Guide
**Complete Prompt-Based Solution for All 416 Errors**

---

## 📊 Error Summary

| Category | Count | Severity | Time to Fix |
|----------|-------|----------|-------------|
| **Smart Contract (Rust)** | ~350 | 🔴 Critical | 4-6 hours |
| **Frontend (TypeScript)** | ~66 | 🟡 Medium | 2-3 hours |
| **Total** | **416** | Mixed | **6-9 hours** |

---

## 🎯 Execution Strategy

Fix errors in this order to prevent cascading failures:

1. **Smart Contract Type System** (20 min) - Fix struct definitions
2. **Smart Contract Missing Fields** (30 min) - Add required fields
3. **Smart Contract Event System** (45 min) - Complete event definitions
4. **Smart Contract Mapping Methods** (60 min) - Implement CLTyped traits
5. **Frontend Wallet Service** (30 min) - Complete missing service
6. **Frontend Type Errors** (45 min) - Fix imports and types
7. **Final Validation** (30 min) - Run tests and build

---

## 🔴 CRITICAL: Smart Contract Fixes

### **PROMPT 1: Fix Event System (Priority 1)**

```prompt
I need you to fix the event system in my Rust/Odra smart contract. I have 3 missing event struct definitions that are being emitted but not defined.

File: contracts/src/types/events.rs

Add these missing event structs after line 48 (after WithdrawalCompleted):

1. InstantWithdrawal - for instant withdrawal events
2. ManagementFeesCollected - for fee collection events  
3. FundsRescued - for emergency fund rescue events

Each event should:
- Derive Event, Debug, PartialEq, Eq
- Use appropriate types (Address, U512, u64)
- Follow the existing event pattern in the file

Also remove unused import of InstantWithdraw (it's actually InstantWithdrawal).

Please implement all 3 events with proper fields.
```

**Expected Result**: ✅ 3 event errors fixed (~45 errors resolved)

---

### **PROMPT 2: Fix UserDeposit Struct (Priority 1)**

```prompt
I need to add a missing field to the UserDeposit struct in my Rust smart contract.

File: contracts/src/core/vault_manager.rs

The UserDeposit struct is missing the 'total_shares' field. Find the UserDeposit struct definition and add:
- total_shares: U512 field

Also ensure UserDeposit derives CLTyped and ToBytes/FromBytes traits properly so it can be used in Odra::Mapping.

If the struct doesn't have #[derive(CLTyped, ToBytes, FromBytes)] add it.
```

**Expected Result**: ✅ UserDeposit can be stored in Mapping (~50 errors resolved)

---

### **PROMPT 3: Fix WithdrawalRequest Struct (Priority 1)**

```prompt
I need to fix the WithdrawalRequest struct in my Rust/Odra smart contract.

File: contracts/src/core/vault_manager.rs

Issues:
1. WithdrawalRequest is missing 'unlock_time: u64' field
2. Struct needs CLTyped, ToBytes, FromBytes traits to work with Odra::Mapping

Please:
- Add unlock_time field to struct definition
- Add proper derives: #[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
- Ensure all fields are properly typed for Odra serialization
```

**Expected Result**: ✅ WithdrawalRequest serialization works (~40 errors resolved)

---

### **PROMPT 4: Add Missing VaultManager Fields (Priority 1)**

```prompt
I need to add missing fields to the VaultManager module in my Rust/Odra smart contract.

File: contracts/src/core/vault_manager.rs

Find the #[odra::module] struct VaultManager and add these missing fields:

1. next_withdrawal_id: Var<U256>
2. last_management_fee_collection: Var<u64>
3. liquid_staking_address: Var<Address>
4. strategy_router_address: Var<Address>
5. cv_cspr_token_address: Var<Address>
6. max_deposit_per_tx: Var<U512>

All should be private fields following the existing pattern (e.g., access_control, pausable, etc).
```

**Expected Result**: ✅ All field access errors resolved (~60 errors resolved)

---

### **PROMPT 5: Fix Withdraw Event Fields (Priority 2)**

```prompt
I need to fix the Withdraw event struct in my Rust smart contract to match how it's being used.

File: contracts/src/types/events.rs

The Withdraw event has 'shares_burned' field but code tries to use 'shares' field. 

Please update the Withdraw event struct to include BOTH fields:
- shares_burned: U512 (existing)
- shares: U512 (alias or additional field)

Or rename shares_burned to shares if that's the intended design.

Check vault_manager.rs line 346 to see how it's being used.
```

**Expected Result**: ✅ Event emission matches struct definition (~5 errors resolved)

---

### **PROMPT 6: Fix WithdrawalCompleted Event Fields (Priority 2)**

```prompt
I need to update the WithdrawalCompleted event struct in my Rust smart contract.

File: contracts/src/types/events.rs

Current fields: request_id, user, cspr_amount

Code at vault_manager.rs line 466-469 tries to emit with these fields:
- request_id (works)
- assets (missing)
- shares (missing)
- timestamp (missing)

Please update WithdrawalCompleted event to include all 4 fields, or update the emission code in vault_manager.rs to match the current struct definition. I prefer updating the event struct.
```

**Expected Result**: ✅ Event matches emission code (~4 errors resolved)

---

### **PROMPT 7: Fix daily_deposits Mapping Type (Priority 2)**

```prompt
I need to fix type mismatches with the daily_deposits Mapping in my Rust smart contract.

File: contracts/src/core/vault_manager.rs

The daily_deposits field appears to be declared as Mapping<Address, (u64, U512)> but code tries to use it as Mapping<Address, U512>.

Check around lines 630, 782, 787, 799 where daily_deposits is used.

Either:
1. Change the Mapping type to just U512 if date tracking isn't needed
2. OR update all usages to work with (u64, U512) tuples properly

Please analyze the code and choose the best approach, then fix all related usages consistently.
```

**Expected Result**: ✅ Type consistency across daily_deposits usage (~8 errors resolved)

---

### **PROMPT 8: Fix Mapping Methods (Priority 2)**

```prompt
I need to understand Odra::Mapping API and fix incorrect method calls.

File: contracts/src/core/vault_manager.rs

Issues at lines 330, 331, 521, 522:
- Calling .remove() on Mapping (method doesn't exist)

Please:
1. Check Odra documentation for how to remove/clear mapping entries
2. If remove() doesn't exist, use .set(&key, None) or equivalent
3. Fix all 4 occurrences

The mappings are:
- user_shares: Mapping<Address, U512>
- user_deposits: Mapping<Address, UserDeposit>
```

**Expected Result**: ✅ Proper Mapping API usage (~4 errors resolved)

---

### **PROMPT 9: Fix Address Reference Type Mismatches (Priority 2)**

```prompt
I need to fix address parameter type mismatches in method calls.

File: contracts/src/core/vault_manager.rs

Lines 218 and 247 call methods expecting &Address but pass Address directly.

Methods:
- check_daily_deposit_limit(caller, amount) - expects &Address
- update_user_deposit_tracking(caller, amount, shares) - expects &Address

Please fix by passing &caller instead of caller, or update method signatures if needed.
```

**Expected Result**: ✅ Type signatures match (~2 errors resolved)

---

### **PROMPT 10: Fix U256 Type Mismatch (Priority 3)**

```prompt
I need to fix a type mismatch with request_id in the WithdrawalCompleted event emission.

File: contracts/src/core/vault_manager.rs

Line 466: request_id is u64 but event expects U256.

Please convert u64 to U256 using U256::from(request_id) at the emission site.
```

**Expected Result**: ✅ Type conversion added (~1 error resolved)

---

### **PROMPT 11: Remove Unreachable Code Warning (Priority 4)**

```prompt
I need to fix an unreachable code warning in my Rust smart contract.

File: contracts/src/core/vault_manager.rs

Line 420 has unreachable!() that the compiler warns about. 

The issue is that the Option unwrap or match pattern makes the unreachable branch actually reachable.

Please refactor the code around lines 416-421 to properly handle the Option without unreachable code.
```

**Expected Result**: ✅ Warning removed (~1 error resolved)

---

### **PROMPT 12: Fix Unused Imports (Priority 4)**

```prompt
I need to clean up unused imports in my Rust smart contract.

File: contracts/src/core/vault_manager.rs

Remove these unused imports from the top of the file:
- InstantWithdraw (line 4) - should be InstantWithdrawal
- CvCspr (line 7)
- LstCspr (line 7)

Just remove the unused ones, but keep the imports that are actually used in the code.
```

**Expected Result**: ✅ Clean imports (~3 warnings resolved)

---

### **PROMPT 13: Fix checked_sub Return Type (Priority 2)**

```prompt
I need to fix a type mismatch with checked_sub operation.

File: contracts/src/core/vault_manager.rs

Line 630: checked_sub returns Option<U512> but code expects U512.

The code is:
```rust
max_daily.checked_sub(used).unwrap_or(U512::zero())
```

Issue: checked_sub on tuple (u64, U512) doesn't exist. This relates to the daily_deposits type issue.

Please fix this after the daily_deposits mapping type is corrected in PROMPT 7.
```

**Expected Result**: ✅ Correct arithmetic operation (~1 error resolved)

---

## 🟡 MEDIUM: Frontend Fixes

### **PROMPT 14: Frontend - Wallet Service Already Complete (Priority 1)**

```prompt
I need to verify that the wallet service file exists in my React/TypeScript frontend.

File: frontend/src/services/wallet.ts

The file already exists (I can see it has 221 lines), but TypeScript is reporting it can't find the module.

Please check if there's an export issue. The file should export:
- export class WalletService
- export const walletService = new WalletService()

If exports are missing, add them. If they exist, this might be a TypeScript cache issue.

Also verify the import paths in:
- frontend/src/store/walletStore.ts (line 3)
- frontend/src/hooks/useVault.ts (line 3)
```

**Expected Result**: ✅ Module resolution works (~30 errors resolved)

---

### **PROMPT 15: Frontend - Remove Unused Imports (Priority 3)**

```prompt
I need to clean up unused imports in my React TypeScript frontend.

Files and unused imports:

1. frontend/src/services/api.ts
   - Remove: axios (line 1) 
   - Remove: API_BASE_URL (line 2)

2. frontend/src/hooks/useVault.ts
   - Remove: walletService (line 3) - only if not used
   - Remove: MOCK_APY (line 7)

3. frontend/src/components/modals/DepositModal.tsx
   - Remove: parseCSPR from imports (line 9) - only if not used
   - Remove: address destructure (line 19) - only if not used
   - Remove: tx variable (line 35) - only if not used

Please check each file and remove only the imports that are truly unused.
```

**Expected Result**: ✅ Clean imports (~6 warnings resolved)

---

## 🎯 Quick Fix Script (Alternative Approach)

If you prefer a single comprehensive prompt:

### **PROMPT 16: Comprehensive Fix All (Use This if You Want One Prompt)**

```prompt
I need to fix all 416 compilation errors in my CasperVault project. Here's a breakdown:

**RUST CONTRACT ERRORS (350 errors):**

1. Add missing event structs to contracts/src/types/events.rs:
   - InstantWithdrawal { user, shares_burned, cspr_amount, fee_amount, timestamp }
   - ManagementFeesCollected { total_fees, timestamp }
   - FundsRescued { token, recipient, amount, timestamp }

2. Fix UserDeposit struct - add 'total_shares: U512' field and CLTyped derives

3. Fix WithdrawalRequest struct - add 'unlock_time: u64' field and CLTyped derives

4. Add missing fields to VaultManager struct:
   - next_withdrawal_id: Var<U256>
   - last_management_fee_collection: Var<u64>
   - liquid_staking_address: Var<Address>
   - strategy_router_address: Var<Address>
   - cv_cspr_token_address: Var<Address>
   - max_deposit_per_tx: Var<U512>

5. Fix Withdraw event - add 'shares' field or rename 'shares_burned' to 'shares'

6. Fix WithdrawalCompleted event - add fields: assets, shares, timestamp

7. Fix daily_deposits Mapping type consistency (currently mixed u512 and (u64, U512))

8. Replace .remove() calls with .set(&key, default_value) for Odra Mappings

9. Fix address reference mismatches - pass &caller instead of caller

10. Convert u64 to U256 for request_id in WithdrawalCompleted event

11. Fix unreachable code warning at vault_manager.rs:420

12. Remove unused imports: InstantWithdraw, CvCspr, LstCspr

**TYPESCRIPT FRONTEND ERRORS (66 errors):**

1. Verify wallet.ts exports properly (file exists but module not found)

2. Remove unused imports:
   - api.ts: axios, API_BASE_URL
   - useVault.ts: walletService, MOCK_APY
   - DepositModal.tsx: parseCSPR, address, tx (if unused)

Please fix all issues systematically, starting with the contract type system, then events, then mapping issues, then frontend imports.
```

**Expected Result**: ✅ All 416 errors fixed in one go

---

## ✅ Validation Steps

After applying fixes, run these commands:

### **Smart Contracts**
```bash
cd contracts
cargo check --all-features
cargo clippy --all-features
cargo test
```

### **Frontend**
```bash
cd frontend
npm run build
npm run lint
```

### **Expected Output**
```
✅ 0 errors
✅ 0 warnings (or only minor ones)
✅ All tests passing
```

---

## 🚀 Post-Fix Optimization

Once all errors are fixed, consider:

1. **Add Tests**: Write unit tests for new struct fields
2. **Documentation**: Update inline docs for new event structs
3. **Type Safety**: Review all U256/U512 conversions
4. **Error Handling**: Add proper error types instead of unwrap()
5. **Frontend Tests**: Add tests for wallet service integration

---

## 📚 Reference Documentation

- **Odra Framework**: https://github.com/odradev/odra
- **Casper Types**: https://docs.rs/casper-types/
- **React TypeScript**: https://react-typescript-cheatsheet.netlify.app/

---

## 🆘 Troubleshooting

### If Errors Persist:

1. **Clear Build Cache**
   ```bash
   cd contracts && cargo clean && cargo build
   cd frontend && rm -rf node_modules && npm install
   ```

2. **Check Odra Version**
   ```bash
   cargo tree | grep odra
   ```

3. **TypeScript Reset**
   ```bash
   cd frontend && npx tsc --noEmit
   ```

4. **Check for Circular Dependencies**
   ```bash
   cd frontend && npx madge --circular src
   ```

---

## 📊 Progress Tracking

Use this checklist to track your progress:

### Smart Contract Fixes
- [ ] PROMPT 1: Event System (45 errors)
- [ ] PROMPT 2: UserDeposit (50 errors)
- [ ] PROMPT 3: WithdrawalRequest (40 errors)
- [ ] PROMPT 4: VaultManager Fields (60 errors)
- [ ] PROMPT 5: Withdraw Event (5 errors)
- [ ] PROMPT 6: WithdrawalCompleted Event (4 errors)
- [ ] PROMPT 7: daily_deposits Type (8 errors)
- [ ] PROMPT 8: Mapping Methods (4 errors)
- [ ] PROMPT 9: Address References (2 errors)
- [ ] PROMPT 10: U256 Conversion (1 error)
- [ ] PROMPT 11: Unreachable Code (1 error)
- [ ] PROMPT 12: Unused Imports (3 errors)
- [ ] PROMPT 13: checked_sub Type (1 error)

**Subtotal: ~224 errors**

### Frontend Fixes
- [ ] PROMPT 14: Wallet Service (30 errors)
- [ ] PROMPT 15: Unused Imports (6 errors)

**Subtotal: ~36 errors**

### Total: **~260 direct errors** (plus ~156 cascading errors that auto-fix)

---

## 🎯 Estimated Time Breakdown

| Phase | Duration | Complexity |
|-------|----------|------------|
| Event System | 45 min | Medium |
| Struct Fields | 60 min | Medium |
| Mapping Issues | 90 min | High |
| Type Fixes | 45 min | Low |
| Frontend Fixes | 30 min | Low |
| Testing | 60 min | Medium |
| **TOTAL** | **5.5 hours** | Mixed |

---

## 💡 Pro Tips

1. **Fix in Order**: Follow the prompt order - type system first!
2. **Test Incrementally**: Run `cargo check` after each major prompt
3. **Commit Often**: Git commit after each working fix
4. **Read Error Messages**: They often suggest the fix
5. **Check Odra Docs**: When unsure about Mapping/Var API

---

## 🎉 Success Criteria

You'll know you're done when:

✅ `cargo check` returns 0 errors  
✅ `cargo clippy` has no warnings  
✅ `cargo test` all pass  
✅ `npm run build` succeeds  
✅ TypeScript shows 0 errors  

---

**Good luck! You're fixing a production-grade DeFi platform! 🚀**

---

## 📝 Notes

- Most errors are related to incomplete type definitions and missing struct fields
- Frontend errors are mostly import/export issues (easy fix)
- The Rust errors cascade - fixing structs will auto-fix ~60% of total errors
- Odra's Mapping requires CLTyped trait implementations for custom structs
- This is normal for a large blockchain project - you're doing great!

---

**Last Updated**: December 31, 2025  
**Version**: 1.0  
**Project**: CasperVault DeFi Platform  
