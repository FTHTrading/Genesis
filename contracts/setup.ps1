#!/usr/bin/env pwsh
#
# setup.ps1  —  Genesis World Token full deployment automation
#
# Phases:
#   1. Install dependencies (if needed)
#   2. Generate wallets (skip if .env already has DEPLOYER_PRIVATE_KEY)
#   3. Compile contracts
#   4. Run local Hardhat sanity-deploy (free, no POL needed)
#   5. Check deployer POL balance
#   6. Deploy to Polygon mainnet (if --mainnet flag passed OR user confirms)
#   7. Verify contract on Polygonscan
#
# Usage:
#   .\setup.ps1                   # wallets + local test deploy
#   .\setup.ps1 --mainnet         # wallets + local test + mainnet deploy
#   .\setup.ps1 --verify-only     # just verify an already-deployed contract
#
# Requires: Node 18+, npm

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$ContractsDir = $PSScriptRoot
Set-Location $ContractsDir

# ── Colours ──────────────────────────────────────────────────────────────────
function Write-Step  { param($msg) Write-Host "`n  ═══ $msg ═══" -ForegroundColor Cyan }
function Write-OK    { param($msg) Write-Host "  ✓ $msg" -ForegroundColor Green }
function Write-Warn  { param($msg) Write-Host "  ⚠ $msg" -ForegroundColor Yellow }
function Write-Fail  { param($msg) Write-Host "  ✗ $msg" -ForegroundColor Red }
function Write-Info  { param($msg) Write-Host "  $msg" }

# ── Flags ────────────────────────────────────────────────────────────────────
$DeployMainnet = $args -contains "--mainnet"
$VerifyOnly    = $args -contains "--verify-only"

Write-Host ""
Write-Host "  ╔══════════════════════════════════════════════════════╗" -ForegroundColor Magenta
Write-Host "  ║     Genesis World Token — Deployment Automation      ║" -ForegroundColor Magenta
Write-Host "  ╚══════════════════════════════════════════════════════╝" -ForegroundColor Magenta
Write-Host ""

# ─────────────────────────────────────────────────────────────────────────────
# Phase 1 — Dependencies
# ─────────────────────────────────────────────────────────────────────────────
Write-Step "Phase 1 — Dependencies"

if (-not (Test-Path "node_modules")) {
    Write-Info "Installing npm dependencies..."
    npm install --silent
    if ($LASTEXITCODE -ne 0) { Write-Fail "npm install failed"; exit 1 }
}
Write-OK "node_modules present"

# ─────────────────────────────────────────────────────────────────────────────
# Phase 2 — Wallet generation
# ─────────────────────────────────────────────────────────────────────────────
Write-Step "Phase 2 — Wallets"

$NeedWallets = $true
if (Test-Path ".env") {
    $envContent = Get-Content ".env" -Raw
    if ($envContent -match "DEPLOYER_PRIVATE_KEY=0x[a-fA-F0-9]{64}") {
        Write-OK ".env already has DEPLOYER_PRIVATE_KEY — skipping wallet generation"
        $NeedWallets = $false
    }
}

if ($NeedWallets) {
    Write-Info "Generating 6 fresh wallets (deployer, admin, treasury, ecosystem, team, liquidity)..."
    npx hardhat run scripts/generate-wallets.ts --network hardhat
    if ($LASTEXITCODE -ne 0) { Write-Fail "Wallet generation failed"; exit 1 }
}

# Load the .env into current session
if (Test-Path ".env") {
    Get-Content ".env" | ForEach-Object {
        if ($_ -match "^\s*([^#=]+)=(.*)$") {
            [System.Environment]::SetEnvironmentVariable($matches[1].Trim(), $matches[2].Trim(), "Process")
        }
    }
    Write-OK ".env loaded into session"
}

# Show deployer address
$deployerKey = $env:DEPLOYER_PRIVATE_KEY
if (-not $deployerKey) { Write-Fail "DEPLOYER_PRIVATE_KEY not set after .env load"; exit 1 }

