use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketState {
    pub borrow_assets: Option<String>,
    pub borrow_assets_usd: Option<f64>,
    pub supply_assets: Option<String>,
    pub supply_assets_usd: Option<f64>,
    pub collateral_assets: Option<String>,
    pub collateral_assets_usd: Option<f64>,
    pub liquidity_assets: Option<String>,
    pub liquidity_assets_usd: Option<f64>,
    pub utilization: Option<f64>,
    pub fee: Option<f64>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketItem {
    pub unique_key: String,
    pub lltv: Option<String>,
    pub oracle_address: Option<String>,
    pub irm_address: Option<String>,
    pub loan_asset: Option<Asset>,
    pub collateral_asset: Option<Asset>,
    pub state: Option<MarketState>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResult {
    pub items: Vec<MarketItem>,
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketStateResponse {
    pub markets: MarketsResult,
}
