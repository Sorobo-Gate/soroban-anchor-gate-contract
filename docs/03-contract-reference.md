# Smart Contract Reference

- **Network:** Stellar Testnet
- **Contract ID:** `CCCSLE7UN2FRLB2HQWEUEXM4365NDYH3QSC6J5TILQWBSTTIDKFWXX2Y`
- **Admin / Treasury Address:** `GAYVUOIPXTTDJSJOQX4FS5ASFLDN5FVFLFWG2G46BROM2D2545TLQAPI`
- **Default Fee:** 200 bps (2.0%)
- **Taget:** `wasm32-unknown-unknown`

## rCore Functions
- `init(admin, treasury, fee_bps)`: Configures global instance storage.
- `create_escrow(payer, beneficiary, token, amount, profile_hash, lock_duration)`: Locks tokens and emits `created` event.
- `release_to_anchor(escrow_id, caller, anchor_disbursement_address)`: Deducts fee, dispatches net payout to anchor, and emits `disbursed` event.
- `refund(escrow_id)`: Unlocks tokens back to payer if lock duration elapsed.
