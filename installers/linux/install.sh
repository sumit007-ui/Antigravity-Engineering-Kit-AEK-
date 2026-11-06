#!/usr/bin/env bash
# Antigravity Engineering Kit (AEK) - Linux Production Installer
# Installs AEK into user-level ~/.local/bin without requiring root or sudo.
set -euo pipefail

VERSION="latest"
INSTALL_DIR=""
LOCAL_ARTIFACT=""
EXPECTED_SHA256=""
ADD_TO_PATH=false
DRY_RUN=false
FORCE=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --local-artifact)
            LOCAL_ARTIFACT="$2"
            shift 2
            ;;
        --expected-sha256)
            EXPECTED_SHA256="$2"
            shift 2
            ;;
        --add-to-path)
            ADD_TO_PATH=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        *)
            echo "Unknown argument: $1" >&2
            exit 1
            ;;
    esac
done

echo "============================================================"
echo "  Antigravity Engineering Kit (AEK) - Linux Installer       "
echo "============================================================"

# 1. Architecture Detection
ARCH=$(uname -m)
case "$ARCH" in
    x86_64|amd64)
        TARGET_ARCH="x86_64"
        TARGET_TRIPLE="x86_64-unknown-linux-gnu"
        ;;
    aarch64|arm64)
        TARGET_ARCH="aarch64"
        TARGET_TRIPLE="aarch64-unknown-linux-gnu"
        ;;
    *)
        echo "Error: Unsupported architecture: $ARCH" >&2
        exit 1
        ;;
esac

echo "[1/6] Detected System Architecture: ${TARGET_ARCH} (${TARGET_TRIPLE})"

# 2. Installation Directory Resolution
if [ -z "$INSTALL_DIR" ]; then
    INSTALL_DIR="${HOME}/.local/bin"
fi

INSTALL_ROOT=$(dirname "$INSTALL_DIR")
TARGET_EXE="${INSTALL_DIR}/aek"
MANIFEST_PATH="${INSTALL_ROOT}/installation.json"

echo "[2/6] Target Installation Directory: ${INSTALL_DIR}"

if [ -f "$TARGET_EXE" ]; then
    if existing_ver=$("$TARGET_EXE" --version 2>/dev/null); then
        echo "  Existing installation detected: ${existing_ver}"
    else
        echo "  Existing file detected at target location."
    fi
fi

if [ "$DRY_RUN" = true ]; then
    echo ""
    echo "[DRY RUN] Installation plan:"
    echo "  - Target Executable: ${TARGET_EXE}"
    echo "  - Installation Dir:  ${INSTALL_DIR}"
    echo "  - Manifest Path:     ${MANIFEST_PATH}"
    echo "  - Update PATH:       ${ADD_TO_PATH}"
    echo ""
    echo "[DRY RUN] No system modifications made."
    exit 0
fi

# 3. Acquire Artifact
TEMP_DIR=$(mktemp -d 2>/dev/null || mktemp -d -t 'aek-install')
cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

TAR_PATH="${TEMP_DIR}/aek-archive.tar.gz"

if [ -n "$LOCAL_ARTIFACT" ]; then
    echo "[3/6] Using provided local artifact: ${LOCAL_ARTIFACT}"
    if [ ! -f "$LOCAL_ARTIFACT" ]; then
        echo "Error: Local artifact '${LOCAL_ARTIFACT}' not found." >&2
        exit 1
    fi
    cp "$LOCAL_ARTIFACT" "$TAR_PATH"
else
    RELEASE_BASE="https://github.com/google-antigravity/aek/releases"
    if [ "$VERSION" = "latest" ]; then
        DOWNLOAD_URL="${RELEASE_BASE}/latest/download/aek-v0.1.0-linux-${TARGET_ARCH}.tar.gz"
    else
        DOWNLOAD_URL="${RELEASE_BASE}/download/v${VERSION}/aek-v${VERSION}-linux-${TARGET_ARCH}.tar.gz"
    fi

    echo "[3/6] Downloading artifact: ${DOWNLOAD_URL}"
    if command -v curl >/dev/null 2>&1; then
        curl -sSL --proto '=https' --tlsv1.2 -o "$TAR_PATH" "$DOWNLOAD_URL"
    elif command -v wget >/dev/null 2>&1; then
        wget -q --https-only -O "$TAR_PATH" "$DOWNLOAD_URL"
    else
        echo "Error: Neither curl nor wget is available for secure download." >&2
        exit 1
    fi
