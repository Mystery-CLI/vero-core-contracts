#![no_std]

mod guardian;
mod task;
mod types;
pub mod events;

use soroban_sdk::{contract, contractimpl, Address, Env};
use types::{ContractError, DataKey};

pub use guardian::{add_guardian, is_guardian};
pub use task::{get_task, register_task};

const VOTE_THRESHOLD: u32 = 3;

#[contract]
pub struct VeroContract;

#[contractimpl]
impl VeroContract {
    pub fn add_guardian(env: Env, admin: Address, guardian: Address) {
        guardian::add_guardian(&env, admin, guardian);
    }

    pub fn register_task(
        env: Env,
        admin: Address,
        task_id: u64,
    ) -> Result<(), ContractError> {
        task::register_task(&env, admin, task_id)
    }

    pub fn vote(env: Env, guardian: Address, task_id: u64) -> Result<(), ContractError> {
        guardian.require_auth();

        if !guardian::is_guardian(&env, &guardian) {
            return Err(ContractError::NotAuthorized);
        }

        let voted_key = DataKey::Voted(task_id, guardian.clone());
        if env.storage().instance().has(&voted_key) {
            return Err(ContractError::DuplicateVote);
        }
        env.storage().instance().set(&voted_key, &true);

        let task_key = DataKey::Task(task_id);
        let mut t: types::Task = env
            .storage()
            .instance()
            .get(&task_key)
            .ok_or(ContractError::NotAuthorized)?;

        t.votes += 1;
        if t.votes >= VOTE_THRESHOLD {
            t.is_done = true;
        }
        env.storage().instance().set(&task_key, &t);
        Ok(())
    }

    pub fn get_task(env: Env, task_id: u64) -> Option<types::Task> {
        task::get_task(&env, task_id)
    }
}
