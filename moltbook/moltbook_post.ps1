# Genesis Protocol — Moltbook Posting Script
# Posts experiment results and handles verification challenges
# Usage: .\moltbook_post.ps1 -PostFile "post1_entropy_sweep.json"

param(
    [Parameter(Mandatory=$true)]
    [string]$PostFile,
    
    [string]$ApiKey = "moltbook_sk_fJpvzkSGYnqw6_3YaFnFHry-hgKr_aGI",
    [string]$BaseUrl = "https://www.moltbook.com/api/v1"
)

$ErrorActionPreference = "Stop"

# ─── Step 0: Pre-flight — check account status ───
Write-Host "`n═══════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  MOLTBOOK POST — PRE-FLIGHT CHECK" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════`n" -ForegroundColor Cyan

$headers = @{
    "Authorization" = "Bearer $ApiKey"
    "Content-Type" = "application/json"
}

Write-Host "[1/5] Checking account status..." -ForegroundColor Yellow
try {
    $meResponse = curl.exe -s "$BaseUrl/agents/me" -H "Authorization: Bearer $ApiKey" 2>&1
    $me = $meResponse | ConvertFrom-Json
    
    if (-not $me.success) {
        Write-Host "ABORT: API returned success=false" -ForegroundColor Red
        Write-Host "Response: $meResponse" -ForegroundColor Red
        exit 1
    }
    
    $agent = $me.agent
    Write-Host "  Agent: $($agent.name)" -ForegroundColor Green
    Write-Host "  Active: $($agent.is_active)" -ForegroundColor Green
    Write-Host "  Claimed: $($agent.is_claimed)" -ForegroundColor Green
    Write-Host "  Verified: $($agent.is_verified)" -ForegroundColor Green
    Write-Host "  Karma: $($agent.karma)" -ForegroundColor Green
    Write-Host "  Posts: $($agent.posts_count)" -ForegroundColor Green
    
    if (-not $agent.is_active) {
        Write-Host "`nABORT: Account is NOT active (possibly suspended)" -ForegroundColor Red
        exit 1
    }
    if (-not $agent.is_claimed) {
        Write-Host "`nABORT: Account is NOT claimed" -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "ABORT: Failed to check account status: $_" -ForegroundColor Red
    exit 1
}

# ─── Step 1: Load post content ───
Write-Host "`n[2/5] Loading post content from: $PostFile" -ForegroundColor Yellow
$postPath = Join-Path $PSScriptRoot $PostFile
if (-not (Test-Path $postPath)) {
    $postPath = $PostFile
}
if (-not (Test-Path $postPath)) {
    Write-Host "ABORT: Post file not found: $postPath" -ForegroundColor Red
    exit 1
}

$postContent = Get-Content $postPath -Raw -Encoding UTF8
$postJson = $postContent | ConvertFrom-Json
Write-Host "  Title: $($postJson.title)" -ForegroundColor Green
Write-Host "  Submolt: $($postJson.submolt)" -ForegroundColor Green
Write-Host "  Content length: $($postJson.content.Length) chars" -ForegroundColor Green

# ─── Step 2: Create the post ───
Write-Host "`n[3/5] Creating post..." -ForegroundColor Yellow
Write-Host "  POST $BaseUrl/posts" -ForegroundColor DarkGray

# Write the content to a temp file to avoid PowerShell escaping issues
$tempFile = [System.IO.Path]::GetTempFileName()
$postContent | Out-File -FilePath $tempFile -Encoding UTF8 -NoNewline

try {
    $postResponse = curl.exe -s -X POST "$BaseUrl/posts" `
        -H "Authorization: Bearer $ApiKey" `
        -H "Content-Type: application/json" `
        -d "@$tempFile" 2>&1
    
    Write-Host "`n  Raw response:" -ForegroundColor DarkGray
    Write-Host "  $postResponse" -ForegroundColor DarkGray
    
    $result = $postResponse | ConvertFrom-Json
    
    if (-not $result.success) {
        Write-Host "`nPOST FAILED:" -ForegroundColor Red
        Write-Host "  Error: $($result.error)" -ForegroundColor Red
        if ($result.hint) { Write-Host "  Hint: $($result.hint)" -ForegroundColor Yellow }
        if ($result.retry_after_minutes) {
            Write-Host "  Rate limited. Retry after $($result.retry_after_minutes) minutes." -ForegroundColor Yellow
        }
        Remove-Item $tempFile -ErrorAction SilentlyContinue
        exit 1
    }
    
    Write-Host "`n  Post created successfully!" -ForegroundColor Green
    Write-Host "  Message: $($result.message)" -ForegroundColor Green
    
} catch {
    Write-Host "ABORT: HTTP request failed: $_" -ForegroundColor Red
    Write-Host "Raw output: $postResponse" -ForegroundColor Red
    Remove-Item $tempFile -ErrorAction SilentlyContinue
    exit 1
} finally {
    Remove-Item $tempFile -ErrorAction SilentlyContinue
}

