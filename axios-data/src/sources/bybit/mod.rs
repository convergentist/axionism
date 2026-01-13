#[derive(Debug, Clone)]
pub enum WssExchange {
    Bybit,
}

pub mod client;
pub use client::BybitWssClient;

pub mod decoder;

pub mod responses;
pub use responses::{
    BybitWssEvent,
    liquidations::{BybitLiquidationData, BybitLiquidationResponse},
    trades::{BybitTradeData, BybitTradeResponse},
};

pub async fn stream_data(
    symbols: Vec<String>,
    streams: Vec<String>,
    source: WssExchange,
) -> Result<tokio::sync::mpsc::Receiver<BybitWssEvent>, crate::errors::ExchangeError> {
    let (tx, rx) = tokio::sync::mpsc::channel(1024);

    let client = match source {
        WssExchange::Bybit => BybitWssClient::new(symbols, streams),
    };

    tokio::spawn(async move {
        if let Err(e) = client.run_data(tx).await {
            tracing::error!("Liquidation stream error: {}", e);
        }
    });

    Ok(rx)
}
