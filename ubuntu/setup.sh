#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Ghostty Greeter — Full Setup Script
# Installs: Ghostty config, ratatui greeter TUI, ble.sh, autocomplete.sh
# ============================================================

echo "==> Installing dependencies..."

# JetBrains Mono font
if ! fc-list | grep -qi "JetBrains Mono"; then
    echo "    Installing JetBrains Mono font..."
    sudo apt install -y fonts-jetbrains-mono 2>/dev/null || {
        mkdir -p ~/.local/share/fonts
        curl -fsSL https://github.com/JetBrains/JetBrainsMono/releases/latest/download/JetBrainsMono-2.304.zip -o /tmp/jbmono.zip
        unzip -o /tmp/jbmono.zip -d /tmp/jbmono
        cp /tmp/jbmono/fonts/ttf/*.ttf ~/.local/share/fonts/
        fc-cache -f
        rm -rf /tmp/jbmono /tmp/jbmono.zip
    }
else
    echo "    JetBrains Mono already installed"
fi

# Rust toolchain
if ! command -v cargo &>/dev/null; then
    echo "    Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "    Rust already installed"
fi
source "$HOME/.cargo/env" 2>/dev/null || true

# ============================================================
echo "==> Configuring Ghostty..."
mkdir -p ~/.config/ghostty

cat > ~/.config/ghostty/config << 'GHOSTTY_CONFIG'
# Theme
theme = TokyoNight Storm

# Font
font-family = JetBrains Mono
font-size = 13

# Window padding for breathing room
window-padding-x = 12
window-padding-y = 8
window-padding-balance = true

# Cursor
cursor-style = bar
cursor-style-blink = true

# Background transparency (slight)
background-opacity = 0.93

# Window
window-width = 120
window-height = 36
window-decoration = auto
confirm-close-surface = false

# Misc
copy-on-select = clipboard
mouse-hide-while-typing = true
GHOSTTY_CONFIG

echo "    Ghostty config written to ~/.config/ghostty/config"

# ============================================================
echo "==> Building ghostty-greeter..."
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

cargo build --release
mkdir -p ~/.local/bin
cp target/release/ghostty-greeter ~/.local/bin/ghostty-greeter
echo "    Installed ghostty-greeter to ~/.local/bin/"

# ============================================================
echo "==> Installing ble.sh (fish-like autosuggestions)..."
if [[ ! -f ~/.local/share/blesh/ble.sh ]]; then
    curl -L https://github.com/akinomyoga/ble.sh/releases/download/nightly/ble-nightly.tar.xz | tar xJf - -C /tmp/
    bash /tmp/ble-nightly/ble.sh --install ~/.local/share
    rm -rf /tmp/ble-nightly
    echo "    ble.sh installed"
else
    echo "    ble.sh already installed"
fi

cat > ~/.blerc << 'BLERC'
# ble.sh config — subtle and non-intrusive

# Autosuggestions: show from history, accept with right arrow
bleopt complete_auto_delay=300
bleopt complete_auto_complete=1
bleopt complete_ambiguous=

# Compact completion menu
bleopt complete_menu_style=dense
bleopt complete_menu_maxlines=8

# Subtle suggestion color
ble-face auto_complete='fg=240'

# Syntax highlighting (Tokyo Night inspired)
ble-face syntax_default='fg=189'
ble-face syntax_command='fg=114'
ble-face syntax_error='fg=204'
ble-face syntax_quoted='fg=180'
ble-face syntax_comment='fg=103'

# No bells
bleopt edit_abell=
bleopt edit_vbell=

# Share history across terminals
bleopt history_share=1
BLERC
echo "    ~/.blerc written"

# ============================================================
echo "==> Installing autocomplete.sh (AI suggestions)..."
if [[ ! -f ~/.local/bin/autocomplete.sh ]]; then
    wget -qO ~/.local/bin/autocomplete.sh https://raw.githubusercontent.com/closedloop-technologies/autocomplete-sh/v0.5.0/autocomplete.sh
    chmod +x ~/.local/bin/autocomplete.sh
    echo "    autocomplete.sh installed"
else
    echo "    autocomplete.sh already installed"
fi

mkdir -p ~/.autocomplete
cat > ~/.autocomplete/config << 'ACSH_CONFIG'
model: openai:	gpt-4o-mini
ACSH_CONFIG
echo "    ~/.autocomplete/config written"

# ============================================================
echo "==> Adding shell integrations to ~/.bashrc..."

# Check if we already added our block
if ! grep -q "ghostty-greeter" ~/.bashrc 2>/dev/null; then
    cat >> ~/.bashrc << 'BASHRC_ADDITIONS'

# ── Ghostty Greeter Setup ─────────────────────────────────────
export PATH="$HOME/.local/bin:$PATH"
. "$HOME/.cargo/env" 2>/dev/null || true

# ble.sh — fish-like autosuggestions, syntax highlighting for bash
[[ $- == *i* ]] && [[ -f ~/.local/share/blesh/ble.sh ]] && source ~/.local/share/blesh/ble.sh --noattach

# autocomplete.sh — AI-powered suggestions (double-Tab ONLY)
# Set your API key: export GEMINI_API_KEY="your-key" or OPENAI_API_KEY
if [[ -f ~/.local/bin/autocomplete.sh ]]; then
    if [[ -n "${GEMINI_API_KEY:-}" ]]; then
        export ACSH_OPENAI_API_KEY="$GEMINI_API_KEY"
        export ACSH_OPENAI_ENDPOINT="https://generativelanguage.googleapis.com/v1beta/openai/chat/completions"
    fi
    source ~/.local/bin/autocomplete.sh
fi

# Ghostty greeter — runs once per interactive terminal
if [ -n "$GHOSTTY_RESOURCES_DIR" ] && command -v ghostty-greeter &>/dev/null; then
    ghostty-greeter
fi

# Attach ble.sh at the end (must be last)
[[ ${BLE_VERSION-} ]] && ble-attach
# ── End Ghostty Greeter Setup ─────────────────────────────────
BASHRC_ADDITIONS
    echo "    bashrc additions written"
else
    echo "    bashrc already has ghostty-greeter setup"
fi

# ============================================================
echo ""
echo "==> Setup complete!"
echo ""
echo "  Open a new Ghostty terminal to see:"
echo "    - Glitch animation splash screen with your name"
echo "    - Press Enter to continue to shell"
echo "    - Press Space for system health dashboard"
echo "    - Fish-like autosuggestions (grey ghost text)"
echo "    - Double-Tab for AI suggestions (needs API key)"
echo ""
echo "  To set up AI suggestions, add to ~/.bashrc:"
echo '    export GEMINI_API_KEY="your-key-here"'
echo ""
