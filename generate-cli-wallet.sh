#!/bin/bash

# Generate CLI Wallet for CasperVault Deployment
# Use this when Casper Wallet extension export doesn't work

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Generate CLI Wallet for Deployment${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if casper-client is installed
if ! command -v casper-client &> /dev/null; then
    echo -e "${YELLOW}casper-client not found. Installing...${NC}"
    cargo install casper-client
    echo -e "${GREEN}✓ casper-client installed${NC}"
    echo ""
fi

# Create keys directory
mkdir -p keys
cd keys

echo -e "${YELLOW}Step 1: Generating new keypair...${NC}"
echo ""

# Generate keys
casper-client keygen .

echo ""
echo -e "${GREEN}✓ Keys generated successfully!${NC}"
echo ""

# Display the public key
PUBLIC_KEY=$(cat public_key_hex)
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  YOUR NEW CLI WALLET${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${GREEN}Public Key (Address):${NC}"
echo "$PUBLIC_KEY"
echo ""
echo -e "${YELLOW}Files created:${NC}"
echo "  - secret_key.pem (PRIVATE KEY - keep safe!)"
echo "  - public_key.pem"
echo "  - public_key_hex (your account address)"
echo ""

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  NEXT: Transfer Tokens${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Now transfer testnet CSPR from your Casper Wallet Extension:"
echo ""
echo "1. Open Casper Wallet extension"
echo "2. Make sure you're on 'Casper Test' network"
echo "3. Click 'Send' or 'Transfer'"
echo "4. Enter recipient address: $PUBLIC_KEY"
echo "5. Amount: 700 CSPR (recommended for deployment + buffer)"
echo "6. Confirm the transaction"
echo ""
echo -e "${YELLOW}Why 700 CSPR?${NC}"
echo "  - Deployment gas: ~600 CSPR"
echo "  - Testing: ~50 CSPR"
echo "  - Buffer: ~50 CSPR"
echo ""

# Wait for transfer
echo -e "${YELLOW}Press Enter after you've sent the transfer...${NC}"
read

echo ""
echo -e "${YELLOW}Checking balance...${NC}"
sleep 3  # Wait a bit for transaction to process

# Check balance
BALANCE=$(casper-client get-balance \
    --node-address http://95.216.67.162:7777 \
    --public-key "$PUBLIC_KEY" 2>/dev/null | grep -oP '\d+' | head -1 || echo "0")

BALANCE_CSPR=$((BALANCE / 1000000000))

if [ "$BALANCE_CSPR" -gt 0 ]; then
    echo -e "${GREEN}✓ Balance: $BALANCE_CSPR CSPR${NC}"
    echo ""
    
    if [ "$BALANCE_CSPR" -ge 600 ]; then
        echo -e "${GREEN}✓ Sufficient balance for deployment!${NC}"
    else
        echo -e "${YELLOW}⚠ You may need more CSPR. Recommended: 700 CSPR${NC}"
        echo "Transfer more from your extension wallet"
    fi
else
    echo -e "${RED}⚠ No balance detected yet${NC}"
    echo ""
    echo "Possible reasons:"
    echo "  - Transfer still processing (wait 1-2 minutes)"
    echo "  - Transfer not sent yet"
    echo "  - Wrong network selected"
    echo ""
    echo "You can check manually at:"
    echo "https://testnet.cspr.live/account/$PUBLIC_KEY"
fi

cd ..

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Setup Complete!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Your CLI wallet is ready for deployment."
echo ""
echo -e "${GREEN}Next steps:${NC}"
echo ""
echo "1. Verify balance:"
echo "   casper-client get-balance \\"
echo "     --node-address http://95.216.67.162:7777 \\"
echo "     --public-key \$(cat keys/public_key_hex)"
echo ""
echo "2. When contracts are ready, run:"
echo "   make deploy-testnet-dry    # Preview deployment"
echo "   make deploy-testnet        # Actual deployment"
echo ""
echo -e "${RED}⚠ IMPORTANT: Keep keys/secret_key.pem safe!${NC}"
echo "This is your private key - never share it!"
echo ""
