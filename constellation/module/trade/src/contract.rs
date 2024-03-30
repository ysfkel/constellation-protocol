use crate::error::Error;
use crate::token;
use crate::{
    storage::registry::{has_registry, write_registry},
    validation::{require_adapter, require_administrator, require_manager, require_registry},
};
use constellation_lib::traits::adapter::dex;
use soroban_sdk::vec;
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, Address, Env, Symbol, Val, Vec,
};
#[contract]
pub struct Trade {}

#[contractimpl]
impl Trade {
    pub fn initialize(e: Env, id: Address) {
        if has_registry(&e) {
            panic_with_error!(&e, Error::AlreadyInitalized);
        }

        write_registry(&e, &id);
    }
    pub fn trade(
        e: Env,
        constellation_token_id: Address,
        exchange_id: Address,
        token_in_id: Address,
        token_out_id: Address,
        amount_in: i128,
        amount_out: i128,
        deadline: u64,
        expiration_ledger: u32,
    ) -> Result<(), Error> {
        let manager = require_manager(&e, &constellation_token_id)?;
        manager.require_auth();

        let mut args: Vec<Val> = vec![&e];

        let registry_id = require_registry(&e)?;

        let adapter_id = require_adapter(&e, &registry_id, &exchange_id)?;

        let exchange_adapter = dex::Client::new(&e, &adapter_id);

        let approve_call_data = exchange_adapter.get_approve_call_data(
            &constellation_token_id,
            &exchange_id,
            &amount_in,
            &expiration_ledger,
        );

        Self::approve_exchange(
            &e,
            &constellation_token_id,
            &token_in_id,
            &approve_call_data,
        );

        let swap_call_data = exchange_adapter.get_swap_call_data(
            &token_in_id.clone(),
            &token_out_id.clone(),
            &amount_in,
            &amount_out,
            &constellation_token_id.clone(),
            &deadline,
        );

        Self::execute_trade(&e, &constellation_token_id, &exchange_id, &swap_call_data);
        Ok(())
    }

    fn approve_exchange(
        e: &Env,
        constellation_token_id: &Address,
        token_id: &Address,
        call_data: &(Symbol /* function */, Vec<Val>),
    ) {
        token::invoke(&e, constellation_token_id, token_id, &call_data);
    }
    fn execute_trade(
        e: &Env,
        constellation_token_id: &Address,
        exchange_id: &Address,
        call_data: &(Symbol /* function */, Vec<Val>),
    ) {
        token::invoke(&e, constellation_token_id, exchange_id, &call_data);
    }
}
