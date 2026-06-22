# SSH-VPN Rust-Based High-Performance Core

An ultra-safe, blazingly fast network core rewritten in **Rust** to support asynchronous **SSH Tunneling**, **VPN encapsulation**, and **Split Tunneling** automation. This core is designed to be the foundation for modern VPN applications, providing 100% stability and zero-cost abstractions.

## 🚀 Architectural Advantages
- **Hybrid SSH Backend**: Supports both `libssh2` (via C bindings) and `russh` (native Rust) for maximum compatibility and safety.
- **Zero Garbage Collection**: Zero runtime pauses ensure consistent ping and throughput.
- **Memory Safety Guarantee**: Compile-time memory verification prevents Buffer Overflows and Data Races.
- **Native Async Stack**: Built using Tokio for massive connection scaling.
- **Split Tunneling**: Built-in support for OS-level routing configuration (Linux, macOS, Windows).
- **SlipNet Support**: Architecture designed to be compatible with DNS tunneling protocols like those found in [SlipNet](https://github.com/anonvector/SlipNet).

## 🛠️ Project Structure
- `src/main.rs`: Entry point and service orchestration.
- `src/ssh/`: SSH engines (LibSSH2 and Russh backends).
- `src/routing.rs`: OS-specific routing table management.
- `tests/`: Comprehensive integration and stability tests.

## 📦 Compilation and Setup

### Prerequisites
- Install Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/tahatehran/SSH-VPN.git
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
