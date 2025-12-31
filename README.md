# CasperVault - Casper Hackathon 2026 🏆

> **Cross-Chain DeFi Aggregator on Casper Network**  
> Combining Liquid Staking + Yield Optimization + Cross-Chain Strategies

---

## 📁 Documentation Structure

This repository contains complete development documentation for CasperVault, broken down into actionable prompts for Claude Sonnet 4.5.

### Core Documents

1. **[casper_project_ideals.md](./casper_project_ideals.md)** - Initial hackathon research and three project proposals
2. **[CasperVault.md](./CasperVault.md)** - Comprehensive 20,000+ word project documentation
3. **[PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md)** - Complete directory structure for development
4. **[PROMPTS_CONTRACTS.md](./PROMPTS_CONTRACTS.md)** - Smart contract development prompts (8 prompts)
5. **[PROMPTS_BACKEND.md](./PROMPTS_BACKEND.md)** - Backend development prompts (6 prompts)
6. **[PROMPTS_FRONTEND.md](./PROMPTS_FRONTEND.md)** - Frontend development prompts (10 prompts)

---

## 🎯 Project Overview

**CasperVault** is a cross-chain DeFi aggregator that:
- Automatically stakes CSPR for liquid staking tokens (lstCSPR)
- Allocates funds across multiple yield strategies (DEX, Lending, Cross-chain)
- Auto-compounds yields daily for maximum returns
- Rebalances strategies every 12 hours for optimal APY
- Provides instant withdrawals (with small fee) or standard 7-day unlocking

### Target Tracks
- ✅ **Interoperability Track** ($2,500) - Cross-chain bridges and strategies
- ✅ **Liquid Staking Track** ($2,500) - lstCSPR integration
- ✅ **Main Track** ($25,000) - Overall innovation

### Key Features
- **12-15% APY** (vs 8-10% standard staking)
- **Instant Withdrawals** (0.5% fee) or Standard (7 days, no fee)
- **Automated Optimization** (rebalancing every 12 hours)
- **Multi-Strategy Allocation** (DEX, Lending, Cross-chain)
- **Real-time Dashboard** (Three.js effects, D3.js visualizations)

---

## 🛠️ Tech Stack

