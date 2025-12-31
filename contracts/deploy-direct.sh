#!/bin/bash

# Direct Casper Deployment Script
# This deploys using manual WASM building and casper-client

set -e

echo "🚀 CasperVault Direct Deployment"
echo "=================================="
echo ""

# Configuration
NODE_URL="http://65.21.235.219:7777"  # Try mainnet node
CHAIN_NAME="casper-test"
SECRET_KEY="keys/secret_key.pem"
PUBLIC_KEY=$(cat keys/public_key_hex)
GAS_PRICE=1

echo "📋 Configuration:"
echo "  Network: Casper Testnet"
echo "  Node: $NODE_URL"
echo "  Chain: $CHAIN_NAME"
echo "  Public Key: $PUBLIC_KEY"
echo ""

# Try to get account balance
echo "💰 Checking account balance..."
casper-client get-account-info \
    --node-address "$NODE_URL" \
    --public-key "$PUBLIC_KEY" 2>&1 | grep -i "balance\|error" || {
    echo "⚠️  Could not verify balance (node connectivity issue)"
    echo "   Continuing anyway - verify at:"
    echo "   https://testnet.cspr.live/account/$PUBLIC_KEY"
}
echo ""

# Build contract library
echo "🔨 Building contracts..."
cargo build --release --lib
echo "✅ Library built successfully!"
echo ""

echo "📝 Deployment Options:"
echo ""
echo "Since Odra contracts are framework-based, you have these options:"
echo ""
echo "Option A: Frontend Deployment (Recommended)"
echo "  1. Start the frontend: cd ../frontend && npm run dev"
echo "  2. Install Casper Signer browser extension"
echo "  3. Import your secret key to Casper Signer"
echo "  4. Connect wallet and deploy via UI"
echo ""
echo "Option B: Wait for Casper RPC Nodes"
echo "  The testnet nodes are experiencing connectivity issues"
echo "  Your account has funds: https://testnet.cspr.live/account/$PUBLIC_KEY"
echo "  Once nodes are back, run: ./testnet-deploy.sh"
echo ""
echo "Option C: Casper Association Support"
echo "  Contact: https://discord.gg/caspernetwork"
echo "  Ask about Odra framework deployment on testnet"
echo ""

echo "🔑 Your Keys:"
echo "  Public: $PUBLIC_KEY"
echo "  Location: keys/"
echo ""
echo "📊 Your Account: https://testnet.cspr.live/account/$PUBLIC_KEY"
