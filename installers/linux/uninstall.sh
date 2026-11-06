#!/usr/bin/env bash
# Antigravity Engineering Kit (AEK) - Linux Uninstaller
set -euo pipefail

INSTALL_DIR="${HOME}/.local/bin"
PURGE=false
FORCE=false
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --purge)
            PURGE=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        *)
            echo "Unknown argument: $1" >&2
            exit 1
            ;;
    esac
done

echo "============================================================"
echo "  Antigravity Engineering Kit (AEK) - Linux Uninstaller     "
echo "============================================================"

INSTALL_ROOT=$(dirname "$INSTALL_DIR")
TARGET_EXE="${INSTALL_DIR}/aek"
MANIFEST_PATH="${INSTALL_ROOT}/installation.json"
CONFIG_DIR="${XDG_CONFIG_HOME:-${HOME}/.config}/aek"

echo "Detected AEK installation root: ${INSTALL_ROOT}"

if [ "$DRY_RUN" = true ]; then
    echo ""
    echo "[DRY RUN] Uninstallation plan:"
    echo "  Files scheduled for deletion:"
    [ -f "$TARGET_EXE" ] && echo "    - ${TARGET_EXE}"
    [ -f "$MANIFEST_PATH" ] && echo "    - ${MANIFEST_PATH}"
    if [ "$PURGE" = true ]; then
        echo "  PURGE MODE ACTIVE - Directories scheduled for deletion:"
        [ -d "$CONFIG_DIR" ] && echo "    - ${CONFIG_DIR} (User configuration)"
        [ -d "${INSTALL_ROOT}/share/aek" ] && echo "    - ${INSTALL_ROOT}/share/aek"
    else
        echo "  Preserving user configuration directory: ${CONFIG_DIR}"
    fi
    echo ""
    echo "[DRY RUN] No files deleted."
    exit 0
fi

if [ "$PURGE" = true ] && [ "$FORCE" = false ]; then
    read -p "Are you sure you want to PURGE all AEK configurations and data? [y/N] " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Uninstallation aborted by user."
        exit 0
    fi
fi

if [ -f "$TARGET_EXE" ]; then
    rm -f "$TARGET_EXE"
    echo "  Removed executable: ${TARGET_EXE}"
fi

if [ -f "$MANIFEST_PATH" ]; then
    rm -f "$MANIFEST_PATH"
    echo "  Removed manifest: ${MANIFEST_PATH}"
fi

if [ "$PURGE" = true ]; then
    if [ -d "$CONFIG_DIR" ]; then
        rm -rf "$CONFIG_DIR"
        echo "  Purged configuration directory: ${CONFIG_DIR}"
    fi
    if [ -d "${INSTALL_ROOT}/share/aek" ]; then
        rm -rf "${INSTALL_ROOT}/share/aek"
        echo "  Purged data directory: ${INSTALL_ROOT}/share/aek"
    fi
else
    echo ""
    echo "Preserved user configuration in: ${CONFIG_DIR}"
    echo "To remove configuration, run: ./uninstall.sh --purge"
fi

echo ""
echo "AEK uninstallation completed successfully."
exit 0

