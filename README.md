# HazeVeil
```
   $$\   $$\                                                          $$\ $$\
   $$ |  $$ |                                                         \__|$$ |
   $$ |  $$ | $$$$$$\  $$$$$$$$\  $$$$$$\        $$\    $$\  $$$$$$\  $$\ $$ |
   $$$$$$$$ | \____$$\ \____$$  |$$  __$$\       \$$\  $$  |$$  __$$\ $$ |$$ |
   $$  __$$ | $$$$$$$ |  $$$$ _/ $$$$$$$$ |       \$$\$$  / $$$$$$$$ |$$ |$$ |
   $$ |  $$ |$$  __$$ | $$  _/   $$   ____|        \$$$  /  $$   ____|$$ |$$ |
   $$ |  $$ |\$$$$$$$ |$$$$$$$$\ \$$$$$$$\          \$  /   \$$$$$$$\ $$ |$$ |
   \__|  \__| \_______|\________| \_______|          \_/     \_______|\__|\__|
```

**Linux Behavioral Variation Simulator**

HazeVeil is an open-source background tool for Linux that introduces dynamic,
human-like variations across all input signals simultaneously. It is designed
for privacy researchers, security professionals, and developers working in the
field of behavioral biometrics.

![License](https://img.shields.io/badge/license-GPL--3.0-blue)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey)
![Rust](https://img.shields.io/badge/built%20with-Rust-orange)
![Tests](https://img.shields.io/badge/tests-28%2F28%20passing-brightgreen)
![Version](https://img.shields.io/badge/version-1.0.0-informational)
[![crates.io](https://img.shields.io/crates/v/hazeveil.svg)](https://crates.io/crates/hazeveil)

---

## The Problem

Modern systems identify users not just by passwords or cookies, but by how they
behave — the precise curves of mouse movement, the rhythm of keystrokes, the
deceleration of scroll inertia, the pressure applied to a touchpad. This class
of technology, known as behavioral biometrics, builds persistent profiles that
can follow users across sessions, applications, and devices without their
knowledge or consent.

Unlike traditional fingerprinting, behavioral biometrics requires no special
permissions, stores no obvious identifiers, and is largely invisible to the
user being profiled.

---

## What HazeVeil Does

HazeVeil runs silently in the background and introduces controlled, statistically
realistic variations across all input channels at once. Each session generates
a unique behavioral signature by blending multiple anonymized profiles from a
library of over 200 patterns, making consistent fingerprinting significantly
harder without affecting normal usage or system performance.

The tool does not block, intercept, or modify actual user input. It operates
at the simulation layer, generating parallel behavioral noise that disrupts
the statistical consistency that profiling systems depend on.

---

## How It Works

On startup, HazeVeil selects 3 to 5 profiles at random from its embedded library
and blends them using randomized fusion ratios. This blend becomes the active
behavioral signature for the session.

Every 50 simulation cycles, the engine applies entropy-based pattern drift,
gradually shifting the blend to prevent detectable transitions. After 100 cycles,
a full re-blend occurs with entirely new pattern combinations and ratios. No two
sessions produce the same behavioral signature.

The simulation engine covers all major input channels simultaneously:

| Channel | Technique |
|---------|-----------|
| Mouse | Adaptive Bezier path curves, Gaussian jitter, micro-tremors, velocity variance |
| Keyboard | Poisson-distributed inter-key delays, dwell time variance, ghost key events |
| Touchpad | Pressure curve simulation, multi-finger gesture randomization, inertia variance |
| Scroll | Inertia deceleration curves, direction drift, momentum variance |
| Window focus | Switching frequency jitter, distraction pause simulation |
| Clipboard | Usage timing randomization |
| Audio | Typing sound pattern variation when a microphone is detected |

All operations run entirely in memory. No behavioral data is ever logged or
transmitted.

---

## What Makes HazeVeil Different

Most privacy tools focus on network-level protection — VPNs, DNS encryption,
tracker blocking. HazeVeil operates at a layer that most tools ignore entirely:
the behavioral layer.

| Feature | HazeVeil | Typical Privacy Tools |
|---------|----------|-----------------------|
| Targets behavioral biometrics | Yes | No |
| Works without root privileges | Yes | Varies |
| No network traffic required | Yes | No |
| In-memory only, no disk writes | Yes | No |
| Multi-channel simultaneous coverage | Yes | No |
| Session-unique signatures | Yes | No |
| Entropy drift between cycles | Yes | No |
| 200+ embedded behavioral profiles | Yes | No |
| Custom pattern training | Yes | No |
| Encrypted local configuration | Yes | Rarely |

---

## Pattern Library

The embedded library contains 15 archetype profiles:

| Archetype | Description |
|-----------|-------------|
| casual-browser | Relaxed web browsing patterns |
| fast-typist | High-speed professional typing |
| senior-user | Slower, deliberate interaction |
| gamer | Fast, precise, low-pause input |
| mobile-switcher | Frequent application switching |
| developer | Keyboard-heavy workflow |
| student | Mixed reading and typing patterns |
| data-entry | Repetitive, consistent keystroke cadence |
| creative | Variable pace with frequent pauses |
| multitasker | High window-switch frequency |
| reader | Scroll-dominant, low keyboard activity |
| erratic | High variance across all channels |
| methodical | Low variance, slow deliberate actions |
| power-user | Fast keyboard, minimal mouse dependency |
| novice | Low speed, high error rate, long pauses |

These 15 archetypes generate 185 additional synthetic variants, giving the
library over 200 distinct profiles. Each session blends 3 to 5 of these
with randomized weights.

---

## Simulation Levels

| Level | Name | Description |
|-------|------|-------------|
| 1 | Minimal | Very subtle variation, nearly imperceptible |
| 2 | Low | Light variation across input channels |
| 3 | Moderate | Balanced — recommended for most use cases |
| 4 | High | Strong variation with noticeable entropy |
| 5 | Maximum | Maximum behavioral masking across all channels |

---

## Requirements

- Linux — x86_64 or ARM64
- Kernel 5.15 or later (6.1+ recommended)
- Wayland or X11 display server
- Supported distributions: Arch Linux, CachyOS, Ubuntu 22.04+, Debian 12+, Fedora 38+, openSUSE

---

## Installation

### Via Cargo (Recommended)
```bash
cargo install hazeveil
```
After installation, open a new terminal window.

### From Source
```bash
git clone https://github.com/anas-cyber-net/hazeveil.git && cd hazeveil && chmod +x install.sh && ./install.sh
```

The installer handles everything automatically:
- Installs Rust if not present
- Installs required system libraries for your distribution
- Builds HazeVeil in release mode with full optimizations
- Installs the binary to `/usr/local/bin/`
- Configures your shell PATH

After installation, open a new terminal window.

---

## Usage

Run HazeVeil from any directory:
```bash
hazeveil
```

On first run, HazeVeil asks you to choose a simulation level and pattern.
After selecting, it starts immediately. By design, no settings are saved
between sessions — each run starts fresh.

**All commands:**

| Command | Description |
|---------|-------------|
| `hazeveil` | Start with interactive level and pattern selection |
| `hazeveil start --level 3` | Start at a specific level |
| `hazeveil start --level 3 --pattern developer` | Start with specific level and pattern |
| `hazeveil start --level 3 --exclude-app vlc` | Exclude an application |
| `hazeveil status` | Show current daemon status |
| `hazeveil stop` | Stop the running daemon |
| `hazeveil train-pattern --name mypattern --duration 15m` | Train a custom pattern |
| `hazeveil --verbose start --level 3` | Start with detailed logging |
| `hazeveil --help` | Show all available commands |

**View full simulation activity:**
```bash
RUST_LOG=trace hazeveil --verbose start --level 5
```

---

## Available Patterns

| Pattern | Description |
|---------|-------------|
| `random` | Blend different patterns each session — recommended |
| `casual-browser` | Casual web browsing style |
| `fast-typist` | High-speed professional typing |
| `developer` | Keyboard-heavy developer workflow |
| `gamer` | Fast and precise input |
| `methodical` | Slow and deliberate interaction |
| `student` | Research and reading patterns |
| `multitasker` | Frequent application switching |
| `novice` | Slow, careful, high error rate |

---

## Training Custom Patterns

HazeVeil can train a custom behavioral pattern from your own usage:
```bash
hazeveil train-pattern --name office-mode --duration 15m
```

This records anonymized input samples for the specified duration, trains a
local pattern, and saves it encrypted to `~/.config/hazeveil/patterns/`.

Activate it with:
```bash
hazeveil start --pattern office-mode
```

---

## Running as a System Service
```bash
sudo cp resources/hazeveil.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now hazeveil
journalctl -u hazeveil -f
```

---

## Building from Source
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Arch / CachyOS
sudo pacman -Syu --needed base-devel pkg-config libevdev openssl

# Ubuntu / Debian
sudo apt install -y build-essential pkg-config libevdev-dev libssl-dev

# Fedora
sudo dnf install -y gcc pkg-config libevdev-devel openssl-devel

# Build
cargo build --release --features "full-simulation,stealth"

# Test
cargo test
```

---

## Security

- Configuration encrypted with XChaCha20-Poly1305
- Key derivation via Argon2id (64MB memory, 3 iterations, 4 parallel threads)
- No behavioral data written to disk
- No network communication
- No root privileges required
- Secrets zeroed from memory after use via zeroize

---

## Proof of Authorship

All source files in this repository are timestamped on the Bitcoin blockchain
via OpenTimestamps. Verification files are included in the `proofs/` directory.

To verify any file:
```bash
ots verify proofs/main.rs.ots
```

---

## Project Structure
```
hazeveil/
├── src/
│   ├── main.rs              Entry point and CLI
│   ├── core_engine.rs       Main simulation orchestration loop
│   ├── ai_pattern.rs        Pattern library and blending engine
│   ├── mouse_engine.rs      Mouse movement simulation
│   ├── keyboard_engine.rs   Keyboard timing simulation
│   ├── touchpad_engine.rs   Touchpad gesture simulation
│   ├── scroll_engine.rs     Scroll inertia simulation
│   ├── timing_engine.rs     Sub-millisecond timing engine
│   ├── window_engine.rs     Window focus simulation
│   ├── clipboard_engine.rs  Clipboard timing simulation
│   ├── audio_engine.rs      Audio pattern simulation
│   ├── context_detector.rs  Display server and app context detection
│   ├── config.rs            Encrypted local configuration
│   ├── daemon.rs            Process and signal management
│   └── utils.rs             Cryptographic utilities
├── tests/
│   ├── engine_test.rs       Engine unit tests
│   ├── pattern_test.rs      Pattern blending and library tests
│   ├── stealth_test.rs      Variance and entropy verification tests
│   └── integration_test.rs  Full simulation flow tests
├── resources/
│   └── hazeveil.service     systemd service unit file
├── proofs/                  OpenTimestamps blockchain authorship proofs
└── install.sh               Automated one-command installer
```

---

## Contributing

Contributions are welcome. Please open an issue before submitting a pull request
to discuss the proposed change. All contributions must be compatible with the
GPL-3.0 license.

---

## Legal and Ethical Notice

HazeVeil is intended strictly for ethical, legal, and authorized purposes
including privacy research, academic study of behavioral biometrics systems,
and authorized security testing.

Do not use this tool to evade lawful monitoring, violate platform terms of
service, or engage in any activity that is illegal in your jurisdiction.
The author assumes no liability for any misuse of this software.

---

## License

Copyright (C) 2026 Anas Malik Jaafar

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or any later version.

See the [LICENSE](LICENSE) file or visit https://www.gnu.org/licenses/gpl-3.0.html
