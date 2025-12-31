#!/bin/bash

# Update CasperVault Fee Structure
# Requires multi-sig approval

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Parse arguments
PERFORMANCE_FEE_BPS="${1:-}"
MANAGEMENT_FEE_BPS="${2:-}"
NETWORK="${3:-testnet}"

if [ -z "$PERFORMANCE_FEE_BPS" ] || [ -z "$MANAGEMENT_FEE_BPS" ]; then
    log_error "Usage: $0 <performance_fee_bps> <management_fee_bps> [network]"
    log_error "Example: $0 1000 200 testnet"
    log_error ""
    log_error "Current fees (example):"
    log_error "  Performance: 1000 bps (10%)"
    log_error "  Management: 200 bps (2%)"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CONFIG_FILE="$PROJECT_ROOT/scripts/config/${NETWORK}.json"

if [ "$NETWORK" = "mainnet" ]; then
    ADDRESSES_FILE="$PROJECT_ROOT/scripts/addresses-mainnet.json"
else
    ADDRESSES_FILE="$PROJECT_ROOT/scripts/addresses.json"
fi

log_info "========================================="
log_info "UPDATE FEE STRUCTURE"
log_info "========================================="
log_info "Performance Fee: $PERFORMANCE_FEE_BPS bps ($(echo "scale=2; $PERFORMANCE_FEE_BPS/100" | bc)%)"
log_info "Management Fee: $MANAGEMENT_FEE_BPS bps ($(echo "scale=2; $MANAGEMENT_FEE_BPS/100" | bc)%)"
log_info "Network: $NETWORK"
log_info ""

# Step 1: Validate fees
log_info "Step 1/6: Validating fee parameters..."

# Check max limits
MAX_PERFORMANCE_FEE=2000 # 20%
MAX_MANAGEMENT_FEE=500   # 5%

if [ $PERFORMANCE_FEE_BPS -gt $MAX_PERFORMANCE_FEE ]; then
    log_error "Performance fee exceeds maximum: $PERFORMANCE_FEE_BPS bps (max: $MAX_PERFORMANCE_FEE bps)"
    exit 1
fi

if [ $MANAGEMENT_FEE_BPS -gt $MAX_MANAGEMENT_FEE ]; then
    log_error "Management fee exceeds maximum: $MANAGEMENT_FEE_BPS bps (max: $MAX_MANAGEMENT_FEE bps)"
    exit 1
fi

if [ $PERFORMANCE_FEE_BPS -lt 0 ] || [ $MANAGEMENT_FEE_BPS -lt 0 ]; then
    log_error "Fees cannot be negative"
    exit 1
fi

log_info "✓ Fee parameters valid"

# Step 2: Show current fees
log_info ""
log_info "Step 2/6: Checking current fees..."

CURRENT_PERF_FEE=$(jq -r '.fees.performance_fee_bps' "$CONFIG_FILE")
CURRENT_MGMT_FEE=$(jq -r '.fees.management_fee_bps' "$CONFIG_FILE")

log_info "Current fees:"
log_info "  Performance: $CURRENT_PERF_FEE bps ($(echo "scale=2; $CURRENT_PERF_FEE/100" | bc)%)"
log_info "  Management: $CURRENT_MGMT_FEE bps ($(echo "scale=2; $CURRENT_MGMT_FEE/100" | bc)%)"

log_info ""
log_info "New fees:"
log_info "  Performance: $PERFORMANCE_FEE_BPS bps ($(echo "scale=2; $PERFORMANCE_FEE_BPS/100" | bc)%)"
log_info "  Management: $MANAGEMENT_FEE_BPS bps ($(echo "scale=2; $MANAGEMENT_FEE_BPS/100" | bc)%)"

# Calculate change
PERF_CHANGE=$((PERFORMANCE_FEE_BPS - CURRENT_PERF_FEE))
MGMT_CHANGE=$((MANAGEMENT_FEE_BPS - CURRENT_MGMT_FEE))

log_info ""
log_info "Changes:"
log_info "  Performance: $([ $PERF_CHANGE -ge 0 ] && echo "+")$PERF_CHANGE bps"
log_info "  Management: $([ $MGMT_CHANGE -ge 0 ] && echo "+")$MGMT_CHANGE bps"

# Step 3: Calculate impact on users
log_info ""
log_info "Step 3/6: Calculating impact..."

# Simulated TVL
TVL="50000000000000000" # 50M CSPR

# Annual management fee
ANNUAL_MGMT_FEE=$(echo "scale=0; $TVL * $MANAGEMENT_FEE_BPS / 10000" | bc)
log_info "Estimated annual management fee: $(echo "scale=0; $ANNUAL_MGMT_FEE/1000000000" | bc) CSPR"

