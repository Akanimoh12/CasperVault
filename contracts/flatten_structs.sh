#!/bin/bash
# Script to flatten storage structs for Casper compilation
# This replaces Var<CustomStruct> with individual Var<primitive> fields

set -e

echo "=== Struct Flattening Automation ==="
echo "Flattening storage structs to primitive fields..."

# Backup files
echo "Creating backups..."
cp src/strategies/lending_strategy.rs src/strategies/lending_strategy.rs.bak
cp src/strategies/crosschain_strategy.rs src/strategies/crosschain_strategy.rs.bak

# LENDING STRATEGY - Flatten LendingPosition
echo "Flattening LendingPosition in lending_strategy.rs..."

# Replace the struct initialization in init()
sed -i '/self\.position\.set(LendingPosition {/,/});/{
  c\
        self.principal.set(U512::zero());\
        self.interest_accrued.set(U512::zero());\
        self.supply_time.set(0);\
        self.c_tokens.set(U512::zero());
}' src/strategies/lending_strategy.rs

# Replace get_or_default() usages with individual field reads
sed -i 's/let current_position = self\.position\.get_or_default();/let current_principal = self.principal.get_or_default();\n        let current_interest = self.interest_accrued.get_or_default();\n        let current_supply_time = self.supply_time.get_or_default();/g' src/strategies/lending_strategy.rs

sed -i 's/let position = self\.position\.get_or_default();/let principal = self.principal.get_or_default();\n        let interest = self.interest_accrued.get_or_default();\n        let supply_time = self.supply_time.get_or_default();\n        let c_tokens = self.c_tokens.get_or_default();/g' src/strategies/lending_strategy.rs

# Replace field access patterns
sed -i 's/current_position\.principal/current_principal/g' src/strategies/lending_strategy.rs
sed -i 's/position\.principal/principal/g' src/strategies/lending_strategy.rs
sed -i 's/position\.interest_accrued/interest/g' src/strategies/lending_strategy.rs
sed -i 's/position\.c_tokens/c_tokens/g' src/strategies/lending_strategy.rs
sed -i 's/position\.supply_time/supply_time/g' src/strategies/lending_strategy.rs

# Replace .set() patterns for position updates
sed -i 's/new_position\.principal/self.principal.set(/g' src/strategies/lending_strategy.rs
sed -i 's/new_position\.interest_accrued/self.interest_accrued.set(/g' src/strategies/lending_strategy.rs
sed -i 's/self\.position\.set(new_position);/);/g' src/strategies/lending_strategy.rs

# CROSSCHAIN STRATEGY - Flatten CrossChainPosition
echo "Flattening CrossChainPosition in crosschain_strategy.rs..."

# Replace Mapping<u8, CrossChainPosition> with individual Mappings
sed -i 's/positions: Mapping<u8, CrossChainPosition>,/bridged_amounts: Mapping<u8, U512>,\n    deployed_amounts: Mapping<u8, U512>,\n    yields_accrued: Mapping<u8, U512>,\n    bridge_times: Mapping<u8, u64>,/g' src/strategies/crosschain_strategy.rs

# Replace position field access
sed -i 's/position\.bridged_amount/bridged_amount/g' src/strategies/crosschain_strategy.rs
sed -i 's/position\.deployed_amount/deployed_amount/g' src/strategies/crosschain_strategy.rs
sed -i 's/position\.yields_accrued/yields_accrued/g' src/strategies/crosschain_strategy.rs
sed -i 's/position\.bridge_time/bridge_time/g' src/strategies/crosschain_strategy.rs

echo ""
echo "✅ Basic replacements complete!"
echo ""
echo "⚠️  Manual review required for:"
echo "  - Complex struct operations (new_position assignments)"
echo "  - Nested field access"
echo "  - Pattern matching on position"
echo ""
echo "Backups created with .bak extension"
echo ""
echo "Run 'cargo check' to verify changes"
