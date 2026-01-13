use crate::{
    clients::wss::WssDecoder, errors::ExchangeError,
    sources::bybit::decoder::BybitDecoder,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::{
    sync::{Mutex, mpsc},
    time::{Duration, sleep},
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{error, info, warn};
use url::Url;

pub struct BybitWssClient {
    base_url: String,
    symbols: Vec<String>,
    streams: Vec<String>,
}

pub use crate::sources::bybit::responses::BybitWssEvent;

impl BybitWssClient {
    pub fn new(symbols: Vec<String>, streams: Vec<String>) -> Self {
        Self {
            base_url: "wss://stream.bybit.com/v5/public/linear".to_string(),
            symbols,
            streams,
        }
    }

    pub async fn run_data(
        &self,
        tx: mpsc::Sender<BybitWssEvent>,
    ) -> Result<(), ExchangeError> {
        let url = Url::parse(&self.base_url)?;
        let (ws_stream, _) = connect_async(url).await?;
        info!("WebSocket connected to Bybit");

        let (writer_half, mut reader_half) = ws_stream.split();
        let writer = Arc::new(Mutex::new(writer_half));

        let mut topics: Vec<String> = Vec::new();

        for i_stream in self.streams.iter() {
            for i_symbol in self.symbols.iter() {
                topics.push(i_stream.to_owned() + "." + &i_symbol.clone());
            }
        }

        let sub_msg = serde_json::json!({
            "op": "subscribe",
            "args": topics
        })
        .to_string();

        writer.lock().await.send(Message::Text(sub_msg)).await?;

        // Heartbeat task
        let hb_writer = writer.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(25)).await;
                if hb_writer
                    .lock()
                    .await
                    .send(Message::Text(r#"{"op":"ping"}"#.into()))
                    .await
                    .is_err()
                {
                    error!("heartbeat failed");
                    break;
                }
            }
        });

        let pong_writer = writer.clone();
        while let Some(msg) = reader_half.next().await {
            match msg {
                Ok(Message::Text(txt)) => {
                    // Handle server pings
                    if txt.contains(r#""op":"ping""#) {
                        let _ = pong_writer
                            .lock()
                            .await
                            .send(Message::Text(r#"{"op":"pong"}"#.into()))
                            .await;
                        continue;
                    }

                    match BybitDecoder::decode(&txt) {
                        // Decode Public trades data
                        Ok(Some(BybitWssEvent::TradeData(event))) => {
                            if tx.send(BybitWssEvent::TradeData(event)).await.is_err() {
                                error!("Receiver dropped; shutting down client.");
                                break;
                            }
                        }

                        // Decode Liquidation data
                        Ok(Some(BybitWssEvent::LiquidationData(event))) => {
                            if tx
                                .send(BybitWssEvent::LiquidationData(event))
                                .await
                                .is_err()
                            {
                                error!("Receiver dropped; shutting down client.");
                                break;
                            }
                        }

                        // Decode Orderbook data
                        Ok(Some(BybitWssEvent::OrderbookData(event))) => {
                            if tx.send(BybitWssEvent::OrderbookData(event)).await.is_err()
                            {
                                error!("Receiver dropped; shutting down client.");
                                break;
                            }
                        }

                        // Decode Funding data
                        Ok(Some(BybitWssEvent::TickerData(event))) => {
                            if tx.send(BybitWssEvent::TickerData(event)).await.is_err() {
                                error!("");
                                break;
                            }
                        }

                        // ignore other messages
                        Ok(None) => {}

                        // Remove dereference
                        Err(e) => warn!("Decode error: {}", e),
                    }
                }
                Ok(Message::Ping(p)) => {
                    let _ = pong_writer.lock().await.send(Message::Pong(p)).await;
                }
                Ok(Message::Close(f)) => {
                    info!("server closed: {:?}", f);
                    break;
                }
                Err(e) => {
                    error!("ws error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
