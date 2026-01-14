#[cfg(test)]

mod test_utils {
    pub const CHAIN_ETHEREUM: i32 = 1;
}

mod tests {

    use crate::test_utils::CHAIN_ETHEREUM;
    use axios_data::{
        onchain::graphs::queries,
        sources::morpho::{MarketStateResponse, MorphoClient},
    };
    use serde::Serialize;

    /// Variables for MarketState
    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PaginatedChainVars {
        pub chain_ids: Vec<i32>,
        pub first: i32,
        pub skip: i32,
    }

    impl Default for PaginatedChainVars {
        fn default() -> Self {
            Self {
                chain_ids: vec![1],
                first: 100,
                skip: 0,
            }
        }
    }

    #[tokio::test]
    async fn test_market_state_query() {
        let mut client = MorphoClient::new();

        let vars = PaginatedChainVars {
            chain_ids: vec![CHAIN_ETHEREUM],
            first: 10,
            skip: 0,
        };

        let result: MarketStateResponse = client
            .query(queries::MARKET_STATE, vars)
            .await
            .expect("MarketState query failed");

        // println!("{:?}", serde_json::to_string_pretty(&result));

        assert!(!result.markets.items.is_empty(), "Should return markets");

        let first = &result.markets.items[0];
        assert!(!first.unique_key.is_empty(), "Market should have uniqueKey");
        assert!(first.state.is_some(), "Market should have state");

        println!(
            "TABLE 2 (Market State): Found {} markets, first: {}",
            result.markets.items.len(),
            &first.unique_key[..20]
        );
    }
}
