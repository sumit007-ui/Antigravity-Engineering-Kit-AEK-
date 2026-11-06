#!/usr/bin/env bash
# Antigravity Engineering Kit (AEK) - Linux Installer Automated Test Harness
set -euo pipefail

echo "Running automated Linux installer test harness..."

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

SANDBOX_DIR=$(mktemp -d 2>/dev/null || mktemp -d -t 'aek-test-sandbox')
cleanup() {
    rm -rf "$SANDBOX_DIR"
}
trap cleanup EXIT

DIST_DIR="${SANDBOX_DIR}/dist"
INSTALL_BIN_DIR="${SANDBOX_DIR}/installed/bin"
MOCK_BIN="${SANDBOX_DIR}/mock-bin/aek"

mkdir -p "$(dirname "$MOCK_BIN")"
cat << 'EOF' > "$MOCK_BIN"
#!/usr/bin/env bash
if [ "${1:-}" = "--version" ]; then
    echo "aek 0.1.0"
    exit 0
fi
echo "Mock AEK binary"
exit 0
EOF
chmod +x "$MOCK_BIN"

# 1. Package test artifact
echo "[Step 1/5] Packaging local test artifact..."
"${REPO_ROOT}/packaging/package-linux.sh" "0.1.0" "x86_64-unknown-linux-gnu" "$MOCK_BIN" "$DIST_DIR"

TAR_FILE=$(find "$DIST_DIR" -name "*.tar.gz" | head -n 1)
SHA_FILE="${TAR_FILE}.sha256"
EXPECTED_HASH=$(awk '{print $1}' "$SHA_FILE")

# 2. Test Dry Run
echo "[Step 2/5] Testing installer dry-run..."
"${SCRIPT_DIR}/install.sh" --local-artifact "$TAR_FILE" --install-dir "$INSTALL_BIN_DIR" --dry-run
if [ -d "$INSTALL_BIN_DIR" ]; then
    echo "Error: Dry run should not create installation directory!" >&2
    exit 1
fi

# 3. Test Installation
echo "[Step 3/5] Testing actual installation..."
"${SCRIPT_DIR}/install.sh" --local-artifact "$TAR_FILE" --install-dir "$INSTALL_BIN_DIR" --expected-sha256 "$EXPECTED_HASH"

INSTALLED_EXE="${INSTALL_BIN_DIR}/aek"
if [ ! -f "$INSTALLED_EXE" ]; then
    echo "Error: Installed executable not found at '${INSTALLED_EXE}'!" >&2
    exit 1
fi

VER_OUTPUT=$("$INSTALLED_EXE" --version)
echo "  Installed executable output: ${VER_OUTPUT}"
if [[ "$VER_OUTPUT" != *"aek"* ]]; then
    echo "Error: Executable output does not match expected version!" >&2
    exit 1
fi

# 4. Test Uninstaller Dry Run
echo "[Step 4/5] Testing uninstaller dry-run..."
"${SCRIPT_DIR}/uninstall.sh" --install-dir "$INSTALL_BIN_DIR" --dry-run
if [ ! -f "$INSTALLED_EXE" ]; then
    echo "Error: Uninstaller dry run should not delete files!" >&2
    exit 1
fi

# 5. Test Uninstallation
echo "[Step 5/5] Testing uninstallation..."
"${SCRIPT_DIR}/uninstall.sh" --install-dir "$INSTALL_BIN_DIR" --force
if [ -f "$INSTALLED_EXE" ]; then
    echo "Error: Uninstallation failed: binary still exists!" >&2
    exit 1
fi

echo ""
echo "[SUCCESS] Linux installer automated test suite passed!"
exit 0
