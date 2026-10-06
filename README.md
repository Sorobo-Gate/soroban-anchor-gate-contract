# SorobanAnchor Gate Contract

SorobanAnchor Gate is a decentralized milestone escrow smart contract for the Stellar network built using Soroban SDK. It enables non-custodial custody of Stellar Asset Contract (SAC) tokens, programmatic timelocked release to Stellar Anchor disbursement accounts, and automated refund capabilities for unreleased milestones.

---

## 1. System Architecture & Scope Boundaries

This repository contains the authoritative smart contract. To maintain clear architectural boundaries, features are categorized by implementation location:

- **IMPLEMENTED HERE (`soroban-anchor-gate-contract`):**
  - Core smart contract (`EscrowGate`) with state machine (`Funded`, `Disbursed`, `Refunded`).
  - Strict authorization controls (`admin.require_auth()`, `payer.require_auth()`).
  - Integer-safe fee calculation (capped at 1,000 bps / 10.00%) routed to protocol treasury.
  - Timelocked refund enforcement based on ledger timestamp.
  - Automatic TTL management for instance and persistent storage.
  - Standardized event emissions (`created`, `disbursed`, `refunded`).
  - Complete 21-test automated Cargo verification suite.

- **IMPLEMENTED IN RELATED REPOSITORY (`Sorobo-Gate/soroban-anchor-gate-app`):**
  - Web UI for milestone creation and Freighter wallet signing (`apps/web`).
  - TypeScript contract client SDK wrapping Soroban XDR (`packages/contract-client`).
  - Go event listener daemon polling Soroban RPC for `disbursed` events (`services/relay`).
  - Off-chain SEP-10 authentication and SEP-31 anchor payment dispatch.

- **PLANNED / BACKLOG:**
  - Multi-token administrative whitelist storage ([Issue #1](https://github.com/Sorobo-Gate/soroban-anchor-gate-contract/issues/1)).
  - Arbiter-mediated multi-party dispute resolution branch ([Issue #2](https://github.com/Sorobo-Gate/soroban-anchor-gate-contract/issues/2)).

---

## 2. Tested Contract Lifecycle

The contract implements a three-state deterministic lifecycle:

```text
               [ create_escrow() ]
                        |
                        v
               +-----------------+
               |     FUNDED      |
               +--------+--------+
                        |
       +----------------+----------------+
       |                                 |
[ release_to_anchor() ]         [ refund() ]
       |                                 |
       v                                 v
+--------------+                 +---------------+
|  DISBURSED   |                 |   REFUNDED    |
+--------------+                 +---------------+
```

1. **`init`**: Configures protocol admin, fee treasury, and fee bps (max 10.00%).
2. **`create_escrow`**: Payer locks tokens, binds a 32-byte recipient profile hash, and sets lock duration. State becomes `Funded`.
3. **`release_to_anchor`**: Payer or admin authorizes payout. Fee is transferred to treasury; net balance is transferred to anchor distribution address. State becomes `Disbursed`.
4. **`refund`**: Payer reclaims 100% of deposited tokens after lock duration expires without disbursement. State becomes `Refunded`.

---

## 3. Verified Stellar Testnet Deployment

The smart contract is deployed and verified live on Stellar Testnet:

| Parameter | Observed Value |
|---|---|
| **Network** | Stellar Testnet (`Test SDF Network ; September 2015`) |
| **Soroban RPC** | `https://soroban-testnet.stellar.org` |
| **Protocol Version** | 29 (Captive Core 29.0.0) |
| **Deployed Contract ID** | `CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT` |
| **WASM Hash** | `ba9eaef277a2943c5a48f38aea849c78acf8cd9306d71646225a15ea41965e32` |
| **Target Architecture** | `wasm32v1-none` (7,515 bytes optimized) |
| **Stellar Expert Link** | [Contract CBIHLECK... on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT) |

Empirical transaction hashes and event logs are detailed in [`evidence/testnet-2026-10-06.md`](evidence/testnet-2026-10-06.md).

---

## 4. Toolchain Prerequisites & Build Guide

### Prerequisites
- Rust stable (1.85+)
- WebAssembly target: `wasm32v1-none`
- Stellar CLI version `28.1.0` or higher

```bash
# Add WebAssembly target
rustup target add wasm32v1-none

# Verify toolchain versions
rustc --version
stellar --version
```

### Build Contract
Compile optimized WebAssembly bytecode:
```bash
stellar contract build
```
Built artifact is generated at:
`target/wasm32v1-none/release/soroban_anchor_escrow.wasm`

### Run Verification Checks & Tests
```bash
# Check formatting
cargo fmt --check

# Run compiler typecheck
cargo check

# Run linter
cargo clippy --all-targets -- -D warnings

# Execute test suite (21 unit and lifecycle tests)
cargo test
```

---

## 5. Documentation Directory

- [`docs/contract-spec.md`](docs/contract-spec.md): Exact function specifications, authorization requirements, and storage mappings.
- [`docs/app-parity.md`](docs/app-parity.md): Cross-repository function, type, event, and numeric width parity matrix.
- [`AUDIT/branch-state.md`](AUDIT/branch-state.md): Branching model audit and CI alignment.
- [`AUDIT/git-history-remediation.md`](AUDIT/git-history-remediation.md): Historical commit analysis and safety rule preservation report.
- [`evidence/index.md`](evidence/index.md): Empirical verification index and claim status matrix.
- [`evidence/testnet-2026-10-05.md`](evidence/testnet-2026-10-05.md): Live Testnet transaction logs and event traces.

---

## 6. Security & Vulnerability Reporting

The contract implements strict authorization guards, checks for integer overflow, and protects against reentrancy and duplicate state transitions. For details on threat boundaries, key custody, and reporting procedures, see [`SECURITY.md`](SECURITY.md).

Report vulnerabilities via [GitHub Security Advisories](https://github.com/Sorobo-Gate/soroban-anchor-gate-contract/security/advisories/new) or directly to `emintechsolutions@gmail.com`.

---

## 7. Contributing

Please review [`CONTRIBUTING.md`](CONTRIBUTING.md) for contribution guidelines, conventional commit standards, selective file staging rules, and PR requirements. All active pull requests must target the `develop` branch.

---

## 8. License

Distributed under the Apache 2.0 License. See [`LICENSE`](LICENSE) for details.