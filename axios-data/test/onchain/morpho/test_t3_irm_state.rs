#[cfg(test)]

mod test_utils {

    pub const TEST_MARKET_KEY: &str =
        "0xb323495f7e4148be5643a4ea4a8221eef163e4bccfdedc2a6f4696baacbc86cc";
    pub const CHAIN_ETHEREUM: i32 = 1;
}

mod tests {

    use crate::test_utils::{CHAIN_ETHEREUM, TEST_MARKET_KEY};

    use axios_data::{
        onchain::graphs::queries,
        sources::morpho::{IrmStateResponse, MorphoClient},
    };
    use serde::Serialize;

    /// Variables for IrmState query
    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct IrmStateVars {
        pub unique_key: String,
        pub chain_id: i32,
        pub curve_points: i32,
    }

    #[tokio::test]
    async fn test_irm_state_query() {
        let mut client = MorphoClient::new();

        let vars = IrmStateVars {
            unique_key: TEST_MARKET_KEY.to_string(),
            chain_id: CHAIN_ETHEREUM,
            curve_points: 20,
        };

        let result: IrmStateResponse = client
            .query(queries::IRM_STATE, vars)
            .await
            .expect("IrmState query failed");

        let market = result.market_by_unique_key.expect("Should return market");

        assert!(market.state.is_some(), "Should have state");
        assert!(market.current_irm_curve.is_some(), "Should have IRM curve");

        let curve = market.current_irm_curve.unwrap();
        assert!(!curve.is_empty(), "IRM curve should have points");

        println!(
            "TABLE 3: IRM curve has {} points, borrowApy={:?}",
            curve.len(),
            market.state.as_ref().and_then(|s| s.borrow_apy)
        );
    }
}
