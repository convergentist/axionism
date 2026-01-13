use crate::errors::ExchangeError;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{error, info, warn};
use url::Url;

#[async_trait::async_trait]
pub trait WssDecoder: Send + Sync + 'static {
    /// Type yielded to application code after decoding.
    type Event: Send + 'static;

    /// Try to parse a text frame. Return Ok(Some(event)) for data frames,
    /// Ok(None) for frames that should be ignored, Err(_) to treat as fatal.
    fn decode(text: &str) -> Result<Option<Self::Event>, Box<ExchangeError>>;
}

#[derive(Clone)]
pub struct WssClient<D>
where
    D: WssDecoder,
{
    pub streams: Vec<String>,
    pub base_url: String,
    // Hold the decoder in Arc so multiple tasks can share if necessary.
    pub decoder: Arc<D>,
}

impl<D> WssClient<D>
where
    D: WssDecoder,
{
    /// Connects and pumps events into the provided channel.
    pub async fn run(
        self,
        tx: mpsc::Sender<<D as WssDecoder>::Event>,
    ) -> Result<(), ExchangeError> {
        // Build URL (adapt if exchange uses different style)
        let stream_names = self.streams.join("/");
        let url_str = format!("{}?streams={}", self.base_url, stream_names);
        let url = Url::parse(&url_str)?;

        info!("Connecting to WebSocket URL: {}", url);
        let (ws_stream, _) = connect_async(url).await?;
        info!("WebSocket connection established.");

        let (write, mut read) = ws_stream.split();
        // Wrap writer in a Mutex so heartbeat can run concurrently
        let write = Arc::new(Mutex::new(write));

        // Spawn a heartbeat task (if the exchange overrides the default no-op)
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(25)).await;
            }
        });

        // Main message processing loop
        loop {
            tokio::select! {
                Some(msg) = read.next() => {
                    match msg {
                        Ok(Message::Text(text)) => {
                            match D::decode(&text) {
                                Ok(Some(event)) => {
                                    if tx.send(event).await.is_err() {
                                        error!("Receiver dropped; shutting down client.");
                                        break;
                                    }
                                }
                                Ok(None) => {} // not a data frame; ignore
                                Err(e) => warn!("Decode error: {} | {}", e, text), // Remove dereference
                            }
                        }
                        Ok(Message::Ping(p)) => {
                            if let Err(e) = write.lock().await.send(Message::Pong(p)).await {
                                error!("Pong failed: {}", e);
                                break;
                            }
                        }
                        Ok(Message::Close(_)) => {
                            info!("Server closed connection.");
                            break;
                        }
                        Err(e) => {
                            error!("WebSocket error: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }
                else => break,
            }
        }

        warn!("WssClient loop terminated.");
        Ok(())
    }
}

#[derive(Clone)]
pub struct WssClientBuilder<D>
where
    D: WssDecoder,
{
    streams: Option<Vec<String>>,
    base_url: Option<String>,
    decoder: Option<Arc<D>>,
}

impl<D> Default for WssClientBuilder<D>
where
    D: WssDecoder,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D> WssClientBuilder<D>
where
    D: WssDecoder,
{
    pub fn new() -> Self {
        Self {
            streams: None,
            base_url: None,
            decoder: None,
        }
    }
    pub fn streams(mut self, streams: Vec<String>) -> Self {
        self.streams = Some(streams);
        self
    }
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }
    pub fn decoder(mut self, decoder: D) -> Self {
        self.decoder = Some(Arc::new(decoder));
        self
    }
    pub fn build(self) -> Result<WssClient<D>, String> {
        Ok(WssClient {
            streams: self.streams.ok_or("missing streams")?,
            base_url: self.base_url.ok_or("missing base_url")?,
            decoder: self.decoder.ok_or("missing decoder")?,
        })
    }
}
