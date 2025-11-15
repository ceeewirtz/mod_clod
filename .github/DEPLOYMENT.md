# Deployment Setup Guide

## Option 1: Vercel GitHub Integration (Easiest)

This is the simplest approach - no GitHub Actions needed!

1. Go to **https://vercel.com/new**
2. Sign in with your GitHub account
3. Click "Import Project"
4. Select your `mod_clod` repository
5. Configure:
   - **Root Directory**: `raycast-hello`
   - Framework Preset: Next.js (auto-detected)
   - Build Command: `cd wasm-raycast && wasm-pack build --target web && cd .. && npm run build`
   - Install Command: Keep default
6. Add **Build Command Override**:
   ```bash
   cd wasm-raycast && cargo install wasm-pack && wasm-pack build --target web && cd .. && npm run build
   ```
7. Click "Deploy"

Vercel will automatically deploy on every push to your main branch!

---

## Option 2: GitHub Actions Deployment

If you want to use the GitHub Actions workflow (`.github/workflows/deploy.yml`), follow these steps:

### Step 1: Get Vercel Credentials

1. Install Vercel CLI locally or in the project:
   ```bash
   npm i -g vercel
   ```

2. Login to Vercel:
   ```bash
   vercel login
   ```

3. Link your project:
   ```bash
   cd raycast-hello
   vercel link
   ```

4. Get your credentials from `.vercel/project.json`:
   ```bash
   cat .vercel/project.json
   ```

   You'll need:
   - `projectId` → Use as `VERCEL_PROJECT_ID`
   - `orgId` → Use as `VERCEL_ORG_ID`

5. Get your Vercel token:
   - Go to https://vercel.com/account/tokens
   - Create a new token
   - Copy it → Use as `VERCEL_TOKEN`

### Step 2: Add GitHub Secrets

1. Go to your GitHub repository
2. Click **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Add these three secrets:
   - `VERCEL_TOKEN`: Your Vercel token
   - `VERCEL_ORG_ID`: Your org ID from project.json
   - `VERCEL_PROJECT_ID`: Your project ID from project.json

### Step 3: Deploy

Once secrets are configured, the workflow will automatically run when you:
- Push to `main` or `master` branch
- Manually trigger it from the Actions tab

---

## Option 3: GitHub Pages (Static Export)

If you prefer GitHub Pages, you need to configure Next.js for static export:

### Add to `next.config.ts`:
```typescript
const nextConfig: NextConfig = {
  output: 'export',
  images: {
    unoptimized: true,
  },
  // ... rest of your config
};
```

### Create `.github/workflows/gh-pages.yml`:
```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main, master]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown

      - name: Install wasm-pack
        run: cargo install wasm-pack

      - name: Build WASM
        working-directory: ./raycast-hello/wasm-raycast
        run: wasm-pack build --target web

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '18'
          cache: 'npm'
          cache-dependency-path: raycast-hello/package-lock.json

      - name: Install deps
        working-directory: ./raycast-hello
        run: npm ci

      - name: Build
        working-directory: ./raycast-hello
        run: npm run build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v2
        with:
          path: ./raycast-hello/out

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v3
```

Then enable GitHub Pages in your repo settings:
- Settings → Pages → Source: GitHub Actions

---

## Recommended Approach

**For beginners**: Use Option 1 (Vercel GitHub Integration)
- Zero configuration
- Automatic deployments
- Built-in preview deployments for PRs

**For advanced users**: Use Option 2 (GitHub Actions)
- More control
- Can customize build process
- Can add testing steps

**For free static hosting**: Use Option 3 (GitHub Pages)
- Completely free
- No external dependencies
- Limited to static export
