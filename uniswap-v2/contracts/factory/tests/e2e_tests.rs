#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use factory::factory;

    /// A helper function used for calling contract messages.
    use ink_e2e::ContractsBackend;

    /// The End-to-End test `Result` type.
    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// We test that we can upload and instantiate the contract using its default constructor.
    #[ink_e2e::test]
    async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let mut constructor = FactoryRef::default();

        // When
        let contract = client
            .instantiate("factory", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let call_builder = contract.call_builder::<Factory>();

        // Then
        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), false));

        Ok(())
    }

    /// We test that we can read and write a value from the on-chain contract.
    #[ink_e2e::test]
    async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let mut constructor = FactoryRef::new(false);
        let contract = client
            .instantiate("factory", &ink_e2e::bob(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let mut call_builder = contract.call_builder::<Factory>();

        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), false));

        // When
        let flip = call_builder.flip();
        let _flip_result = client
            .call(&ink_e2e::bob(), &flip)
            .submit()
            .await
            .expect("flip failed");

        // Then
        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), true));

        Ok(())
    }
}