# Derive address using node inline
$deployerAddr = node -e @"
const { ethers } = require('ethers');
const w = new ethers.Wallet('$deployerKey');
process.stdout.write(w.address);
"@
Write-OK "Deployer address: $deployerAddr"

# ─────────────────────────────────────────────────────────────────────────────
# Phase 3 — Compile
# ─────────────────────────────────────────────────────────────────────────────
Write-Step "Phase 3 — Compile contracts"

npx hardhat compile
if ($LASTEXITCODE -ne 0) { Write-Fail "Compilation failed"; exit 1 }
Write-OK "All contracts compiled"

if ($VerifyOnly) {
    Write-Step "Verify-only mode"
    $deploymentFile = Get-ChildItem "deployments\137.json" -ErrorAction SilentlyContinue
    if (-not $deploymentFile) { Write-Fail "deployments\137.json not found"; exit 1 }
    $dep = Get-Content $deploymentFile | ConvertFrom-Json
    Write-Info "Verifying $($dep.address) on Polygonscan..."
    npx hardhat verify --network polygon $dep.address $env:GENESIS_ADMIN $env:GENESIS_TREASURY $env:GENESIS_ECOSYSTEM $env:GENESIS_TEAM $env:GENESIS_LIQUIDITY
    Write-OK "Verification submitted"
    exit 0
}

# ─────────────────────────────────────────────────────────────────────────────
# Phase 4 — Local Hardhat test deploy
# ─────────────────────────────────────────────────────────────────────────────
Write-Step "Phase 4 — Local test deploy (Hardhat network)"

npx hardhat run scripts/deploy.ts --network hardhat
if ($LASTEXITCODE -ne 0) { Write-Fail "Local test deploy failed"; exit 1 }
Write-OK "Local deploy succeeded — contract logic confirmed"

# ─────────────────────────────────────────────────────────────────────────────
# Phase 5 — POL balance check
# ─────────────────────────────────────────────────────────────────────────────
Write-Step "Phase 5 — POL balance check"

$balanceScript = @"
const { ethers } = require('ethers');
async function main() {
  const rpc = process.env.POLYGON_RPC_URL;
  const provider = new ethers.JsonRpcProvider(rpc);
  const bal = await provider.getBalance('$deployerAddr');
  const pol = parseFloat(ethers.formatEther(bal));
  console.log(JSON.stringify({ pol, wei: bal.toString() }));
}
main().catch(e => { console.error(e.message); process.exit(1); });
"@

$env:POLYGON_RPC_URL = $env:POLYGON_RPC_URL   # ensure it's set

$balResult = node -e $balanceScript 2>&1
try {
    $balObj = $balResult | ConvertFrom-Json
    $pol = $balObj.pol
} catch {
    $pol = 0
}

if ($pol -lt 0.5) {
    Write-Host ""
    Write-Warn "Deployer wallet has $pol POL. Need at least 0.5 POL for gas."
    Write-Host ""
    Write-Host "  ┌─────────────────────────────────────────────────────┐" -ForegroundColor Yellow
    Write-Host "  │  FUND THIS ADDRESS BEFORE MAINNET DEPLOY             │" -ForegroundColor Yellow
    Write-Host "  │                                                       │" -ForegroundColor Yellow
    Write-Host "  │  $deployerAddr  │" -ForegroundColor Yellow
    Write-Host "  │                                                       │" -ForegroundColor Yellow
    Write-Host "  │  Send 1+ POL on Polygon mainnet (chain ID 137)       │" -ForegroundColor Yellow
    Write-Host "  │                                                       │" -ForegroundColor Yellow
    Write-Host "  │  Sources:                                             │" -ForegroundColor Yellow
    Write-Host "  │    Coinbase / Binance / Kraken → withdraw POL/MATIC  │" -ForegroundColor Yellow
    Write-Host "  │    Bridge: https://portal.polygon.technology/bridge   │" -ForegroundColor Yellow
    Write-Host "  └─────────────────────────────────────────────────────┘" -ForegroundColor Yellow
    Write-Host ""

    if (-not $DeployMainnet) {
        Write-OK "Local deploy complete. Run '.\setup.ps1 --mainnet' after funding."
        exit 0
    }

    # Wait mode — poll for balance
    Write-Info "Waiting for POL deposit (checking every 30s, Ctrl+C to cancel)..."
    $attempts = 0
    while ($pol -lt 0.5 -and $attempts -lt 60) {
        Start-Sleep -Seconds 30
        $attempts++
        $balResult = node -e $balanceScript 2>&1
        try { $pol = ($balResult | ConvertFrom-Json).pol } catch { $pol = 0 }
        Write-Info "[$attempts/60] Balance: $pol POL"
    }
    if ($pol -lt 0.5) {
        Write-Fail "Timed out waiting for POL. Re-run .\setup.ps1 --mainnet after funding."
        exit 1
    }
}

