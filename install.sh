#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Ghostty Greeter — Cross-Platform One-Line Installer
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Noah-Gallego/ghostty-greeter/master/install.sh | bash
#   — OR —
#   git clone https://github.com/Noah-Gallego/ghostty-greeter.git
#   cd ghostty-greeter && bash install.sh
# ============================================================

REPO_URL="https://github.com/Noah-Gallego/ghostty-greeter.git"
CLONE_DIR=""

step()  { echo "==> $*"; }
info()  { echo "[✓] $*"; }
warn()  { echo "[!] $*" >&2; }

# Detect OS
OS=""
case "$(uname -s)" in
    Darwin*) OS="macos" ;;
    Linux*)  OS="linux" ;;
    *)       OS="unknown" ;;
esac

info "Detected OS: $OS"

# If running from a cloned repo, use that. Otherwise clone first.
if [[ -f "$(dirname "$0")/mac/setup.sh" || -f "$(dirname "$0")/ubuntu/setup.sh" ]]; then
    CLONE_DIR="$(cd "$(dirname "$0")" && pwd)"
    info "Using existing repo at $CLONE_DIR"
else
    CLONE_DIR="${HOME}/ghostty-greeter"
    if [[ -d "$CLONE_DIR/.git" ]]; then
        info "Repo already cloned at $CLONE_DIR, pulling latest..."
        git -C "$CLONE_DIR" pull
    else
        step "Cloning ghostty-greeter..."
        git clone "$REPO_URL" "$CLONE_DIR"
    fi
fi

# Run the platform-specific setup
case "$OS" in
    macos)
        step "Running macOS setup..."
        bash "$CLONE_DIR/mac/setup.sh"
        ;;
    linux)
        step "Running Ubuntu/Linux setup..."
        bash "$CLONE_DIR/ubuntu/setup.sh"
        ;;
    *)
        warn "Unsupported OS. Only macOS and Linux are supported."
        exit 1
        ;;
esac

info "All done! Open Ghostty to see your greeter."
