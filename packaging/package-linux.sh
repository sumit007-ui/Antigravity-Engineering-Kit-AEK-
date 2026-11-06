#!/usr/bin/env bash
# Antigravity Engineering Kit (AEK) - Linux Packaging Script
set -euo pipefail

VERSION="${1:-}"
TARGET_TRIPLE="${2:-x86_64-unknown-linux-gnu}"
BINARY_PATH="${3:-target/release/aek}"
OUTPUT_DIR="${4:-dist}"

if [ -z "$VERSION" ]; then
    if [ -f "Cargo.toml" ]; then
        VERSION=$(grep -m1 '^version\s*=' Cargo.toml | cut -d '"' -f2 || true)
    fi
    if [ -z "$VERSION" ]; then
        VERSION="0.1.0"
    fi
fi

ARCH="x86_64"
if [[ "$TARGET_TRIPLE" == *"aarch64"* ]]; then
    ARCH="aarch64"
fi

ARTIFACT_NAME="aek-v${VERSION}-linux-${ARCH}.tar.gz"

echo "Packaging AEK for Linux..."
echo "  Version:       ${VERSION}"
echo "  Target Triple: ${TARGET_TRIPLE}"
echo "  Binary:        ${BINARY_PATH}"
echo "  Output Dir:    ${OUTPUT_DIR}"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary '$BINARY_PATH' not found. Run 'cargo build --release' first." >&2
    exit 1
fi

mkdir -p "$OUTPUT_DIR"
STAGING_DIR="${OUTPUT_DIR}/staging-linux-${ARCH}"
rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR"

cp "$BINARY_PATH" "${STAGING_DIR}/aek"
chmod +x "${STAGING_DIR}/aek"

[ -f "README.md" ] && cp "README.md" "${STAGING_DIR}/"
[ -f "LICENSE-APACHE" ] && cp "LICENSE-APACHE" "${STAGING_DIR}/"
[ -f "LICENSE-MIT" ] && cp "LICENSE-MIT" "${STAGING_DIR}/"

TAR_PATH="${OUTPUT_DIR}/${ARTIFACT_NAME}"
tar -czf "$TAR_PATH" -C "$STAGING_DIR" .

if command -v sha256sum >/dev/null 2>&1; then
    SHA256=$(sha256sum "$TAR_PATH" | awk '{print $1}')
elif command -v shasum >/dev/null 2>&1; then
    SHA256=$(shasum -a 256 "$TAR_PATH" | awk '{print $1}')
else
    echo "Error: Neither sha256sum nor shasum is available" >&2
    exit 1
fi

echo "${SHA256}  ${ARTIFACT_NAME}" > "${TAR_PATH}.sha256"
rm -rf "$STAGING_DIR"

SIZE=$(stat -c%s "$TAR_PATH" 2>/dev/null || stat -f%z "$TAR_PATH" 2>/dev/null || wc -c < "$TAR_PATH")

echo ""
echo "Successfully packaged: ${TAR_PATH}"
echo "  Size:   ${SIZE} bytes"
echo "  SHA256: ${SHA256}"
echo "  Hash File: ${TAR_PATH}.sha256"

