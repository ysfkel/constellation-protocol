use crate::error::Error;
use crate::helpers::deploy;
use crate::storage::max_components::{read_max_components, write_max_components};
use crate::storage::token_list::{read_token_list, write_token_list};
use crate::storage::DataKey;
use crate::token::{constellation_token, initialize_token};
use crate::types::{CreateConstellationTokenArgs, SimpleArgs};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

#[contract]
pub struct Factory {}

#[contractimpl]
impl Factory {
    // todo ! fn initialize can be removed
    pub fn initialize(e: Env, max_components: Option<u32>) -> Result<(), Error> {
        if let Some(max) = max_components {
            if max == 0 {
                return Err(Error::ZeroValue);
            }
            write_max_components(&e, max);
        }

        Ok(())
    }

    pub fn simple(e: Env, args: SimpleArgs) {
        let key = DataKey::SimpleArgsKey;
        e.storage().instance().set(&key, &args);
    }

    pub fn get_simple(e: Env) -> SimpleArgs {
        let key = DataKey::SimpleArgsKey;
        e.storage().instance().get(&key).unwrap()
    }

    pub fn create_constellation_token(
        e: Env,
        decimal: u32,
        name: String,
        symbol: String,
        admin: Address,
        manager: Address,
        components: Vec<Address>,
        amounts: Vec<i128>,
        deployer: Address,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>
    ) -> Result<(), Error> {
        if let Some(max) = read_max_components(&e) {
            if args.components.len() > max {
                return Err(Error::ExceedsMaxComponents);
            }
        }

        let address = deploy(&e, deployer, wasm_hash, salt);

        // todo - handle initalization error
        initialize_token(
            &e, &address, decimal, name, symbol, admin, manager, components, amounts,
        );

        write_token_list(&e, address);

        // todo - ADD EVENT

        Ok(())
    }

    pub fn get_token_list(e: Env) -> Vec<Address> {
        read_token_list(&e)
    }

    pub fn set_max_components(e: Env, max_components: u32) -> Result<(), Error> {
        if max_components == 0 {
            return Err(Error::ZeroValue);
        }

        write_max_components(&e, max_components);

        // todo - ADD EVENT

        Ok(())
    }

    pub fn get_max_components(e: Env) -> Option<u32> {
        read_max_components(&e)
    }
}
