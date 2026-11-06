# Antigravity Engineering Kit (AEK) - Release Verification Script (PowerShell)
[CmdletBinding()]
param(
    [string]$DistDir = "dist",
    [string]$ManifestPath = "dist/release-manifest.json"
)

$ErrorActionPreference = "Stop"

Write-Host "Verifying AEK Release Artifacts in '$DistDir'..."
Write-Host "============================================================"

if (-not (Test-Path $DistDir)) {
    Write-Error "Distribution directory '$DistDir' does not exist."
    exit 1
}

$shaFiles = Get-ChildItem -Path $DistDir -Filter "*.sha256"
if ($shaFiles.Count -eq 0) {
    Write-Error "No .sha256 checksum files found in '$DistDir'."
    exit 1
}

$allValid = $true
foreach ($sf in $shaFiles) {
    $content = (Get-Content $sf.FullName).Trim()
    $parts = -split $content
    $expectedHash = $parts[0].ToLower()
    $artifactName = $sf.BaseName

    $artifactPath = Join-Path $DistDir $artifactName
    if (-not (Test-Path $artifactPath)) {
        Write-Host "  [-] Artifact missing for checksum: $artifactName" -ForegroundColor Red
        $allValid = $false
        continue
    }

    $actualHash = (Get-FileHash -Path $artifactPath -Algorithm SHA256).Hash.ToLower()
    if ($actualHash -eq $expectedHash) {
        Write-Host "  [+] $artifactName : SHA256 OK ($expectedHash)" -ForegroundColor Green
    } else {
        Write-Host "  [-] $artifactName : SHA256 MISMATCH!" -ForegroundColor Red
        Write-Host "      Expected: $expectedHash"
        Write-Host "      Actual:   $actualHash"
        $allValid = $false
    }
}

if (Test-Path $ManifestPath) {
    Write-Host "`nValidating Release Manifest: $ManifestPath"
    try {
        $json = Get-Content $ManifestPath -Raw | ConvertFrom-Json
        Write-Host "  [+] Version:         $($json.version)"
        Write-Host "  [+] Release Channel: $($json.release_channel)"
        Write-Host "  [+] Artifact Count:  $($json.artifacts.Count)"

        foreach ($art in $json.artifacts) {
            $artFile = Join-Path $DistDir $art.filename
            if (Test-Path $artFile) {
                $actualSha = (Get-FileHash -Path $artFile -Algorithm SHA256).Hash.ToLower()
                if ($actualSha -eq $art.sha256.ToLower()) {
                    Write-Host "  [+] Manifest match: $($art.filename) ($($art.target_triple))" -ForegroundColor Green
                } else {
                    Write-Host "  [-] Manifest hash mismatch for: $($art.filename)" -ForegroundColor Red
                    $allValid = $false
                }
            } else {
                Write-Host "  [-] Manifest referenced file missing: $($art.filename)" -ForegroundColor Red
                $allValid = $false
            }
        }
    } catch {
        Write-Host "  [-] Failed to parse manifest JSON: $_" -ForegroundColor Red
        $allValid = $false
    }
}

Write-Host "============================================================"
if ($allValid) {
    Write-Host "[SUCCESS] All release artifacts verified." -ForegroundColor Green
    exit 0
} else {
    Write-Error "Release verification failed."
    exit 1
}

