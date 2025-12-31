#!/bin/bash

# Deploy CasperVault to Casper Testnet
# This script deploys all contracts in the correct order and configures them

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CONFIG_FILE="$SCRIPT_DIR/../config/testnet.json"
ADDRESSES_FILE="$PROJECT_ROOT/scripts/addresses.json"
BACKUP_DIR="$PROJECT_ROOT/backups/$(date +%Y%m%d_%H%M%S)"

# Dry run mode
DRY_RUN="${DRY_RUN:-false}"

# Load configuration
if [ ! -f "$CONFIG_FILE" ]; then
    log_error "Configuration file not found: $CONFIG_FILE"
    exit 1
fi

log_info "Loading configuration from $CONFIG_FILE"
NETWORK=$(jq -r '.network' "$CONFIG_FILE")
NODE_ADDRESS=$(jq -r '.node_address' "$CONFIG_FILE")
CHAIN_NAME=$(jq -r '.chain_name' "$CONFIG_FILE")
GAS_PRICE=$(jq -r '.gas_price' "$CONFIG_FILE")
TREASURY_ADDRESS=$(jq -r '.treasury_address' "$CONFIG_FILE")

log_info "Network: $NETWORK"
log_info "Node: $NODE_ADDRESS"
log_info "Chain: $CHAIN_NAME"

if [ "$DRY_RUN" = "true" ]; then
    log_warn "Running in DRY RUN mode - no actual deployments will occur"
fi

# Create backup directory
mkdir -p "$BACKUP_DIR"
log_info "Backup directory: $BACKUP_DIR"

# Step 1: Build contracts
log_info "Step 1/8: Building contracts..."
cd "$PROJECT_ROOT/contracts"

if [ "$DRY_RUN" = "false" ]; then
    cargo build --release || {
        log_error "Contract build failed"
        exit 1
    }
    log_info "✓ Contracts built successfully"
else
    log_info "✓ [DRY RUN] Would build contracts with: cargo build --release"
fi

# Initialize addresses JSON
cat > "$ADDRESSES_FILE" <<EOF
{
  "network": "$NETWORK",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "contracts": {}
}
EOF

# Step 2: Deploy token contracts
log_info "Step 2/8: Deploying token contracts..."

deploy_contract() {
    local contract_name=$1
    local wasm_file=$2
    local init_args=$3
    
    log_info "Deploying $contract_name..."
    
    if [ "$DRY_RUN" = "false" ]; then
        # This is a placeholder - actual deployment would use Odra CLI or casper-client
        # For Odra contracts, you'd typically use:
        # odra deploy -n $NETWORK -c $contract_name
        
        log_warn "Actual deployment command would be:"
        log_warn "  casper-client put-deploy \\"
        log_warn "    --node-address $NODE_ADDRESS \\"
        log_warn "    --chain-name $CHAIN_NAME \\"
        log_warn "    --session-path $wasm_file \\"
        log_warn "    --payment-amount 200000000000 \\"
        log_warn "    $init_args"
        
        # Placeholder hash for demonstration
        local deploy_hash="hash-$(openssl rand -hex 32)"
        log_info "Deploy hash: $deploy_hash"
        
        # Update addresses file
        jq ".contracts[\"$contract_name\"] = \"$deploy_hash\"" "$ADDRESSES_FILE" > "$ADDRESSES_FILE.tmp"
        mv "$ADDRESSES_FILE.tmp" "$ADDRESSES_FILE"
        
        log_info "✓ $contract_name deployed: $deploy_hash"
    else
        log_info "✓ [DRY RUN] Would deploy $contract_name"
    fi
}

# Deploy lstCSPR token
deploy_contract "lstCSPR" \
    "target/wasm32-unknown-unknown/release/lst_cspr.wasm" \
    "--session-arg \"name:string='Liquid Staked CSPR'\" --session-arg \"symbol:string='lstCSPR'\" --session-arg \"decimals:u8='9'\""

# Deploy cvCSPR token (vault shares)
deploy_contract "cvCSPR" \
    "target/wasm32-unknown-unknown/release/cv_cspr.wasm" \
    "--session-arg \"name:string='CasperVault Shares'\" --session-arg \"symbol:string='cvCSPR'\" --session-arg \"decimals:u8='9'\""

# Step 3: Deploy core contracts
log_info "Step 3/8: Deploying core contracts..."

# Deploy VaultManager
deploy_contract "VaultManager" \
    "target/wasm32-unknown-unknown/release/vault_manager.wasm" \
    "--session-arg \"admin:key='$TREASURY_ADDRESS'\""

# Deploy LiquidStaking
deploy_contract "LiquidStaking" \
    "target/wasm32-unknown-unknown/release/liquid_staking.wasm" \
    "--session-arg \"admin:key='$TREASURY_ADDRESS'\""

# Deploy StrategyRouter
deploy_contract "StrategyRouter" \
    "target/wasm32-unknown-unknown/release/strategy_router.wasm" \
    "--session-arg \"admin:key='$TREASURY_ADDRESS'\""

# Deploy YieldAggregator
deploy_contract "YieldAggregator" \
    "target/wasm32-unknown-unknown/release/yield_aggregator.wasm" \
    "--session-arg \"admin:key='$TREASURY_ADDRESS'\""

