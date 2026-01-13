use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BybitLiquidationResponse {
    pub topic: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub ts: u64,
    pub data: Vec<BybitLiquidationData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BybitLiquidationData {
    #[serde(rename = "T")]
    pub liquidation_ts: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "v")]
    pub amount: String,
    #[serde(rename = "p")]
    pub price: String,
}
