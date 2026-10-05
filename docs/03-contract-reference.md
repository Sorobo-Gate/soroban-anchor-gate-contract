# Smart Contract Reference

- **Network:** Stellar Testnet
- **Verified Contract ID:** `CD36A2JQEEQSBTKOE6T5PB3BPV7IGIYDSSOBOOK6NE4RSOWGNC2HXXDA`
- **Companion App Contract ID:** `CCCSLE7UN2FRLB2HQWEUEXM4365NDYH3QSC6J5TILQWBSTTIDKFWXX2Y`
- **Admin / Treasury Address:** `GAC6AIE7NVRD5FKLZZXLFNBZKCF4E5PETYC2O2MNHDP2CL5Z2C4KZUBW`
- **Default Fee:** 200 bps (2.0%)
- **Target Architecture:** `wasm32v1-none`

For the complete technical specification including storage data keys, exact error codes, and TTL configuration, refer to [`docs/contract-spec.md`](contract-spec.md).

---

## Core Functions
- `init(admin, treasury, fee_bps)`: Configures global instance storage.
- `create_escrow(payer, beneficiary, token, amount, profile_hash, lock_duration)`: Locks tokens and emits `created` event.
- `release_to_anchor(escrow_id, caller, anchor_disbursement_address)`: Deducts fee, dispatches net payout to anchor, and emits `disbursed` event.
- `refund(escrow_id)`: Unlocks tokens back to payer if lock duration elapsed.

---

## Public Functions

### `init`
Initializes contract instance configuration.

```rust
pub fn init(
    env: Env,
    admin: Address,
    treasury: Address,
    fee_bps: u32,
) -> Result<(), EscrowError>;
```

- **Authorization:** Requires `admin.require_auth()`.
- **Guards:** Rejects execution with `AlreadyInitialized` if already configured; enforces `fee_bps <= 1000` (10% ceiling).

---

### `create_escrow`
Transfers and locks SAC tokens from the caller into the contract address.

```rust
pub fn create_escrow(
    env: Env,
    payer: Address,
    beneficiary: Address,
    token: Address,
    amount: i128,
    profile_hash: BytesN<32>,
    lock_duration: u64,
) -> Result<u64, EscrowError>;
```

- **Authorization:** Requires `payer.require_auth()`.
- **Emits:** Event topic `(symbol_short!("created"), escrow_id)` with payload `(payer, amount, profile_hash)`.
- **Errors:** `ZeroAmount`, `NotInitialized`, `InvalidStatus`.

---

### `release_to_anchor`
Deducts the protocol fee, transfers net funds to the anchor's distribution account, and transitions status to Disbursed.

```rust
pub fn release_to_anchor(
    env: Env,
    escrow_id: u64,
    caller: Address,
    anchor_disbursement_address: Address,
) -> Result<(), EscrowError>;
```

- **Authorization:** Requires `caller.require_auth()`. `caller` must be either payer or admin.
- **Emits:** Event topic `(Symbol::new(&env, "disbursed"), escrow_id)` with payload `(profile_hash, payout_amount)`.
- **Errors:** `EscrowNotFound`, `Unauthorized`, `InvalidStatus`.

---

### `refund`
Returns 100% of the locked tokens to the payer if the lock duration has expired.

```rust
pub fn refund(env: Env, escrow_id: u64) -> Result<(), EscrowError>;
```

- **Authorization:** Requires `record.payer.require_auth()`.
- **Emits:** Event topic `(Symbol::new(&env, "refunded"), escrow_id)` with payload `amount`.
- **Errors:** `EscrowNotFound`, `InvalidStatus`, `UnlockTimeNotReached`.