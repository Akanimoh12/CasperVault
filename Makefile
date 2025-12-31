# CasperVault Makefile
# Common commands for building, testing, and deploying

.PHONY: all build test clean deploy-testnet deploy-mainnet verify health help

# Default target
all: build test

# Build contracts
build:
	@echo "Building contracts..."
	cd contracts && cargo build --release
	@echo "✓ Build complete"

# Build for specific target
build-wasm:
	@echo "Building WASM..."
	cd contracts && cargo build --release --target wasm32-unknown-unknown
	@echo "✓ WASM build complete"

# Run all tests
test:
	@echo "Running tests..."
	cd contracts && cargo test
	@echo "✓ Tests passed"

# Run unit tests only
test-unit:
	@echo "Running unit tests..."
	cd contracts && cargo test --test vault_unit_tests
	cd contracts && cargo test --test staking_unit_tests
	cd contracts && cargo test --test strategy_unit_tests
	@echo "✓ Unit tests passed"

# Run integration tests
test-integration:
	@echo "Running integration tests..."
	cd contracts && cargo test --test vault_integration
	cd contracts && cargo test --test strategy_integration
	@echo "✓ Integration tests passed"

# Run E2E tests
test-e2e:
	@echo "Running E2E tests..."
	cd contracts && cargo test --test full_user_journey
	cd contracts && cargo test --test multi_user_scenario
	@echo "✓ E2E tests passed"

# Run security tests
test-security:
	@echo "Running security tests..."
	cd contracts && cargo test --test attack_scenarios
	@echo "✓ Security tests passed"

# Generate test coverage
coverage:
	@echo "Generating coverage report..."
	cd contracts && cargo tarpaulin --out Html --output-dir ../coverage
	@echo "✓ Coverage report: coverage/index.html"

# Format code
format:
	@echo "Formatting code..."
	cd contracts && cargo fmt
	@echo "✓ Code formatted"

# Check code formatting
format-check:
	@echo "Checking code formatting..."
	cd contracts && cargo fmt -- --check
	@echo "✓ Format check passed"

# Run clippy linter
lint:
	@echo "Running clippy..."
	cd contracts && cargo clippy -- -D warnings
	@echo "✓ Lint check passed"

# Clean build artifacts
clean:
	@echo "Cleaning..."
	cd contracts && cargo clean
	rm -rf coverage/
	rm -f scripts/addresses*.json
	rm -f *.log
	@echo "✓ Clean complete"

# Deploy to testnet (dry run first)
deploy-testnet-dry:
	@echo "Testnet deployment (DRY RUN)..."
	DRY_RUN=true bash scripts/deploy/deploy-testnet.sh
	@echo "✓ Dry run complete"

# Deploy to testnet
deploy-testnet:
	@echo "Deploying to testnet..."
	bash scripts/deploy/deploy-testnet.sh
	@echo "✓ Testnet deployment complete"

# Deploy to mainnet (dry run first - REQUIRED)
deploy-mainnet-dry:
	@echo "Mainnet deployment (DRY RUN)..."
	DRY_RUN=true bash scripts/deploy/deploy-mainnet.sh
	@echo "✓ Dry run complete"

# Deploy to mainnet
deploy-mainnet:
	@echo "Deploying to mainnet..."
	@echo "WARNING: This will deploy to MAINNET"
	@read -p "Are you sure? Type 'yes' to continue: " confirm; \
	if [ "$$confirm" = "yes" ]; then \
		DRY_RUN=false bash scripts/deploy/deploy-mainnet.sh; \
	else \
		echo "Deployment cancelled"; \
	fi

# Verify deployment
verify:
	@echo "Verifying deployment..."
	bash scripts/verify/verify-deployment.sh testnet
	@echo "✓ Verification complete"

verify-mainnet:
	@echo "Verifying mainnet deployment..."
	bash scripts/verify/verify-deployment.sh mainnet
	@echo "✓ Mainnet verification complete"

# Test full flow
test-flow:
	@echo "Testing full flow..."
	bash scripts/deploy/test-full-flow.sh
	@echo "✓ Full flow test complete"

