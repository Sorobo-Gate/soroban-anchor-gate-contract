# Security Policy: `soroban-anchor-gate-contract`

## 1. Threat Boundary & System Overview

`EscrowGate` is an onchain milestone escrow smart contract for the Stellar network built with the Soroban SDK.

### What the Smart Contract Protects:
- **Custody of Locked Tokens:** Stellar Asset Contract (SAC) tokens transferred into the contract address cannot be withdrawn by unauthorized third parties.
- **Deterministic State Lifecycle:** Escrow state transitions strictly follow `Funded -> Disbursed` or `Funded -> Refunded`. Terminal states are immutable and prevent duplicate release or duplicate refund.
- **Protocol Fee Cap:** Fee basis points cannot exceed 1,000 bps (10.00%). Fee math utilizes checked arithmetic to prevent integer overflow.
- **Timelock Guarantees:** Refunds are mathematically prevented from execution prior to the expiration of the `unlock_timestamp` ledger time.

### Outside the Smart Contract Threat Boundary:
- **Off-chain Banking Rails:** The contract emits cryptographic events (`disbursed`) containing a 32-byte hash commitment (`profile_hash`). Off-chain fiat wire execution, SEP-10 authentication, and SEP-31 compliance are the sole responsibility of the relay daemon and partner Stellar Anchors.
- **Anchor Solvency:** The contract cannot enforce that an off-chain Anchor fulfills fiat wire transfers once funds are released to the anchor's distribution address.

---

## 2. Authorization Model

Every state-mutating operation enforces strict cryptographic authorization:

| Contract Function | Enforced Authorization | Permitted Actors | Access Control Defense |
|---|---|---|---|
| `init` | `admin.require_auth()` | Protocol Administrator | Can only be called once; guarded by `DataKey::Admin` existence check. |
| `create_escrow` | `payer.require_auth()` | Depositing Account | Tokens are debited directly from caller's account via SAC transfer. |
| `release_to_anchor` | `caller.require_auth()` | Original Payer OR Contract Admin | Rejects any third-party caller with `EscrowError::Unauthorized`. |
| `refund` | `record.payer.require_auth()` | Original Payer | Rejects any other caller; enforces `ledger.timestamp >= unlock_timestamp`. |

---

## 3. Key Custody & Storage Lifecycle

- **Non-Custodial Architecture:** Users sign all transactions directly through supported Stellar wallet providers (e.g. Freighter). No private keys or secret seeds are ever held or accessed by the protocol.
- **Instance & Persistent TTL Management:** The contract automatically extends the TTL of both its instance configuration (`DataKey::Admin`, `Treasury`, `FeeBps`, `EscrowCounter`) and persistent escrow records (`DataKey::Escrow(id)`) to prevent unintended state expiration on the Stellar network.

---

## 4. Supported Versions

| Version Range | Network | Status | Security Maintenance |
|---|---|---|---|
| `0.1.x` | Stellar Testnet | Active MVP | Supported |
| `< 0.1.0` | N/A | Obsolete | Unsupported |

---

## 5. Audit & Verification Status

- **Formal Audit Disclaimer:** This smart contract has **not** undergone a third-party smart contract security audit. Do not deploy to Stellar Mainnet with real capital without an independent third-party audit.
- **Empirical Testing:** 21 automated unit, authorization, and lifecycle integration tests pass with 100% success rate in `src/test.rs`.
- **Live Testnet Verification:** Full lifecycle transactions (installation, deployment, initialization, creation, fee distribution, disbursement, timelock expiry, and refund) verified empirically on Stellar Testnet for contract `CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT` (see [`evidence/testnet-2026-10-06.md`](evidence/testnet-2026-10-06.md)).
- **Automated Dependency Auditing:** Dependabot scans `cargo` dependencies and `github-actions` weekly.

---

## 6. Known Security Limitations & Backlog

1. **Unrestricted Token Acceptance (Issue #1):** Currently, any SAC token address can be specified during `create_escrow`. A multi-token administrative whitelist is tracked on the backlog.
2. **Binary Settlement (Issue #2):** Escrows resolve entirely to disbursement or entirely to refund. Fractional dispute resolution with an arbiter role is tracked on the backlog.
3. **Reserved Error Code:** Error code `UnlockTimePassed` (7) is reserved for future time-bounded dispute resolution and is not raised in the MVP flow.

---

## 7. Reporting a Vulnerability

If you discover a potential vulnerability, please report it responsibly:

- **Primary Reporting Channel:** [GitHub Private Security Advisory](https://github.com/Sorobo-Gate/soroban-anchor-gate-contract/security/advisories/new)
- **Direct Maintainer Security Contact:** `emintechsolutions@gmail.com`
- **Notice on Deprecated Channels:** The previous unverified domain `security@sorobananchorgate.io` and generic Telegram references have been permanently removed as unverified channels.

Please include:
1. Target commit SHA or contract ID.
2. Reproduction steps and proof-of-concept code.
3. Expected impact.

Please do **not** disclose security vulnerabilities publicly via open GitHub issues or pull requests until a coordinated fix is released.
