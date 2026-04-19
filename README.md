# 🛰️ Rust-LAN-Navigator (RLN) v2.0

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](./Dockerfile)

**RLN** is a privacy-first, zero-trust network orchestrator designed for the modern LAN. It provides instant visibility, cryptographic identity, and secure data movement without ever leaving your local subnet. No cloud, no telemetry, just pure local control.

---

## ✨ Key Features

- **🔍 Hybrid Discovery**: Simultaneous Layer 2 (ARP/NDP) and Layer 3 (mDNS) scanning for full IPv4 and IPv6 coverage.
- **🏷️ Vendor Identification**: Automatic device vendor lookup via the bundled OUI database — no "Unknown" labels.
- **📈 Stateful Monitoring**: Track network "drift" by comparing current scans against historical SQLite snapshots.
- **🛡️ Zero-Trust Identity**: Permanent, cryptographic Peer IDs (Ed25519) powered by `iroh`'s built-in key primitives.
- **🧠 Intelligent Fingerprinting**: Local ML models (via `tract`) identify device types based on network behavior.
- **🚀 High-Speed Streaming**: Encrypted, resilient file movement using QUIC-based P2P streaming (via `iroh` v0.98).
- **📊 Modern TUI**: A rich, interactive terminal dashboard built with `ratatui`.

---

## 🛠️ Technical Stack

- **Runtime**: [Tokio](https://tokio.rs/) (Async I/O)
- **Networking**: [pnet](https://github.com/libpnet/libpnet), [async-arp](https://github.com/skullim/async-arp), [surge-ping](https://github.com/pariahprologue/surge-ping)
- **Vendor Lookup**: [mac_oui](https://crates.io/crates/mac_oui) (bundled Wireshark OUI database)
- **P2P/Transfer**: [Iroh v0.98](https://iroh.computer/) / QUIC — latest stable release
- **Identity**: Built-in to `iroh` (`iroh::SecretKey` / `iroh::PublicKey`)
- **Intelligence**: [Tract](https://github.com/sonos/tract) (ONNX Inference)
- **Storage**: SQLite via [rusqlite](https://github.com/rusqlite/rusqlite)
- **UI**: [Ratatui](https://ratatui.rs/) & [Clap v4](https://clap.rs/)

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: Latest stable version.
- **Privileges**: RLN requires low-level network access.
  - **Linux**: `CAP_NET_RAW` capabilities or `sudo`.
  - **Windows**: Administrator privileges (Npcap recommended).

### Local Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/rln.git
   cd rln
   ```

2. **Build and Run**:
   ```bash
   cargo run --release -- --dashboard
   ```

### Running with Docker (Recommended)

RLN is fully containerized with host networking support for seamless LAN scanning.

```bash
docker-compose up --build
```

---

## 📂 Project Structure

```text
src/
├── discovery/      # ARP/NDP and mDNS scanning engines + OUI vendor lookup
├── intelligence/   # ICMP probes, ML fingerprinting, and topology mapping
├── identity/       # Iroh-native Ed25519 key management and Peer ID
├── transfer/       # Iroh QUIC-based P2P streaming and SHA-256 verification
├── storage/        # SQLite snapshot engine and drift detection logic
└── tui/            # Ratatui event loop and dashboard views
```

---

## 🔑 Identity & Peer IDs

RLN uses `iroh`'s native Ed25519 key primitives (`iroh::SecretKey` / `iroh::PublicKey`) to generate a unique node identity on first boot. The identity is saved to `data/identity.key` (chmod 600).

In the TUI, your node's `EndpointId` is displayed in the System Logs at startup. Other nodes use this ID to dial you directly for file transfers:

```
[ID] Peer ID: b09ceb10b3ca3e0aca54dace1998ba495911c5780dcd69b7307fb6bbd945504d
```

To send a file, press `s` and type:
```
<peer_endpoint_id> <filepath>
```

---

## 🛡️ Security & Privacy

- **Zero-Cloud Guarantee**: All data stays on your machine. AI analysis and database storage happen 100% locally.
- **Privilege Separation**: Scanning components are isolated to minimize the surface area requiring elevated permissions.
- **Immutable Identity**: Your Peer ID is your source of truth. If you lose your identity file, you lose your trusted status with other nodes.
- **SHA-256 Integrity**: Every P2P file transfer includes a streaming SHA-256 digest verified by the receiver before the file is accepted.

---

## ⚖️ License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
