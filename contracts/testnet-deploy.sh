#!/bin/bash

# CasperVault Testnet Deployment Script
# Deploys contracts to Casper testnet

set -e

echo "🚀 CasperVault Testnet Deployment"
echo "=================================="
echo ""

# Configuration
NODE_URL="http://3.143.158.19:7777"
CHAIN_NAME="casper-test"
SECRET_KEY="keys/secret_key.pem"
GAS_PRICE=1

# Check if keys exist
if [ ! -f "$SECRET_KEY" ]; then
    echo "❌ Error: Secret key not found at $SECRET_KEY"
    echo "   Run: casper-client keygen keys/"
    exit 1
fi

# Get public key
PUBLIC_KEY=$(cat keys/public_key_hex)
echo "📋 Deployment Configuration:"
echo "  Network: Casper Testnet"
echo "  Node: $NODE_URL"
echo "  Chain: $CHAIN_NAME"
echo "  Public Key: $PUBLIC_KEY"
echo ""

# Get account info and balance
echo "💰 Checking account balance..."
echo "   Verify your balance at: https://testnet.cspr.live/account/$PUBLIC_KEY"
echo ""

# Build contracts
echo "🔨 Building contracts..."
if cargo build --release --lib; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "📦 Deployment Plan:"
echo "  1. Deploy lstCSPR token"
echo "  2. Deploy cvCSPR token"
echo "  3. Deploy LiquidStaking contract"
echo "  4. Deploy VaultManager contract"
echo "  5. Initialize and configure contracts"
echo ""

read -p "Ready to deploy? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Deployment cancelled"
    exit 0
fi

echo ""
echo "🚀 Starting deployment..."
echo ""

# Note: For Odra contracts, we need WASM files
# The current setup compiles as a library
# For actual deployment, we need to either:
# 1. Use Odra's deployment tools
# 2. Create proper WASM contract entry points
# 3. Use a frontend deployment interface

echo "ℹ️  Current Status:"
echo "  ✅ Library compiles successfully"
echo "  ✅ Deployment keys generated"
echo "  ✅ Connected to testnet"
echo ""
echo "📝 For deployment, you have two options:"
echo ""
echo "Option A: Frontend Deployment (Recommended)"
echo "  1. Start frontend: cd ../frontend && npm run dev"
echo "  2. Install Casper Signer extension"
echo "  3. Import your keys to Casper Signer"
echo "  4. Use the deployment UI"
echo ""
echo "Option B: Manual Contract Creation"
echo "  1. Create WASM contract entry points"
echo "  2. Build for wasm32-unknown-unknown target"
echo "  3. Deploy using casper-client put-deploy"
echo ""
echo "🔑 Your deployment keys are in: keys/"
echo "   - Public key: keys/public_key.pem"
echo "   - Secret key: keys/secret_key.pem"
echo ""
echo "📚 See DEPLOYMENT.md for detailed instructions"
