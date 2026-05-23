#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Ghostty Greeter — macOS One-Command Setup
# Replicates the full terminal environment:
#   Ghostty + Tokyo Night Storm + btop + ble.sh + greeter TUI
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CONFIGS_DIR="$SCRIPT_DIR/configs"

BACKUP_DIR="$HOME/.config/ghostty-greeter-backups/$(date +%Y%m%d-%H%M%S)"

warn()  { echo "[!] $*" >&2; }
info()  { echo "[✓] $*"; }
step()  { echo ""; echo "==> $*"; }

# Prompt before overwriting existing configs
confirm_overwrite() {
    local file="$1"
    if [[ -f "$file" ]]; then
        read -rp "Overwrite existing $file? [y/N] " ans
        [[ "$ans" =~ ^[Yy]$ ]]
    else
        return 0
    fi
}

# Safe copy with backup
safe_copy() {
    local src="$1" dst="$2"
    if [[ -f "$dst" ]]; then
        mkdir -p "$BACKUP_DIR"
        cp "$dst" "$BACKUP_DIR/$(basename "$dst")"
        info "Backed up $(basename "$dst") to $BACKUP_DIR"
    fi
    cp "$src" "$dst"
    info "Installed $(basename "$dst")"
}

step "Checking prerequisites"

if ! command -v brew &>/dev/null; then
    warn "Homebrew not found. Install from https://brew.sh then re-run."
    exit 1
fi

info "Homebrew OK"

# ============================================================
step "Installing / updating dependencies"
# ============================================================

# Ghostty
if ! brew list --cask ghostty &>/dev/null 2>&1; then
    brew install --cask ghostty
    info "Ghostty installed"
else
    info "Ghostty already installed"
fi

# btop
if ! command -v btop &>/dev/null; then
    brew install btop
    info "btop installed"
else
    info "btop already installed"
fi

# CLI tools
brew install bat eza git-delta 2>/dev/null || true
info "CLI tools OK (bat, eza, delta)"

# JetBrains Mono font
if ! brew list --cask font-jetbrains-mono &>/dev/null 2>&1; then
    brew install --cask font-jetbrains-mono 2>/dev/null || {
        info "Falling back to manual font install..."
        mkdir -p ~/Library/Fonts
        curl -fsSL -o /tmp/jbmono.zip \
            "https://github.com/JetBrains/JetBrainsMono/releases/latest/download/JetBrainsMono-2.304.zip"
        unzip -o /tmp/jbmono.zip -d /tmp/jbmono
        cp /tmp/jbmono/fonts/ttf/*.ttf ~/Library/Fonts/
        rm -rf /tmp/jbmono /tmp/jbmono.zip
    }
    info "JetBrains Mono font installed"
else
    info "JetBrains Mono font already installed"
fi

# Rust (needed to build greeter)
if ! command -v cargo &>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    info "Rust installed"
else
    source "$HOME/.cargo/env" 2>/dev/null || true
    info "Rust already installed"
fi

# ============================================================
step "Installing configs"
# ============================================================

# Ghostty
mkdir -p ~/.config/ghostty
safe_copy "$CONFIGS_DIR/ghostty/config" ~/.config/ghostty/config

# ble.sh
if [[ ! -f ~/.local/share/blesh/ble.sh ]]; then
    info "Installing ble.sh..."
    curl -L https://github.com/akinomyoga/ble.sh/releases/download/nightly/ble-nightly.tar.xz | tar xJf - -C /tmp/
    bash /tmp/ble-nightly/ble.sh --install ~/.local/share
    rm -rf /tmp/ble-nightly
    info "ble.sh installed"
else
    info "ble.sh already installed"
fi
safe_copy "$CONFIGS_DIR/blerc" ~/.blerc

# btop
mkdir -p ~/.config/btop/themes
safe_copy "$CONFIGS_DIR/btop/btop.conf" ~/.config/btop/btop.conf
safe_copy "$CONFIGS_DIR/btop/themes/tokyo_night_storm.theme" ~/.config/btop/themes/tokyo_night_storm.theme

# Shell configs
if confirm_overwrite ~/.zshrc; then
    safe_copy "$CONFIGS_DIR/zshrc" ~/.zshrc
fi

if confirm_overwrite ~/.bashrc; then
    safe_copy "$CONFIGS_DIR/bashrc" ~/.bashrc
fi

# Ensure bash_profile sources bashrc
if [[ -f ~/.bash_profile ]] && ! grep -q 'source.*\.bashrc\|\..*\.bashrc' ~/.bash_profile; then
    cat >> ~/.bash_profile << 'BPADD'

# Source bashrc for interactive shells
if [ -f "$HOME/.bashrc" ]; then
    . "$HOME/.bashrc"
fi
BPADD
    info "Updated ~/.bash_profile"
fi

# ============================================================
step "Building ghostty-greeter"
# ============================================================

cd "$SCRIPT_DIR"
cargo build --release
mkdir -p ~/.local/bin
cp target/release/ghostty-greeter ~/.local/bin/ghostty-greeter
chmod +x ~/.local/bin/ghostty-greeter
info "ghostty-greeter installed to ~/.local/bin/"

# ============================================================
step "Adding Ghostty to Dock"
# ============================================================

if ! defaults read com.apple.dock persistent-apps 2>/dev/null | grep -q "Ghostty"; then
    defaults write com.apple.dock persistent-apps -array-add \
        '<dict><key>tile-data</key><dict><key>file-data</key><dict><key>_CFURLString</key><string>/Applications/Ghostty.app</string><key>_CFURLStringType</key><integer>0</integer></dict></dict></dict>'
    killall Dock
    info "Ghostty added to Dock"
else
    info "Ghostty already in Dock"
fi

# ============================================================
echo ""
echo "🎉 Setup complete!"
echo ""
echo "  What's installed:"
echo "    • Ghostty terminal with Tokyo Night Storm theme"
echo "    • ghostty-greeter TUI (glitch animation + system info)"
echo "    • btop with Tokyo Night Storm theme"
echo "    • ble.sh syntax highlighting & autosuggestions"
echo "    • bat, eza, delta (themed CLI tools)"
echo ""
echo "  Open Ghostty to see the greeter. Press:"
echo "    Enter  → shell"
echo "    Space  → btop system monitor"
echo "    Esc/q  → quit greeter"
echo ""
echo "  Tip: If you use autocomplete.sh or Steel.dev, add your"
echo "       API keys to ~/.zshrc (placeholders are there)."
echo ""
