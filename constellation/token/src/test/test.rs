extern crate std;

use super::test_interface::initialize_token;
use crate::contract::ConstellationTokenClient;
use crate::error::Error;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, BytesN, Env, IntoVal, Symbol, Val, Vec,
};
use soroban_sdk::{Address, String};

pub mod token {
    soroban_sdk::contractimport!(file = "../../libs/soroban_token_contract.wasm");
}

pub mod adapter {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_adapter_soroswap.wasm"
    );
}
fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_constellation_token<'a>(e: &Env) -> ConstellationTokenClient<'a> {
    let contract_id = &e.register_contract(None, crate::contract::ConstellationToken {});
    let ct: ConstellationTokenClient<'_> = ConstellationTokenClient::new(e, contract_id);
    ct
}

fn create_adapter<'a>(e: &Env, router: &Address, factory: &Address) -> adapter::Client<'a> {
    adapter::Client::new(e, &e.register_contract_wasm(None, adapter::WASM))
}

#[test]
fn test_initialize_should_panic_with_already_initalized() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, 100];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);
    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::AlreadyInitalized)));
    // assert_eq!(ct.get_admin(), admin);
    assert_eq!(ct.get_manager().unwrap(), manager);
    assert_eq!(ct.get_components().len(), 3);
}

#[test]
fn test_initialize_should_panic_with_components_amounts_length_mismatch() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, 100];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::ComponentsAmountsLengthMismatch)));
}

#[test]
fn test_initialize_should_panic_with_zero_components() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![&e];
    let amounts: Vec<i128> = vec![&e];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::ZeroComponents)));
}

#[test]
fn test_initialize_should_panic_with_zero_or_negative_amount_not_allowed_1() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, -1];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::ZeroOrNegativeAmount)));
}

#[test]
fn test_initialize_should_panic_with_zero_or_negative_amount_not_allowed_2() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, 0];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::ZeroOrNegativeAmount)));
}

#[test]
fn test_initialize_successful() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, 100];
    let decimal: u32 = 6;
    let name: String = "c_token".into_val(&e);
    let symbol: String = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(ct.get_admin().unwrap(), admin);
    assert_eq!(ct.get_manager().unwrap(), manager);
    assert_eq!(ct.get_components().len(), 3);
}

#[test]
fn test_set_manager_panics_with_authorization_failed() {
    let e = Env::default();
    e.mock_all_auths();
    let new_manager = Address::generate(&e);
 

    let (ct, admin, manager,_) = initialize_token(
        &e,
        create_constellation_token(&e),
    );
    ct.set_manager(&new_manager);
    assert_eq!(
        e.auths(),
        std::vec![(
            manager.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ct.address.clone(),
                    "set_manager".into_val(&e),
                    (&new_manager,).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(ct.get_manager().unwrap(), new_manager);
}

#[test]
fn mint_reverts_with_zero_or_negative_amount_not_allowed() {
    let e = Env::default();
    e.mock_all_auths();
    let mint_to = Address::generate(&e);
    let new_manager = Address::generate(&e);

    // let token1 = create_token_contract(&e, &Address::generate(&e));
    // let token2 = create_token_contract(&e, &Address::generate(&e));
    // let token3 = create_token_contract(&e, &Address::generate(&e));

    let (ct, _, _, _) = initialize_token(
        &e,
        create_constellation_token(&e), 
    );

    let restult = ct.try_mint(&mint_to, &i128::from(0));
    assert_eq!(restult, Err(Ok(Error::ZeroOrNegativeAmount.into())));
}
