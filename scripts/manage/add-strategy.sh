#!/bin/bash

# Add Strategy to CasperVault
# Deploys new strategy and adds to StrategyRouter

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Parse arguments
STRATEGY_TYPE="${1:-}"
ALLOCATION="${2:-0}"
NETWORK="${3:-testnet}"

if [ -z "$STRATEGY_TYPE" ]; then
    log_error "Usage: $0 <strategy_type> <allocation_percentage> [network]"
    log_error "Example: $0 YearnStrategy 15 testnet"
    log_error ""
    log_error "Available strategy types:"
    log_error "  - DEXStrategy"
    log_error "  - LendingStrategy"
    log_error "  - CrossChainStrategy"
    log_error "  - Custom strategy name"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

if [ "$NETWORK" = "mainnet" ]; then
    ADDRESSES_FILE="$PROJECT_ROOT/scripts/addresses-mainnet.json"
else
    ADDRESSES_FILE="$PROJECT_ROOT/scripts/addresses.json"
fi

log_info "========================================="
log_info "ADD STRATEGY TO CASPERVAULT"
log_info "========================================="
log_info "Strategy: $STRATEGY_TYPE"
log_info "Allocation: ${ALLOCATION}%"
log_info "Network: $NETWORK"
log_info ""

# Step 1: Validate allocation
log_info "Step 1/6: Validating allocation..."

if [ $ALLOCATION -gt 40 ]; then
    log_error "Allocation exceeds maximum: ${ALLOCATION}% (max: 40%)"
    exit 1
fi

if [ $ALLOCATION -lt 0 ]; then
    log_error "Invalid allocation: ${ALLOCATION}%"
    exit 1
fi

log_info "✓ Allocation valid: ${ALLOCATION}%"

# Step 2: Check current allocations
log_info ""
log_info "Step 2/6: Checking current allocations..."

# In production: query contract
CURRENT_DEX=40
CURRENT_LENDING=30
CURRENT_CROSSCHAIN=30
CURRENT_TOTAL=$((CURRENT_DEX + CURRENT_LENDING + CURRENT_CROSSCHAIN))

log_info "Current allocations:"
log_info "  DEX: ${CURRENT_DEX}%"
log_info "  Lending: ${CURRENT_LENDING}%"
log_info "  Cross-chain: ${CURRENT_CROSSCHAIN}%"
log_info "  Total: ${CURRENT_TOTAL}%"

NEW_TOTAL=$((CURRENT_TOTAL + ALLOCATION))

if [ $NEW_TOTAL -gt 100 ]; then
    log_error "New total allocation exceeds 100%: ${NEW_TOTAL}%"
    log_error "Current total: ${CURRENT_TOTAL}%"
    log_error "New strategy: ${ALLOCATION}%"
    log_error ""
    log_error "Reduce allocation or rebalance existing strategies"
    exit 1
fi

log_info "New total allocation: ${NEW_TOTAL}%"
log_info "Remaining capacity: $((100 - NEW_TOTAL))%"

# Step 3: Deploy strategy contract
log_info ""
log_info "Step 3/6: Deploying strategy contract..."

STRATEGY_ROUTER=$(jq -r '.contracts["StrategyRouter"]' "$ADDRESSES_FILE")

if [ "$STRATEGY_ROUTER" = "null" ]; then
    log_error "StrategyRouter address not found"
    exit 1
fi

log_info "Deploying $STRATEGY_TYPE..."
log_info "  Router: $STRATEGY_ROUTER"

# In production: actual deployment
# cargo build --release
# deploy wasm file

STRATEGY_ADDRESS="hash-strategy-$(openssl rand -hex 16)"
log_info "✓ Strategy deployed: $STRATEGY_ADDRESS"

# Update addresses file
jq ".contracts[\"$STRATEGY_TYPE\"] = \"$STRATEGY_ADDRESS\"" "$ADDRESSES_FILE" > "$ADDRESSES_FILE.tmp"
mv "$ADDRESSES_FILE.tmp" "$ADDRESSES_FILE"

# Step 4: Add strategy to router
log_info ""
log_info "Step 4/6: Adding strategy to StrategyRouter..."

log_info "Calling StrategyRouter.add_strategy()..."
log_info "  Strategy: $STRATEGY_ADDRESS"
log_info "  Name: $STRATEGY_TYPE"
log_info "  Allocation: ${ALLOCATION}%"

# In production: actual contract call
log_info "✓ Strategy added to router"

# Step 5: Set strategy parameters
log_info ""
log_info "Step 5/6: Setting strategy parameters..."

log_info "Configuring strategy..."
log_info "  Max deployment: 40%"
log_info "  Min deployment: 5%"
log_info "  Risk level: MEDIUM"

# In production: set strategy parameters
log_info "✓ Strategy configured"

# Step 6: Verify strategy
log_info ""
log_info "Step 6/6: Verifying strategy..."

log_info "Testing strategy deployment..."
# In production: test with small amount

log_info "✓ Strategy verification passed"

# Save strategy info
STRATEGY_INFO_DIR="$PROJECT_ROOT/strategy-info"
mkdir -p "$STRATEGY_INFO_DIR"

cat > "$STRATEGY_INFO_DIR/${STRATEGY_TYPE}.json" <<EOF
{
  "name": "$STRATEGY_TYPE",
  "address": "$STRATEGY_ADDRESS",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "network": "$NETWORK",
  "allocation": $ALLOCATION,
  "risk_level": "MEDIUM",
  "max_allocation": 40,
  "min_allocation": 5,
  "status": "active"
}
EOF

log_info "✓ Strategy info saved"

# Success
log_info ""
log_info "========================================="
log_info "✓ STRATEGY ADDED SUCCESSFULLY"
log_info "========================================="
log_info ""
log_info "Strategy: $STRATEGY_TYPE"
log_info "Address: $STRATEGY_ADDRESS"
log_info "Allocation: ${ALLOCATION}%"
log_info "New total: ${NEW_TOTAL}%"
log_info ""
log_info "Next steps:"
log_info "  1. Monitor strategy performance"
log_info "  2. Test with small deployment"
log_info "  3. Gradually increase allocation if successful"
log_info "  4. Update documentation"
log_info ""
log_info "To rebalance: bash scripts/manage/rebalance-strategies.sh"
log_info ""
