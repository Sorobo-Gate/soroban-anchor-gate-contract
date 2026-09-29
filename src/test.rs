use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    token::Client as TokenClient,
    token::StellarAssetClient,
    Address, BytesN, Env,
};

#[test]
fn test_escrow_lifecycle_full() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let treasury = Address::generate(&env);
    let payer = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let anchor_addr = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_client = TokenClient::new(&env, &token_contract.address());
    let token_admin_client = StellarAssetClient::new(&env, &token_contract.address());

    token_admin_client.mint(&payer, &100_000);

    let contract_id = env.register(EscrowGate, ());
    let client = EscrowGateClient::new(&env, &contract_id);

    // Initialize with 200 bps (2%)
    client.init(&admin, &treasury, &200);

    let profile_hash = BytesN::from_array(&env, &[7u8; 32]);
    let escrow_id = client.create_escrow(
        &payer,
        &beneficiary,
        &token_contract.address(),
        &10_000,
        &profile_hash,
        &1000,
    );

    assert_eq!(escrow_id, 1);
    assert_eq!(token_client.balance(&contract_id), 10_000);

    // Release to anchor
    client.release_to_anchor(&escrow_id, &payer, &anchor_addr);

    // Verify 2% fee split
    assert_eq!(token_client.balance(&treasury), 200);
    assert_eq!(token_client.balance(&anchor_addr), 9_800);
    assert_eq!(token_client.balance(&contract_id), 0);
}
