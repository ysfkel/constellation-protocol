use super::event;
use super::helpers::{lock, redeem};
use crate::admin::read_administrator;
use crate::admin::{has_administrator, write_administrator};
use crate::allowance::*;
use crate::balance::*;
use crate::component::{read_components, write_components};
use crate::error::Error;
use crate::error::{check_nonnegative_amount, check_zero_or_negative_amount};
use crate::manager::{read_manager, write_manager};
use crate::metadata::*;
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
// use crate::module::{read_module, remove_module, write_module};
use crate::storage_types::Component;
use crate::storage_types::{
    AllowanceDataKey, AllowanceValue, DataKey, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
};
use crate::traits::{ConstellationTokenInterface, Module};
use soroban_sdk::{
    contract, contractimpl, contracttype, log, panic_with_error, symbol_short, token,
    token::Interface, Address, Env, IntoVal, String, Symbol, Val, Vec,
};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};
#[contract]
pub struct ConstellationToken;

#[contractimpl]
impl ConstellationToken {
    //////////////////////////////////////////////////////////////////
    ///////// mutable functions //////////////////////////////////////
    //////////////////////////////////////////////////////////////////
    pub fn set_admin(e: Env, new_admin: Address) -> Result<(), Error> {
        let admin = match read_administrator(&e) {
            Some(admin) => {
                admin.require_auth();
                admin
            }
            None => return Err(Error::RequiresAdministrator),
        };

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_administrator(&e, &new_admin);
        TokenUtils::new(&e).events().set_admin(admin, new_admin);

        Ok(())
    }

    //////////////////////////////////////////////////////////////////
    ///////// Read Only functions ////////////////////////////////////
    //////////////////////////////////////////////////////////////////

    pub fn get_allowance(e: Env, from: Address, spender: Address) -> Option<AllowanceValue> {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        let allowance = e.storage().temporary().get::<_, AllowanceValue>(&key);
        allowance
    }

    pub fn get_admin(e: Env) -> Option<Address> {
        read_administrator(&e)
    }
}

#[contractimpl]
impl ConstellationTokenInterface for ConstellationToken {
    //////////////////////////////////////////////////////////////////
    ///////// mutable functions //////////////////////////////////////
    //////////////////////////////////////////////////////////////////

    /// Initializes the deployed constellation token
    ///
    /// # Arguments
    ///
    /// - `e` Runtime environment.
    /// - `decimal` Token decimal
    /// - `components` Component tokens of this token
    /// - `units` Amounts of each componet token required to mint constellation token
    /// - `name` Name of token
    /// - `symbol` Symbol of token
    /// - `admin` Token administrator
    /// - `manager` Manages constellation token components and rebalancing
    fn initialize(
        e: Env,
        decimal: u32,
        components: Vec<Address>,
        units: Vec<i128>,
        name: String,
        symbol: String,
        admin: Address,
        manager: Address,
    ) -> Result<(), Error> {
        if has_administrator(&e) {
            panic_with_error!(&e, Error::AlreadyInitalized);
        }

        if decimal > u8::MAX.into() {
            panic_with_error!(&e, Error::ValueTooLargeOverFlow);
        }

        write_administrator(&e, &admin);
        write_manager(&e, &manager);
        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        );
        let components = write_components(&e, &components, &units);
        event::initialize(&e, components);
        Ok(())
    }

    /// Mints ew constellatio tokens
    /// Returns error if administrator is not set
    ///
    /// # Arguments
    /// - `e` Runtime environment
    /// - `to` Address receiver
    /// - `amount` Amount of constellation tokens to mint
    ///
    /// `to` Address should have balance of component tokens equal to or greater than amount * unit (component unit)
    fn mint(e: Env, to: Address, amount: i128) -> Result<(), Error> {
        check_zero_or_negative_amount(&e, amount);
        let admin = match read_administrator(&e) {
            Some(admin) => {
                admin.require_auth();
                admin
            }
            None => return Err(Error::RequiresAdministrator),
        };

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Locks component tokens
        lock(&e, &to, amount);

        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(admin, to, amount);

        Ok(())
    }

    /// Mints ew constellatio tokens
    /// Returns error if administrator is not set
    ///
    /// # Arguments
    /// - `spender`
    /// - `e` Runtime environment
    /// - `to` Address receiver
    /// - `amount` Amount of constellation tokens to mint
    ///
    /// `to` Address should have balance of component tokens equal to or greater than amount * unit (component unit)
    fn redeem(e: Env, to: Address, amount: i128) -> Result<(), Error> {
        check_zero_or_negative_amount(&e, amount);
        let admin = match read_administrator(&e) {
            Some(admin) => {
                admin.require_auth();
                admin
            }
            None => return Err(Error::RequiresAdministrator),
        };
        redeem(&e, &to, amount);
        event::redeem(&e, admin, to, amount);
        Ok(())
    }

    fn set_manager(e: Env, new_manager: Address) -> Result<(), Error> {
        let manager = match read_manager(&e) {
            Some(manager) => {
                manager.require_auth();
                manager
            }
            None => return Err(Error::RequiresManager),
        };

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_manager(&e, &new_manager);
        event::set_manager(&e, manager, new_manager);
        Ok(())
    }

    //////////////////////////////////////////////////////////////////
    ///////// Read Only functions ////////////////////////////////////
    //////////////////////////////////////////////////////////////////

    fn get_components(e: Env) -> Vec<Component> {
        read_components(&e)
    }

    fn get_manager(e: Env) -> Option<Address> {
        read_manager(&e)
    }
}

#[contractimpl]
impl token::Interface for ConstellationToken {
    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_zero_or_negative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);

        TokenUtils::new(&e).events().burn(from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_zero_or_negative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);

        TokenUtils::new(&e).events().burn(from, amount)
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_zero_or_negative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_zero_or_negative_amount(&e, amount);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}

// impl Module for ConstellationToken {

//     fn add_module(e: Env, module_id: Address) {
//         let manager = match read_manager(&e) {
//             Some(manager) => {
//                 manager.require_auth();
//                 manager
//             }
//             None => return Err(Error::RequiresManager),
//         };

//         write_module(&e, &module_id);
//     }
//     fn remove_module(e: Env, module_id: Address) {
//         let manager = match read_manager(&e) {
//             Some(manager) => {
//                 manager.require_auth();
//                 manager
//             }
//             None => return Err(Error::RequiresManager),
//         };

//         remove_module(&e, &module_id);
//     }

//     fn invoke(e: Env, target_exchange: Address, method_name: Symbol, args: Vec<Val>){
//         let module = match read_module(&e) {
//             Some(module) => {
//                 module.require_auth();
//                 module
//             }
//             None => return Err(Error::RequiresModule),
//         };
//     }
// }