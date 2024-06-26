use soroban_sdk::{contracttype, symbol_short, Address, Env, IntoVal, String, Symbol, Vec};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

use crate::storage::types::Component;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Redeem {
    spender: Address,
    from: Address,
    amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Initialize {
    addresses: Vec<Address>,
    units: Vec<i128>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetManager {
    old_manager: Address,
    new_manager: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetRegistry {
    registry: Address,
}

pub(crate) fn redeem(e: &Env, spender: Address, from: Address, amount: i128) {
    let topics = (Symbol::new(e, "redeem"),);
    e.events().publish(
        topics,
        Redeem {
            spender,
            from,
            amount,
        },
    );
}

pub(crate) fn set_manager(e: &Env, old_manager: Address, new_manager: Address) {
    let topics = (Symbol::new(e, "set_manager"),);
    e.events().publish(
        topics,
        SetManager {
            old_manager,
            new_manager,
        },
    );
}

pub(crate) fn set_registry(e: &Env, registry: Address) {
    let topics = (Symbol::new(e, "set_registry"),);
    e.events().publish(topics, SetRegistry { registry });
}

pub(crate) fn initialize(e: &Env, addresses: Vec<Address>, units: Vec<i128>) {
    let topics = (Symbol::new(e, "intialize"), e.current_contract_address());
    e.events().publish(topics, Initialize { addresses, units });
}
