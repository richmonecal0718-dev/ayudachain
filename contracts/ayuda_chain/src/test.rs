#![cfg(test)]

use super::{AyudaChain, AyudaChainClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

/// Sets up a fresh environment, deploys the contract, and initializes it with
/// an admin (representing the LGU / disaster relief agency).
fn setup() -> (Env, AyudaChainClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(AyudaChain, ());
    let client = AyudaChainClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    (env, client, admin)
}

#[test]
fn test_happy_path_full_relief_flow() {
    // Test 1 (Happy path): the MVP transaction executes successfully end-to-end:
    // verify household -> allocate voucher -> register merchant -> spend voucher.
    let (env, client, _admin) = setup();

    let household = Address::generate(&env);
    let merchant = Address::generate(&env);

    client.register_beneficiary(&household);
    client.register_merchant(&merchant);
    client.allocate_voucher(&household, &1000);
    client.spend_voucher(&household, &merchant, &400);

    assert_eq!(client.get_balance(&household), 600);
    assert_eq!(client.get_balance(&merchant), 400);
}

#[test]
#[should_panic(expected = "insufficient voucher balance")]
fn test_edge_case_insufficient_balance() {
    // Test 2 (Edge case): a household tries to spend more vouchers than it holds.
    let (env, client, _admin) = setup();

    let household = Address::generate(&env);
    let merchant = Address::generate(&env);

    client.register_beneficiary(&household);
    client.register_merchant(&merchant);
    client.allocate_voucher(&household, &100);

    // Attempting to spend more than the allocated balance must fail.
    client.spend_voucher(&household, &merchant, &500);
}

#[test]
fn test_state_verification_after_allocation() {
    // Test 3 (State verification): assert that contract storage reflects the
    // correct state after the MVP allocation transaction.
    let (env, client, _admin) = setup();

    let household = Address::generate(&env);

    client.register_beneficiary(&household);
    assert_eq!(client.is_beneficiary(&household), true);
    assert_eq!(client.get_balance(&household), 0);

    client.allocate_voucher(&household, &750);
    assert_eq!(client.get_balance(&household), 750);

    // A second allocation should accumulate on top of the existing balance.
    client.allocate_voucher(&household, &250);
    assert_eq!(client.get_balance(&household), 1000);
}

#[test]
#[should_panic(expected = "household is not a verified beneficiary")]
fn test_edge_case_allocation_to_unverified_household() {
    // Test 4 (Edge case): the admin tries to allocate vouchers to a household
    // that has never been verified (no "ghost beneficiary" allocations allowed).
    let (env, client, _admin) = setup();

    let unverified_household = Address::generate(&env);
    client.allocate_voucher(&unverified_household, &500);
}

#[test]
#[should_panic(expected = "merchant is not a registered partner")]
fn test_edge_case_spend_at_unregistered_merchant() {
    // Test 5 (Edge case): a verified household tries to spend vouchers at a
    // merchant that was never registered as a partner e-wallet.
    let (env, client, _admin) = setup();

    let household = Address::generate(&env);
    let unregistered_merchant = Address::generate(&env);

    client.register_beneficiary(&household);
    client.allocate_voucher(&household, &300);

    client.spend_voucher(&household, &unregistered_merchant, &100);
}