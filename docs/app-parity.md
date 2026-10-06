# Cross-Repository App & Relay Parity Specification

**Audit Date:** 2026-10-05  
**Contract Repository:** `Sorobo-Gate/soroban-anchor-gate-contract`  
**Application Repository:** `Sorobo-Gate/soroban-anchor-gate-app` (`apps/web`, `packages/contract-client`, `services/relay`)

---

## 1. Cross-Repository Parity Matrix

| Concept | Contract Representation | Application Representation | Parity Match? | Architectural Resolution & Fix |
|---|---|---|---|---|
| **Contract Initialization** | `init(admin: Address, treasury: Address, fee_bps: u32)` | `buildInitTx(admin: string, anchorAddress: string)` | ❌ **MISMATCH** | Contract requires 3 arguments (`admin`, `treasury`, `fee_bps`). The TS client in `packages/contract-client` currently constructs only 2 arguments. **Resolution:** Contract is authoritative. TS client must be updated to `buildInitTx(admin, treasury, feeBps)`. |
| **Escrow Creation Signature** | `create_escrow(payer, beneficiary, token, amount: i128, profile_hash: BytesN<32>, lock_duration: u64)` | `buildCreateEscrowTx({ payer, beneficiary, token, amount: bigint, profileHashHex, lockDurationSeconds: bigint })` | ✅ **MATCH** | Exact argument ordering and types. Profile hash converted to raw 32 bytes via `xdr.ScVal.scvBytes`. |
| **Release to Anchor Signature** | `release_to_anchor(escrow_id: u64, caller: Address, anchor_disbursement_address: Address)` | `buildReleaseToAnchorTx({ escrowId: bigint, caller: string, anchorDisbursementAddress: string })` | ✅ **MATCH** | Exact argument ordering and type mapping (`u64`, `Address`, `Address`). |
| **Refund Signature** | `refund(escrow_id: u64)` | `buildRefundTx(escrowId: bigint)` | ✅ **MATCH** | Single `u64` parameter. |
| **State Variants** | `EscrowStatus::Funded`<br>`EscrowStatus::Disbursed`<br>`EscrowStatus::Refunded` | `EscrowContractState`<br>• `Funded`<br>• `Disbursed`<br>• `Refunded` | ✅ **MATCH** | 3-state deterministic lifecycle. Prior references to non-existent `Settled` variant were corrected in app UI. |
| **Event Name & Topic** | `Symbol::new(&env, "disbursed")`<br>Emitted as `("disbursed", escrow_id: u64)` | Poller topic filter: `"disbursed"`<br>Base64 symbol: `AAAADwAAAAFkaXNidXJzZWQ=` | ✅ **MATCH** | Historical mismatch: Go relay previously logged polling for `DisbursementAuthorized`. Contract emits `"disbursed"`. Relay subscriber has been aligned to decode `"disbursed"`. |
| **Event Payload Structure** | `(record.profile_hash: BytesN<32>, payout_amount: i128)` | `EventPayload`: `ProfileHashHex: string` (64 chars), `PayoutAmount: *big.Int` | ✅ **MATCH** | Profile hash is 32-byte hex; payout amount parsed as arbitrary-precision big integer. |
| **Numeric Domain: Payout Amount** | `i128` (signed 128-bit integer) | Go: `*big.Int`<br>TS SDK: `bigint` | ✅ **MATCH** | **Critical Parity Rule:** Go relay previously scaffolded `int64` for `PayoutAmount`, which caused silent integer truncation on amounts exceeding $9.22 \times 10^{18}$ base units. Aligned to `*big.Int` in `subscriber.go`. |
| **Numeric Domain: Escrow ID** | `u64` (unsigned 64-bit integer) | Go: `uint64`<br>TS SDK: `bigint` | ✅ **MATCH** | Deterministic sequential counter representation. |
| **Profile Commitment Format** | `BytesN<32>` (raw 32-byte array) | Client: SHA-256 digest of banking coordinates (64 hex characters) | ✅ **MATCH** | Strictly validated with `/^[0-9a-fA-F]{64}$/`. Raw bytes submitted onchain. |
| **Stellar Network Coordinates** | Testnet | Testnet passphrase:<br>`Test SDF Network ; September 2015`<br>RPC: `https://soroban-testnet.stellar.org` | ✅ **MATCH** | Identical network passphrase and RPC URLs used across web, relay, and contract tests. |
| **Contract ID Environment Name** | Authoritative Testnet ID:<br>`CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT` | Web: `NEXT_PUBLIC_ESCROW_CONTRACT_ID`<br>Relay: `SOROBAN_CONTRACT_ID` | ⚠️ **UPDATE REQUIRED IN APP** | The verified deployment is `CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT`. The companion app repo currently references a stale ID and must be updated in the app workflow. |
| **Fee Semantics** | In basis points (`u32`, 0..=1000 bps). Deducted onchain from principal in `release_to_anchor`. | Client displays net disbursement; off-chain relay passes net payout to SEP-31 anchor. | ✅ **MATCH** | Onchain arithmetic is atomic; no client-side fee manipulation possible. |
| **Anchor Disbursement Address** | Passed as `Address` parameter to `release_to_anchor`. | Selected by payer/admin from active Anchor deposit coordinates. | ✅ **MATCH** | Contract verifies payer/admin auth before transferring tokens to destination anchor. |

