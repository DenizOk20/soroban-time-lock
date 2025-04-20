#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, token, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]

pub enum Datakey {
    Init,
    Balance
}

#[derive(Clone)]
#[contracttype]

pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]

pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]

pub struct ClaimableBalanceContract;

fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();
    
    match time_bound.kind {
        TimeBoundKind::Before => {
            ledger_timestamp < time_bound.timestamp
        }
        TimeBoundKind::After => {
            ledger_timestamp > time_bound.timestamp
        }
    }
}

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }
        if claimants.is_empty() {
            panic!("Claimants list cannot be empty");
        }
        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }

        from.require_auth();
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(),&amount);

        env.storage().instance().set(&Datakey::Balance, &ClaimableBalance {
            token,
            amount,
            time_bound,
            claimants,
        });

        env.storage().instance().set(&Datakey::Init, &());
    }
    pub fn claim(env: Env, claimant: Address) {
        
        claimant.require_auth();
        
        let claimable_balance: ClaimableBalance =
            env.storage().instance().get(&Datakey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant) {
            panic!("claimant is not allowed to claim this balance");
        }

        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );
        // Remove the balance entry to prevent any further claims.
        env.storage().instance().remove(&Datakey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&Datakey::Init)
}

mod test;