# Genesis Protocol — One-Shot VPS Ignition (Windows → Hetzner)
#
# Usage:
#   .\scripts\ignite.ps1 -IP 203.0.113.10 -Domain genesis.example.com
#
# This script:
#   1. Creates a tarball of the committed source (no target/, no .env)
#   2. SCPs it to the VPS
#   3. SSHes in and runs the full bootstrap: user creation, unpack, deploy.sh
#   4. Tails the service logs so you see first epoch ticks
#
# Prerequisites:
#   - Hetzner CX21 provisioned with Ubuntu 22.04
#   - Domain A record pointing to VPS IP
#   - OpenSSH client available (Windows 10+ built-in)
#   - You can SSH as root@IP (password or key)

param(
    [Parameter(Mandatory=$true)]
    [string]$IP,

    [Parameter(Mandatory=$true)]
    [string]$Domain
)

$ErrorActionPreference = "Stop"

function Write-Step($msg) {
    $ts = Get-Date -Format "HH:mm:ss"
    Write-Host "`n[$ts] $msg" -ForegroundColor Green
}

function Write-Fail($msg) {
    Write-Host "[FAIL] $msg" -ForegroundColor Red
    exit 1
}

# ─── Pre-flight ──────────────────────────────────────────
Push-Location $PSScriptRoot\..

Write-Step "Pre-flight: verifying local repo..."

if (-not (Test-Path ".git")) {
    Write-Fail "Not in a git repository. Run from genesis-protocol root."
}

$status = git status --porcelain
if ($status) {
    Write-Host "Warning: uncommitted changes detected. Tarball uses last commit only." -ForegroundColor Yellow
}

# ─── Step 1: Create tarball ──────────────────────────────
Write-Step "Creating source tarball from HEAD..."
$tarball = "genesis-deploy.tar"
git archive --format=tar --output=$tarball HEAD
if (-not (Test-Path $tarball)) {
    Write-Fail "Failed to create tarball"
}
$size = [math]::Round((Get-Item $tarball).Length / 1KB, 1)
Write-Host "  Tarball created: ${size} KB"

# ─── Step 2: SCP tarball to VPS ──────────────────────────
Write-Step "Uploading tarball to root@${IP}..."
scp -o StrictHostKeyChecking=accept-new $tarball "root@${IP}:/root/genesis-deploy.tar"
if ($LASTEXITCODE -ne 0) { Write-Fail "SCP failed" }
Write-Host "  Upload complete."

# ─── Step 3: Create bootstrap script ────────────────────
# We generate a single bootstrap script that runs entirely on the VPS.
# This avoids multiple SSH round-trips and handles the full provisioning.

$bootstrap = @"
#!/usr/bin/env bash
set -euo pipefail

DOMAIN="$Domain"

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'
step() { echo -e "\n\${GREEN}[\$(date +%H:%M:%S)]\${NC} \$1"; }
fail() { echo -e "\${RED}[FAIL]\${NC} \$1"; exit 1; }

# ── System update ────────────────────────────────────────
step "Updating system packages..."
apt update -qq && apt upgrade -y -qq
apt install -y -qq build-essential pkg-config libssl-dev curl git adduser

# ── Create genesis user ─────────────────────────────────
if id "genesis" &>/dev/null; then
    step "User 'genesis' already exists"
else
    step "Creating user 'genesis'..."
    adduser --disabled-password --gecos "Genesis Protocol" genesis
    usermod -aG sudo genesis
    # Allow passwordless sudo for deployment
    echo "genesis ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/genesis
    chmod 440 /etc/sudoers.d/genesis
fi

# ── Unpack source ────────────────────────────────────────
step "Unpacking source to /home/genesis/genesis-protocol..."
mkdir -p /home/genesis/genesis-protocol
tar -xf /root/genesis-deploy.tar -C /home/genesis/genesis-protocol
chown -R genesis:genesis /home/genesis/genesis-protocol

