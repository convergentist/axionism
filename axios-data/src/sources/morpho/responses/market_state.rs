use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// Deserializes a field that can be either a JSON string or number into String
fn string_or_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<Value> = Option::deserialize(deserializer)?;
    Ok(opt.map(|v| match v {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        _ => v.to_string(),
    }))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub count_total: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub address: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<i64>,
    pub price_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketState {
    #[serde(deserialize_with = "string_or_number", default)]
    pub borrow_assets: Option<String>,
    pub borrow_assets_usd: Option<f64>,
    #[serde(deserialize_with = "string_or_number", default)]
    pub supply_assets: Option<String>,
    pub supply_assets_usd: Option<f64>,
    #[serde(deserialize_with = "string_or_number", default)]
    pub collateral_assets: Option<String>,
    pub collateral_assets_usd: Option<f64>,
    #[serde(deserialize_with = "string_or_number", default)]
    pub liquidity_assets: Option<String>,
    pub liquidity_assets_usd: Option<f64>,
    pub utilization: Option<f64>,
    pub fee: Option<f64>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketItem {
    pub unique_key: String,
    #[serde(deserialize_with = "string_or_number", default)]
    pub lltv: Option<String>,
    pub oracle_address: Option<String>,
    pub irm_address: Option<String>,
    pub loan_asset: Option<Asset>,
    pub collateral_asset: Option<Asset>,
    pub state: Option<MarketState>,
    pub chain_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResult {
    pub items: Vec<MarketItem>,
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketStateResponse {
    pub markets: MarketsResult,
}
