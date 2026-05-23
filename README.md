<div align="center">

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  ASCII HEADER                                                -->
<!-- ═══════════════════════════════════════════════════════════ -->

<pre style="color: #7aa2f7;">
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║   ██████╗  ██╗  ██╗ ██████╗ ███████╗████████╗████████╗██╗   ║
║  ██╔════╝  ██║  ██║██╔═══██╗██╔════╝╚══██╔══╝╚══██╔══╝██║   ║
║  ██║  ███╗ ███████║██║   ██║███████╗   ██║      ██║   ██║   ║
║  ██║   ██║ ██╔══██║██║   ██║╚════██║   ██║      ██║   ██║   ║
║  ╚██████╔╝ ██║  ██║╚██████╔╝███████║   ██║      ██║   ██║   ║
║   ╚═════╝  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝      ╚═╝   ╚═╝   ║
║                                                               ║
║           <b>TUI Greeter for Ghostty</b>                         ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
</pre>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  BADGES                                                      -->
<!-- ═══════════════════════════════════════════════════════════ -->

[![Rust](https://img.shields.io/badge/Rust-000?logo=rust&logoColor=white&style=flat-square)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-7aa2f7?style=flat-square)](LICENSE)
[![macOS](https://img.shields.io/badge/macOS-supported-9ece6a?style=flat-square&logo=apple&logoColor=white)](mac/)
[![Linux](https://img.shields.io/badge/Linux-supported-e0af68?style=flat-square&logo=linux&logoColor=white)](ubuntu/)
[![Tokyo Night](https://img.shields.io/badge/theme-Tokyo%20Night%20Storm-7aa2f7?style=flat-square)](https://github.com/folke/tokyonight.nvim)

<br>

**A glitch-animated terminal greeter that launches btop, ble.sh syntax highlighting,**  
**and a fully themed Tokyo Night Storm environment — in one command.**

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  ONE-LINE INSTALL  (THE MONEY SHOT)                          -->
<!-- ═══════════════════════════════════════════════════════════ -->

```bash
curl -fsSL https://raw.githubusercontent.com/Noah-Gallego/ghostty-greeter/master/install.sh | bash
```

<sub>Or clone manually → see [Quick Start](#quick-start) below</sub>

<br>

</div>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  DEMO  (ASCII VISUAL)                                        -->
<!-- ═══════════════════════════════════════════════════════════ -->

## 🎥 What It Looks Like

```text
┌───────────────────────────────────────────────────────────────────────┐
│                                                               │
│     ▓▓░░█▒ NOAH   ██░▒▓  │  ← glitch decode in ~2.8s     │
│     ░▒█░██▓░██▒░███  ▒░▓│                               │
│     ▒▒▒░█░▓██░░▒████  ░▒│                               │
│                                                               │
│     ██░▒ GALLEGO  ▓▒█░░▓▒▒│  ← second wave decode            │
│     ▓░█░██▒▓█░░▒▒██  ░▒█│                               │
│     ░▒░█░▒█░░▓▒██████▒░│                               │
│                                                               │
│         "Golden hour coding."   █  ← time-of-day greeting         │
│                                                               │
│     Mon 23 Jun 14:32    Mac.local   M4 (10t)  📜 36 GB  💾 994 GB  │
│                                                               │
│              ⏎ Enter    continue                              │
│              ♥ Space    system health  → btop                  │
│                                                               │
└───────────────────────────────────────────────────────────────────────┘
```

> **💡 Press `Enter`** → drops into a themed shell  
> **💡 Press `Space`** → launches **btop** with Tokyo Night Storm theme  
> **💡 Press `Esc` / `q`** → back to greeter (or quit)

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  FEATURE GRID                                                -->
<!-- ═══════════════════════════════════════════════════════════ -->

## ⚡ Features

<table>
<tr>
<td width="33%" valign="top">

### 🎨 Glitch Animation
A terminal boot sequence that decodes your name from noise — like a sci-fi movie intro, but it's your terminal.

</td>
<td width="33%" valign="top">

### 🤖 Nerd Font Icons
CPU, RAM, disk, and date — all rendered with actual icons thanks to JetBrains Mono Nerd Font.

</td>
<td width="33%" valign="top">

### 📊 btop Integration
Press `Space` and you're in a fully themed system monitor. Press `Esc` and you're back at the greeter.

</td>
</tr>
<tr>
<td width="33%" valign="top">

### 🔘 Tokyo Night Storm
Every tool — Ghostty, btop, ble.sh, bat, eza, delta — shares one cohesive dark color palette.

</td>
<td width="33%" valign="top">

### 💾 ble.sh Syntax Highlight
Fish-like autosuggestions + real-time syntax highlighting in bash/zsh. No config needed.

</td>
<td width="33%" valign="top">

### 🚀 One-Command Setup
Clone, run `bash install.sh`, and your entire terminal environment is replicated. Backups included.

</td>
</tr>
</table>

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  QUICK START                                                 -->
<!-- ═══════════════════════════════════════════════════════════ -->

## 🚀 Quick Start

### Option A: One-Liner (fastest)

```bash
curl -fsSL https://raw.githubusercontent.com/Noah-Gallego/ghostty-greeter/master/install.sh | bash
```

### Option B: Clone + Run

```bash
git clone https://github.com/Noah-Gallego/ghostty-greeter.git
cd ghostty-greeter
bash install.sh
```

### What Gets Installed

| Tool | Purpose |
|------|---------|
| [Ghostty](https://ghostty.org) | GPU-accelerated terminal emulator |
| `ghostty-greeter` | This TUI binary (Rust + ratatui) |
| [btop](https://github.com/aristocratos/btop) | System resource monitor |
| [ble.sh](https://github.com/akinomyoga/ble.sh) | Bash syntax highlighting & autosuggestions |
| [bat](https://github.com/sharkdp/bat) | Syntax-highlighted `cat` |
| [eza](https://github.com/eza-community/eza) | Modern `ls` with icons & git status |
| [delta](https://github.com/dandavison/delta) | Beautiful git diffs |
| JetBrains Mono Nerd Font | Terminal font with thousands of icons |

> **✅ Safe to re-run.** The installer is idempotent — it checks what's already installed, backs up existing configs, and only changes what it needs to.

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  PLATFORM MATRIX                                             -->
<!-- ═══════════════════════════════════════════════════════════ -->

## 💻 Platforms

| Platform | Directory | Install Command | Notes |
|----------|-----------|-----------------|-------|
| **macOS** (Apple Silicon) | [`mac/`](mac/) | `cd mac && bash setup.sh` | `system_profiler` for chip info |
| **Ubuntu / Linux** | [`ubuntu/`](ubuntu/) | `cd ubuntu && bash setup.sh` | NVIDIA GPU via `nvidia-smi` |

> Want to add Windows/WSL? The greeter is Rust — cross-compiling is straightforward. PRs welcome!

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  THEME PREVIEW                                               -->
<!-- ═══════════════════════════════════════════════════════════ -->

## 🎨 The Full Tokyo Night Storm Stack

Everything shares one palette. No jarring color clashes when you switch between tools.

```text
┌───────────────────────────────────────────────────────────────────────┐
│  🎭 Ghostty   │  #1a1b26 background  │  #7aa2f7 blue accent      │
│  📊 btop      │  #c0caf5 foreground  │  #9ece6a green success    │
│  ✍️  ble.sh    │  #f7768e error red    │  #bb9af7 magenta magic    │
│  📖 bat       │  #e0af68 warning      │  #7dcfff cyan info        │
│  📂 eza       │  #ff9e64 orange       │  transparent 0.93 bg     │
└───────────────────────────────────────────────────────────────────────┘
```

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  CONTROLS                                                    -->
<!-- ═══════════════════════════════════════════════════════════ -->

## 🎮 Controls

| Key | Action |
|-----|--------|
| `Enter` | Continue to shell |
| `Space` | Launch **btop** system monitor |
| `Esc` / `q` | Back to greeter (from btop) or quit |

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  WHAT YOU GET (SHELL LIFE)                                   -->
<!-- ═══════════════════════════════════════════════════════════ -->

## 💰 What The Setup Gives You

Your shell becomes a fully themed development environment:

- **📝 Syntax highlighting** — ble.sh colors commands, strings, errors, variables in real time
- **🔍 Fish-like autosuggestions** — grayed-out completions from history, accept with → Right Arrow
- **🐱 `cat` → `bat`** — syntax-highlighted file previews with line numbers
- **📂 `ls` → `eza`** — icons, git status, tree view, grouped directories
- **📉 `git diff` → `delta`** — side-by-side diffs with line numbers and Tokyo Night colors
- **🎭 Greeter on every new Ghostty window** — never a boring terminal again

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  REPO STRUCTURE                                              -->
<!-- ═══════════════════════════════════════════════════════════ -->

## 📦 Repo Structure

```
ghostty-greeter/
├── install.sh              └─ one-liner entry point (auto-detects OS)
├── README.md               └─ this file
├── mac/
│   ├── setup.sh            └─ macOS setup script
│   ├── src/main.rs         └─ greeter source (ratatui)
│   ├── Cargo.toml          └─ Rust manifest
│   └── configs/            └─ dotfiles that get copied to ~/
│       ├── ghostty/config
│       ├── zshrc
│       ├── bashrc
│       ├── blerc
│       └── btop/
└── ubuntu/
    ├── setup.sh            └─ Ubuntu/Linux setup script
    ├── src/main.rs         └─ greeter source
    ├── Cargo.toml          └─ Rust manifest
    └── configs/            └─ Linux dotfiles
```

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  BUILT WITH                                                  -->
<!-- ═══════════════════════════════════════════════════════════ -->

## 🛠️ Built With

<p>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" height="24"></a>
  <a href="https://ratatui.rs"><img src="https://img.shields.io/badge/ratatui-000000?style=for-the-badge&logo=rust&logoColor=white" height="24"></a>
  <a href="https://ghostty.org"><img src="https://img.shields.io/badge/Ghostty-1a1b26?style=for-the-badge&logo=ghostty&logoColor=white" height="24"></a>
  <a href="https://github.com/folke/tokyonight.nvim"><img src="https://img.shields.io/badge/Tokyo%20Night%20Storm-1a1b26?style=for-the-badge" height="24"></a>
</p>

<br>

<!-- ═══════════════════════════════════════════════════════════ -->
<!--  FOOTER                                                      -->
<!-- ═══════════════════════════════════════════════════════════ -->

<div align="center">

**Made with ♥ by [Noah Gallego](https://github.com/Noah-Gallego)**

<sub>If this makes your terminal cooler, drop a ⭐ on the repo.</sub>

</div>
