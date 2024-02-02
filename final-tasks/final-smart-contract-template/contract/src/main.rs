#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// This code imports necessary aspects of external crates that we will use in our contract code.
extern crate alloc;

// Importing Rust types.
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
// Importing aspects of the Casper platform.
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
// Importing specific Casper types.
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, Parameter, URef,
};

// Creating constants for values within the contract package.
const CONTRACT_PACKAGE_NAME: &str = "counter_package_name";
const CONTRACT_ACCESS_UREF: &str = "counter_access_uref";

// Creating constants for the various contract entry points.
const ENTRY_POINT_COUNTER_SET: &str = "counter_set";
const ENTRY_POINT_COUNTER_GET: &str = "counter_get";

// Creating constants for values within the contract.
const CONTRACT_VERSION_KEY: &str = "version";
const CONTRACT_KEY: &str = "counter";

// Variables
const COUNT_KEY: &str = "count";
const LAST_UPDATED_KEY: &str = "last_updated";
const ARG_VALUE: &str = "value";

#[no_mangle]
pub extern "C" fn counter_set() {
    // Value:
    let value: i32 = runtime::get_named_arg(ARG_VALUE);
    let uref_value = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    storage::write(uref_value, value); // Increment the count by 1.

    // Last Updated Timestamp:
    let now: u64 = runtime::get_blocktime().into();
    let uref_timestamp = runtime::get_key(LAST_UPDATED_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    storage::write(uref_timestamp, now); // Update last updated timestamp
}

// Entry point that returns the count value.
#[no_mangle]
pub extern "C" fn counter_get() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    let result: i32 = storage::read(uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);

    let typed_result = CLValue::from_t(result).unwrap_or_revert();

    runtime::ret(typed_result); // Return the count value.
}

// Entry point that executes automatically when a caller installs the contract.
#[no_mangle]
pub extern "C" fn call() {
    // Initialize the count to 0, locally.
    let count_start = storage::new_uref(0_i32);
    let last_updated = storage::new_uref(0_u64);

    // In the named keys of the contract, add a key for the count.
    let mut counter_named_keys = NamedKeys::new();

    counter_named_keys.insert(COUNT_KEY.to_string(), count_start.into());
    counter_named_keys.insert(LAST_UPDATED_KEY.to_string(), last_updated.into());

    // Create the entry points for this contract.
    let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_GET,
        Vec::new(),
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    counter_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_SET,
        Vec::from([Parameter::new(ARG_VALUE, CLType::I32)]),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Create a new contract package that can be upgraded.
    let (stored_contract_hash, contract_version) = storage::new_contract(
        counter_entry_points,
        Some(counter_named_keys),
        Some(String::from(CONTRACT_PACKAGE_NAME)),
        Some(String::from(CONTRACT_ACCESS_UREF)),
    );

    /* To create a locked contract instead, use new_locked_contract and throw away the contract version returned.
    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None); */

    // Store the contract version in the context's named keys.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash.
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}
