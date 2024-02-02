#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use casper_types::{runtime_args::RuntimeArgs, ApiError, ContractHash, Key};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

const COUNTER_KEY: &str = "counter";
const COUNTER_SET: &str = "counter_set";
const COUNTER_GET: &str = "counter_get";
const ARG_VALUE: &str = "value";

#[no_mangle]
pub extern "C" fn call() {
    // Read the Counter smart contract's ContractHash.
    let contract_hash = {
        let counter_uref = runtime::get_key(COUNTER_KEY).unwrap_or_revert_with(ApiError::GetKey);
        if let Key::Hash(hash) = counter_uref {
            ContractHash::new(hash)
        } else {
            runtime::revert(ApiError::ContractNotFound);
        }
    };

    // Call Counter to get the current value.
    let first_counter_value: i32 =
        runtime::call_contract(contract_hash, COUNTER_GET, RuntimeArgs::new());

    // Set Counter value.
    let mut args = RuntimeArgs::new();
    args.insert(ARG_VALUE, first_counter_value + 10)
        .unwrap_or_revert_with(ApiError::InvalidArgument);
    let _: () = runtime::call_contract(contract_hash, COUNTER_SET, args);

    // Call Counter to get the new value.
    let new_counter_value: i32 =
        runtime::call_contract(contract_hash, COUNTER_GET, RuntimeArgs::new());

    // Expect counter to increment by one.
    if new_counter_value != first_counter_value + 10 {
        runtime::revert(ApiError::InvalidSystemContract);
    }
}
