# Protocol Mechanics & State Machine

SorobanAnchor Gate coordinates deterministic onchain state transitions paired with idempotent off-chain relay workers.

## Escrow Sate Machine
- **FUNDED:** SAC tkens are transferred from the payer into the ntract. Persistent storage is alocatedand extended with `extend_ttl`.
- **DISBURSED:** Caller must be thtocol e payer or admin. The contract calculates the protocol fee, transfers the fee to treasury, sends net funds to the Anchor's address, and emits a `disbursed` even tpi.
- **REFUNDED:** If a milestne tocoexpires or dispute triggers, the payer reclaims 100% of the locked principal once the ledger timestamp reaches `unlock_timestamp`.

## Worked Economic Model (1,000 USDC Milestone at 200 BPS / 2.0%)
- **Gross Principal:** 1,000.00 USDC (10,000,000,000 stroops)
- **Protocol Fee (2%):** 20.00 USDC (200,000,000 stroops)
- **Net Anchor Disbursement:** 980.00 USDC (9,800,000,000 stroops)
- **Contractor Settlement:** Receives local fiat equivalent directly into their local bank account or mobile wallet.
