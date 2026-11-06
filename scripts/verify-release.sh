#!/usr/bin/env bash
# Antigravity Engineering Kit (AEK) - Release Verification Script (Bash)
set -euo pipefail

DIST_DIR="${1:-dist}"
MANIFEST_PATH="${2:-dist/release-manifest.json}"

echo "Verifying AEK Release Artifacts in '${DIST_DIR}'..."
echo "============================================================"

if [ ! -d "$DIST_DIR" ]; then
    echo "Error: Distribution directory '${DIST_DIR}' does not exist." >&2
    exit 1
fi

all_valid=true

shopt -s nullglob
sha_files=("${DIST_DIR}"/*.sha256)
shopt -u nullglob

if [ ${#sha_files[@]} -eq 0 ]; then
    echo "Error: No .sha256 checksum files found in '${DIST_DIR}'." >&2
    exit 1
fi

for sf in "${sha_files[@]}"; do
    expected_hash=$(awk '{print $1}' "$sf")
    artifact_name=$(basename "$sf" .sha256)
    artifact_path="${DIST_DIR}/${artifact_name}"

    if [ ! -f "$artifact_path" ]; then
        echo "  [-] Artifact missing for checksum: ${artifact_name}"
        all_valid=false
        continue
    fi

    if command -v sha256sum >/dev/null 2>&1; then
        actual_hash=$(sha256sum "$artifact_path" | awk '{print $1}')
    else
        actual_hash=$(shasum -a 256 "$artifact_path" | awk '{print $1}')
    fi

    if [ "${actual_hash,,}" = "${expected_hash,,}" ]; then
        echo "  [+] ${artifact_name} : SHA256 OK (${expected_hash})"
    else
        echo "  [-] ${artifact_name} : SHA256 MISMATCH!"
        echo "      Expected: ${expected_hash}"
        echo "      Actual:   ${actual_hash}"
        all_valid=false
    fi
done

if [ -f "$MANIFEST_PATH" ]; then
    echo ""
    echo "Validating Release Manifest: ${MANIFEST_PATH}"
    if command -v jq >/dev/null 2>&1; then
        version=$(jq -r '.version' "$MANIFEST_PATH")
        channel=$(jq -r '.release_channel' "$MANIFEST_PATH")
        echo "  [+] Version:         ${version}"
        echo "  [+] Release Channel: ${channel}"
    else
        echo "  [*] jq not found; skipping advanced JSON checks"
    fi
fi

echo "============================================================"
if [ "$all_valid" = true ]; then
    echo "[SUCCESS] All release artifacts verified."
    exit 0
else
    echo "Error: Release verification failed." >&2
    exit 1
fi
