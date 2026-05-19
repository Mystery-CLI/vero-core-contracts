use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub id: u64,
    pub votes: u32,
    pub is_done: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Guardian(Address),
    Task(u64),
    Voted(u64, Address), // (task_id, guardian)
    Admin,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ContractError {
    NotAuthorized = 1,
    DuplicateVote = 2,
}
