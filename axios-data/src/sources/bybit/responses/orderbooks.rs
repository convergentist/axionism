use serde::Deserialize;

/// Orderbook type abstraction
#[derive(Debug, Clone, PartialEq)]
pub enum BybitOrderbookType {
    Snapshot,
    Delta,
}

/// Bybit orderbook WebSocket response
#[derive(Deserialize, Debug, Clone)]
pub struct BybitOrderbookResponse {
    /// Topic string: "orderbook.{depth}.{symbol}"
    pub topic: String,
    /// Message type: "snapshot" or "delta"
    #[serde(rename = "type")]
    pub ty: String,
    /// Server timestamp in milliseconds
    pub ts: u64,
    /// Orderbook data payload
    pub data: Vec<BybitOrderbookData>,
    /// Cross timestamp (optional, for latency measurement)
    #[serde(default)]
    pub cts: Option<u64>,
}

/// Bybit orderbook data
#[derive(Deserialize, Debug, Clone)]
pub struct BybitOrderbookData {
    /// Orderbook timestamp (exchange)
    #[serde(rename = "ts")]
    pub orderbook_ts: u64,
    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,
    /// Bids: [[price, size], ...]
    #[serde(rename = "b")]
    pub bids: Vec<BybitPriceLevel>,
    /// Asks: [[price, size], ...]
    #[serde(rename = "a")]
    pub asks: Vec<BybitPriceLevel>,
    /// Update ID
    #[serde(rename = "u")]
    pub update_id: u64,
    /// Cross sequence
    #[serde(rename = "seq")]
    pub sequence: u64,
}

/// Price level [price, size] as strings
#[derive(Deserialize, Debug, Clone)]
pub struct BybitPriceLevel(pub String, pub String);

impl BybitPriceLevel {
    /// Get price as f64
    pub fn price(&self) -> f64 {
        self.0.parse().unwrap_or(0.0)
    }

    /// Get size as f64
    pub fn size(&self) -> f64 {
        self.1.parse().unwrap_or(0.0)
    }
}

impl BybitOrderbookData {
    /// Parse the update type from the response
    pub fn update_type(ty: &str) -> BybitOrderbookType {
        match ty {
            "snapshot" => BybitOrderbookType::Snapshot,
            _ => BybitOrderbookType::Delta,
        }
    }
}

impl BybitOrderbookResponse {
    /// Parse the message type from string
    pub fn message_type(&self) -> BybitOrderbookType {
        if self.ty == "snapshot" {
            BybitOrderbookType::Snapshot
        } else {
            BybitOrderbookType::Delta
        }
    }

    /// Extract symbol from the topic string
    pub fn symbol(&self) -> Option<&str> {
        // Topic format: "orderbook.{depth}.{symbol}"
        self.topic.split('.').nth(2)
    }

    /// Extract depth from the topic string
    pub fn depth(&self) -> Option<u32> {
        self.topic.split('.').nth(1)?.parse().ok()
    }
}
