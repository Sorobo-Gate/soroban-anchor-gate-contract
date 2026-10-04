# Smart Contract Reference

- **Network:** Stellar Testnet
- **Contract ID:** `CCCSLE7UN2FRLB2HQWEUEXM4365NDYH3QSC6J5TILQWBSTTIDKFWXX2Y`
- **Admin / Treasury Address:** `GAYVUOIPXTTDJSJOQX4FS5ASFLDN5FVFLFWG2G46BROM2D2545TLQAPI`
- **Default Fee:** 200 bps (2.0%)
- **Target:** `wasm32-unknown-unknown`

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
- **Emits:** Event topic `(Symbol::new("created"), escrow_id)` with payload `(payer, amount, profile_hash)`.
- **Errors:** `ZeroAmount`, `NotInitialized`.

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
- **Emits:** Event topic `(Symbol::new("disbursed"), escrow_id)` with payload `(profile_hash, payout_amount)`.
- **Errors:** `EscrowNotFound`, `Unauthorized`, `InvalidStatus`.

---

### `refund`
Returns 100% of the locked tokens to the payer if the lock duration has expired.

```rust
pub fn refund(env: Env, escrow_id: u64) -> Result<(), EscrowError>;
```

- **Authorization:** Requires `record.payer.require_auth()`.
- **Emits:** Event topic `(Symbol::new("refunded"), escrow_id)` with payload `amount`.
- **Errors:** `EscrowNotFound`, `InvalidStatus`, `UnlockTimeNotReached`.