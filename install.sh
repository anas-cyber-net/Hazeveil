#!/bin/bash

set -e

cat << 'EOF'

   $$\   $$\                                                          $$\ $$\
   $$ |  $$ |                                                         \__|$$ |
   $$ |  $$ | $$$$$$\  $$$$$$$$\  $$$$$$\        $$\    $$\  $$$$$$\  $$\ $$ |
   $$$$$$$$ | \____$$\ \____$$  |$$  __$$\       \$$\  $$  |$$  __$$\ $$ |$$ |
   $$  __$$ | $$$$$$$ |  $$$$ _/ $$$$$$$$ |       \$$\$$  / $$$$$$$$ |$$ |$$ |
   $$ |  $$ |$$  __$$ | $$  _/   $$   ____|        \$$$  /  $$   ____|$$ |$$ |
   $$ |  $$ |\$$$$$$$ |$$$$$$$$\ \$$$$$$$\          \$  /   \$$$$$$$\ $$ |$$ |
   \__|  \__| \_______|\________| \_______|          \_/     \_______|\__|\__|

   HazeVeil Installer v0.1.0
==================================================

EOF

if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo "ERROR: HazeVeil is Linux only."
    exit 1
fi

if ! command -v cargo &>/dev/null; then
    echo "[*] Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

echo "[*] Rust found: $(rustc --version)"
echo ""

echo "[*] Installing system dependencies..."
if command -v pacman &>/dev/null; then
    sudo pacman -Syu --needed --noconfirm base-devel pkg-config libevdev openssl
elif command -v apt &>/dev/null; then
    sudo apt update && sudo apt install -y build-essential pkg-config libevdev-dev libssl-dev
elif command -v dnf &>/dev/null; then
    sudo dnf install -y gcc pkg-config libevdev-devel openssl-devel
elif command -v zypper &>/dev/null; then
    sudo zypper install -y gcc pkg-config libevdev-devel libopenssl-devel
else
    echo "WARNING: Unknown package manager. Install manually: libevdev, openssl, pkg-config, gcc"
fi

echo ""
echo "[*] Building HazeVeil (release mode)..."
cargo build --release --features "full-simulation,stealth"
echo ""

echo "[*] Installing hazeveil to /usr/local/bin/..."
sudo cp target/release/hazeveil /usr/local/bin/hazeveil
sudo chmod +x /usr/local/bin/hazeveil
echo ""

if [ -f "$HOME/.config/fish/config.fish" ]; then
    if ! grep -q "cargo/bin" "$HOME/.config/fish/config.fish"; then
        echo 'fish_add_path ~/.cargo/bin' >> "$HOME/.config/fish/config.fish"
        echo "[*] Added cargo to fish PATH"
    fi
fi

if [ -f "$HOME/.bashrc" ]; then
    if ! grep -q "cargo/bin" "$HOME/.bashrc"; then
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.bashrc"
        echo "[*] Added cargo to bash PATH"
    fi
fi

if [ -f "$HOME/.zshrc" ]; then
    if ! grep -q "cargo/bin" "$HOME/.zshrc"; then
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.zshrc"
        echo "[*] Added cargo to zsh PATH"
    fi
fi

cat << 'EOF'

==================================================
 HazeVeil installed successfully!

 Usage:
   hazeveil            → start (asks level on first run)
   hazeveil setup      → change level/pattern anytime
   hazeveil status     → check if running
   hazeveil stop       → stop the daemon
   hazeveil --help     → all commands

==================================================
EOF