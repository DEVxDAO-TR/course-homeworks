#[cfg(test)]
mod tests {
    // Outlining aspects of the Casper test support crate to include.
    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        PRODUCTION_RUN_GENESIS_REQUEST,
    };
    // Custom Casper types that will be used within this test.
    use casper_types::{runtime_args, ContractHash, RuntimeArgs};

    const COUNTER_V1_WASM: &str = "contract.wasm"; // The first version of the contract
    const COUNTER_CALL_WASM: &str = "contract-call.wasm"; // Session code that calls the contract

    const CONTRACT_KEY: &str = "counter"; // Named key referencing this contract
    const COUNT_KEY: &str = "count"; // Named key referencing the value to increment/decrement
    const CONTRACT_VERSION_KEY: &str = "version"; // Key maintaining the version of a contract package

    const ENTRY_POINT_COUNTER_GET: &str = "counter_get"; // Entry point to decrement the count value
    const ENTRY_POINT_COUNTER_SET: &str = "counter_set"; // Entry point to increment the count value
    const ENTRY_POINT_COUNTER_DECREMENT: &str = "counter_decrement"; // Entry point to decrement the count value

    #[test]
    /// Install version 1 of the counter contract and check its available entry points.
    /// Only the increment entry point should be available.
    /// The decrement call should fail, because that entry point should not be in this version.
    /// Test summary:
    /// - Install the counter-v1.wasm contract.
    /// - Check the contract hash.
    /// - Check the contract version is 1.
    /// - Verify the initial value of count is 0.
    /// - Test the counter_inc entry point and increment the counter.
    /// - Verify that the count value is now 1.
    /// - Call the decrement entry point, which should fail.
    /// - Ensure the count value was not decremented and is still 1.
    fn install_version1_and_check_entry_points() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&*PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

        // Install the contract.
        let contract_v1_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_V1_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_v1_installation_request)
            .expect_success()
            .commit();

        // Check the contract hash.
        let contract_v1_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // Verify the first contract version is 1.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");

        let version_key = *account
            .named_keys()
            .get(CONTRACT_VERSION_KEY)
            .expect("version uref should exist");

        let version = builder
            .query(None, version_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32.");

        assert_eq!(version, 1);

        // Verify the initial value of count is 0.
        let contract = builder
            .get_contract(contract_v1_hash)
            .expect("this contract should exist");

        let count_key = *contract
            .named_keys()
            .get(COUNT_KEY)
            .expect("count uref should exist in the contract named keys");

        let count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(count, 0);

        // Use session code to increment the counter.
        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_v1_hash
            },
        )
        .build();

        builder.exec(session_code_request).expect_success().commit();

        // Verify the value of count is now 10.
        let incremented_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(incremented_count, 10);

        // Call the decrement entry point, which should not be in this version.
        let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v1_hash,
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .build();

        // Try executing the decrement entry point and expect an error.
        builder
            .exec(contract_decrement_request)
            .expect_failure()
            .commit();

        // Ensure the count value was not decremented.
        let current_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(current_count, 10);
    }
}