Write-OK "Deployer balance: $pol POL — sufficient for gas"

# ─────────────────────────────────────────────────────────────────────────────
# Phase 6 — Mainnet deploy
# ─────────────────────────────────────────────────────────────────────────────
Write-Step "Phase 6 — Polygon mainnet deploy"

if (-not $DeployMainnet) {
    Write-Warn "Mainnet deploy skipped (add --mainnet flag to proceed)"
    Write-Info ""
    Write-Info "When ready:"
    Write-Info "  .\setup.ps1 --mainnet"
    Write-OK "Setup complete. Deployer funded and local deploy verified."
    exit 0
}

Write-Warn "About to deploy to Polygon MAINNET. This spends real POL."
$confirm = Read-Host "  Type YES to continue"
if ($confirm -ne "YES") {
    Write-Info "Cancelled."
    exit 0
}

npx hardhat run scripts/deploy.ts --network polygon
if ($LASTEXITCODE -ne 0) { Write-Fail "Mainnet deploy failed"; exit 1 }

# ─────────────────────────────────────────────────────────────────────────────
# Phase 7 — Polygonscan verification
# ─────────────────────────────────────────────────────────────────────────────
Write-Step "Phase 7 — Polygonscan verification"

$depFile = "deployments\137.json"
if (Test-Path $depFile) {
    $dep = Get-Content $depFile | ConvertFrom-Json
    Write-Info "Waiting 30s for Polygonscan to index the contract..."
    Start-Sleep -Seconds 30

    npx hardhat verify --network polygon `
        $dep.address `
        $env:GENESIS_ADMIN `
        $env:GENESIS_TREASURY `
        $env:GENESIS_ECOSYSTEM `
        $env:GENESIS_TEAM `
        $env:GENESIS_LIQUIDITY

    if ($LASTEXITCODE -eq 0) {
        Write-OK "Contract verified on Polygonscan"
        Write-Info "View: https://polygonscan.com/address/$($dep.address)"
    } else {
        Write-Warn "Verification failed (can retry: .\setup.ps1 --verify-only)"
    }
} else {
    Write-Warn "deployments\137.json not found — verification skipped"
}

Write-Host ""
Write-Host "  ╔══════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "  ║         Genesis World Token — DEPLOYED               ║" -ForegroundColor Green
Write-Host "  ╚══════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Info "Contract: $($dep.address)"
Write-Info "Network:  Polygon Mainnet (chain 137)"
Write-Info ""
Write-Info "Next steps:"
Write-Info "  1. Add GENESIS_TOKEN_ADDRESS=$($dep.address) to Genesis/.env"
Write-Info "  2. Transfer admin role to cold hardware wallet"
Write-Info "  3. Fund ecosystem wallet with initial grant GENESIS"
Write-Info "  4. Set up x402 USDC payment lane (see Genesis/.env.example)"
Write-Host ""
