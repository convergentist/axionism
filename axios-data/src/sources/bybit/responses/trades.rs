use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BybitTradeResponse {
    pub topic: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub ts: u64,
    pub data: Vec<BybitTradeData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BybitTradeData {
    #[serde(rename = "T")]
    pub trade_ts: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "v")]
    pub amount: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "L")]
    pub direction: String,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "BT")]
    pub block_trade: bool,
    #[serde(rename = "RPI")]
    pub rpi_trade: bool,
    #[serde(rename = "seq")]
    pub sequence: u64,
}
