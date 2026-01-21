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
                first: 10,
                skip: 0,
            }
        }
    }

    #[tokio::test]
    async fn test_vault_historical_data() {
        let mut client = MorphoClient::new();

        let vars = VaultDataVars {
            chain_ids: vec![CHAIN_ETHEREUM],
            first: 1000,
            skip: 0,
        };

        let result: VaultStateResponse = client
            .query(queries::VAULT_DATA, vars)
            .await
            .expect("VaultData query failed");

        let n_vaults = result.vaults.items.len();

        println!("\nnumber of vaults: {:?}", n_vaults);

        let sel_vault = 0;

        println!(
            "Total Assets (USD) {:?}",
            result.vaults.items[sel_vault].state.clone().unwrap()
        );
    }
}
