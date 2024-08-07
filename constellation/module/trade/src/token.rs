use soroban_sdk::{auth::InvokerContractAuthEntry, Address, Env, String, Symbol, Val, Vec};
pub(crate) mod constellation_token {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        file = "../../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
}

/// Invokes the invoke function of the constellation token to trade / exchange tokens
///
/// # Arguments
///
/// - `e` The runtime environment.
/// - `constellation_token_id` Target constellation token id
/// - `target_exchange_id` Target exchange identifier
/// - `function` Name of function to invoke on target exchange
/// - `data` Function arguments
pub(crate) fn invoke(
    e: &Env,
    constellation_token_id: &Address,
    target_contract_id: &Address,
    call_data: &(Symbol, Vec<Val>),
    auth_entries: &Vec<InvokerContractAuthEntry>,
) {
    let client = constellation_token::Client::new(&e, &constellation_token_id);
    client.invoke(
        &e.current_contract_address(),
        target_contract_id,
        call_data,
        auth_entries,
    );
}

pub(crate) fn get_manager(e: &Env, constellation_token_id: &Address) -> Option<Address> {
    let client = constellation_token::Client::new(&e, &constellation_token_id);
    client.get_manager()
}

pub(crate) fn update_units(
    e: &Env,
    balance_before_trade_token_in: i128,
    balance_before_trade_token_out: i128,
    token_in: &Address,
    token_out: &Address,
    constellation_token_id: &Address,
) {
    let client = constellation_token::Client::new(&e, &constellation_token_id);
    client.update_units(
        &(token_in.clone(), balance_before_trade_token_in),
        &(token_out.clone(), balance_before_trade_token_out),
    );
}