# Check system health
health:
	@echo "Checking system health..."
	bash scripts/monitor/check-health.sh testnet

health-mainnet:
	@echo "Checking mainnet health..."
	bash scripts/monitor/check-health.sh mainnet

# Monitor events
monitor:
	@echo "Monitoring events (Ctrl+C to stop)..."
	bash scripts/monitor/monitor-events.sh testnet

monitor-mainnet:
	@echo "Monitoring mainnet events (Ctrl+C to stop)..."
	bash scripts/monitor/monitor-events.sh mainnet

# Add validator
add-validator:
	@read -p "Validator address: " addr; \
	read -p "Validator name: " name; \
	bash scripts/manage/add-validator.sh $$addr $$name testnet

# Add strategy
add-strategy:
	@read -p "Strategy type: " type; \
	read -p "Allocation %: " alloc; \
	bash scripts/manage/add-strategy.sh $$type $$alloc testnet

# Update fees
update-fees:
	@read -p "Performance fee (bps): " perf; \
	read -p "Management fee (bps): " mgmt; \
	bash scripts/manage/update-fees.sh $$perf $$mgmt testnet

# Upgrade contract
upgrade:
	@read -p "Contract name: " contract; \
	read -p "WASM path: " wasm; \
	bash scripts/deploy/upgrade-contract.sh $$contract $$wasm testnet

# Install dependencies
install-deps:
	@echo "Installing dependencies..."
	rustup target add wasm32-unknown-unknown
	cargo install odra-casper-livenet-env
	cargo install casper-client
	cargo install cargo-tarpaulin
	@echo "✓ Dependencies installed"

# Setup development environment
setup:
	@echo "Setting up development environment..."
	make install-deps
	make build
	make test
	@echo "✓ Setup complete"

# Pre-commit checks
pre-commit: format-check lint test
	@echo "✓ All pre-commit checks passed"

# CI pipeline
ci: build format-check lint test coverage
	@echo "✓ CI pipeline complete"

# Show help
help:
	@echo "CasperVault Makefile Commands:"
	@echo ""
	@echo "Building:"
	@echo "  make build              - Build contracts"
	@echo "  make build-wasm         - Build WASM targets"
	@echo "  make clean              - Clean build artifacts"
	@echo ""
	@echo "Testing:"
	@echo "  make test               - Run all tests"
	@echo "  make test-unit          - Run unit tests"
	@echo "  make test-integration   - Run integration tests"
	@echo "  make test-e2e           - Run E2E tests"
	@echo "  make test-security      - Run security tests"
	@echo "  make coverage           - Generate coverage report"
	@echo "  make test-flow          - Test full user flow"
	@echo ""
	@echo "Code Quality:"
	@echo "  make format             - Format code"
	@echo "  make format-check       - Check formatting"
	@echo "  make lint               - Run clippy"
	@echo "  make pre-commit         - Run all pre-commit checks"
	@echo ""
	@echo "Deployment:"
	@echo "  make deploy-testnet-dry - Testnet dry run"
	@echo "  make deploy-testnet     - Deploy to testnet"
	@echo "  make deploy-mainnet-dry - Mainnet dry run (REQUIRED)"
	@echo "  make deploy-mainnet     - Deploy to mainnet"
	@echo "  make verify             - Verify testnet deployment"
	@echo "  make verify-mainnet     - Verify mainnet deployment"
	@echo ""
	@echo "Management:"
	@echo "  make add-validator      - Add validator to whitelist"
	@echo "  make add-strategy       - Add new strategy"
	@echo "  make update-fees        - Update fee structure"
	@echo "  make upgrade            - Upgrade contract"
	@echo ""
	@echo "Monitoring:"
	@echo "  make health             - Check system health"
	@echo "  make health-mainnet     - Check mainnet health"
	@echo "  make monitor            - Monitor events"
	@echo "  make monitor-mainnet    - Monitor mainnet events"
	@echo ""
	@echo "Setup:"
	@echo "  make install-deps       - Install dependencies"
	@echo "  make setup              - Full development setup"
	@echo ""
