# CasperVault Frontend Deployment Guide

## Deploy to Vercel

### Prerequisites
- Vercel account (sign up at [vercel.com](https://vercel.com))
- Vercel CLI installed: `npm i -g vercel`
- Git repository connected (optional but recommended)

### Method 1: Deploy via Vercel CLI (Fastest)

1. **Navigate to frontend directory**:
   ```bash
   cd frontend
   ```

2. **Login to Vercel**:
   ```bash
   vercel login
   ```

3. **Deploy**:
   ```bash
   # Deploy to preview (staging)
   vercel
   
   # Deploy to production
   vercel --prod
   ```

4. **Follow the prompts**:
   - Set up and deploy? `Y`
   - Which scope? Select your account
   - Link to existing project? `N` (first time) or `Y` (subsequent deploys)
   - Project name? `caspervault` or your preferred name
   - Directory? `./` (current directory)
   - Want to override settings? `N`

5. **Access your deployment**:
   - Preview: `https://caspervault-xxx.vercel.app`
   - Production: `https://caspervault.vercel.app`

### Method 2: Deploy via Vercel Dashboard (Recommended for Teams)

1. **Push your code to GitHub/GitLab/Bitbucket**

2. **Go to Vercel Dashboard**:
   - Visit [vercel.com/dashboard](https://vercel.com/dashboard)
   - Click "Add New..." → "Project"

3. **Import your repository**:
   - Select your Git provider
   - Choose the `CasperVault` repository
   - Click "Import"

4. **Configure project**:
   - **Framework Preset**: Vite (auto-detected)
   - **Root Directory**: `frontend`
   - **Build Command**: `npm run build` (auto-detected)
   - **Output Directory**: `dist` (auto-detected)
   - **Install Command**: `npm install` (auto-detected)

5. **Add Environment Variables** (if needed):
   ```
   VITE_API_URL=https://api.caspervault.io
   VITE_CASPER_NETWORK=casper-test
   VITE_NODE_ADDRESS=https://testnet.cspr.live
   ```

6. **Deploy**:
   - Click "Deploy"
   - Wait 2-3 minutes for build to complete
   - Your site will be live at `https://your-project.vercel.app`

### Method 3: Deploy from Local Build

1. **Build the project locally**:
   ```bash
   npm run build
   ```

2. **Deploy the dist folder**:
   ```bash
   vercel --prod ./dist
   ```

## Configuration Files

### vercel.json
Already configured with:
- ✅ SPA routing (all routes redirect to index.html)
- ✅ Security headers (XSS, frame options, etc.)
- ✅ Asset caching (1 year for static assets)
- ✅ Clean URLs
- ✅ Production environment

### Environment Variables

Create a `.env.production` file (not committed to git):

```env
VITE_API_URL=https://api.caspervault.io
VITE_CASPER_NETWORK=casper-test
VITE_NODE_ADDRESS=https://testnet.cspr.live
VITE_CONTRACT_VAULT_MANAGER=hash-xxxxx
VITE_CONTRACT_LIQUID_STAKING=hash-xxxxx
VITE_CONTRACT_STRATEGY_ROUTER=hash-xxxxx
```

Or add them in Vercel Dashboard:
- Go to Project Settings → Environment Variables
- Add each variable
- Choose "Production", "Preview", or "Development" scope

## Custom Domain Setup

### Add Custom Domain

1. **In Vercel Dashboard**:
   - Go to your project
   - Settings → Domains
   - Click "Add"

2. **Enter your domain**:
   ```
   app.caspervault.io
   ```

3. **Configure DNS**:
   
   **Option A - Vercel Nameservers** (Recommended):
   - Point your domain's nameservers to Vercel
   - Vercel handles SSL automatically
   
   **Option B - CNAME Record**:
   ```
   Type: CNAME
   Name: app (or @)
   Value: cname.vercel-dns.com
   ```

4. **SSL Certificate**:
   - Automatically provisioned by Vercel
   - Usually takes 1-2 minutes

## Performance Optimization

### Already Configured
- ✅ Asset compression (Brotli + Gzip)
- ✅ HTTP/2 push
- ✅ Edge caching
- ✅ Automatic image optimization
- ✅ Code splitting (Vite)
- ✅ Tree shaking

### Additional Optimizations

1. **Preload critical assets** in `index.html`:
   ```html
   <link rel="preload" href="/assets/critical.js" as="script">
   <link rel="preload" href="/assets/critical.css" as="style">
   ```

2. **Enable analytics**:
   - Vercel Dashboard → Analytics → Enable
   - Track Core Web Vitals

3. **Monitor performance**:
   - Use Vercel Speed Insights
   - Check Lighthouse scores

## Deployment Checklist

Before deploying to production:

- [ ] Test build locally: `npm run build && npm run preview`
- [ ] Check for console errors in production build
- [ ] Verify all environment variables are set
- [ ] Test wallet connection on testnet
- [ ] Verify API endpoints are accessible
- [ ] Test all routes and navigation
- [ ] Check mobile responsiveness
- [ ] Run Lighthouse audit (aim for 90+ scores)
- [ ] Test with different wallets (CSPR.click, etc.)
- [ ] Verify particle animations work smoothly
- [ ] Check WebSocket connections
- [ ] Test deposit/withdraw flows

## Continuous Deployment

### Automatic Deployments

Vercel automatically deploys:
- **Production**: When you push to `main` or `master` branch
- **Preview**: When you push to any other branch or open a PR

### Branch Configuration

In Vercel Dashboard:
- Settings → Git → Production Branch → `main`
- Enable "Auto-expose System Environment Variables"
- Enable "Automatically expose preview URLs"

## Troubleshooting

### Build Fails

1. **Clear cache and rebuild**:
   ```bash
   vercel --prod --force
   ```

2. **Check Node version**:
   - Vercel uses Node 18 by default
   - Specify version in `package.json`:
     ```json
     "engines": {
       "node": "18.x"
     }
     ```

### Routes Not Working

- Ensure `vercel.json` has the rewrite rule (already configured)
- Check browser console for errors

### Environment Variables Not Loaded

- Prefix all variables with `VITE_`
- Restart dev server after changes
- Redeploy after adding variables in Vercel

### Slow Performance

1. **Analyze bundle size**:
   ```bash
   npm run build -- --analyze
   ```

2. **Reduce bundle size**:
   - Lazy load routes
   - Remove unused dependencies
   - Optimize images

## Post-Deployment

### Monitor Your Deployment

1. **Check deployment status**:
   ```bash
   vercel ls
   ```

2. **View logs**:
   ```bash
   vercel logs
   ```

3. **Inspect build details**:
   - Visit Vercel Dashboard → Deployments
   - Click on specific deployment
   - View build logs, runtime logs, and analytics

### Share Your App

Your app is now live at:
- **Vercel URL**: `https://caspervault.vercel.app`
- **Custom Domain**: `https://app.caspervault.io` (if configured)

Share with:
- Team members for testing
- Beta users for feedback
- Community for demo
- Hackathon judges for evaluation

## Support

- **Vercel Docs**: [vercel.com/docs](https://vercel.com/docs)
- **Vite Deployment**: [vitejs.dev/guide/static-deploy.html#vercel](https://vitejs.dev/guide/static-deploy.html#vercel)
- **Vercel Support**: [vercel.com/support](https://vercel.com/support)

---

**Happy Deploying! 🚀**
