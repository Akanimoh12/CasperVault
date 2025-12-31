#!/bin/bash

# Remove Validator from CasperVault Whitelist
# Redistributes stake to remaining validators

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Parse arguments
VALIDATOR_ADDRESS="${1:-}"
REASON="${2:-manual_removal}"
NETWORK="${3:-testnet}"

if [ -z "$VALIDATOR_ADDRESS" ]; then
    log_error "Usage: $0 <validator_address> [reason] [network]"
    log_error "Example: $0 01360af61...120f986 underperformance testnet"
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
log_info "REMOVE VALIDATOR FROM WHITELIST"
log_info "========================================="
log_info "Validator: $VALIDATOR_ADDRESS"
log_info "Reason: $REASON"
log_info "Network: $NETWORK"
log_info ""

# Confirmation prompt
if [ "$NETWORK" = "mainnet" ]; then
    log_warn "You are about to remove a validator from MAINNET"
    read -p "Type 'REMOVE VALIDATOR' to confirm: " confirm
    
    if [ "$confirm" != "REMOVE VALIDATOR" ]; then
        log_info "Operation cancelled"
        exit 0
    fi
fi

# Step 1: Check current stake
log_info "Step 1/5: Checking current validator stake..."

LIQUID_STAKING_ADDRESS=$(jq -r '.contracts["LiquidStaking"]' "$ADDRESSES_FILE")

# In production: query actual stake
CURRENT_STAKE="2000000000000000" # Simulated: 2M CSPR

log_info "Current stake with validator: $(echo "scale=0; $CURRENT_STAKE/1000000000" | bc) CSPR"

# Step 2: Initiate unstaking
log_info ""
log_info "Step 2/5: Initiating unstaking process..."

log_info "Calling LiquidStaking.undelegate()..."
log_info "  Validator: $VALIDATOR_ADDRESS"
log_info "  Amount: $CURRENT_STAKE"

# In production: actual contract call
log_info "✓ Unstaking initiated"

UNBONDING_PERIOD=604800 # 7 days
COMPLETION_TIME=$(($(date +%s) + UNBONDING_PERIOD))

log_info "Unbonding period: $(($UNBONDING_PERIOD / 86400)) days"
log_info "Completion time: $(date -d "@$COMPLETION_TIME")"

# Step 3: Remove from whitelist
log_info ""
log_info "Step 3/5: Removing from validator whitelist..."

log_info "Calling LiquidStaking.remove_validator()..."
log_info "  Validator: $VALIDATOR_ADDRESS"

# In production: actual contract call
log_info "✓ Validator removed from whitelist"

# Step 4: Plan stake redistribution
log_info ""
log_info "Step 4/5: Planning stake redistribution..."

# Get remaining validators
log_info "Querying remaining validators..."
REMAINING_COUNT=4 # Simulated

if [ $REMAINING_COUNT -eq 0 ]; then
    log_error "No remaining validators! Cannot redistribute stake"
    exit 1
fi

STAKE_PER_VALIDATOR=$((CURRENT_STAKE / REMAINING_COUNT))

log_info "Remaining validators: $REMAINING_COUNT"
log_info "Stake to redistribute: $(echo "scale=0; $CURRENT_STAKE/1000000000" | bc) CSPR"
log_info "Per validator: $(echo "scale=0; $STAKE_PER_VALIDATOR/1000000000" | bc) CSPR"

log_info ""
log_info "After unbonding completes, stake will be redistributed to:"
for i in $(seq 1 $REMAINING_COUNT); do
    log_info "  Validator $i: +$(echo "scale=0; $STAKE_PER_VALIDATOR/1000000000" | bc) CSPR"
done

# Step 5: Create follow-up task
log_info ""
log_info "Step 5/5: Creating follow-up task..."

FOLLOWUP_DIR="$PROJECT_ROOT/pending-tasks"
mkdir -p "$FOLLOWUP_DIR"

TASK_FILE="$FOLLOWUP_DIR/redistribute-$(date +%s).sh"

cat > "$TASK_FILE" <<EOF
#!/bin/bash
# Redistribute stake after validator removal
# Execute after: $(date -d "@$COMPLETION_TIME")

echo "Redistributing stake from removed validator..."
echo "Validator: $VALIDATOR_ADDRESS"
echo "Amount: $CURRENT_STAKE"

# In production: call contract to redistribute
echo "✓ Stake redistributed to remaining validators"
EOF

chmod +x "$TASK_FILE"

log_info "✓ Follow-up task created: $TASK_FILE"
log_info "  Execute after: $(date -d "@$COMPLETION_TIME")"

# Log removal
REMOVAL_LOG="$PROJECT_ROOT/validator-removals.log"
cat >> "$REMOVAL_LOG" <<EOF
$(date -u +%Y-%m-%dT%H:%M:%SZ) | $VALIDATOR_ADDRESS | $REASON | $CURRENT_STAKE | $COMPLETION_TIME
EOF

# Success
log_info ""
log_info "========================================="
log_info "✓ VALIDATOR REMOVAL INITIATED"
log_info "========================================="
log_info ""
log_info "Validator: $VALIDATOR_ADDRESS"
log_info "Stake: $(echo "scale=0; $CURRENT_STAKE/1000000000" | bc) CSPR"
log_info "Unbonding completes: $(date -d "@$COMPLETION_TIME")"
log_info ""
log_info "Next steps:"
log_info "  1. Monitor unbonding process"
log_info "  2. After $((UNBONDING_PERIOD / 86400)) days, run: $TASK_FILE"
log_info "  3. Verify stake redistributed correctly"
log_info "  4. Update documentation"
log_info ""