# ── Run deploy.sh as genesis user ────────────────────────
step "Handing off to deploy.sh as genesis user..."
su - genesis -c "cd ~/genesis-protocol && bash scripts/deploy.sh \$DOMAIN"

# ── Post-deploy verification ────────────────────────────
step "Running post-deploy checks..."

sleep 3

# Service status
if systemctl is-active --quiet genesis; then
    step "Genesis service: RUNNING"
else
    fail "Genesis service not running. Check: journalctl -u genesis -n 50"
fi

# Firewall status
UFW_STATUS=\$(ufw status | head -1)
step "Firewall: \$UFW_STATUS"

# Caddy status
if systemctl is-active --quiet caddy; then
    step "Caddy: RUNNING"
else
    echo "  Warning: Caddy not active yet"
fi

# Local endpoint
LOCAL_STATUS=\$(curl -s -o /dev/null -w "%{http_code}" --max-time 10 http://localhost:3000/status 2>/dev/null || echo "000")
step "Local /status: HTTP \$LOCAL_STATUS"

# Public endpoint (TLS may take a moment)
sleep 5
PUBLIC_STATUS=\$(curl -s -o /dev/null -w "%{http_code}" --max-time 15 "https://\$DOMAIN/status" 2>/dev/null || echo "000")
if [ "\$PUBLIC_STATUS" = "200" ]; then
    step "Public https://\$DOMAIN/status: 200 OK — TLS ACTIVE"
else
    step "Public HTTPS: \$PUBLIC_STATUS — TLS may still be provisioning (check in 60s)"
fi

# ── Summary ──────────────────────────────────────────────
echo ""
echo "══════════════════════════════════════════════════════════"
echo "  GENESIS PROTOCOL — IGNITION COMPLETE"
echo "══════════════════════════════════════════════════════════"
echo ""
echo "  Domain:     https://\$DOMAIN"
echo "  Service:    systemctl status genesis"
echo "  Logs:       journalctl -u genesis -f"
echo "  Validate:   GENESIS_DOMAIN=\$DOMAIN GENESIS_PROTO=https bash scripts/validate.sh hour0"
echo ""
echo "  Adapter:    DISABLED (edit .env to enable at Hour 1)"
echo ""
echo "  Next steps:"
echo "    1. Watch logs:  journalctl -u genesis -f"
echo "    2. Hour 0 validation:  GENESIS_DOMAIN=\$DOMAIN GENESIS_PROTO=https bash scripts/validate.sh hour0"
echo "    3. Hour 1 — enable adapter: edit .env, systemctl restart genesis"
echo ""
"@

Write-Step "Uploading bootstrap script to VPS..."
$bootstrapPath = [System.IO.Path]::GetTempFileName()
# Write with Unix line endings
$bootstrap -replace "`r`n", "`n" | Set-Content -Path $bootstrapPath -NoNewline -Encoding utf8
scp -o StrictHostKeyChecking=accept-new $bootstrapPath "root@${IP}:/root/genesis-bootstrap.sh"
if ($LASTEXITCODE -ne 0) { Write-Fail "Failed to upload bootstrap script" }
Remove-Item $bootstrapPath -ErrorAction SilentlyContinue

# ─── Step 4: Execute bootstrap on VPS ────────────────────
Write-Step "Executing bootstrap on VPS (this takes 5-10 minutes)..."
Write-Host "  Building Rust from source on first run — be patient." -ForegroundColor Yellow
Write-Host ""
ssh -o StrictHostKeyChecking=accept-new "root@${IP}" "chmod +x /root/genesis-bootstrap.sh && bash /root/genesis-bootstrap.sh"
if ($LASTEXITCODE -ne 0) { Write-Fail "Bootstrap failed on VPS" }

# ─── Step 5: Tail logs ──────────────────────────────────
Write-Step "Tailing Genesis logs (Ctrl+C to stop)..."
Write-Host ""
ssh "root@${IP}" "journalctl -u genesis -f -n 30"

# ─── Cleanup ─────────────────────────────────────────────
Pop-Location
Remove-Item $tarball -ErrorAction SilentlyContinue
