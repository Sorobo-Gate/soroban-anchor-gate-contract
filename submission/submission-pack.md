# Submission Pack: SorobanAnchor Gate Contract

**Submission Package Date:** 2026-10-05
**Project:** SorobanAnchor Gate
**Authoritative Contract Repository:** [https://github.com/Sorobo-Gate/soroban-anchor-gate-contract](https://github.com/Sorobo-Gate/soroban-anchor-gate-contract)
**Companion Application Repository:** [https://github.com/Sorobo-Gate/soroban-anchor-gate-app](https://github.com/Sorobo-Gate/soroban-anchor-gate-app)

---

## 1. System Overview & Inter-Repository Architecture

SorobanAnchor Gate provides decentralized, non-custodial milestone escrow smart contracts on Stellar/Soroban coupled with off-chain event monitoring and Stellar Anchor payment rails.

- **`soroban-anchor-gate-contract` (This Repository):**
  - Contains the authoritative Soroban WebAssembly smart contract (`EscrowGate`).
  - Manages token custody, deterministic milestone lifecycles (`Funded -> Disbursed | Refunded`), protocol fee splitting, and timelocked refund enforcement.
  - Implements 21 comprehensive unit and lifecycle tests with 100% pass rate.
  - Compiles to `wasm32v1-none` using `stellar contract build`.

- **`soroban-anchor-gate-app` (Companion Repository):**
  - `apps/web`: Next.js frontend with Freighter wallet signing and milestone creation forms.
  - `packages/contract-client`: TypeScript SDK wrapping Soroban XDR operations.
  - `services/relay`: Go event listener daemon polling `disbursed` events and coordinating with SEP-31 Anchors.

---

## 2. Verified Stellar Testnet Deployment

| Parameter | Observed Deployment Coordinates |
|---|---|
| **Network** | Stellar Testnet (`Test SDF Network ; September 2015`) |
| **Soroban RPC Endpoint** | `https://soroban-testnet.stellar.org` |
| **Protocol Version** | 29 (Captive Core 29.0.0) |
| **Contract ID** | `CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT` |
| **WASM Hash** | `ba9eaef277a2943c5a48f38aea849c78acf8cd9306d71646225a15ea41965e32` |
| **Explorer Link** | [Stellar Expert Contract Explorer](https://stellar.expert/explorer/testnet/contract/CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT) |

---

## 3. Empirical Transaction Evidence Log

All transactions executed live on Stellar Testnet on 2026-10-06:

1. **WASM Installation:**
   `7ebefb3e922463f555177ffc384a9bab725ec11e4546e029689f2cb03ecdc604`
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/7ebefb3e922463f555177ffc384a9bab725ec11e4546e029689f2cb03ecdc604)
2. **Contract Instance Deployment:**
   `0000278938dfd9e78537a455dce5e27285e848f41be54f8d77e6a5aa2195ab2b`
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/0000278938dfd9e78537a455dce5e27285e848f41be54f8d77e6a5aa2195ab2b)
3. **Contract Initialization (`init`):**
   `1ee57cf0d72c32d51ee102a038144c6d395a5e38b31b5b4c8afc600cfaa7c079`
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/1ee57cf0d72c32d51ee102a038144c6d395a5e38b31b5b4c8afc600cfaa7c079)
4. **Escrow 1 Creation (`create_escrow`):**
   `e0b574823c2c8bd9e8b9e97ae561c69e536fc38b6f61a21553d1f6e27e75980d`
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/e0b574823c2c8bd9e8b9e97ae561c69e536fc38b6f61a21553d1f6e27e75980d)
5. **Escrow 1 Disbursement (`release_to_anchor`):**
   `0acc0aa60858bc579fb774ef4ea4788ec62c47db6c9ed4524f8b0cd77436dbeb`
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/0acc0aa60858bc579fb774ef4ea4788ec62c47db6c9ed4524f8b0cd77436dbeb)
6. **Escrow 2 Creation (`create_escrow`):**
   `b1e24c057057c8b8764ae1a59f6da482ee3e13affef4dfa955bbe17583b789da`
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/b1e24c057057c8b8764ae1a59f6da482ee3e13affef4dfa955bbe17583b789da)
7. **Escrow 2 Timelocked Refund (`refund`):**
   `9f1cf6f0f93db563de4b9d1a4daad34861c27ae8036486a525ba92b6df889f03`
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/9f1cf6f0f93db563de4b9d1a4daad34861c27ae8036486a525ba92b6df889f03)

---

## 4. Documentation Index

- [Smart Contract Specification (`docs/contract-spec.md`)](../docs/contract-spec.md)
- [App & Relay Parity Specification (`docs/app-parity.md`)](../docs/app-parity.md)
- [Branch Strategy Audit (`AUDIT/branch-state.md`)](../AUDIT/branch-state.md)
- [Git History Remediation Audit (`AUDIT/git-history-remediation.md`)](../AUDIT/git-history-remediation.md)
- [Evidence Index (`evidence/index.md`)](../evidence/index.md)
- [Stellar Testnet Verification Report (`evidence/testnet-2026-10-05.md`)](../evidence/testnet-2026-10-05.md)
- [Security Policy (`SECURITY.md`)](../SECURITY.md)
- [Contributing Guidelines (`CONTRIBUTING.md`)](../CONTRIBUTING.md)

---

## 5. Known Limitations & Backlog Roadmap

1. **Multi-Token Whitelisting ([Issue #1](https://github.com/Sorobo-Gate/soroban-anchor-gate-contract/issues/1)):**
   Current MVP allows any SAC token. Future release will introduce admin whitelist storage and validation.
2. **Arbiter Dispute Resolution ([Issue #2](https://github.com/Sorobo-Gate/soroban-anchor-gate-contract/issues/2)):**
   Current settlement is binary (100% release or 100% refund). Future release will implement third-party arbiter split resolution using basis points.
3. **Reserved Error Code:**
   `UnlockTimePassed` (7) is reserved for time-bounded dispute arbitration.
