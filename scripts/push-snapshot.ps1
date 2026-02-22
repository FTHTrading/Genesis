# push-snapshot.ps1
# Polls the live server and saves status-snapshot.json to docs/ for GitHub Pages.
# Also watches docs/system-state.json for changes (raw world state).
# Commits and pushes both files when the status changes.
#
# Usage: Run in a separate terminal while the server is running.
#   cd C:\Users\Kevan\genesis-protocol
#   powershell -ExecutionPolicy Bypass -File scripts\push-snapshot.ps1

param(
    [int]$IntervalSeconds = 60,
    [string]$StatusUrl = 'http://localhost:3000/status'
)

$RepoRoot      = Split-Path -Parent $PSScriptRoot
$SnapFile      = Join-Path $RepoRoot "docs\system-state.json"
$StatusOut     = Join-Path $RepoRoot "docs\status-snapshot.json"
$LastHash      = ""
$LastStatusEp  = -1

Set-Location $RepoRoot

Write-Host "Genesis Protocol - snapshot pusher"
Write-Host "Status URL : $StatusUrl"
Write-Host "Status out : $StatusOut"
Write-Host "Interval   : every $IntervalSeconds seconds"
Write-Host "Press Ctrl+C to stop."
Write-Host ""

while ($true) {
    Start-Sleep -Seconds $IntervalSeconds

    $Epoch = $null; $Pop = $null
    try {
        $StatusJson = (Invoke-WebRequest -Uri $StatusUrl -UseBasicParsing -TimeoutSec 5).Content
        $status     = $StatusJson | ConvertFrom-Json
        $Epoch      = $status.epoch
        $Pop        = $status.population
        Set-Content -Path $StatusOut -Value $StatusJson -Encoding UTF8
        Write-Host "[$(Get-Date -Format 'HH:mm:ss')] Status fetched: ep=$Epoch pop=$Pop" -ForegroundColor DarkGray
    } catch {
        Write-Host "[$(Get-Date -Format 'HH:mm:ss')] Could not reach $StatusUrl - server down?" -ForegroundColor Yellow
    }

    $WorldChanged = $false
    if (Test-Path $SnapFile) {
        $Hash = (Get-FileHash $SnapFile -Algorithm MD5).Hash
        if ($Hash -ne $LastHash) { $LastHash = $Hash; $WorldChanged = $true }
    }

    $StatusChanged = ($Epoch -ne $null) -and ($Epoch -ne $LastStatusEp)
    if (-not $StatusChanged -and -not $WorldChanged) {
        Write-Host "[$(Get-Date -Format 'HH:mm:ss')] No change." -ForegroundColor DarkGray
        continue
    }

    if ($StatusChanged) { $LastStatusEp = $Epoch }

    $Msg = if ($Epoch -ne $null) { "chore: snapshot epoch $Epoch pop=$Pop [skip ci]" } else { "chore: update snapshot [skip ci]" }
    Write-Host "[$(Get-Date -Format 'HH:mm:ss')] Committing: $Msg" -ForegroundColor Cyan

    if (Test-Path $StatusOut)  { git add docs/status-snapshot.json 2>&1 | Out-Null }
    if (Test-Path $SnapFile)   { git add docs/system-state.json    2>&1 | Out-Null }

    $CommitOut = git commit -m $Msg 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  commit skipped (nothing staged)" -ForegroundColor DarkGray
        continue
    }

    $PushOut = git push 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  pushed OK" -ForegroundColor Green
    } else {
        Write-Host "  push failed: $PushOut" -ForegroundColor Red
    }
}
