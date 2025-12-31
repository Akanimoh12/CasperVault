#!/bin/bash

# CasperVault Smart Contracts Deployment Script
# Deploys contracts to Casper Network (testnet or mainnet)

set -e

NETWORK=${1:-testnet}
SECRET_KEY=${CASPER_SECRET_KEY:-""}

echo "🚀 CasperVault Deployment Script"
echo "================================="
echo ""
echo "Network: $NETWORK"
echo ""

# Check if casper-client is installed
if ! command -v casper-client &> /dev/null; then
    echo "❌ Error: casper-client not found!"
    echo ""
    echo "Please install casper-client:"
    echo "  cargo install casper-client"
    echo ""
    exit 1
fi

# Check if secret key is provided
if [ -z "$SECRET_KEY" ]; then
    echo "⚠️  Warning: CASPER_SECRET_KEY environment variable not set"
    echo ""
    echo "Usage:"
    echo "  export CASPER_SECRET_KEY=/path/to/secret_key.pem"
    echo "  ./deploy.sh $NETWORK"
    echo ""
    echo "For now, using placeholder. You'll need to update this for actual deployment."
    echo ""
    SECRET_KEY="./secret_key.pem"
fi

# Set network parameters
if [ "$NETWORK" == "mainnet" ]; then
    NODE_ADDRESS="http://65.21.235.219:7777"
    CHAIN_NAME="casper"
    GAS_PRICE=1
elif [ "$NETWORK" == "testnet" ]; then
    NODE_ADDRESS="http://3.143.158.19:7777"
    CHAIN_NAME="casper-test"
    GAS_PRICE=1
else
    echo "❌ Invalid network: $NETWORK"
    echo "Usage: ./deploy.sh [testnet|mainnet]"
    exit 1
fi

echo "📋 Network Configuration:"
echo "  Node: $NODE_ADDRESS"
echo "  Chain: $CHAIN_NAME"
echo ""

# Check if WASM files exist
if [ ! -d "wasm" ] || [ -z "$(ls -A wasm 2>/dev/null)" ]; then
    echo "⚠️  No WASM files found. Building contracts..."
    ./build.sh
    echo ""
fi

echo "📦 Deployment Summary:"
echo "  ℹ️  Note: Odra contracts require proper binary integration"
echo ""
echo "For proper Odra deployment, you need:"
echo "  1. Odra CLI tools installed"
echo "  2. Proper contract entry points in binaries"
echo "  3. Contract schema generation"
echo ""
echo "Alternative deployment methods:"
echo ""
echo "Method 1: Using Odra SDK (Recommended)"
echo "  - Add odra-casper-livenet-env to Cargo.toml"
echo "  - Use odra::test_env for testing"
echo "  - Deploy via Odra's deployment tools"
echo ""
echo "Method 2: Direct casper-client deployment"
echo "  - Build WASM with proper entry points"
echo "  - Deploy using casper-client put-deploy"
echo ""
echo "Method 3: Using Casper Signer & Frontend"
echo "  - Build frontend with contract deployment UI"
echo "  - Use Casper Signer browser extension"
echo "  - Deploy interactively"
echo ""

# Create deployment config
cat > deployment.json <<EOF
{
  "network": "$NETWORK",
  "chain_name": "$CHAIN_NAME",
  "node_address": "$NODE_ADDRESS",
  "contracts": {
    "vault_manager": {
      "wasm": "wasm/caspervault_vault_manager.wasm",
      "init_args": {
        "admin": "account-hash-...",
        "fee_bps": 1000
      }
    },
    "liquid_staking": {
      "wasm": "wasm/caspervault_liquid_staking.wasm",
      "init_args": {
        "admin": "account-hash-...",
        "min_delegation": "500000000000"
      }
    },
    "lst_cspr": {
      "wasm": "wasm/caspervault_lst_cspr.wasm",
      "init_args": {
        "name": "Liquid Staked CSPR",
        "symbol": "lstCSPR",
        "decimals": 9
      }
    },
    "cv_cspr": {
      "wasm": "wasm/caspervault_cv_cspr.wasm",
      "init_args": {
        "name": "CasperVault Share",
        "symbol": "cvCSPR",
        "decimals": 9
      }
    }
  }
}
EOF

echo "✅ Deployment configuration saved to: deployment.json"
echo ""
echo "📚 Next Steps:"
echo "  1. Review deployment.json and update account addresses"
echo "  2. Ensure you have sufficient CSPR for gas fees"
echo "  3. For testnet, get CSPR from faucet: https://testnet.cspr.live/tools/faucet"
echo "  4. Update deployment method based on your needs"
echo ""
echo "🔗 Useful Links:"
echo "  - Casper Testnet Explorer: https://testnet.cspr.live"
echo "  - Casper Mainnet Explorer: https://cspr.live"
echo "  - Odra Documentation: https://odra.dev"
echo "  - Casper Documentation: https://docs.casper.network"
