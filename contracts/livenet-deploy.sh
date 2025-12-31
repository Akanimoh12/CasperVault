#!/bin/bash
# Deploy using Odra framework (requires odra-casper-livenet)

echo "🔧 Installing Odra Casper Livenet..."
cargo install odra-casper-livenet

echo "🚀 Deploying VaultManager..."
odra-casper-livenet deploy \
  --network casper-test \
  --secret-key keys/secret_key.pem \
  --contract VaultManager

echo "✅ Deployment complete!"
