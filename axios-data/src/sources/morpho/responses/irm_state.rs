use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrmMarketState {
    pub borrow_apy: Option<f64>,
    pub supply_apy: Option<f64>,
    pub avg_borrow_apy: Option<f64>,
    pub avg_supply_apy: Option<f64>,
    pub rate_at_u_target: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrmCurvePoint {
    pub utilization: Option<f64>,
    pub borrow_apy: Option<f64>,
    pub supply_apy: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrmMarket {
    pub unique_key: String,
    pub irm_address: Option<String>,
    pub state: Option<IrmMarketState>,
    pub current_irm_curve: Option<Vec<IrmCurvePoint>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrmStateResponse {
    pub market_by_unique_key: Option<IrmMarket>,
}
