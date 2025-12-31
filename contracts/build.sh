#!/bin/bash

# CasperVault Smart Contracts Build Script
# Builds WASM binaries for Casper Network deployment

set -e

echo "🔨 Building CasperVault Smart Contracts..."
echo "=========================================="
echo ""

# Clean previous builds
echo "📦 Cleaning previous builds..."
cargo clean

# Create output directory
mkdir -p wasm

# Build all contracts for WASM target
echo ""
echo "🏗️  Building contracts..."

# Add WASM target if not already added
rustup target add wasm32-unknown-unknown 2>/dev/null || true

# Build the library
echo "  → Building library..."
cargo build --release --target wasm32-unknown-unknown --lib

# Build each binary
echo "  → Building vault_manager..."
cargo build --release --target wasm32-unknown-unknown --bin caspervault_vault_manager 2>&1 || echo "    ⚠️  Binary needs proper Odra integration"

echo "  → Building liquid_staking..."
cargo build --release --target wasm32-unknown-unknown --bin caspervault_liquid_staking 2>&1 || echo "    ⚠️  Binary needs proper Odra integration"

echo "  → Building lst_cspr token..."
cargo build --release --target wasm32-unknown-unknown --bin caspervault_lst_cspr 2>&1 || echo "    ⚠️  Binary needs proper Odra integration"

echo "  → Building cv_cspr token..."
cargo build --release --target wasm32-unknown-unknown --bin caspervault_cv_cspr 2>&1 || echo "    ⚠️  Binary needs proper Odra integration"

# Copy WASM files
echo ""
echo "📋 Copying WASM binaries..."
if [ -d "target/wasm32-unknown-unknown/release" ]; then
    cp target/wasm32-unknown-unknown/release/*.wasm wasm/ 2>/dev/null || echo "  ℹ️  No WASM binaries found"
fi

# Optimize WASM files (if wasm-opt is available)
if command -v wasm-opt &> /dev/null; then
    echo ""
    echo "⚡ Optimizing WASM binaries..."
    for wasm in wasm/*.wasm; do
        if [ -f "$wasm" ]; then
            echo "  → Optimizing $(basename $wasm)..."
            wasm-opt -Oz "$wasm" -o "$wasm.opt"
            mv "$wasm.opt" "$wasm"
        fi
    done
else
    echo ""
    echo "ℹ️  wasm-opt not found. Install binaryen for WASM optimization:"
    echo "   Ubuntu/Debian: sudo apt install binaryen"
    echo "   macOS: brew install binaryen"
fi

echo ""
echo "✅ Build complete!"
echo ""
echo "📊 Build Summary:"
echo "  - Library: ✅ Compiled"
echo "  - Binaries: Need Odra integration (see deploy.sh)"
echo ""
echo "Next steps:"
echo "  1. For testnet: ./deploy.sh testnet"
echo "  2. For mainnet: ./deploy.sh mainnet"
