# Makefile for CasperVault Smart Contracts

.PHONY: help build test clean deploy-testnet deploy-mainnet

# Default target
help:
	@echo "CasperVault Smart Contracts - Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build           - Build all contracts"
	@echo "  test            - Run all tests"
	@echo "  clean           - Clean build artifacts"
	@echo "  deploy-testnet  - Deploy to Casper testnet"
	@echo "  check           - Run cargo check"
	@echo "  fmt             - Format code"
	@echo "  clippy          - Run clippy linter"

# Build all contracts
build:
	@echo "Building contracts..."
	cargo build --release

# Build specific contracts
build-vault:
	cargo build --release --bin caspervault_vault_manager

build-staking:
	cargo build --release --bin caspervault_liquid_staking

build-tokens:
	cargo build --release --bin caspervault_lst_cspr
	cargo build --release --bin caspervault_cv_cspr

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Clean build artifacts
clean:
	@echo "Cleaning..."
	cargo clean

# Check code
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Deploy to testnet (placeholder)
deploy-testnet:
	@echo "Deploying to Casper testnet..."
	@echo "Run deployment script: ./scripts/deploy-testnet.sh"

# Deploy to mainnet (placeholder)
deploy-mainnet:
	@echo "Deploying to Casper mainnet..."
	@echo "WARNING: This will deploy to production!"
	@echo "Run deployment script: ./scripts/deploy-mainnet.sh"

# Run all checks before commit
pre-commit: fmt clippy test
	@echo "All checks passed!"
