# Protocol Verification & Evidence Index

This document maps all protocol features, architectural assertions, and external claims in the `Sorobo-Gate/soroban-anchor-gate-contract` repository to empirical verification artifacts.

---

## 1. Claim Verification Matrix

| Claim | Evidence Type | Source | Date Checked | Status |
|---|---|---|---|---|
| **Contract Initialization (`init`)** | Live Testnet Transaction | [`evidence/testnet-2026-10-05.md`](testnet-2026-10-05.md) (Tx `52d45fb1...`) | 2026-10-05 | `VERIFIED` |
| **Duplicate Init Defense** | Live Testnet Rejection | [`evidence/testnet-2026-10-05.md`](testnet-2026-10-05.md) (`Error(Contract, #2)`) | 2026-10-05 | `VERIFIED` |
| **Escrow Creation & Token Inflow** | Live Testnet Transaction | [`evidence/testnet-2026-10-05.md`](testnet-2026-10-05.md) (Tx `a2de3a38...`) | 2026-10-05 | `VERIFIED` |
| **Anchor Disbursement & Fee Split** | Live Testnet Transaction | [`evidence/testnet-2026-10-05.md`](testnet-2026-10-05.md) (Tx `28a4bb40...`) | 2026-10-05 | `VERIFIED` |
| **Timelock Expiry & Refund** | Live Testnet Transaction | [`evidence/testnet-2026-10-05.md`](testnet-2026-10-05.md) (Tx `c59866c8...`) | 2026-10-05 | `VERIFIED` |
| **Comprehensive Test Matrix (20 tests)** | Automated Cargo Suite | `src/test.rs` (100% pass rate) | 2026-10-05 | `TESTED LOCALLY` |
| **WASM Optimization & Target Build** | Stellar CLI 28.1.0 Build | `target/wasm32v1-none/release/*.wasm` (7,476 B) | 2026-10-05 | `VERIFIED` |
| **Strict Compiler & Linter Cleanliness** | Clippy & Rustfmt Checks | `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` | 2026-10-05 | `VERIFIED` |
| **Event Symbol Parity with Relay (`disbursed`)** | Source & Cross-Repo Audit | [`docs/app-parity.md`](../docs/app-parity.md), `subscriber.go` | 2026-10-05 | `VERIFIED` |
| **Arbitrary-Precision Payout Math (i128)** | Source & Test Verification | `src/lib.rs` (checked math), `subscriber.go` (`*big.Int`) | 2026-10-05 | `VERIFIED` |
| **Instance Storage TTL Auto-Extension** | Soroban SDK Storage TTL | `src/lib.rs` (`extend_ttl` on instance storage) | 2026-10-05 | `TESTED LOCALLY` |
| **Multi-Token Whitelist Storage** | Backlog Issue #1 | Repository Issue #1 | 2026-10-05 | `KNOWN LIMITATION` |
| **Arbiter Dispute Resolution Branch** | Backlog Issue #2 | Repository Issue #2 | 2026-10-05 | `KNOWN LIMITATION` |
| **Automated SEP-31 Off-Ramp Gateway** | Related App Monorepo | `Sorobo-Gate/soroban-anchor-gate-app` | 2026-10-05 | `KNOWN LIMITATION` |

---

## 2. Status Definitions

- **`VERIFIED`**: Validated empirically with live onchain Stellar Testnet transactions or direct compiler/linter execution.
- **`TESTED LOCALLY`**: Validated through deterministic unit and integration test suites in the local sandbox.
- **`LOGICALLY COVERED`**: Code path implemented and reviewed; reliant on future external protocol integrations.
- **`UNVERIFIED`**: Stated in design documentation but lacks empirical proof.
- **`KNOWN LIMITATION`**: Intentional design boundary or non-MVP feature documented in issue tracker.
- **`BLOCKED`**: Dependent on external tooling or upstream dependencies currently unavailable.
