# CasperVault Frontend

> Modern, white-themed React frontend for CasperVault - a cross-chain DeFi aggregator on Casper Network

## 🎨 Design System

### Theme
- **Background**: Clean white with subtle gradients
- **Primary Color**: Sky Blue (#0ea5e9)
- **Accent Color**: Purple (#d946ef)
- **Typography**: Inter (body), Poppins (headings), Fira Code (mono)

### Components
All design system components follow a consistent, minimalist aesthetic with smooth animations.

## 🚀 Tech Stack

- **Framework**: React 18 + Vite 5
- **Language**: TypeScript
- **Styling**: TailwindCSS 3
- **State Management**: Zustand 4, TanStack Query 5
- **UI Components**: Headless UI 2, Framer Motion 11
- **3D Graphics**: Three.js, React Three Fiber
- **Data Visualization**: D3.js 7, Recharts 2
- **Blockchain**: Casper JS SDK 2
- **Icons**: React Icons 5

## 📦 Project Structure

```
src/
├── components/
│   ├── layout/          # Navbar, Footer, Layout
│   ├── common/          # Button, Card, Input, Modal, Badge, Loader
│   ├── three/           # ParticleBackground, FloatingElements
│   ├── wallet/          # WalletButton, WalletModal
│   └── charts/          # TVLChart, APYChart, AllocationPieChart
├── pages/               # Dashboard, Strategies, Analytics, Portfolio
├── hooks/               # Custom React hooks
├── services/            # API, Wallet, WebSocket services
├── store/               # Zustand stores
├── types/               # TypeScript type definitions
├── utils/               # Helper functions and constants
└── config/              # Casper network configuration
```

## 🛠️ Installation

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

## 🎯 Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint

## 📱 Design System Components

### Button
Variants: `primary`, `secondary`, `outline`, `success`, `danger`
Sizes: `sm`, `md`, `lg`

### Card
White cards with hover effects and optional icons

### Input
Form inputs with labels, errors, and helper text

### Modal
Accessible modals with Headless UI

### Badge
Status badges with color variants

### Loader
Animated loading spinners

## 📄 License

MIT License - Built for Casper Hackathon 2026

---

Built with ❤️ using React + Vite + TypeScript + TailwindCSS
