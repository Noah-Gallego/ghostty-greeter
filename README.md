# Ghostty Greeter

A ratatui-based TUI greeter and system health dashboard for the [Ghostty](https://ghostty.org) terminal. Launches with a glitch animation splash screen, then drops you into a live system monitor or your shell.

![Tokyo Night Storm themed](https://img.shields.io/badge/theme-Tokyo%20Night%20Storm-7aa2f7)

## Features

- **Glitch animation** splash screen with ASCII art name reveal
- **System health dashboard** — CPU, memory, disk, network, processes
- **Platform-native monitoring** — Apple Silicon GPU + battery (mac) / NVIDIA GPU + Dropbox sync (ubuntu)
- **Tokyo Night Storm** color scheme throughout
- **One-command setup** installs everything: Ghostty config, fonts, CLI tools, syntax highlighting

## Platforms

| Directory | Target | GPU | Extra Panel |
|-----------|--------|-----|-------------|
| `mac/` | macOS (Apple Silicon) | M-series chip info via `system_profiler` | Battery (pmset/ioreg) |
| `ubuntu/` | Ubuntu / Linux | NVIDIA via `nvidia-smi` | Dropbox sync status |

## Quick Start

```bash
# Clone the repo
git clone https://github.com/Noah-Gallego/ghostty-greeter.git
cd ghostty-greeter

# Run the setup for your platform
# macOS:
cd mac && bash setup.sh

# Ubuntu/Linux:
cd ubuntu && bash setup.sh
```

The setup script handles everything:
- Installs Rust, Ghostty, JetBrains Mono font
- Builds the greeter binary
- Configures Ghostty with Tokyo Night Storm theme
- Installs **ble.sh** (fish-like syntax highlighting + autosuggestions)
- Installs **bat** (syntax-highlighted `cat`), **eza** (modern `ls`), **delta** (beautiful git diffs)
- Sets up a git-aware Tokyo Night bash prompt
- Wires the greeter to auto-launch in Ghostty

## Controls

| Key | Action |
|-----|--------|
| `Enter` | Continue to shell |
| `Space` | Open system health dashboard |
| `← →` | Switch dashboard tabs |
| `↑ ↓` | Scroll process list |
| `Esc` / `q` | Back / quit |

## What the Setup Gives You

- **Syntax highlighting** in your shell via ble.sh (Tokyo Night colors)
- **Git-aware prompt** showing branch + dirty/staged state
- **`cat` → `bat`** with syntax highlighting
- **`ls` → `eza`** with icons and color
- **`git diff` → `delta`** with side-by-side, line numbers, Tokyo Night colors
- All colors matched to the **Tokyo Night Storm** palette
