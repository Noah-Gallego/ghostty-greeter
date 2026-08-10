# Ghostty Greeter

A Rust terminal greeter and shell environment setup for Ghostty on macOS and Ubuntu/Linux.

<p align="center">
  <img src="https://skillicons.dev/icons?i=rust,apple,linux&theme=light" alt="Rust, macOS, and Linux" />
</p>

## Overview

The installer configures Ghostty, builds a Ratatui-based greeter, and sets up a themed terminal workflow with btop, ble.sh, shell configuration, and optional AI command suggestions.

## Features

- Cross-platform installer with macOS and Linux paths
- Rust TUI greeter launched from interactive Ghostty shells
- Tokyo Night Storm Ghostty and btop configuration
- JetBrains Mono setup
- ble.sh autosuggestions and syntax highlighting
- Optional `autocomplete.sh` integration for double-Tab AI suggestions
- Safe macOS config backups before replacement

## Installation

The repository installer detects macOS or Linux and delegates to the platform-specific setup:

```bash
bash install.sh
```

The installer supports a cloned checkout or the documented one-line GitHub install command. Platform scripts install prerequisites, build the release binary, and copy it to `~/.local/bin/`.

## Configuration

The platform directories contain Ghostty, btop, ble.sh, and shell configuration files. AI suggestions require a suitable API key configured in the shell; do not place credentials in tracked files.

## Status and limitations

The supported paths are macOS and Ubuntu/Linux. The setup scripts modify user-level shell and terminal configuration and may install packages, fonts, Rust, Ghostty, and other command-line tools.

## Preview / Media

No project screenshot or other honest visual preview is present in the tracked files; this repository is a terminal TUI and setup-script project.

## License

MIT
