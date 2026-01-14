#[cfg(test)]

mod test_utils {
    pub const CHAIN_ETHEREUM: i32 = 1;
}

mod tests {

    use crate::test_utils::CHAIN_ETHEREUM;
    use axios_data::{
        onchain::graphs::queries,
        sources::morpho::{MorphoClient, VaultStateResponse},
    };
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VaultDataVars {
        pub chain_ids: Vec<i32>,
        pub first: i32,
        pub skip: i32,
    }

    impl Default for VaultDataVars {
        fn default() -> Self {
            Self {
                chain_ids: vec![1],
                first: 100,
                skip: 0,
            }
        }
    }

    #[tokio::test]
    async fn test_vault_data_query() {
        let mut client = MorphoClient::new();

        let vars = VaultDataVars {
            chain_ids: vec![CHAIN_ETHEREUM],
            first: 10,
            skip: 0,
        };

        let result: VaultStateResponse = client
            .query(queries::VAULT_DATA, vars)
            .await
            .expect("VaultData query failed");

        assert!(!result.vaults.items.is_empty(), "Should return vaults");

        let first = &result.vaults.items[0];
        assert!(!first.address.is_empty(), "Vault should have address");
        assert!(first.state.is_some(), "Vault should have state");

        println!(
            "TABLE 10: Found {} vaults, first: {} ({}), TVL=${:?}",
            result.vaults.items.len(),
            first.name.as_deref().unwrap_or("?"),
            &first.address[..10],
            first.state.as_ref().and_then(|s| s.total_assets_usd)
        );

        println!("\nall data: {:?}", result);
    }
}
