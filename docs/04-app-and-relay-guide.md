# Architecture & Relayer Guide

This document outlines the operational integration between the smart contract and the companion application stack hosted in [`Sorobo-Gate/soroban-anchor-gate-app`](https://github.com/Sorobo-Gate/soroban-anchor-gate-app).

---

## 1. Companion Repository Structure

The off-chain services and client applications are maintained in `Sorobo-Gate/soroban-anchor-gate-app`:
- `apps/web`: Next.js web application for escrow management and Freighter wallet interaction.
- `packages/contract-client`: TypeScript SDK wrapping contract simulation and transaction assembly.
- `services/relay`: Go event listener daemon polling Soroban RPC for `disbursed` events and coordinating with SEP-31 Anchors.

---

## 2. Configuration Environment Variables

```env
STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
SOROBAN_RPC_URL="https://soroban-testnet.stellar.org"
NEXT_PUBLIC_ESCROW_CONTRACT_ID="CD36A2JQEEQSBTKOE6T5PB3BPV7IGIYDSSOBOOK6NE4RSOWGNC2HXXDA"
SOROBAN_CONTRACT_ID="CD36A2JQEEQSBTKOE6T5PB3BPV7IGIYDSSOBOOK6NE4RSOWGNC2HXXDA"
RELAY_SIGNER_SECRET="S..."
TARGET_ANCHOR_DOMAIN="testanchor.stellar.org"
PORT=8080
```

---

## 3. Go Relay Daemon Operations

The Go relay service monitors the Soroban RPC endpoint for `disbursed` events:
1. **Event Polling:** Ticker-based polling (4-second interval) filtering for contract ID and `disbursed` topic.
2. **Numeric Safety:** Decodes `amount` into `*big.Int` to safely support full `i128` values without precision loss.
3. **Idempotency Store:** Tracks processed `event_id` records in memory to prevent duplicate off-chain payments.
4. **Anchor Dispatch:** Matches the 32-byte `profile_hash` against off-chain KYC/banking records and submits the payment to the local anchor.

For complete cross-repository contract parity details, see [`docs/app-parity.md`](app-parity.md).