### Smart Contracts
- **Language**: Rust
- **Framework**: Odra (https://odra.dev/)
- **Testing**: Odra Test Environment
- **Deployment**: Casper Testnet → Mainnet

### Backend
- **Runtime**: Node.js + TypeScript
- **API**: Express (REST) + WebSocket
- **Database**: Supabase (PostgreSQL)
- **Queue**: Bull (Redis-backed)
- **Bots**: Optimizer, Compounder, Relayer, Monitor
- **Deployment**: Docker + PM2

### Frontend
- **Framework**: React 18 + TypeScript + Vite
- **Styling**: TailwindCSS (white-themed, modern)
- **State**: Zustand + React Query
- **Animations**: Framer Motion
- **3D Effects**: Three.js + @react-three/fiber
- **Charts**: D3.js + Recharts
- **Icons**: React Icons
- **Wallet**: CSPR.click SDK
- **Deployment**: Vercel

---

## 📦 Development Prompts

### Smart Contracts (8 Prompts - 3-4 days)

1. **Project Setup & Core Structure** - Cargo setup, VaultManager, LiquidStaking skeletons
2. **Liquid Staking Implementation** - Stake/unstake, validator selection, rewards
3. **Vault Manager & Share Calculations** - ERC-4626 compliance, deposit/withdraw
4. **Strategy System & Router** - IStrategy trait, DEX/Lending/CrossChain strategies
5. **Yield Aggregator & Auto-Compounding** - Harvest yields, compound logic, APY tracking
6. **Security Features & Access Control** - Multi-sig, pause, rate limits, monitoring
7. **Complete Testing Suite** - Unit, integration, e2e, security tests (90%+ coverage)
8. **Deployment Scripts & Documentation** - Testnet/mainnet deployment, verification

### Backend (6 Prompts - 2-3 days)

1. **Project Setup & Architecture** - Express, WebSocket, Supabase, contract wrappers
2. **Contract Wrappers & Casper SDK** - Type-safe wrappers, transaction handling
3. **Yield Optimizer Bot** - APY fetching, allocation calculation, rebalancing
4. **Auto-Compounder Bot** - Yield harvesting, token swapping, compounding
5. **REST API & WebSocket Server** - Portfolio, strategies, analytics endpoints
6. **Monitoring, Logging & Deployment** - Prometheus metrics, Docker, PM2

### Frontend (10 Prompts - 3-4 days)

1. **Project Setup & Design System** - TailwindCSS white theme, component library
2. **Three.js Particle Background** - Animated particles, mouse interaction
3. **Layout & Navigation** - Navbar, footer, routing, mobile responsive
4. **Wallet Integration (CSPR.click)** - Connect/disconnect, balance, signing
5. **Dashboard Page** - TVL, APY, stats, charts, real-time updates
6. **Deposit & Withdraw Modals** - Beautiful UI, validation, transaction tracking
7. **Strategies Page** - Strategy cards, performance metrics, detailed views
8. **Analytics Page** - Advanced D3.js visualizations, TVL trends, yield distribution
9. **WebSocket Integration** - Real-time updates, notifications, live stats
10. **Testing, Optimization & Deployment** - Unit tests, e2e tests, Vercel deployment

---

## 📅 Development Timeline

### Day 1-4: Smart Contracts (Critical Path)
- Day 1: Prompts 1-2 (Setup + Liquid Staking)
- Day 2: Prompts 3-4 (Vault Manager + Strategies)
- Day 3: Prompts 5-6 (Yield Aggregator + Security)
- Day 4: Prompts 7-8 (Testing + Deployment)

### Day 2-4: Backend (Parallel)
- Day 2: Prompts 1-2 (Setup + Contract Wrappers)
- Day 3: Prompts 3-4 (Optimizer + Compounder Bots)
- Day 4: Prompts 5-6 (API + Deployment)

### Day 3-6: Frontend (Depends on Backend API)
- Day 3: Prompts 1-3 (Setup + Layout + Wallet)
- Day 4: Prompts 4-6 (Dashboard + Modals + Strategies)
- Day 5: Prompts 7-8 (Analytics + WebSocket)
- Day 6: Prompts 9-10 (Testing + Optimization + Deployment)

---

## 🚀 Quick Start

### For Claude Sonnet 4.5 Users

1. **Read the comprehensive documentation**:
   ```
   Open CasperVault.md and review all sections
   ```

2. **Start with contracts** (most critical):
   ```
   Open PROMPTS_CONTRACTS.md
   Execute Prompt 1, test, then Prompt 2, etc.
   ```

3. **Build backend services**:
   ```
   Open PROMPTS_BACKEND.md
   Execute sequentially
   ```

4. **Create frontend**:
   ```
   Open PROMPTS_FRONTEND.md
   Execute sequentially
   ```

### For Manual Development

1. **Clone and setup**:
   ```bash
   # Smart Contracts
   cd contracts
   cargo build
   
   # Backend
   cd backend
   npm install
   
   # Frontend
   cd frontend
   npm install
   ```

2. **Environment Variables**:
   ```bash
   # Copy .env.example files in each directory
   cp .env.example .env
   # Fill in your values
   ```

3. **Run Development**:
   ```bash
   # Contracts (testing)
   cd contracts && cargo test
   
   # Backend
   cd backend && npm run dev
   
   # Frontend
   cd frontend && npm run dev
   ```

---

## 🎨 Design Specifications

### Color Palette (White Theme)
- **Primary**: #0ea5e9 (Bright Blue)
- **Accent**: #d946ef (Vibrant Purple)
- **Success**: #10b981 (Green)
- **Warning**: #f59e0b (Amber)
- **Danger**: #ef4444 (Red)
- **Background**: #ffffff (Pure White)
- **Grays**: 50-900 scale for text/borders

### Typography
- **Headers**: Poppins (Bold, 600-800 weight)
- **Body**: Inter (Regular, 300-500 weight)
- **Mono**: Fira Code (Code blocks)

### Components
- **Cards**: White with subtle shadows, rounded corners (1rem)
- **Buttons**: Primary blue, hover effects, loading states
- **Inputs**: White with blue focus rings
- **Charts**: D3.js with smooth animations
- **Background**: Three.js particle effect (toggleable)

---

## 🔐 Security Features

1. **Smart Contract Level**:
   - ReentrancyGuard on all state-changing functions
   - Multi-signature for critical operations
   - Emergency pause mechanism
   - Rate limiting on deposits/withdrawals
   - Validator health monitoring

2. **Backend Level**:
   - API rate limiting (100 req/15min)
   - WebSocket authentication
   - Secure environment variables
   - Transaction monitoring
   - Anomaly detection

3. **Frontend Level**:
   - Input validation
   - XSS protection
   - Secure WebSocket connections
   - Wallet signature verification
   - Error boundaries

---

## 📊 Success Metrics

### Technical Metrics
- **Smart Contracts**: 90%+ test coverage
- **Backend**: 99.9% uptime, <100ms API response
- **Frontend**: Lighthouse score >90, <2s load time
- **Security**: No critical vulnerabilities

### Business Metrics
- **TVL Target**: $100K+ at launch
- **User Target**: 200+ users in first month
- **APY Target**: 12-15% average
- **Uptime**: 99.9%+

---

## 🏆 Hackathon Submission

### What Makes This Win:

1. **Innovation**: First cross-chain DeFi aggregator on Casper combining liquid staking + yield optimization
2. **Technical Excellence**: Production-ready code, comprehensive tests, modern stack
3. **User Experience**: Beautiful UI with real-time updates and smooth animations
4. **Interoperability**: Targets both special tracks (Interoperability + Liquid Staking)
5. **Completeness**: Full-stack solution with smart contracts, backend, frontend, and documentation
6. **Scalability**: Designed to handle thousands of users and millions in TVL

### Submission Checklist:
- ✅ Smart contracts deployed to Casper Testnet
- ✅ Backend services running on production servers
- ✅ Frontend deployed to Vercel with custom domain
- ✅ Comprehensive documentation and README
- ✅ Video demo showcasing all features
- ✅ GitHub repository with clean commit history
- ✅ Security audit report (if time permits)
- ✅ Testnet faucet guide for judges

---

## 📝 Additional Resources

- **Casper Documentation**: https://docs.casper.network/
- **Odra Framework**: https://odra.dev/docs/
- **CSPR.click Wallet**: https://www.csprclick.io/
- **Hackathon Page**: https://dorahacks.io/hackathon/casper-hackathon-2026
- **Casper Discord**: Join for support and community

---

## 👥 Team Structure (Recommended)

- **Smart Contract Developer**: Focus on Rust/Odra, security, testing
- **Backend Developer**: Node.js, bots, APIs, database, DevOps
- **Frontend Developer**: React, Three.js, D3.js, UI/UX, animations

*Can be done by 1-2 developers with full-stack capabilities using these prompts.*

---

## 📞 Support

For questions or issues:
- Review documentation thoroughly
- Check [CasperVault.md](./CasperVault.md) for detailed technical specs
- Use prompts sequentially - each builds on the previous
- Test after each prompt before moving forward

---

## 🎉 Good Luck!

You have everything you need to build a hackathon-winning project. Follow the prompts sequentially, test thoroughly, and ship something amazing!

**Deadline**: January 4, 2026  
**Let's win this! 🚀**

---

*Built with ❤️ for Casper Hackathon 2026*
