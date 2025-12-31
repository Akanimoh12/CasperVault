# 🚀 CasperVault Testnet Deployment - Quick Start

## ✅ Prerequisites Complete

- [x] Casper Client installed
- [x] Deployment keys generated
- [x] Build successful (0 errors)

## 🔑 Your Deployment Keys

**Public Key:**
```
01ecc4c23a00a1fc46f03a12dff79b17679a28006ceb454c5b6f0b537873a54037
```

**Location:**
- Public Key: `keys/public_key.pem`
- Secret Key: `keys/secret_key.pem` (Keep this secure!)

## 💰 Step 1: Get Testnet CSPR

### Option A: Using the Faucet (Recommended)

1. **Visit the testnet faucet:**
   https://testnet.cspr.live/tools/faucet

2. **Paste your public key:**
   ```
   01ecc4c23a00a1fc46f03a12dff79b17679a28006ceb454c5b6f0b537873a54037
   ```

3. **Request funds** (you'll receive ~1000 CSPR)

4. **Wait 1-2 minutes** for the transaction to confirm

5. **Verify your balance:**
   ```bash
   casper-client get-account-info \
     --node-address http://3.143.158.19:7777 \
     --public-key 01ecc4c23a00a1fc46f03a12dff79b17679a28006ceb454c5b6f0b537873a54037
   ```

### Option B: Using Casper Signer

1. Install Casper Signer browser extension
2. Import your keys
3. Use the extension's faucet integration

## 🚀 Step 2: Deploy Contracts

Once you have testnet funds, you have several deployment options:

### Option A: Run the Deployment Script

```bash
cd contracts
./testnet-deploy.sh
```

This will:
- Check your balance
- Build contracts
- Guide you through deployment

### Option B: Frontend Deployment (Most User-Friendly)

```bash
# 1. Start the frontend
cd frontend
npm install
npm run dev

# 2. Open http://localhost:3000
# 3. Connect Casper Signer
# 4. Use the deployment interface
```

### Option C: Manual Deployment (Advanced)

For manual deployment with casper-client, you'll need WASM contract files. Since we're using Odra framework, the recommended approach is either Option A or B.

## 📊 Deployment Checklist

- [ ] Get testnet CSPR from faucet
- [ ] Verify balance (at least 500 CSPR recommended)
- [ ] Run deployment script
- [ ] Deploy lstCSPR token
- [ ] Deploy cvCSPR token
- [ ] Deploy LiquidStaking contract
- [ ] Deploy VaultManager contract
- [ ] Initialize contracts
- [ ] Test basic functions
- [ ] Verify on explorer

## 🔍 After Deployment

### Check Your Deployed Contracts

Visit the Casper testnet explorer:
https://testnet.cspr.live/account/01ecc4c23a00a1fc46f03a12dff79b17679a28006ceb454c5b6f0b537873a54037

### Test Contract Functions

```bash
# Example: Check contract state
casper-client query-global-state \
  --node-address http://3.143.158.19:7777 \
  --state-root-hash <STATE_ROOT> \
  --key <CONTRACT_HASH>
```

## 📚 Helpful Resources

- **Testnet Explorer:** https://testnet.cspr.live
- **Testnet Faucet:** https://testnet.cspr.live/tools/faucet
- **Full Documentation:** See DEPLOYMENT.md
- **Casper Docs:** https://docs.casper.network

## 🆘 Troubleshooting

### "Account not found"
→ You need testnet CSPR. Use the faucet (see Step 1)

### "Insufficient funds"
→ Request more CSPR from the faucet (wait 24h between requests)

### "Build failed"
→ Run `cargo check --lib` to see errors

### "Contract deployment failed"
→ Check gas payment amount (increase if needed)

## 💡 Quick Commands

```bash
# Check balance
casper-client get-account-info \
  --node-address http://3.143.158.19:7777 \
  --public-key $(cat keys/public_key_hex)

# Build contracts
cargo build --release --lib

# Deploy to testnet
./testnet-deploy.sh

# Check deployment status
casper-client get-deploy \
  --node-address http://3.143.158.19:7777 \
  <DEPLOY_HASH>
```

---

## ⚡ Quick Start Summary

1. **Get funds:** Visit https://testnet.cspr.live/tools/faucet
2. **Paste key:** `01ecc4c23a00a1fc46f03a12dff79b17679a28006ceb454c5b6f0b537873a54037`
3. **Wait:** 1-2 minutes
4. **Deploy:** Run `./testnet-deploy.sh`

**That's it! Your contracts are ready to deploy!** 🎉
