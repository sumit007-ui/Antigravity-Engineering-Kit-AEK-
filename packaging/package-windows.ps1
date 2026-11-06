# Antigravity Engineering Kit (AEK) - Windows Packaging Script
[CmdletBinding()]
param(
    [string]$Version = "",
    [string]$TargetTriple = "x86_64-pc-windows-msvc",
    [string]$BinaryPath = "target/release/aek.exe",
    [string]$OutputDir = "dist"
)

$ErrorActionPreference = "Stop"

if (-not $Version) {
    if (Test-Path "Cargo.toml") {
        $cargoContent = Get-Content "Cargo.toml" -Raw
        if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
            $Version = $matches[1]
        }
    }
    if (-not $Version) {
        $Version = "0.1.0"
    }
}

$Arch = if ($TargetTriple -match "x86_64") { "x86_64" } elseif ($TargetTriple -match "aarch64") { "aarch64" } else { "x86_64" }
$ArtifactName = "aek-v$Version-windows-$Arch.zip"

Write-Host "Packaging AEK for Windows..."
Write-Host "  Version:       $Version"
Write-Host "  Target Triple: $TargetTriple"
Write-Host "  Binary:        $BinaryPath"
Write-Host "  Output Dir:    $OutputDir"

if (-not (Test-Path $BinaryPath)) {
    Write-Error "Binary '$BinaryPath' not found. Build release binary first: cargo build --release"
    exit 1
}

if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

$StagingDir = Join-Path $OutputDir "staging-windows-$Arch"
if (Test-Path $StagingDir) {
    Remove-Item -Path $StagingDir -Recurse -Force
}
New-Item -ItemType Directory -Path $StagingDir -Force | Out-Null

# Copy binary and documentation
Copy-Item -Path $BinaryPath -Destination (Join-Path $StagingDir "aek.exe")
if (Test-Path "README.md") { Copy-Item "README.md" (Join-Path $StagingDir "README.md") }
if (Test-Path "LICENSE-APACHE") { Copy-Item "LICENSE-APACHE" (Join-Path $StagingDir "LICENSE-APACHE") }
if (Test-Path "LICENSE-MIT") { Copy-Item "LICENSE-MIT" (Join-Path $StagingDir "LICENSE-MIT") }

$ZipPath = Join-Path $OutputDir $ArtifactName
if (Test-Path $ZipPath) {
    Remove-Item $ZipPath -Force
}

Compress-Archive -Path "$StagingDir\*" -DestinationPath $ZipPath -Force

# Compute SHA-256
$HashInfo = Get-FileHash -Path $ZipPath -Algorithm SHA256
$Sha256 = $HashInfo.Hash.ToLower()
$ShaPath = "$ZipPath.sha256"
"$Sha256 *$ArtifactName" | Out-File -FilePath $ShaPath -Encoding ascii

Remove-Item -Path $StagingDir -Recurse -Force

$Size = (Get-Item $ZipPath).Length
Write-Host "`nSuccessfully packaged: $ZipPath"
Write-Host "  Size:   $Size bytes"
Write-Host "  SHA256: $Sha256"
Write-Host "  Hash File: $ShaPath"
exit 0
