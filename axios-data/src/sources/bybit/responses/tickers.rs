use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BybitTickersResponse {
    pub topic: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub cs: i64,
    pub ts: u64,
    pub data: Vec<BybitTickerData>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BybitTickerData {
    pub symbol: String,
    pub tick_direction: TickDirection,
    pub price_24h_pcnt: String,
    pub last_price: String,
    pub prev_price_24h: String,
    pub high_price_24h: String,
    pub low_price_24h: String,
    pub prev_price_1h: String,
    pub mark_price: String,
    pub index_price: String,
    pub open_interest: String,
    pub open_interest_value: String,
    pub turnover_24h: String,
    pub volume_24h: String,
    pub next_funding_time: String,
    pub funding_rate: String,
    pub bid1_price: String,
    pub bid1_size: String,
    pub ask1_price: String,
    pub ask1_size: String,
    pub delivery_time: u64,
    pub basis_rate: String,
    pub delivery_fee_rate: String,
    pub predicted_delivery_price: String,
    pub pre_open_price: String,
    pub pre_qty: String,
    pub cur_pre_listing_phase: CurPreListingPhase,
    pub funding_interval_hour: String,
    pub funding_cap: String,
    pub basis_rate_year: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum TickDirection {
    PlusTick,
    ZeroPlusTick,
    MinusTick,
    ZeroMinusTick,
}

#[derive(Deserialize, Debug, Clone)]
pub enum CurPreListingPhase {
    NotStarted,
    Finished,
    CallAuction,
    CallAuctionNoCancel,
    CrossMatching,
    ContinuousTrading,
}
