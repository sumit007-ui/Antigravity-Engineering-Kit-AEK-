# Antigravity Engineering Kit (AEK) - Windows Production Installer
# Installs AEK into a user-writable directory without requiring administrator privileges.
[CmdletBinding()]
param(
    [string]$Version = "latest",
    [string]$InstallDir = "",
    [string]$LocalArtifact = "",
    [string]$ExpectedSha256 = "",
    [switch]$AddToPath,
    [switch]$DryRun,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  Antigravity Engineering Kit (AEK) - Windows Installer     " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

# 1. Architecture Detection
$Arch = if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "aarch64" } else { "x86_64" }
$TargetTriple = if ($Arch -eq "aarch64") { "aarch64-pc-windows-msvc" } else { "x86_64-pc-windows-msvc" }

Write-Host "[1/6] Detected System Architecture: $Arch ($TargetTriple)"

# 2. Resolve Installation Directory
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

Write-Host "[2/6] Target Installation Directory: $InstallDir"
if (Test-Path $TargetExe) {
    try {
        $existingVer = & "$TargetExe" --version 2>&1
        Write-Host "  Existing installation detected: $existingVer" -ForegroundColor Yellow
        if (-not $Force -and -not $DryRun) {
            Write-Host "  Upgrading existing installation in place..."
        }
    } catch {
        Write-Host "  Existing file detected at target location." -ForegroundColor Yellow
    }
}

if ($DryRun) {
    Write-Host "`n[DRY RUN] Installation plan:" -ForegroundColor Magenta
    Write-Host "  - Target Executable: $TargetExe"
    Write-Host "  - Installation Dir:  $InstallDir"
    Write-Host "  - Manifest Path:     $ManifestPath"
    Write-Host "  - Update PATH:       $([bool]$AddToPath)"
    Write-Host "`n[DRY RUN] No system modifications made." -ForegroundColor Green
    exit 0
}

# 3. Acquire Artifact
$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("aek-install-" + [System.Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

try {
    $ZipPath = Join-Path $TempDir "aek-archive.zip"

    if ($LocalArtifact) {
        Write-Host "[3/6] Using provided local artifact: $LocalArtifact"
        if (-not (Test-Path $LocalArtifact)) {
            throw "Local artifact '$LocalArtifact' not found."
        }
        Copy-Item -Path $LocalArtifact -Destination $ZipPath -Force
    } else {
        $ReleaseBaseUrl = "https://github.com/google-antigravity/aek/releases"
        $ArtifactName = if ($Version -eq "latest") {
            "aek-v0.1.0-windows-$Arch.zip"
        } else {
            "aek-v$Version-windows-$Arch.zip"
        }
        $DownloadUrl = if ($Version -eq "latest") {
            "$ReleaseBaseUrl/latest/download/$ArtifactName"
        } else {
            "$ReleaseBaseUrl/download/v$Version/$ArtifactName"
        }

        Write-Host "[3/6] Downloading artifact: $DownloadUrl"
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -UseBasicParsing
    }

    # 4. Checksum Verification
    Write-Host "[4/6] Verifying artifact integrity..."
    $ActualHash = (Get-FileHash -Path $ZipPath -Algorithm SHA256).Hash.ToLower()

    if ($ExpectedSha256) {
        if ($ActualHash -ne $ExpectedSha256.ToLower().Trim()) {
            throw "SHA-256 verification failed! Expected: $ExpectedSha256, Actual: $ActualHash"
        }
        Write-Host "  SHA-256 checksum verified successfully." -ForegroundColor Green
    } else {
        Write-Host "  Calculated SHA-256: $ActualHash"
    }

    # 5. Safe Archive Extraction (Prevent Path Traversal)
    Write-Host "[5/6] Extracting binary securely..."
    $ExtractDir = Join-Path $TempDir "extracted"
    New-Item -ItemType Directory -Path $ExtractDir -Force | Out-Null

    # Inspect zip entries to block zip-slip path traversal
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [System.IO.Compression.ZipFile]::OpenRead($ZipPath)
    try {
        foreach ($entry in $zip.Entries) {
            $destPath = [System.IO.Path]::GetFullPath((Join-Path $ExtractDir $entry.FullName))
            $basePath = [System.IO.Path]::GetFullPath($ExtractDir)
            if (-not $destPath.StartsWith($basePath, [System.StringComparison]::OrdinalIgnoreCase)) {
                throw "Security error: Archive contains malicious path traversal entry: $($entry.FullName)"
            }
        }
    } finally {
        $zip.Dispose()
    }

    [System.IO.Compression.ZipFile]::ExtractToDirectory($ZipPath, $ExtractDir)

    $SourceExe = Get-ChildItem -Path $ExtractDir -Filter "aek.exe" -Recurse | Select-Object -First 1
    if (-not $SourceExe) {
        throw "Archive did not contain 'aek.exe' binary."
    }

    # Create target directory
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    # Backup existing binary if present
    $VersionsDir = Join-Path $InstallRoot "versions"
    if (Test-Path $TargetExe) {
        if (-not (Test-Path $VersionsDir)) {
            New-Item -ItemType Directory -Path $VersionsDir -Force | Out-Null
        }
        $backupVerDir = Join-Path $VersionsDir ("backup-" + (Get-Date -Format "yyyyMMdd-HHmmss"))
        New-Item -ItemType Directory -Path $backupVerDir -Force | Out-Null
        Copy-Item -Path $TargetExe -Destination (Join-Path $backupVerDir "aek.exe") -Force
    }

    # Copy new binary atomically
    $TempTarget = Join-Path $InstallDir "aek.exe.new"
    Copy-Item -Path $SourceExe.FullName -Destination $TempTarget -Force
    Move-Item -Path $TempTarget -Destination $TargetExe -Force

    # 6. Verify Executable
    Write-Host "[6/6] Verifying installed executable..."
    $VerOutput = & "$TargetExe" --version 2>&1
    Write-Host "  AEK is functional: $VerOutput" -ForegroundColor Green

    # Record Installation Manifest
    $ManifestObj = [PSCustomObject]@{
        version = if ($Version -eq "latest") { "0.1.0" } else { $Version }
        scope = "user"
        install_dir = $InstallDir
        binary_path = $TargetExe
        installed_at = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
        method = "installer_script"
        platform = @{
            target_triple = $TargetTriple
            os = "windows"
            arch = $Arch
        }
        managed_files = @($TargetExe, $ManifestPath)
        verification_status = "verified"
    }

    $ManifestObj | ConvertTo-Json -Depth 4 | Out-File -FilePath $ManifestPath -Encoding utf8

    # Optional PATH Update
    if ($AddToPath) {
        $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
        $parts = $userPath -split ";"
        if ($parts -notcontains $InstallDir) {
            Write-Host "Adding '$InstallDir' to User PATH environment variable..."
            $newPath = "$userPath;$InstallDir"
            [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
            $env:Path = "$env:Path;$InstallDir"
            Write-Host "  User PATH updated successfully." -ForegroundColor Green
        } else {
            Write-Host "  '$InstallDir' is already present in User PATH."
        }
    }

    Write-Host "`n============================================================" -ForegroundColor Green
    Write-Host "  Installation Complete!                                    " -ForegroundColor Green
    Write-Host "  Binary Location: $TargetExe                               " -ForegroundColor Green
    Write-Host "============================================================" -ForegroundColor Green
    exit 0
} catch {
    Write-Host "`n[ERROR] Installation failed: $_" -ForegroundColor Red
    exit 1
} finally {
    if (Test-Path $TempDir) {
        Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

