# CasperVault Project Structure

> **Complete Development Structure for Hackathon Implementation**

---

## 📁 Root Directory Structure

```
caspervault/
├── contracts/                          # Rust smart contracts
├── frontend/                           # React frontend application
├── backend/                            # Backend services & bots
├── docs/                              # Documentation
├── scripts/                           # Deployment & utility scripts
├── tests/                             # Integration tests
├── .github/                           # GitHub Actions CI/CD
├── docker-compose.yml                 # Docker services
├── .gitignore
├── README.md
└── LICENSE
```

---

## 🦀 Smart Contracts Directory (`/contracts`)

```
contracts/
├── src/
│   ├── core/
│   │   ├── mod.rs
│   │   ├── vault_manager.rs           # Main vault logic
│   │   ├── liquid_staking.rs          # Staking mechanism
│   │   ├── strategy_router.rs         # Strategy allocation
│   │   └── yield_aggregator.rs        # Yield collection
│   │
│   ├── tokens/
│   │   ├── mod.rs
│   │   ├── lst_cspr.rs                # Liquid staking token
│   │   └── cv_cspr.rs                 # Vault share token
│   │
│   ├── strategies/
│   │   ├── mod.rs
│   │   ├── strategy_interface.rs      # IStrategy trait
│   │   ├── dex_strategy.rs            # DEX LP strategy
│   │   ├── lending_strategy.rs        # Lending protocol
│   │   └── crosschain_strategy.rs     # Bridge + deploy
│   │
│   ├── bridges/
│   │   ├── mod.rs
│   │   ├── bridge_adapter.rs          # Bridge abstraction
│   │   └── message_relay.rs           # Cross-chain messaging
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── validator_registry.rs      # Validator management
│   │   ├── price_oracle.rs            # Price feeds
│   │   ├── access_control.rs          # Permissions
│   │   ├── reentrancy_guard.rs        # Security
│   │   └── math.rs                    # Math utilities
│   │
│   ├── types/
│   │   ├── mod.rs
│   │   ├── errors.rs                  # Custom errors
│   │   ├── events.rs                  # Contract events
│   │   └── structs.rs                 # Shared structs
│   │
│   └── lib.rs                         # Main entry point
│
├── tests/
│   ├── integration/
│   │   ├── vault_tests.rs
│   │   ├── staking_tests.rs
│   │   ├── strategy_tests.rs
│   │   └── end_to_end_tests.rs
│   │
│   └── unit/
│       ├── vault_unit_tests.rs
│       ├── staking_unit_tests.rs
│       └── token_unit_tests.rs
│
├── scripts/
│   ├── deploy.sh                      # Deployment script
│   ├── setup-testnet.sh               # Testnet setup
│   └── verify-contracts.sh            # Contract verification
│
├── Cargo.toml                         # Rust dependencies
├── Makefile                           # Build commands
└── README.md                          # Contract documentation
```

---

## ⚛️ Frontend Directory (`/frontend`)

