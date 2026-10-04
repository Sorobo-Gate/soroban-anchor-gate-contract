# SorobanAnchor Gate Documentation

SorobanAnchor Gate is an open-source decentralized middleware and milestone escrow protocol connecting Stellar's WebAssembly smart contracts (Soroban) with the Stellar Anchor Network (SEPs).

---

## The Real-World Problem

Cross-border commercial disbursements, freelance payouts, and vendor milestones suffer from systemic friction:
* Traditional wire mechanisms (SWIFT, intermediary correspondent banks) routinely levy 3%–7% in foreign exchange fees and require 2 to 5 business days for clearance.
* While decentralized escrow contracts enable trustless, programmatic custody in stablecoins (e.g., USDC), recipient vendors and contractors in emerging economies still face a steep off-ramp hurdle. They must manually manage private keys, withdraw to secondary crypto exchange platforms, complete separate KYC verification, and convert to fiat.
* Soroban contracts operate strictly in an isolated WebAssembly sandbox. Smart contracts cannot make external HTTP requests or communicate natively with off-chain regulated Stellar Anchors (SEP-24 / SEP-31).

SorobanAnchor Gate closes this gap by enabling onchain milestone completions to programmatically trigger automated fiat disbursements via non-custodial cryptographic relaying.

---

## System Overview

```text
+----------------+      +-------------------+      +----------------+      +---------------+
| Payer Client   | ---> | EscrowGate (WASM) | ---> | Go Event Relay | ---> | Stellar Anchor| ---> Local Bank /
| Locks USDC     |      | CCCSLE7...XX2Y    |      | Daemon         |      | (SEP-10/31)   |      Mobile Money
+----------------+      +-------------------+      +----------------+      +---------------+
```

1. **Escrow Funding:** The payer deposits SAC-compatible tokens into the EscrowGate contract.
2. **Identity Linkage:** The contractor’s destination banking rail is hashed into a 32-byte identifier (`profile_hash`) and pinned onchain.
3. **Milestone Authorization:** The client approves milestone release; the contract deducts the protocol fee and transfers the net stablecoin balance directly to the Anchor distribution address.
4. **Relay Verification:** An off-chain Go daemon monitors contract events, executes a cryptographic SEP-10 handshake, and dispatches the SEP-31 payment instruction.
5. **Fiat Settlement:** The Anchor executes the local fiat payout directly into the contractor's bank or mobile wallet.