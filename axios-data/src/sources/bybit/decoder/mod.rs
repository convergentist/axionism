use crate::{
    clients::wss::WssDecoder,
    errors::ExchangeError,
    sources::bybit::responses::{
        BybitWssEvent, liquidations::BybitLiquidationResponse,
        orderbooks::BybitOrderbookResponse, tickers::BybitTickersResponse,
        trades::BybitTradeResponse,
    },
};
use serde_json::Value;

pub struct BybitDecoder;

impl WssDecoder for BybitDecoder {
    type Event = BybitWssEvent;

    fn decode(text: &str) -> Result<Option<Self::Event>, Box<ExchangeError>> {
        let json: Value =
            serde_json::from_str(text).map_err(|e| Box::new(ExchangeError::from(e)))?;

        // Check if this is a subscription success message
        if let Some(success) = json.get("success") {
            if success.as_bool() == Some(true) {
                tracing::info!("Successfully subscribed to Bybit WebSocket");
                return Ok(None);
            }
        }

        // Check if this is a pong response
        if let Some(op) = json.get("op") {
            if op.as_str() == Some("pong") {
                return Ok(None); // Ignore pong messages
            }
        }

        // Check if this has a topic field
        if let Some(topic) = json.get("topic") {
            if let Some(topic_str) = topic.as_str() {
                // Liquidation Data
                if topic_str.starts_with("allLiquidation.") {
                    // Try to parse as liquidation response
                    match serde_json::from_str::<BybitLiquidationResponse>(text) {
                        Ok(env) => {
                            // Return the first liquidation data item if available
                            if let Some(liq_data) = env.data.into_iter().next() {
                                Ok(Some(Self::Event::LiquidationData(liq_data)))
                            } else {
                                Ok(None)
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse liquidation data: {}", e);
                            Ok(None)
                        }
                    }
                // Public Trades Data
                } else if topic_str.starts_with("publicTrade.") {
                    // Try to parse
                    match serde_json::from_str::<BybitTradeResponse>(text) {
                        Ok(env) => {
                            if let Some(trade_data) = env.data.into_iter().next() {
                                Ok(Some(Self::Event::TradeData(trade_data)))
                            } else {
                                Ok(None)
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse public trade data: {}", e);
                            Ok(None)
                        }
                    }
                // Orderbooks Data
                } else if topic_str.starts_with("orderbook.") {
                    // Try to parse
                    match serde_json::from_str::<BybitOrderbookResponse>(text) {
                        Ok(env) => {
                            if let Some(orderbook_data) = env.data.into_iter().next() {
                                Ok(Some(Self::Event::OrderbookData(orderbook_data)))
                            } else {
                                Ok(None)
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse order book data: {}", e);
                            Ok(None)
                        }
                    }
                // Funding Rates Data
                } else if topic_str.starts_with("tickers") {
                    // Try to parse
                    match serde_json::from_str::<BybitTickersResponse>(text) {
                        Ok(env) => {
                            if let Some(funding_data) = env.data.into_iter().next() {
                                Ok(Some(Self::Event::TickerData(funding_data)))
                            } else {
                                Ok(None)
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse funding rates data: {}", e);
                            Ok(None)
                        }
                    }
                } else {
                    Ok(None) // Ignore other topics
                }
            } else {
                Ok(None)
            }
        } else {
            // Log unknown message types for debugging
            tracing::debug!("Received unknown message type: {}", text);
            Ok(None)
        }
    }
}