```
frontend/
├── public/
│   ├── index.html
│   ├── favicon.ico
│   ├── logo.svg
│   └── assets/
│       ├── images/
│       └── animations/
│
├── src/
│   ├── components/
│   │   ├── Layout/
│   │   │   ├── Header.tsx             # Navigation header
│   │   │   ├── Footer.tsx             # Footer component
│   │   │   ├── Sidebar.tsx            # Mobile sidebar
│   │   │   └── Layout.tsx             # Main layout wrapper
│   │   │
│   │   ├── Dashboard/
│   │   │   ├── DashboardPage.tsx      # Main dashboard
│   │   │   ├── PortfolioCard.tsx      # Portfolio summary
│   │   │   ├── APYChart.tsx           # APY visualization
│   │   │   ├── StrategyAllocation.tsx # Pie/donut chart
│   │   │   ├── PerformanceGraph.tsx   # Line chart (D3.js)
│   │   │   └── StatsGrid.tsx          # Key metrics grid
│   │   │
│   │   ├── Vault/
│   │   │   ├── DepositModal.tsx       # Deposit interface
│   │   │   ├── WithdrawModal.tsx      # Withdraw interface
│   │   │   ├── TransactionHistory.tsx # User transactions
│   │   │   └── VaultStats.tsx         # Vault statistics
│   │   │
│   │   ├── Strategies/
│   │   │   ├── StrategiesPage.tsx     # All strategies view
│   │   │   ├── StrategyCard.tsx       # Individual strategy
│   │   │   ├── StrategyDetails.tsx    # Strategy deep dive
│   │   │   └── StrategyComparison.tsx # Compare strategies
│   │   │
│   │   ├── Analytics/
│   │   │   ├── AnalyticsPage.tsx      # Analytics dashboard
│   │   │   ├── TVLChart.tsx           # TVL over time
│   │   │   ├── UserMetrics.tsx        # User statistics
│   │   │   └── YieldBreakdown.tsx     # Yield sources
│   │   │
│   │   ├── Wallet/
│   │   │   ├── WalletConnect.tsx      # Connect button
│   │   │   ├── WalletModal.tsx        # Wallet selection
│   │   │   ├── AccountModal.tsx       # Account details
│   │   │   └── NetworkSwitch.tsx      # Network selector
│   │   │
│   │   ├── Shared/
│   │   │   ├── Button.tsx             # Reusable button
│   │   │   ├── Modal.tsx              # Modal wrapper
│   │   │   ├── Card.tsx               # Card component
│   │   │   ├── Input.tsx              # Input field
│   │   │   ├── LoadingSpinner.tsx     # Loading state
│   │   │   ├── Toast.tsx              # Notification toast
│   │   │   ├── Skeleton.tsx           # Loading skeleton
│   │   │   ├── Tooltip.tsx            # Info tooltip
│   │   │   └── Badge.tsx              # Status badge
│   │   │
│   │   └── Animations/
│   │       ├── ParticleBackground.tsx # Three.js background
│   │       ├── LoadingAnimation.tsx   # Custom loading
│   │       └── TransitionWrapper.tsx  # Page transitions
│   │
│   ├── hooks/
│   │   ├── useVault.ts                # Vault interactions
│   │   ├── useWallet.ts               # Wallet connection
│   │   ├── useStrategies.ts           # Strategy data
│   │   ├── useBalance.ts              # Token balances
│   │   ├── useTransactions.ts         # Transaction history
│   │   ├── useAPY.ts                  # APY calculations
│   │   └── useWebSocket.ts            # Real-time updates
│   │
│   ├── services/
│   │   ├── contract/
│   │   │   ├── VaultContract.ts       # Vault contract wrapper
│   │   │   ├── StakingContract.ts     # Staking contract
│   │   │   ├── TokenContract.ts       # Token contracts
│   │   │   └── StrategyContract.ts    # Strategy contracts
│   │   │
│   │   ├── api/
│   │   │   ├── apiClient.ts           # API client
│   │   │   ├── portfolioApi.ts        # Portfolio endpoints
│   │   │   ├── analyticsApi.ts        # Analytics endpoints
│   │   │   └── priceApi.ts            # Price feed API
│   │   │
│   │   └── websocket/
│   │       └── wsClient.ts            # WebSocket client
│   │
│   ├── utils/
│   │   ├── formatting.ts              # Number/date formatting
│   │   ├── calculations.ts            # APY/yield calculations
│   │   ├── validation.ts              # Input validation
│   │   ├── constants.ts               # App constants
│   │   └── helpers.ts                 # Utility functions
│   │
│   ├── store/
│   │   ├── walletStore.ts             # Wallet state (Zustand)
│   │   ├── vaultStore.ts              # Vault state
│   │   ├── uiStore.ts                 # UI state
│   │   └── index.ts                   # Store exports
│   │
│   ├── types/
│   │   ├── contracts.ts               # Contract types
│   │   ├── vault.ts                   # Vault types
│   │   ├── strategies.ts              # Strategy types
│   │   └── api.ts                     # API types
│   │
│   ├── styles/
│   │   ├── globals.css                # Global styles
│   │   ├── theme.ts                   # Theme configuration
│   │   └── animations.css             # CSS animations
│   │
│   ├── config/
│   │   ├── contracts.ts               # Contract addresses
│   │   ├── networks.ts                # Network configs
│   │   └── env.ts                     # Environment variables
│   │
│   ├── App.tsx                        # Main app component
│   ├── main.tsx                       # Entry point
│   └── vite-env.d.ts                  # Vite types
│
├── .env.example                       # Environment template
├── .env.local                         # Local environment
├── package.json                       # Dependencies
├── tsconfig.json                      # TypeScript config
├── vite.config.ts                     # Vite configuration
├── tailwind.config.js                 # Tailwind config
├── postcss.config.js                  # PostCSS config
└── README.md                          # Frontend documentation
```

---

## 🔧 Backend Directory (`/backend`)