fi

# 4. Checksum Verification
echo "[4/6] Verifying artifact integrity..."
if command -v sha256sum >/dev/null 2>&1; then
    ACTUAL_HASH=$(sha256sum "$TAR_PATH" | awk '{print $1}')
elif command -v shasum >/dev/null 2>&1; then
    ACTUAL_HASH=$(shasum -a 256 "$TAR_PATH" | awk '{print $1}')
else
    echo "Error: Neither sha256sum nor shasum is installed." >&2
    exit 1
fi

if [ -n "$EXPECTED_SHA256" ]; then
    if [ "${ACTUAL_HASH,,}" != "${EXPECTED_SHA256,,}" ]; then
        echo "Error: SHA-256 verification failed!" >&2
        echo "  Expected: ${EXPECTED_SHA256}" >&2
        echo "  Actual:   ${ACTUAL_HASH}" >&2
        exit 1
    fi
    echo "  SHA-256 checksum verified successfully."
else
    echo "  Calculated SHA-256: ${ACTUAL_HASH}"
fi

# 5. Safe Archive Extraction
echo "[5/6] Extracting binary securely..."
EXTRACT_DIR="${TEMP_DIR}/extracted"
mkdir -p "$EXTRACT_DIR"

# Test tar contents to prevent directory traversal
if tar -tzf "$TAR_PATH" | grep -E '(^\.\./|/\.\./)' >/dev/null 2>&1; then
    echo "Error: Malicious archive detected containing directory traversal paths!" >&2
    exit 1
fi

tar -xzf "$TAR_PATH" -C "$EXTRACT_DIR"

SOURCE_EXE=$(find "$EXTRACT_DIR" -name "aek" -type f | head -n 1)
if [ -z "$SOURCE_EXE" ]; then
    echo "Error: Archive did not contain 'aek' executable binary." >&2
    exit 1
fi

mkdir -p "$INSTALL_DIR"

# Backup existing binary
if [ -f "$TARGET_EXE" ]; then
    VERSIONS_DIR="${INSTALL_ROOT}/versions"
    BACKUP_DIR="${VERSIONS_DIR}/backup-$(date +%Y%m%d-%H%M%S)"
    mkdir -p "$BACKUP_DIR"
    cp "$TARGET_EXE" "${BACKUP_DIR}/aek"
fi

# Copy binary atomically
cp "$SOURCE_EXE" "${TARGET_EXE}.tmp"
chmod +x "${TARGET_EXE}.tmp"
mv "${TARGET_EXE}.tmp" "$TARGET_EXE"

# 6. Verify Executable
echo "[6/6] Verifying installed executable..."
if VER_OUTPUT=$("$TARGET_EXE" --version 2>&1); then
    echo "  AEK is functional: ${VER_OUTPUT}"
else
    echo "Error: Installed binary failed execution test." >&2
    exit 1
fi

# Record Installation Manifest
cat <<EOF > "$MANIFEST_PATH"
{
  "version": "${VERSION}",
  "scope": "user",
  "install_dir": "${INSTALL_DIR}",
  "binary_path": "${TARGET_EXE}",
  "installed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "method": "installer_script",
  "platform": {
    "target_triple": "${TARGET_TRIPLE}",
    "os": "linux",
    "arch": "${TARGET_ARCH}"
  },
  "managed_files": [
    "${TARGET_EXE}",
    "${MANIFEST_PATH}"
  ],
  "verification_status": "verified"
}
EOF

# Update PATH if requested
if [ "$ADD_TO_PATH" = true ]; then
    SHELL_RC=""
    if [ -n "${BASH_VERSION:-}" ]; then
        SHELL_RC="${HOME}/.bashrc"
    elif [ -n "${ZSH_VERSION:-}" ]; then
        SHELL_RC="${HOME}/.zshrc"
    fi

    if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
        if ! grep -q "$INSTALL_DIR" "$SHELL_RC"; then
            echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "$SHELL_RC"
            echo "Added ${INSTALL_DIR} to ${SHELL_RC}"
        fi
    fi
fi

echo ""
echo "============================================================"
echo "  Installation Complete!                                    "
echo "  Binary Location: ${TARGET_EXE}                            "
echo "============================================================"
exit 0

