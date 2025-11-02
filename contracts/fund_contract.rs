// contracts/fund_contract.rs
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, Map};

#[contract]
pub struct FundContract;

#[contractimpl]
impl FundContract {
    pub fn new_project(env: Env, project_id: Symbol, recipient: Address, amount: i128) {
        env.storage().set(&project_id, &(recipient, amount, false));
    }

    pub fn release_funds(env: Env, project_id: Symbol) {
        let (recipient, amount, released): (Address, i128, bool) =
            env.storage().get(&project_id).unwrap();

        if !released {
            // Here we’d call Stellar payment logic in a real setup
            env.storage().set(&project_id, &(recipient, amount, true));
        }
    }
}
