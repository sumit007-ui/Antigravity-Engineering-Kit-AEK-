# Antigravity Engineering Kit (AEK) - Automated Windows Installer Test Harness
# Tests installation and uninstallation workflows in an isolated temporary sandbox.
[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

Write-Host "Running automated Windows installer test harness..." -ForegroundColor Cyan

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$BinPath = Join-Path $RepoRoot "target\release\aek.exe"

if (-not (Test-Path $BinPath)) {
    Write-Host "Building release binary for test..."
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to build release binary."
    }
}

$SandboxDir = Join-Path ([System.IO.Path]::GetTempPath()) ("aek-test-sandbox-" + [System.Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $SandboxDir -Force | Out-Null

try {
    $DistDir = Join-Path $SandboxDir "dist"
    $InstallBinDir = Join-Path $SandboxDir "installed\bin"

    # 1. Package test artifact
    Write-Host "[Step 1/5] Packaging local test artifact..."
    $global:LASTEXITCODE = 0
    & (Join-Path $RepoRoot "packaging\package-windows.ps1") -BinaryPath $BinPath -OutputDir $DistDir
    if (-not $? -or $global:LASTEXITCODE -ne 0) {
        throw "Packaging failed."
    }

    $zipFile = Get-ChildItem -Path $DistDir -Filter "*.zip" | Select-Object -First 1
    $shaFile = Get-ChildItem -Path $DistDir -Filter "*.zip.sha256" | Select-Object -First 1
    $expectedHash = (Get-Content $shaFile.FullName).Trim().Split(" ")[0]

    # 2. Test Dry Run
    Write-Host "[Step 2/5] Testing installer dry-run..."
    $global:LASTEXITCODE = 0
    & (Join-Path $PSScriptRoot "install.ps1") -LocalArtifact $zipFile.FullName -InstallDir $InstallBinDir -DryRun
    if (Test-Path $InstallBinDir) {
        throw "Dry run should not create installation directory!"
    }

    # 3. Test Installation
    Write-Host "[Step 3/5] Testing actual installation..."
    $global:LASTEXITCODE = 0
    & (Join-Path $PSScriptRoot "install.ps1") -LocalArtifact $zipFile.FullName -InstallDir $InstallBinDir -ExpectedSha256 $expectedHash
    if (-not $? -or $global:LASTEXITCODE -ne 0) {
        throw "Installation failed."
    }

    $installedExe = Join-Path $InstallBinDir "aek.exe"
    if (-not (Test-Path $installedExe)) {
        throw "Installed executable not found at '$installedExe'!"
    }

    $verOutput = & "$installedExe" --version 2>&1
    Write-Host "  Installed executable output: $verOutput"
    if (-not ($verOutput -match "aek")) {
        throw "Executable output does not match expected version!"
    }

    # 4. Test Uninstaller Dry Run
    Write-Host "[Step 4/5] Testing uninstaller dry-run..."
    $global:LASTEXITCODE = 0
    & (Join-Path $PSScriptRoot "uninstall.ps1") -InstallDir $InstallBinDir -DryRun
    if (-not (Test-Path $installedExe)) {
        throw "Uninstaller dry run should not delete files!"
    }

    # 5. Test Uninstallation
    Write-Host "[Step 5/5] Testing uninstallation..."
    $global:LASTEXITCODE = 0
    & (Join-Path $PSScriptRoot "uninstall.ps1") -InstallDir $InstallBinDir -Force
    if (Test-Path $installedExe) {
        throw "Uninstallation failed: binary still exists!"
    }

    Write-Host "`n[SUCCESS] Windows installer automated test suite passed!" -ForegroundColor Green
    exit 0
} catch {
    Write-Host "`n[FAIL] Test harness encountered error: $_" -ForegroundColor Red
    exit 1
} finally {
    if (Test-Path $SandboxDir) {
        Remove-Item -Path $SandboxDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}