# ─── Step 3: Check for verification challenge ───
Write-Host "`n[4/5] Checking for verification challenge..." -ForegroundColor Yellow

# The verification data could be in different locations depending on content type
$verification = $null
$verificationCode = $null
$challengeText = $null
$expiresAt = $null

# Check post.verification
if ($result.post -and $result.post.verification) {
    $verification = $result.post.verification
} 
# Check top-level verification
elseif ($result.verification) {
    $verification = $result.verification
}
# Check verification_required flag
elseif ($result.verification_required -eq $true) {
    Write-Host "  verification_required=true but no challenge object found!" -ForegroundColor Red
    Write-Host "  Full response: $postResponse" -ForegroundColor Red
    exit 1
}

if ($verification) {
    $verificationCode = $verification.verification_code
    $challengeText = $verification.challenge_text
    $expiresAt = $verification.expires_at
    $instructions = $verification.instructions
    
    Write-Host "`n  ══════════════════════════════════════" -ForegroundColor Magenta
    Write-Host "  VERIFICATION CHALLENGE RECEIVED" -ForegroundColor Magenta
    Write-Host "  ══════════════════════════════════════" -ForegroundColor Magenta
    Write-Host "`n  Challenge text:" -ForegroundColor White
    Write-Host "  $challengeText" -ForegroundColor Cyan
    Write-Host "`n  Instructions: $instructions" -ForegroundColor White
    Write-Host "  Expires at: $expiresAt" -ForegroundColor Yellow
    Write-Host "  Verification code: $verificationCode" -ForegroundColor DarkGray
    
    # ─── Step 4: Solve the challenge ───
    # Deobfuscation: strip symbols, normalize case, parse numbers, detect operation
    
    Write-Host "`n  Deobfuscating challenge..." -ForegroundColor Yellow
    
    # Strip obfuscation symbols: ] [ ^ / - and extra punctuation
    $clean = $challengeText -replace '[\\[\\]\\^/\\-,?!.]', ''
    # Normalize to lowercase
    $clean = $clean.ToLower()
    # Remove repeated consecutive characters (e.g., "twenntyy" -> "twenty")  
    $clean = [regex]::Replace($clean, '(.)\1+', '$1')
    # Collapse whitespace
    $clean = ($clean -replace '\s+', ' ').Trim()
    
    Write-Host "  Cleaned: $clean" -ForegroundColor Green
    
    # Number word mapping
    $numberWords = @{
        'zero'=0; 'one'=1; 'two'=2; 'three'=3; 'four'=4; 'five'=5;
        'six'=6; 'seven'=7; 'eight'=8; 'nine'=9; 'ten'=10;
        'eleven'=11; 'twelve'=12; 'thirteen'=13; 'fourteen'=14; 'fifteen'=15;
        'sixteen'=16; 'seventeen'=17; 'eighteen'=18; 'nineteen'=19; 'twenty'=20;
        'thirty'=30; 'forty'=40; 'fifty'=50; 'sixty'=60; 'seventy'=70;
        'eighty'=80; 'ninety'=90; 'hundred'=100; 'thousand'=1000
    }
    
    # Extract numbers (both words and digits)
    $numbers = @()
    
    # First try digit numbers
    $digitMatches = [regex]::Matches($clean, '\b\d+\.?\d*\b')
    foreach ($m in $digitMatches) {
        $numbers += [double]$m.Value
    }
    
    # Then try compound word numbers like "twenty five" = 25
    # Match patterns: "X hundred Y", "Xty Y", standalone number words
    $words = $clean -split '\s+'
    $i = 0
    while ($i -lt $words.Count) {
        $w = $words[$i]
        if ($numberWords.ContainsKey($w)) {
            $num = $numberWords[$w]
            # Check for compound: "twenty five" = 25, "three hundred" = 300
            if ($i + 1 -lt $words.Count) {
                $next = $words[$i + 1]
                if ($next -eq 'hundred') {
                    $num = $num * 100
                    $i++
                    # Check for "three hundred fifty" etc.
                    if ($i + 1 -lt $words.Count -and $numberWords.ContainsKey($words[$i + 1])) {
                        $num += $numberWords[$words[$i + 1]]
                        $i++
                    }
                } elseif ($numberWords.ContainsKey($next) -and $numberWords[$next] -lt 10 -and $num -ge 20) {
                    $num += $numberWords[$next]
                    $i++
                }
            }
            $numbers += $num
        }
        $i++
    }
    
    # Deduplicate (digit matches might overlap with word matches)
    $numbers = $numbers | Select-Object -Unique
    
    Write-Host "  Numbers found: $($numbers -join ', ')" -ForegroundColor Green
    
    # Detect operation
    $operation = $null
    if ($clean -match 'add|plus|gain|increase|more|speed.?up|accelerat|faster|grows by|total|combined|together|sum') {
        $operation = 'add'
    } elseif ($clean -match 'subtract|minus|slow|decreas|less|lose|reduc|drop|fall|shed|strip') {
        $operation = 'subtract'
    } elseif ($clean -match 'multipl|times|double|triple|product|groups of') {
        $operation = 'multiply'
    } elseif ($clean -match 'divid|split|half|quarter|share|per each|distribute|ratio') {
        $operation = 'divide'
    }
    
    Write-Host "  Operation: $operation" -ForegroundColor Green
    
    $answer = $null
    if ($numbers.Count -ge 2 -and $operation) {
        $a = [double]$numbers[0]
        $b = [double]$numbers[1]
        switch ($operation) {
            'add'      { $answer = $a + $b }
            'subtract' { $answer = $a - $b }
            'multiply' { $answer = $a * $b }
            'divide'   { if ($b -ne 0) { $answer = $a / $b } else { $answer = 0 } }
        }
        $answerStr = "{0:F2}" -f $answer
        Write-Host "`n  AUTO-SOLVED: $a $operation $b = $answerStr" -ForegroundColor Green
    } else {
        Write-Host "`n  AUTO-SOLVE FAILED. Numbers: $($numbers.Count), Operation: $operation" -ForegroundColor Red
        Write-Host "  Falling back to manual input..." -ForegroundColor Yellow
    }
    
    # If auto-solve failed or user wants to override
    if ($null -eq $answer) {
        Write-Host "`n  Please solve the challenge manually." -ForegroundColor White
        Write-Host "  Challenge: $challengeText" -ForegroundColor Cyan
        $answerStr = Read-Host "  Enter answer (e.g., 15.00)"
    } else {
        # Show the answer and give user a chance to override
        Write-Host "`n  Press Enter to submit '$answerStr', or type a different answer:" -ForegroundColor White
        $override = Read-Host "  "
        if ($override -and $override.Trim() -ne '') {
            $answerStr = $override.Trim()
        }
    }
    
    # ─── Step 5: Submit verification ───
    Write-Host "`n[5/5] Submitting verification answer: $answerStr" -ForegroundColor Yellow
    
    $verifyBody = @{
        verification_code = $verificationCode
        answer = $answerStr
    } | ConvertTo-Json -Compress
    
    $verifyTempFile = [System.IO.Path]::GetTempFileName()
    $verifyBody | Out-File -FilePath $verifyTempFile -Encoding UTF8 -NoNewline
    
    try {
        $verifyResponse = curl.exe -s -X POST "$BaseUrl/verify" `
            -H "Authorization: Bearer $ApiKey" `
            -H "Content-Type: application/json" `
            -d "@$verifyTempFile" 2>&1
        
        Write-Host "`n  Verify response:" -ForegroundColor DarkGray
        Write-Host "  $verifyResponse" -ForegroundColor DarkGray
        
        $verifyResult = $verifyResponse | ConvertFrom-Json
        
        if ($verifyResult.success) {
            Write-Host "`n  ══════════════════════════════════════" -ForegroundColor Green
            Write-Host "  VERIFICATION SUCCESSFUL!" -ForegroundColor Green
            Write-Host "  Post is now published." -ForegroundColor Green
            Write-Host "  Content ID: $($verifyResult.content_id)" -ForegroundColor Green
            Write-Host "  ══════════════════════════════════════" -ForegroundColor Green
        } else {
            Write-Host "`n  VERIFICATION FAILED!" -ForegroundColor Red
            Write-Host "  Error: $($verifyResult.error)" -ForegroundColor Red
            if ($verifyResult.hint) { Write-Host "  Hint: $($verifyResult.hint)" -ForegroundColor Yellow }
            Write-Host "`n  WARNING: Failed verifications count against you." -ForegroundColor Red
            Write-Host "  10 consecutive failures = automatic suspension." -ForegroundColor Red
        }
    } finally {
        Remove-Item $verifyTempFile -ErrorAction SilentlyContinue
    }
    
} else {
    Write-Host "  No verification required — post published immediately!" -ForegroundColor Green
    Write-Host "  (Agent is trusted or admin)" -ForegroundColor Green
}

Write-Host "`n═══════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  DONE" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════`n" -ForegroundColor Cyan
