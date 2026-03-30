#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Ghostty Greeter — macOS Setup (Apple Silicon)
# Installs: Ghostty, greeter TUI, ble.sh, bat, eza, delta
# Configures: Tokyo Night Storm theme everywhere
# ============================================================

echo "==> Installing dependencies..."

# Homebrew check
if ! command -v brew &>/dev/null; then
    echo "ERROR: Homebrew is required. Install from https://brew.sh"
    exit 1
fi

# JetBrains Mono font
if ! system_profiler SPFontsDataType 2>/dev/null | grep -qi "JetBrains Mono"; then
    echo "    Installing JetBrains Mono font..."
    brew install --cask font-jetbrains-mono 2>/dev/null || {
        mkdir -p ~/Library/Fonts
        curl -fsSL https://github.com/JetBrains/JetBrainsMono/releases/latest/download/JetBrainsMono-2.304.zip -o /tmp/jbmono.zip
        unzip -o /tmp/jbmono.zip -d /tmp/jbmono
        cp /tmp/jbmono/fonts/ttf/*.ttf ~/Library/Fonts/
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

# Ghostty
if ! command -v ghostty &>/dev/null && [ ! -d "/Applications/Ghostty.app" ]; then
    echo "    Installing Ghostty..."
    brew install --cask ghostty
else
    echo "    Ghostty already installed"
fi

# CLI tools
echo "    Installing CLI tools (bat, eza, delta)..."
brew install bat git-delta eza 2>/dev/null || true

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

# macOS specific
macos-titlebar-style = transparent
macos-option-as-alt = true

# Misc
copy-on-select = clipboard
mouse-hide-while-typing = true
GHOSTTY_CONFIG

echo "    Ghostty config written"

# ============================================================
echo "==> Building ghostty-greeter..."
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

cargo build --release
mkdir -p ~/.local/bin
cp target/release/ghostty-greeter ~/.local/bin/ghostty-greeter
echo "    Installed ghostty-greeter to ~/.local/bin/"

# ============================================================
echo "==> Installing ble.sh (syntax highlighting + autosuggestions)..."
if [[ ! -f ~/.local/share/blesh/ble.sh ]]; then
    curl -L https://github.com/akinomyoga/ble.sh/releases/download/nightly/ble-nightly.tar.xz | tar xJf - -C /tmp/
    bash /tmp/ble-nightly/ble.sh --install ~/.local/share
    rm -rf /tmp/ble-nightly
    echo "    ble.sh installed"
else
    echo "    ble.sh already installed"
fi

# ble.sh Tokyo Night theme config
cat > ~/.blerc << 'BLERC'
# ble.sh config — Tokyo Night Storm theme
bleopt complete_auto_delay=300
bleopt complete_auto_complete=1
bleopt complete_ambiguous=
bleopt complete_menu_style=dense
bleopt complete_menu_maxlines=8

ble-face auto_complete='fg=60'
ble-face syntax_default='fg=189'
ble-face syntax_command='fg=114'
ble-face syntax_quoted='fg=180'
ble-face syntax_error='fg=204'
ble-face syntax_comment='fg=103'
ble-face syntax_varname='fg=117'
ble-face syntax_delimiter='fg=146'
ble-face syntax_param_expansion='fg=141'
ble-face syntax_history_expansion='fg=141'
ble-face syntax_glob='fg=81'
ble-face syntax_brace='fg=81'
ble-face syntax_tilde='fg=81'
ble-face syntax_document='fg=180'
ble-face syntax_document_begin='fg=180,bold'
ble-face command_builtin='fg=81'
ble-face command_function='fg=117'
ble-face command_file='fg=114'
ble-face command_keyword='fg=141'
ble-face command_alias='fg=81'
ble-face command_directory='fg=81,bold'
ble-face filename_directory='fg=81,bold'
ble-face filename_executable='fg=114'
ble-face filename_link='fg=73,underline'
ble-face filename_orphan='fg=204,strike'
ble-face region_target='fg=36,bg=60'
ble-face region_insert='fg=36,bg=17'

bleopt edit_abell=
bleopt edit_vbell=
bleopt history_share=1
BLERC
echo "    ~/.blerc written"

# ============================================================
echo "==> Configuring bat (Tokyo Night theme)..."
mkdir -p "$(bat --config-dir)/themes"
curl -fsSL -o "$(bat --config-dir)/themes/tokyonight_storm.tmTheme" \
    "https://raw.githubusercontent.com/folke/tokyonight.nvim/main/extras/sublime/tokyonight_storm.tmTheme" 2>/dev/null || true
bat cache --build 2>/dev/null || true
echo "    bat theme installed"

# ============================================================
echo "==> Configuring git delta (Tokyo Night diffs)..."
git config --global core.pager delta
git config --global interactive.diffFilter 'delta --color-only'
git config --global delta.navigate true
git config --global delta.dark true
git config --global delta.side-by-side true
git config --global delta.line-numbers true
git config --global delta.syntax-theme "TwoDark"
git config --global delta.file-style "bold blue"
git config --global delta.hunk-header-style "omit"
git config --global delta.minus-style "syntax #37222c"
git config --global delta.minus-emph-style "syntax #713137"
git config --global delta.plus-style "syntax #20303b"
git config --global delta.plus-emph-style "syntax #2c5a66"
git config --global delta.line-numbers-minus-style "#914c54"
git config --global delta.line-numbers-plus-style "#449dab"
git config --global delta.line-numbers-zero-style "#3b4261"
echo "    delta configured"

# ============================================================
echo "==> Writing themed ~/.bashrc..."

cat > ~/.bashrc << 'BASHRC'
export PATH="$HOME/.local/bin:$PATH"
. "$HOME/.local/bin/env" 2>/dev/null || true
. "$HOME/.cargo/env" 2>/dev/null || true

# ── ble.sh — syntax highlighting & autosuggestions ────────────
[[ $- == *i* ]] && [[ -f ~/.local/share/blesh/ble.sh ]] && source ~/.local/share/blesh/ble.sh --noattach

# ── Tokyo Night Storm — Prompt ────────────────────────────────
_c_reset='\[\e[0m\]'
_c_dim='\[\e[38;2;115;130;175m\]'
_c_sky='\[\e[38;2;125;207;255m\]'
_c_green='\[\e[38;2;158;206;106m\]'
_c_mauve='\[\e[38;2;187;154;247m\]'
_c_peach='\[\e[38;2;255;158;100m\]'
_c_red='\[\e[38;2;247;118;142m\]'
_c_teal='\[\e[38;2;115;218;202m\]'
_c_blue='\[\e[38;2;122;162;247m\]'
_c_yellow='\[\e[38;2;224;175;104m\]'
_c_sub='\[\e[38;2;169;177;214m\]'
_c_fg='\[\e[38;2;192;202;245m\]'

__git_prompt() {
    local branch
    branch=$(git symbolic-ref --short HEAD 2>/dev/null || git rev-parse --short HEAD 2>/dev/null)
    [[ -z "$branch" ]] && return
    local dirty="" staged=""
    local status
    status=$(git status --porcelain 2>/dev/null)
    [[ -n "$status" ]] && dirty="*"
    echo "$status" | grep -q '^[MADRC]' && staged="+"
    local state="${dirty}${staged}"
    local color="${_c_green}"
    [[ -n "$dirty" ]] && color="${_c_peach}"
    echo -e " ${_c_dim}on ${color}\uf126 ${branch}${_c_red}${state}${_c_reset}"
}

__set_prompt() {
    local last_exit=$?
    local exit_indicator=""
    [[ $last_exit -ne 0 ]] && exit_indicator=" ${_c_red}[${last_exit}]${_c_reset}"
    local dir="${_c_blue}\w${_c_reset}"
    local git
    git=$(__git_prompt)
    local arrow="${_c_mauve}\$${_c_reset}"
    PS1="\n${_c_sky}\u${_c_dim}@${_c_teal}\h ${dir}${git}${exit_indicator}\n${arrow} "
}
PROMPT_COMMAND="__set_prompt"

# ── LS Colors (Tokyo Night) ──────────────────────────────────
export LS_COLORS='di=1;38;2;122;162;247:ln=38;2;115;218;202:so=38;2;187;154;247:pi=38;2;224;175;104:ex=1;38;2;158;206;106:bd=38;2;224;175;104;48;2;52;56;80:cd=38;2;224;175;104;48;2;52;56;80:su=38;2;247;118;142;1:sg=38;2;224;175;104:tw=38;2;158;206;106;48;2;52;56;80:ow=38;2;122;162;247;48;2;52;56;80:*.tar=38;2;255;158;100:*.gz=38;2;255;158;100:*.zip=38;2;255;158;100:*.jpg=38;2;187;154;247:*.png=38;2;187;154;247:*.gif=38;2;187;154;247:*.svg=38;2;187;154;247:*.mp4=38;2;115;218;202:*.pdf=38;2;247;118;142:*.md=38;2;125;207;255:*.json=38;2;224;175;104:*.yml=38;2;224;175;104:*.toml=38;2;224;175;104:*.rs=38;2;255;158;100:*.py=38;2;122;162;247:*.js=38;2;224;175;104:*.ts=38;2;122;162;247:*.go=38;2;115;218;202:*.sh=38;2;158;206;106'

# ── Tool configs ─────────────────────────────────────────────
export BAT_THEME="tokyonight_storm"
alias cat='bat --paging=never'
alias catp='bat'
alias ls='eza --icons --group-directories-first'
alias ll='eza --icons --group-directories-first -la'
alias lt='eza --icons --group-directories-first --tree --level=2'
alias la='eza --icons --group-directories-first -a'
export GIT_PAGER='delta'
alias grep='grep --color=auto'
alias gs='git status'
alias gd='git diff'
alias gl='git log --oneline --graph --decorate -15'
alias gp='git push'

# ── Ghostty Greeter ──────────────────────────────────────────
if [ -n "$GHOSTTY_RESOURCES_DIR" ] && command -v ghostty-greeter &>/dev/null; then
    ghostty-greeter
fi

# ── ble.sh attach (must be last) ─────────────────────────────
[[ ${BLE_VERSION-} ]] && ble-attach
BASHRC
echo "    ~/.bashrc written"

# ============================================================
echo "==> Adding source ~/.bashrc to ~/.bash_profile..."
if ! grep -q 'source.*\.bashrc\|\..*\.bashrc' ~/.bash_profile 2>/dev/null; then
    cat >> ~/.bash_profile << 'BPADDITION'

# Source bashrc for interactive shells
if [ -f "$HOME/.bashrc" ]; then
    . "$HOME/.bashrc"
fi
BPADDITION
    echo "    bash_profile updated"
else
    echo "    bash_profile already sources bashrc"
fi

# ============================================================
# Add Ghostty to Dock
if ! defaults read com.apple.dock persistent-apps 2>/dev/null | grep -q "Ghostty"; then
    echo "==> Adding Ghostty to Dock..."
    defaults write com.apple.dock persistent-apps -array-add '<dict><key>tile-data</key><dict><key>file-data</key><dict><key>_CFURLString</key><string>/Applications/Ghostty.app</string><key>_CFURLStringType</key><integer>0</integer></dict></dict></dict>'
    killall Dock
    echo "    Ghostty added to Dock"
else
    echo "    Ghostty already in Dock"
fi

# ============================================================
echo ""
echo "==> Setup complete!"
echo ""
echo "  Open Ghostty from your Dock to see:"
echo "    - Glitch animation splash with your name"
echo "    - Press Enter for shell, Space for health dashboard"
echo "    - Tokyo Night syntax highlighting everywhere"
echo "    - Git-aware prompt with branch + dirty state"
echo "    - bat (cat), eza (ls), delta (git diff) all themed"
echo "    - Fish-like autosuggestions via ble.sh"
echo ""
