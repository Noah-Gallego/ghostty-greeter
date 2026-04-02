# Ghostty Greeter

A ratatui-based TUI greeter for the [Ghostty](https://ghostty.org) terminal. Launches with a glitch animation splash screen, then drops you into [btop](https://github.com/aristocratos/btop) for system monitoring or your shell.

![Tokyo Night Storm themed](https://img.shields.io/badge/theme-Tokyo%20Night%20Storm-7aa2f7)

## Features

- **Glitch animation** splash screen with ASCII art name reveal
- **Nerd Font icons** — system info strip with CPU, RAM, disk, date
- **btop integration** — press Space to launch a fully themed btop session, Esc returns to greeter
- **Tokyo Night Storm** color scheme throughout (Ghostty, btop, greeter)
- **One-command setup** installs everything: Ghostty config, fonts, CLI tools, syntax highlighting

## Platforms

| Directory | Target | OS Icon | Notes |
|-----------|--------|---------|-------|
| `mac/` | macOS (Apple Silicon) |  Apple | `system_profiler` for chip info |
| `ubuntu/` | Ubuntu / Linux |  Linux | NVIDIA GPU via `nvidia-smi` |

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
- Installs Rust, Ghostty, JetBrains Mono Nerd Font
- Builds the greeter binary
- Configures Ghostty with Tokyo Night Storm theme
- Installs **btop** with a custom Tokyo Night Storm theme
- Installs **ble.sh** (fish-like syntax highlighting + autosuggestions)
- Installs **bat** (syntax-highlighted `cat`), **eza** (modern `ls`), **delta** (beautiful git diffs)
- Sets up a git-aware Tokyo Night bash prompt
- Wires the greeter to auto-launch in Ghostty

## Controls

| Key | Action |
|-----|--------|
| `Enter` | Continue to shell |
| `Space` | Open btop system monitor |
| `Esc` / `q` | Back / quit |

## What the Setup Gives You

- **Syntax highlighting** in your shell via ble.sh (Tokyo Night colors)
- **Git-aware prompt** showing branch + dirty/staged state
- **`cat` → `bat`** with syntax highlighting
- **`ls` → `eza`** with icons and color
- **`git diff` → `delta`** with side-by-side, line numbers, Tokyo Night colors
- All colors matched to the **Tokyo Night Storm** palette
