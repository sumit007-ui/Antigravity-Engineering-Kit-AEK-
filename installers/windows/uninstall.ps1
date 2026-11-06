# Antigravity Engineering Kit (AEK) - Windows Uninstaller
[CmdletBinding()]
param(
    [string]$InstallDir = "",
    [switch]$Purge,
    [switch]$Force,
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  Antigravity Engineering Kit (AEK) - Windows Uninstaller   " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

if (-not $InstallDir) {
    $baseDir = if ($env:LOCALAPPDATA) {
        Join-Path $env:LOCALAPPDATA "Programs\aek"
    } else {
        Join-Path $env:USERPROFILE "AppData\Local\Programs\aek"
    }
    $InstallDir = Join-Path $baseDir "bin"
}

$InstallRoot = Split-Path -Path $InstallDir -Parent
$TargetExe = Join-Path $InstallDir "aek.exe"
$ManifestPath = Join-Path $InstallRoot "installation.json"
$ConfigDir = if ($env:APPDATA) {
    Join-Path $env:APPDATA "aek"
} else {
    Join-Path $env:USERPROFILE ".aek"
}

Write-Host "Detected AEK installation root: $InstallRoot"

if ($DryRun) {
    Write-Host "`n[DRY RUN] Uninstallation plan:" -ForegroundColor Magenta
    Write-Host "  Files scheduled for deletion:"
    if (Test-Path $TargetExe) { Write-Host "    - $TargetExe" }
    if (Test-Path $ManifestPath) { Write-Host "    - $ManifestPath" }
    if ($Purge) {
        Write-Host "  PURGE MODE ACTIVE - Directories scheduled for deletion:" -ForegroundColor Red
        if (Test-Path $ConfigDir) { Write-Host "    - $ConfigDir (User configuration)" }
        if (Test-Path $InstallRoot) { Write-Host "    - $InstallRoot (Installation root)" }
    } else {
        Write-Host "  Preserving user configuration directory: $ConfigDir" -ForegroundColor Green
    }
    Write-Host "`n[DRY RUN] No files deleted." -ForegroundColor Green
    exit 0
}

# Confirmation for purge
if ($Purge -and -not $Force) {
    $confirmation = Read-Host "Are you sure you want to PURGE all AEK configurations and data? [y/N]"
    if ($confirmation -ne "y" -and $confirmation -ne "Y") {
        Write-Host "Uninstallation aborted by user."
        exit 0
    }
}

# Delete binary
if (Test-Path $TargetExe) {
    Remove-Item -Path $TargetExe -Force
    Write-Host "  Removed executable: $TargetExe"
}

# Delete manifest
if (Test-Path $ManifestPath) {
    Remove-Item -Path $ManifestPath -Force
    Write-Host "  Removed manifest: $ManifestPath"
}

# If purge mode, remove config directory and install root
if ($Purge) {
    if (Test-Path $ConfigDir) {
        Remove-Item -Path $ConfigDir -Recurse -Force
        Write-Host "  Purged configuration directory: $ConfigDir"
    }
    if (Test-Path $InstallRoot) {
        Remove-Item -Path $InstallRoot -Recurse -Force
        Write-Host "  Purged installation root: $InstallRoot"
    }
} else {
    Write-Host "`nPreserved user configuration in: $ConfigDir" -ForegroundColor Green
    Write-Host "To remove configuration, run: .\uninstall.ps1 -Purge"
}

Write-Host "`nAEK uninstallation completed successfully." -ForegroundColor Green
exit 0