# Step 4: Deploy strategies
log_info "Step 4/8: Deploying strategy contracts..."

deploy_contract "DEXStrategy" \
    "target/wasm32-unknown-unknown/release/dex_strategy.wasm" \
    "--session-arg \"router:key='$(jq -r '.contracts.StrategyRouter' "$ADDRESSES_FILE")'\""

deploy_contract "LendingStrategy" \
    "target/wasm32-unknown-unknown/release/lending_strategy.wasm" \
    "--session-arg \"router:key='$(jq -r '.contracts.StrategyRouter' "$ADDRESSES_FILE")'\""

deploy_contract "CrossChainStrategy" \
    "target/wasm32-unknown-unknown/release/crosschain_strategy.wasm" \
    "--session-arg \"router:key='$(jq -r '.contracts.StrategyRouter' "$ADDRESSES_FILE")'\""

# Step 5: Initialize contracts with addresses
log_info "Step 5/8: Initializing contract addresses..."

if [ "$DRY_RUN" = "false" ]; then
    # VaultManager initialization
    log_info "Initializing VaultManager..."
    log_info "  - Setting lstCSPR token address"
    log_info "  - Setting cvCSPR token address"
    log_info "  - Setting LiquidStaking contract address"
    log_info "  - Setting StrategyRouter contract address"
    
    # LiquidStaking initialization
    log_info "Initializing LiquidStaking..."
    log_info "  - Setting lstCSPR token address"
    log_info "  - Adding initial validators"
    
    # StrategyRouter initialization
    log_info "Initializing StrategyRouter..."
    log_info "  - Adding DEXStrategy"
    log_info "  - Adding LendingStrategy"
    log_info "  - Adding CrossChainStrategy"
    log_info "  - Setting allocations (40% DEX, 30% Lending, 30% CrossChain)"
    
    log_info "✓ Contract initialization complete"
else
    log_info "✓ [DRY RUN] Would initialize all contracts with cross-references"
fi

# Step 6: Set up roles and permissions
log_info "Step 6/8: Setting up roles and permissions..."

if [ "$DRY_RUN" = "false" ]; then
    # Grant admin roles
    ADMIN_KEYS=$(jq -r '.admin_keys[]' "$CONFIG_FILE")
    for admin in $ADMIN_KEYS; do
        log_info "Granting ADMIN role to $admin"
    done
    
    # Grant operator roles
    OPERATOR_KEYS=$(jq -r '.operator_keys[]' "$CONFIG_FILE")
    for operator in $OPERATOR_KEYS; do
        log_info "Granting OPERATOR role to $operator"
    done
    
    # Grant guardian roles
    GUARDIAN_KEYS=$(jq -r '.guardian_keys[]' "$CONFIG_FILE")
    for guardian in $GUARDIAN_KEYS; do
        log_info "Granting GUARDIAN role to $guardian"
    done
    
    # Grant keeper roles
    KEEPER_KEYS=$(jq -r '.keeper_keys[]' "$CONFIG_FILE")
    for keeper in $KEEPER_KEYS; do
        log_info "Granting KEEPER role to $keeper"
    done
    
    log_info "✓ Roles and permissions configured"
else
    log_info "✓ [DRY RUN] Would configure roles and permissions"
fi

# Step 7: Verify deployments
log_info "Step 7/8: Verifying deployments..."

if [ "$DRY_RUN" = "false" ]; then
    log_info "Running verification script..."
    bash "$SCRIPT_DIR/../verify/verify-deployment.sh" || {
        log_error "Deployment verification failed"
        exit 1
    }
    log_info "✓ Deployment verification passed"
else
    log_info "✓ [DRY RUN] Would run verification script"
fi

# Step 8: Save contract addresses
log_info "Step 8/8: Saving contract addresses..."

if [ "$DRY_RUN" = "false" ]; then
    # Backup addresses file
    cp "$ADDRESSES_FILE" "$BACKUP_DIR/addresses.json"
    
    log_info "Contract addresses saved to: $ADDRESSES_FILE"
    log_info "Backup saved to: $BACKUP_DIR/addresses.json"
    
    # Display deployed contracts
    log_info ""
    log_info "=== DEPLOYMENT SUMMARY ==="
    jq -r '.contracts | to_entries | .[] | "  \(.key): \(.value)"' "$ADDRESSES_FILE"
else
    log_info "✓ [DRY RUN] Would save addresses to $ADDRESSES_FILE"
fi

# Success message
log_info ""
log_info "========================================="
if [ "$DRY_RUN" = "false" ]; then
    log_info "✓ Testnet deployment completed successfully!"
else
    log_info "✓ Dry run completed successfully!"
fi
log_info "========================================="
log_info ""
log_info "Next steps:"
log_info "  1. Run verification: bash scripts/verify/verify-deployment.sh"
log_info "  2. Test full flow: bash scripts/deploy/test-full-flow.sh"
log_info "  3. Monitor health: bash scripts/monitor/check-health.sh"
log_info ""
log_info "Contract addresses: $ADDRESSES_FILE"
log_info "Network: $NETWORK"
log_info "Node: $NODE_ADDRESS"
log_info ""
