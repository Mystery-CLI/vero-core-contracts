#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};
use vero_core_contracts::VeroContractClient;

fn setup() -> (Env, Address, VeroContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, vero_core_contracts::VeroContract);
    let client = VeroContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    (env, admin, client)
}

#[test]
fn test_add_guardian_and_register_task() {
    let (_env, admin, client) = setup();
    let guardian = Address::generate(&_env);

    client.add_guardian(&admin, &guardian);
    client.register_task(&admin, &1u64).unwrap();

    let task = client.get_task(&1u64).unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.votes, 0);
    assert!(!task.is_done);
}

#[test]
fn test_three_votes_flips_is_done() {
    let (env, admin, client) = setup();

    let g1 = Address::generate(&env);
    let g2 = Address::generate(&env);
    let g3 = Address::generate(&env);

    client.add_guardian(&admin, &g1);
    client.add_guardian(&admin, &g2);
    client.add_guardian(&admin, &g3);
    client.register_task(&admin, &42u64).unwrap();

    client.vote(&g1, &42u64).unwrap();
    client.vote(&g2, &42u64).unwrap();
    client.vote(&g3, &42u64).unwrap();

    let task = client.get_task(&42u64).unwrap();
    assert_eq!(task.votes, 3);
    assert!(task.is_done);
}

#[test]
fn test_duplicate_vote_rejected() {
    let (env, admin, client) = setup();
    let g = Address::generate(&env);

    client.add_guardian(&admin, &g);
    client.register_task(&admin, &7u64).unwrap();
    client.vote(&g, &7u64).unwrap();

    let result = client.try_vote(&g, &7u64);
    assert!(result.is_err());
}

#[test]
fn test_non_guardian_vote_rejected() {
    let (env, admin, client) = setup();
    let stranger = Address::generate(&env);

    client.register_task(&admin, &99u64).unwrap();

    let result = client.try_vote(&stranger, &99u64);
    assert!(result.is_err());
}