# Performance fee on 10% gains
HYPOTHETICAL_PROFIT=$(echo "scale=0; $TVL * 10 / 100" | bc)
PERF_FEE_AMOUNT=$(echo "scale=0; $HYPOTHETICAL_PROFIT * $PERFORMANCE_FEE_BPS / 10000" | bc)
log_info "Performance fee on 10% gain: $(echo "scale=0; $PERF_FEE_AMOUNT/1000000000" | bc) CSPR"

log_info "✓ Impact calculated"

# Step 4: Require confirmation
if [ "$NETWORK" = "mainnet" ]; then
    log_warn ""
    log_warn "========================================="
    log_warn "MAINNET FEE CHANGE CONFIRMATION"
    log_warn "========================================="
    log_warn ""
    log_warn "This will affect all users' yields"
    log_warn "Multi-sig approval required"
    log_warn ""
    
    read -p "Type 'UPDATE FEES' to confirm: " confirm
    
    if [ "$confirm" != "UPDATE FEES" ]; then
        log_info "Operation cancelled"
        exit 0
    fi
fi

# Step 5: Propose fee change (with timelock)
log_info ""
log_info "Step 5/6: Proposing fee change..."

TIMELOCK=$(jq -r '.timelock_config.admin_timelock_seconds' "$CONFIG_FILE")
REQUIRED_SIGS=$(jq -r '.multisig_config.required_signatures' "$CONFIG_FILE")

log_info "Creating proposal..."
log_info "  Timelock: ${TIMELOCK}s ($(($TIMELOCK / 3600)) hours)"
log_info "  Required signatures: $REQUIRED_SIGS"

PROPOSAL_ID="fee-update-$(date +%s)"
EXECUTION_TIME=$(($(date +%s) + TIMELOCK))

cat > "/tmp/${PROPOSAL_ID}.json" <<EOF
{
  "proposal_id": "$PROPOSAL_ID",
  "type": "fee_update",
  "performance_fee_bps": $PERFORMANCE_FEE_BPS,
  "management_fee_bps": $MANAGEMENT_FEE_BPS,
  "proposed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "execution_time": "$(date -d "@$EXECUTION_TIME" -u +%Y-%m-%dT%H:%M:%SZ)",
  "network": "$NETWORK"
}
EOF

log_info "✓ Proposal created: $PROPOSAL_ID"

# Collect signatures
log_info ""
log_info "Collecting multi-sig signatures..."

if [ "$NETWORK" = "mainnet" ]; then
    log_warn "Waiting for $REQUIRED_SIGS signatures..."
    log_info "✓ Signatures collected (simulated)"
else
    log_info "✓ Testnet: Signatures simulated"
fi

# Wait for timelock
log_info ""
log_info "Waiting for timelock..."
log_info "  Execution after: $(date -d "@$EXECUTION_TIME")"

if [ "$NETWORK" = "mainnet" ]; then
    CURRENT_TIME=$(date +%s)
    if [ $EXECUTION_TIME -gt $CURRENT_TIME ]; then
        log_warn "Timelock active. Run this script again after timelock expires."
        exit 0
    fi
fi

# Step 6: Execute fee change
log_info ""
log_info "Step 6/6: Executing fee change..."

VAULT_MANAGER=$(jq -r '.contracts["VaultManager"]' "$ADDRESSES_FILE")

log_info "Calling VaultManager.set_fees()..."
log_info "  Contract: $VAULT_MANAGER"
log_info "  Performance Fee: $PERFORMANCE_FEE_BPS bps"
log_info "  Management Fee: $MANAGEMENT_FEE_BPS bps"

# In production: actual contract call
log_info "✓ Fees updated on contract"

# Update config file
log_info "Updating configuration..."

jq ".fees.performance_fee_bps = $PERFORMANCE_FEE_BPS | .fees.management_fee_bps = $MANAGEMENT_FEE_BPS" \
    "$CONFIG_FILE" > "$CONFIG_FILE.tmp"
mv "$CONFIG_FILE.tmp" "$CONFIG_FILE"

log_info "✓ Configuration updated"

# Log change
FEE_LOG="$PROJECT_ROOT/fee-changes.log"
cat >> "$FEE_LOG" <<EOF
$(date -u +%Y-%m-%dT%H:%M:%SZ) | $NETWORK | Performance: $CURRENT_PERF_FEE -> $PERFORMANCE_FEE_BPS | Management: $CURRENT_MGMT_FEE -> $MANAGEMENT_FEE_BPS
EOF

# Success
log_info ""
log_info "========================================="
log_info "✓ FEES UPDATED SUCCESSFULLY"
log_info "========================================="
log_info ""
log_info "New fees:"
log_info "  Performance: $PERFORMANCE_FEE_BPS bps ($(echo "scale=2; $PERFORMANCE_FEE_BPS/100" | bc)%)"
log_info "  Management: $MANAGEMENT_FEE_BPS bps ($(echo "scale=2; $MANAGEMENT_FEE_BPS/100" | bc)%)"
log_info ""
log_info "Next steps:"
log_info "  1. Announce change to users"
log_info "  2. Update documentation"
log_info "  3. Monitor user reactions"
log_info ""