```
backend/
├── src/
│   ├── services/
│   │   ├── optimizer/
│   │   │   ├── index.ts               # Yield optimizer bot
│   │   │   ├── apyFetcher.ts          # Fetch APYs from strategies
│   │   │   ├── allocator.ts           # Calculate optimal allocation
│   │   │   └── rebalancer.ts          # Execute rebalancing
│   │   │
│   │   ├── compounder/
│   │   │   ├── index.ts               # Auto-compounder bot
│   │   │   ├── harvester.ts           # Harvest yields
│   │   │   ├── swapper.ts             # Swap tokens if needed
│   │   │   └── reinvestor.ts          # Reinvest yields
│   │   │
│   │   ├── relayer/
│   │   │   ├── index.ts               # Bridge relayer
│   │   │   ├── eventListener.ts       # Listen to bridge events
│   │   │   ├── proofGenerator.ts      # Generate proofs
│   │   │   └── submitter.ts           # Submit cross-chain txs
│   │   │
│   │   └── monitor/
│   │       ├── index.ts               # Security monitor
│   │       ├── anomalyDetector.ts     # Detect anomalies
│   │       ├── alerter.ts             # Send alerts
│   │       └── metrics.ts             # Collect metrics
│   │
│   ├── api/
│   │   ├── supabase/
│   │   │   ├── functions/
│   │   │   │   ├── get-portfolio.ts   # User portfolio
│   │   │   │   ├── get-apy-history.ts # Historical APY
│   │   │   │   ├── get-transactions.ts # Transaction history
│   │   │   │   └── get-analytics.ts   # Analytics data
│   │   │   │
│   │   │   └── migrations/
│   │   │       ├── 001_create_tables.sql
│   │   │       ├── 002_add_indexes.sql
│   │   │       └── 003_add_functions.sql
│   │   │
│   │   └── rest/
│   │       ├── routes/
│   │       │   ├── portfolio.ts
│   │       │   ├── strategies.ts
│   │       │   └── analytics.ts
│   │       │
│   │       └── server.ts              # Express server
│   │
│   ├── contracts/
│   │   ├── VaultContract.ts           # Contract wrappers
│   │   ├── StakingContract.ts
│   │   └── StrategyContract.ts
│   │
│   ├── utils/
│   │   ├── logger.ts                  # Logging utility
│   │   ├── config.ts                  # Configuration
│   │   ├── database.ts                # DB connection
│   │   └── helpers.ts                 # Helper functions
│   │
│   └── types/
│       ├── contracts.ts               # Contract types
│       ├── database.ts                # DB types
│       └── events.ts                  # Event types
│
├── config/
│   ├── development.json               # Dev config
│   ├── testnet.json                   # Testnet config
│   └── production.json                # Production config
│
├── scripts/
│   ├── start-optimizer.sh             # Start optimizer bot
│   ├── start-compounder.sh            # Start compounder bot
│   └── start-monitor.sh               # Start monitoring
│
├── Dockerfile                         # Docker image
├── docker-compose.yml                 # Local services
├── package.json                       # Dependencies
├── tsconfig.json                      # TypeScript config
└── README.md                          # Backend documentation
```

---

## 📜 Scripts Directory (`/scripts`)

```
scripts/
├── setup/
│   ├── install-dependencies.sh        # Install all dependencies
│   ├── setup-testnet.sh               # Configure testnet
│   └── init-supabase.sh               # Initialize Supabase
│
├── deploy/
│   ├── deploy-contracts.sh            # Deploy smart contracts
│   ├── deploy-frontend.sh             # Deploy frontend (Vercel)
│   ├── deploy-backend.sh              # Deploy backend services
│   └── deploy-all.sh                  # Full deployment
│
├── test/
│   ├── test-contracts.sh              # Run contract tests
│   ├── test-frontend.sh               # Run frontend tests
│   └── test-integration.sh            # Integration tests
│
└── utils/
    ├── get-faucet.sh                  # Request testnet tokens
    ├── verify-deployment.sh           # Verify deployments
    └── generate-keys.sh               # Generate keypairs
```

---

## 🧪 Tests Directory (`/tests`)

```
tests/
├── integration/
│   ├── vault-deposit-withdraw.test.ts
│   ├── strategy-deployment.test.ts
│   ├── yield-harvesting.test.ts
│   └── cross-chain-flow.test.ts
│
├── e2e/
│   ├── user-journey.test.ts           # Full user flow
│   ├── deposit-flow.test.ts           # Deposit process
│   ├── withdraw-flow.test.ts          # Withdrawal process
│   └── wallet-connect.test.ts         # Wallet connection
│
├── fixtures/
│   ├── contracts.ts                   # Contract fixtures
│   ├── users.ts                       # User fixtures
│   └── strategies.ts                  # Strategy fixtures
│
└── helpers/
    ├── setup.ts                       # Test setup
    ├── teardown.ts                    # Test cleanup
    └── utils.ts                       # Test utilities
```

---

## 📚 Documentation Directory (`/docs`)

