use crate::sources::Exchange;
use async_rate_limiter::RateLimiter;
use reqwest::Client;
use tokio::time::Duration;

#[derive(Clone)]
pub struct HttpClient {
    pub client: Client,
    pub exchange: Exchange,
    pub rate_limiter: RateLimiter,
    pub base_url: String,
    pub timeout: Duration,
}

#[derive(Clone)]
pub struct HttpClientBuilder {
    client: Option<Client>,
    exchange: Option<Exchange>,
    rate_limiter: Option<RateLimiter>,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl Default for HttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClientBuilder {
    pub fn new() -> Self {
        HttpClientBuilder {
            client: None,
            exchange: None,
            rate_limiter: None,
            base_url: None,
            timeout: None,
        }
    }

    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    pub fn rate_limiter(mut self, rate_limiter: RateLimiter) -> Self {
        self.rate_limiter = Some(rate_limiter);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<HttpClient, String> {
        let client = self.client.ok_or("Missing client")?;
        let exchange = self.exchange.ok_or("Missing exchange")?;
        let rate_limiter = self.rate_limiter.ok_or("Missing rate_limiter")?;
        let base_url = self.base_url.ok_or("Missing base_url")?;
        let timeout = self.timeout.ok_or("Missing timeout")?;

        Ok(HttpClient {
            client,
            exchange,
            rate_limiter,
            base_url,
            timeout,
        })
    }
}
