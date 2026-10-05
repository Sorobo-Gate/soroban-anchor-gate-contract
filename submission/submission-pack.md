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
| **Contract ID** | `CD36A2JQEEQSBTKOE6T5PB3BPV7IGIYDSSOBOOK6NE4RSOWGNC2HXXDA` |
| **WASM Hash** | `fdec17f890b77468c542ba8d8d6d9bcacddaa576b304f5e8cce607296bbe9a3d` |
| **Explorer Link** | [Stellar Expert Contract Explorer](https://stellar.expert/explorer/testnet/contract/CD36A2JQEEQSBTKOE6T5PB3BPV7IGIYDSSOBOOK6NE4RSOWGNC2HXXDA) |

---

## 3. Empirical Transaction Evidence Log

All transactions executed live on Stellar Testnet on 2026-10-05:

1. **WASM Installation:**  
   `7767bb85389218f85c4a0444d26df5e7744ec5d55ed526c1c0ee2757d51f9895`  
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/7767bb85389218f85c4a0444d26df5e7744ec5d55ed526c1c0ee2757d51f9895)
2. **Contract Instance Deployment:**  
   `0e0d6173f77cbcc71806f459b109c34f65a9fc93833432e38cfcddad01bce350`  
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/0e0d6173f77cbcc71806f459b109c34f65a9fc93833432e38cfcddad01bce350)
3. **Contract Initialization (`init`):**  
   `52d45fb1d428dc21cb950683fe36f7577c229df985a9dec31b96c3ce0de6cb78`  
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/52d45fb1d428dc21cb950683fe36f7577c229df985a9dec31b96c3ce0de6cb78)
4. **Escrow 1 Creation (`create_escrow`):**  
   `a2de3a3810ee6f0ea9e2b9709c6a733b428f38f335d5ba2fa2df11bc5994be80`  
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/a2de3a3810ee6f0ea9e2b9709c6a733b428f38f335d5ba2fa2df11bc5994be80)
5. **Escrow 1 Disbursement (`release_to_anchor`):**  
   `28a4bb40726fb8628450d94ab84ff678104a99e5628ec013c9da2287209e7979`  
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/28a4bb40726fb8628450d94ab84ff678104a99e5628ec013c9da2287209e7979)
6. **Escrow 2 Creation (`create_escrow`):**  
   `3f7d23fafe024d10a645f176e6720abacfcd2ffa4414c4a2716ed9c40fa7a2f8`  
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/3f7d23fafe024d10a645f176e6720abacfcd2ffa4414c4a2716ed9c40fa7a2f8)
7. **Escrow 2 Timelocked Refund (`refund`):**  
   `c59866c8998313a01425cadc602e6861763073d0197ea909e6a6884fe5a4be59`  
   [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/c59866c8998313a01425cadc602e6861763073d0197ea909e6a6884fe5a4be59)

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