```
docs/
├── architecture/
│   ├── system-design.md               # System architecture
│   ├── contract-design.md             # Contract architecture
│   └── data-flow.md                   # Data flow diagrams
│
├── api/
│   ├── contract-api.md                # Smart contract API
│   ├── rest-api.md                    # REST API docs
│   └── websocket-api.md               # WebSocket API
│
├── guides/
│   ├── developer-guide.md             # Developer guide
│   ├── deployment-guide.md            # Deployment guide
│   ├── testing-guide.md               # Testing guide
│   └── security-guide.md              # Security guide
│
├── user/
│   ├── user-guide.md                  # User documentation
│   ├── faq.md                         # FAQ
│   └── troubleshooting.md             # Troubleshooting
│
└── assets/
    ├── diagrams/                      # Architecture diagrams
    ├── screenshots/                   # UI screenshots
    └── videos/                        # Demo videos
```

---

## 🐙 GitHub Workflows (`.github/workflows`)

```
.github/
├── workflows/
│   ├── contracts-ci.yml               # Contract testing
│   ├── frontend-ci.yml                # Frontend testing
│   ├── backend-ci.yml                 # Backend testing
│   ├── deploy-testnet.yml             # Deploy to testnet
│   ├── deploy-production.yml          # Deploy to production
│   └── security-audit.yml             # Security checks
│
└── ISSUE_TEMPLATE/
    ├── bug_report.md
    ├── feature_request.md
    └── security_report.md
```

---

## 🐳 Docker Configuration

```
docker/
├── Dockerfile.contracts               # Contracts build
├── Dockerfile.frontend                # Frontend build
├── Dockerfile.backend                 # Backend build
├── Dockerfile.optimizer               # Optimizer bot
└── Dockerfile.compounder              # Compounder bot
```

---

## 📋 Root Configuration Files

```
/
├── .gitignore                         # Git ignore rules
├── .env.example                       # Environment template
├── .eslintrc.js                       # ESLint config
├── .prettierrc                        # Prettier config
├── docker-compose.yml                 # Docker services
├── Makefile                           # Build commands
├── package.json                       # Root package.json
├── README.md                          # Project README
├── LICENSE                            # MIT License
└── CONTRIBUTING.md                    # Contribution guide
```

---

## 📊 Key Files Purpose

### Smart Contracts
- **vault_manager.rs**: Core vault logic, deposits, withdrawals
- **liquid_staking.rs**: CSPR staking and lstCSPR minting
- **strategy_router.rs**: Route funds to optimal strategies
- **yield_aggregator.rs**: Harvest and compound yields

### Frontend
- **DashboardPage.tsx**: Main user interface
- **DepositModal.tsx**: Deposit CSPR interface
- **APYChart.tsx**: Visualize yields (D3.js/Three.js)
- **useVault.ts**: Contract interaction hook

### Backend
- **optimizer/index.ts**: Yield optimization bot
- **compounder/index.ts**: Auto-compounding bot
- **relayer/index.ts**: Cross-chain bridge relayer
- **monitor/index.ts**: Security monitoring

---

## 🚀 Quick Start Commands

```bash
# Install all dependencies
make install

# Setup development environment
make setup-dev

# Run contract tests
make test-contracts

# Start frontend development server
make dev-frontend

# Start backend services
make dev-backend

# Deploy to testnet
make deploy-testnet

# Run full test suite
make test-all
```

---

## 📁 File Naming Conventions

### Smart Contracts (Rust)
- **Modules**: `snake_case` (e.g., `vault_manager.rs`)
- **Structs**: `PascalCase` (e.g., `VaultManager`)
- **Functions**: `snake_case` (e.g., `deposit`, `calculate_shares`)

### Frontend (TypeScript/React)
- **Components**: `PascalCase` (e.g., `DashboardPage.tsx`)
- **Hooks**: `camelCase` with `use` prefix (e.g., `useVault.ts`)
- **Utils**: `camelCase` (e.g., `formatting.ts`)
- **Types**: `PascalCase` (e.g., `VaultState`)

### Backend (TypeScript/Node)
- **Services**: `camelCase` (e.g., `apyFetcher.ts`)
- **Classes**: `PascalCase` (e.g., `YieldOptimizer`)
- **Functions**: `camelCase` (e.g., `fetchAllAPYs`)

---

## 🎯 Development Priorities

### Day 1-2: Foundation
- Smart contract skeletons
- Basic frontend structure
- Development environment setup

### Day 3-4: Core Features
- Complete vault + staking contracts
- Strategy implementations
- Frontend integration

### Day 5: Polish
- UI/UX improvements
- Testing and bug fixes
- Documentation

### Day 6: Launch
- Final testing
- Testnet deployment
- Demo preparation

---

**Status**: Ready for Development 🚀  
**Last Updated**: December 31, 2025
