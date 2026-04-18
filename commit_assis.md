### Phase 1: Foundation & Telemetry
| Milestone | Recommended Commit Message |
| :--- | :--- |
| **1.1** | `feat(storage): initialize sqlite schema and snapshot CRUD operations` |
| **1.2** | `feat(discovery): implement L2 ARP/NDP and L3 mDNS scanners with privilege checks` |
| **1.3** | `feat(discovery): add drift detection logic and L2 packet mocks for testing` |

### Phase 2: Identity & Secure Comms
| Milestone | Recommended Commit Message |
| :--- | :--- |
| **2.1** | `feat(identity): implement Ed25519 key generation and secure local storage` |
| **2.2** | `feat(transfer): initialize P2P listening and custom TLS handshake for Peer IDs` |

### Phase 3: Visualization & TUI
| Milestone | Recommended Commit Message |
| :--- | :--- |
| **3.1** | `feat(tui): set up async event loop with tokio channels for background updates` |
| **3.2** | `feat(tui): implement dashboard views and LLDP topology visualization` |

### Phase 4: Intelligence & Data Movement
| Milestone | Recommended Commit Message |
| :--- | :--- |
| **4.1** | `feat(intelligence): add ONNX-based ML device fingerprinting via ort/tract` |
| **4.2** | `feat(transfer): implement verified file streaming with SHA-256 and progress tracking` |

### Phase 5: Hardening & Release
| Milestone | Recommended Commit Message |
| :--- | :--- |
| **5.1** | `refactor(security): implement privilege separation for scanning threads` |
| **5.2** | `fix(platform): add graceful fallbacks for Windows without Npcap` |

---

### 💡 Pro-Tips for your RLN Commits:
- **Scope your commits**: Use the module names in the parentheses (e.g., `discovery`, `tui`, `identity`) to show exactly what part of the system changed.
- **Breaking Changes**: If a change requires a configuration reset (like changing the Peer ID format), use a `!` (e.g., `feat(identity)!: migration to new key format`).
- **Initial Commit**: Since I've already initialized the skeleton for you, your very first commit might look like:
  `chore: initialize project structure and dependencies for v2.0`