---

## 2. Event Contract Resolution Detail

### The Historical Discrepancy:
- The initial relay design specification specified listening for `DisbursementAuthorized`.
- The contract implementation in `src/lib.rs` published:
  ```rust
  env.events().publish(
      (Symbol::new(&env, "disbursed"), escrow_id),
      (record.profile_hash, payout_amount),
  );
  ```
- Rather than altering contract bytecode and invalidating deployed Testnet verification, the Go relay subscriber in `services/relay/internal/listener/subscriber.go` was updated to decode `"disbursed"` events.

### The Decoded Event Contract:
- **Topic 0:** `Symbol("disbursed")` (or base64 `AAAADwAAAAFkaXNidXJzZWQ=`)
- **Topic 1:** `escrow_id` (`u64`)
- **Data (Payload):** Tuple `(profile_hash: BytesN<32>, payout_amount: i128)`
- **Subscriber Handling:**
  1. `DecodeDisbursedEvent` validates topic symbol.
  2. Extracts `escrow_id` as `uint64`.
  3. Decodes `payout_amount` directly into `*big.Int` (preventing `int64` truncation).
  4. Hex-encodes `profile_hash` into a 64-character lowercase hex string.
  5. Verifies idempotency against `IdempotencyStore` before dispatching.

---

## 3. Authoritative Application Handoff Coordinates

For synchronization with the companion application repository (`Sorobo-Gate/soroban-anchor-gate-app`):

| Parameter | Authoritative Value |
|---|---|
| **Contract ID** | `CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT` |
| **Network** | Stellar Testnet |
| **Network Passphrase** | `Test SDF Network ; September 2015` |
| **Soroban RPC URL** | `https://soroban-testnet.stellar.org` |
| **WASM Hash** | `ba9eaef277a2943c5a48f38aea849c78acf8cd9306d71646225a15ea41965e32` |
| **Verification Token** | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` (Native XLM SAC) |
| **State Enum** | `Funded` (0), `Disbursed` (1), `Refunded` (2) |
| **Event Topics** | `("created", escrow_id: u64)`, `("disbursed", escrow_id: u64)`, `("refunded", escrow_id: u64)` |
| **Contract Functions** | • `init(admin: Address, treasury: Address, fee_bps: u32) -> Result<(), EscrowError>`<br>• `create_escrow(payer: Address, beneficiary: Address, token: Address, amount: i128, profile_hash: BytesN<32>, lock_duration: u64) -> Result<u64, EscrowError>`<br>• `release_to_anchor(escrow_id: u64, caller: Address, anchor_disbursement_address: Address) -> Result<(), EscrowError>`<br>• `refund(escrow_id: u64) -> Result<(), EscrowError>` |
