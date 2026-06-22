# SSH-VPN Rust-Based High-Performance Core

[![English](https://img.shields.io/badge/English-README-blue?style=flat-square)](README.en.md) | [![فارسی](https://img.shields.io/badge/فارسی-README-green?style=flat-square)](README.md)

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat-square)](https://www.rust-lang.org) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)](LICENSE) [![Tokio](https://img.shields.io/badge/Tokio-Async-blue?style=flat-square)](https://tokio.rs)

An ultra-safe, blazingly fast network core rewritten in **Rust** to support asynchronous **SSH Tunneling**, **VPN encapsulation**, and **Split Tunneling** automation. This core is designed to be the foundation for modern VPN applications, providing 100% stability and zero-cost abstractions.

## 🚀 Architectural Advantages

- **Hybrid SSH Backend**: Supports both `libssh2` (via C bindings) and `russh` (native Rust) for maximum compatibility and safety.
- **Zero Garbage Collection**: Zero runtime pauses ensure consistent ping and throughput.
- **Memory Safety Guarantee**: Compile-time memory verification prevents Buffer Overflows and Data Races.
- **Native Async Stack**: Built using Tokio for massive connection scaling.
- **Split Tunneling**: Built-in support for OS-level routing configuration (Linux, macOS, Windows).
- **SlipNet Support**: Architecture designed to be compatible with DNS tunneling protocols like those found in [SlipNet](https://github.com/anonvector/SlipNet).

## 🛠️ Project Structure

```
core-rust-vpn/
├── src/
│   ├── main.rs           # Entry point and service orchestration
│   ├── routing.rs        # OS-specific routing table management
│   └── ssh/
│       ├── mod.rs        # SSH engines module
│       ├── libssh2_backend.rs  # LibSSH2 backend (C bindings)
│       └── russh_backend.rs   # Russh backend (native Rust)
├── tests/
│   └── core_tests.rs     # Comprehensive integration and stability tests
├── Cargo.toml           # Dependencies and project configuration
└── README.md            # Project documentation (Persian)
```

## 📦 Compilation and Setup

### Prerequisites

- Install Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Git for cloning the repository

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/movtigroup/core-rust-vpn.git
   cd SSH-VPN
   ```

2. Build the project in release mode:
   ```bash
   cargo build --release
   ```

### Execution

Run the compiled binary:
```bash
cargo run
```

### Testing

Run the stability and structural tests:
```bash
cargo test -- --nocapture
```

## 📝 Configuration

The core can be configured via `SshConfig` struct. It supports:

- SSH User/Password authentication.
- Local Port Forwarding (SOCKS5/HTTP Proxy compatible).
- Split Tunneling IP ranges.
- Choice between C-based and Native Rust SSH engines.

## ⚖️ License

Released under the MIT License.
