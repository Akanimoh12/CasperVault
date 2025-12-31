#!/bin/bash

# Quick start script for CasperVault Backend

set -e

echo "🚀 CasperVault Backend - Quick Start"
echo "===================================="
echo ""

# Check Node.js version
echo "📋 Checking Node.js version..."
node_version=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$node_version" -lt 18 ]; then
  echo "❌ Node.js 18+ required. Current version: $(node -v)"
  exit 1
fi
echo "✅ Node.js version: $(node -v)"
echo ""

# Check if .env exists
if [ ! -f ".env" ]; then
  echo "⚠️  .env file not found"
  echo "📝 Creating .env from .env.example..."
  cp ../.env.example .env
  echo "✅ .env created - Please edit it with your configuration"
  echo ""
fi

# Install dependencies
if [ ! -d "node_modules" ]; then
  echo "📦 Installing dependencies..."
  npm install
  echo "✅ Dependencies installed"
  echo ""
fi

# Build TypeScript
echo "🔨 Building TypeScript..."
npm run build
echo "✅ Build complete"
echo ""

# Run tests
echo "🧪 Running tests..."
npm test -- --passWithNoTests
echo "✅ Tests passed"
echo ""

# Success message
echo "✨ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Edit .env file with your configuration"
echo "2. Deploy smart contracts and add contract hashes"
echo "3. Run: npm run dev"
echo ""
echo "Available commands:"
echo "  npm run dev              - Start development server"
echo "  npm run build            - Build TypeScript"
echo "  npm start                - Start production server"
echo "  npm test                 - Run tests"
echo "  npm run lint             - Check code style"
echo ""
