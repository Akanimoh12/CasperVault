# 🔐 Security & Keys Guide

## **IMPORTANT: Never Commit Sensitive Data!**

This project contains sensitive information that should **NEVER** be committed to git:
- Private keys (`.pem` files)
- Environment variables (`.env` files)
- Wallet addresses and secrets
- API keys and tokens

---

## **Protected Files & Directories**

The following are automatically ignored by `.gitignore`:

### Keys & Secrets
```
keys/                    # All wallet keys
*.pem                   # Private/public key files
*.key                   # Key files
*secret*                # Any file with "secret" in name
.env                    # Environment variables
.env.local              # Local environment variables
.env.production         # Production environment variables
```

### Sensitive Configuration
```
account_info.json       # Account details
public_key_hex          # Hex-encoded public keys
```

---

## **Setup Instructions**

### 1. Create Your Keys (First Time)
```bash
# Run the wallet generation script
./generate-cli-wallet.sh

# This creates (in keys/ directory):
# - secret_key.pem       (PRIVATE - never share!)
# - public_key.pem       (PUBLIC - safe to share)
# - public_key_hex       (PUBLIC - safe to share)
```

### 2. Configure Environment Variables

**Root Directory:**
```bash
cp .env.example .env
# Edit .env with your values
```

**Backend:**
```bash
cd backend
cp .env.example .env
# Edit backend/.env with your values
```

**Frontend:**
```bash
cd frontend
cp .env.example .env
# Edit frontend/.env with your values
```

---

## **What's Safe to Commit?**

✅ **SAFE:**
- `.env.example` files (templates without real values)
- `keys/.gitkeep` (placeholder to keep directory in git)
- Public documentation
- Code files without hardcoded secrets

❌ **NEVER COMMIT:**
- `.env` files with real values
- `keys/*.pem` files
- `public_key_hex` file
- Any file with actual private keys, passwords, or API keys

---

## **Checking for Sensitive Data**

Before pushing to git, always verify:

```bash
# Check what's being tracked
git status

# Check if any sensitive files are staged
git diff --cached

# Search for potential secrets in code
grep -r "secret_key\|private_key\|password\|api_key" . --exclude-dir=node_modules --exclude-dir=target
```

---

## **If You Accidentally Committed Secrets**

### Immediate Actions:

1. **Remove from git history:**
```bash
# Remove file from git but keep locally
git rm --cached keys/secret_key.pem

# Commit the removal
git commit -m "Remove sensitive files from tracking"

# For deeper history cleanup (if already pushed)
git filter-branch --force --index-filter \
  "git rm --cached --ignore-unmatch keys/secret_key.pem" \
  --prune-empty --tag-name-filter cat -- --all

# Force push (WARNING: rewrites history)
git push origin --force --all
```

2. **Rotate the compromised keys:**
```bash
# Generate new wallet keys
./generate-cli-wallet.sh

# Update contract deployments with new keys
# Transfer funds from old wallet to new wallet
```

3. **Update all services:**
- Re-deploy contracts with new admin keys
- Update backend environment variables
- Rotate API keys and secrets

---

## **Best Practices**

1. ✅ **Always use environment variables** for sensitive data
2. ✅ **Keep `.env.example` updated** as a template
3. ✅ **Review `.gitignore`** before committing
4. ✅ **Use separate keys** for development and production
5. ✅ **Regularly rotate secrets** (especially API keys)
6. ✅ **Never hardcode secrets** in source code
7. ✅ **Use a password manager** to store production secrets
8. ✅ **Limit access** to production keys (need-to-know basis)

---

## **Production Deployment Security**

For production deployment:

1. **Use environment-specific keys:**
   - Development: Testnet keys with limited funds
   - Production: Mainnet keys with proper security

2. **Secure storage:**
   - Use HashiCorp Vault, AWS Secrets Manager, or similar
   - Never store production keys on developer machines

3. **Access control:**
   - Multi-signature wallets for contract administration
   - Role-based access control (RBAC) for team members
   - Audit logs for all secret access

4. **Monitoring:**
   - Alert on unusual transactions
   - Monitor for unauthorized key usage
   - Regular security audits

---

## **Emergency Contacts**

If you suspect a security breach:

1. **Immediately pause contracts** (if emergency pause is enabled)
2. **Rotate all compromised credentials**
3. **Notify team members**
4. **Review transaction history for unauthorized access**
5. **Document the incident** for post-mortem analysis

---

## **References**

- [Casper Security Best Practices](https://docs.casper.network/security/)
- [Git Secret Management](https://git-secret.io/)
- [OWASP Secrets Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html)

---

**Remember: Security is everyone's responsibility. When in doubt, don't commit it!** 🔒
