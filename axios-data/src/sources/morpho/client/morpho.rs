use crate::onchain::{TimeseriesOptions, HistoricalChunk, GraphQLRequest, GraphQLResponse, graphs::queries};
use serde::{Deserialize, Serialize};
use crate::onchain::Interval;
use std::time::{Duration, Instant};
pub const MORPHO_API: &str = "https://api.morpho.org/graphql";
const RATE_LIMIT_REQUESTS: u32 = 5000;
const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(300);

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    GraphQL(Vec<String>),
    NoData,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::GraphQL(errors) => write!(f, "GraphQL errors: {}", errors.join(", ")),
            Error::NoData => write!(f, "No data returned"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

pub struct MorphoClient {
    client: reqwest::Client,
    request_count: u32,
    window_start: Instant,
}

impl MorphoClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            request_count: 0,
            window_start: Instant::now(),
        }
    }

    /// Execute a GraphQL query with automatic rate limiting
    pub async fn query<V, R>(
        &mut self,
        query: &'static str,
        variables: V,
    ) -> Result<R, Error>
    where
        V: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        self.enforce_rate_limit().await;

        let request = GraphQLRequest { query, variables };
        let response = self
            .client
            .post(MORPHO_API)
            .json(&request)
            .send()
            .await?
            .json::<GraphQLResponse<R>>()
            .await?;

        self.request_count += 1;

        if let Some(errors) = response.errors {
            return Err(Error::GraphQL(
                errors.into_iter().map(|e| e.message).collect(),
            ));
        }

        response.data.ok_or(Error::NoData)
    }

    async fn enforce_rate_limit(&mut self) {
        let elapsed = self.window_start.elapsed();
        if elapsed >= RATE_LIMIT_WINDOW {
            self.request_count = 0;
            self.window_start = Instant::now();
        } else if self.request_count >= RATE_LIMIT_REQUESTS {
            let sleep_duration = RATE_LIMIT_WINDOW - elapsed;
            tokio::time::sleep(sleep_duration).await;
            self.request_count = 0;
            self.window_start = Instant::now();
        }
    }

    /// Fetch all markets with pagination
    pub async fn fetch_all_markets(
        &mut self,
        chain_ids: Vec<i32>,
    ) -> Result<Vec<serde_json::Value>, Error> {
        let mut all_markets = Vec::new();
        let mut skip = 0;
        let page_size = 100;

        loop {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Vars {
                chain_ids: Vec<i32>,
                first: i32,
                skip: i32,
            }

            let vars = Vars {
                chain_ids: chain_ids.clone(),
                first: page_size,
                skip,
            };

            let response: serde_json::Value =
                self.query(queries::MARKET_STATE, vars).await?;
            let empty_item = vec![];
            let items = response["markets"]["items"].as_array().unwrap_or(&empty_item);
            let count = items.len();

            all_markets.extend(items.iter().cloned());

            if count < page_size as usize {
                break;
            }
            skip += page_size;
        }

        Ok(all_markets)
    }

    /// Fetch 1 month of historical data for a market, chunked to avoid timeouts
    pub async fn fetch_monthly_history(
        &mut self,
        unique_key: &str,
        chain_id: i32,
        interval: Interval,
    ) -> Result<Vec<HistoricalChunk>, Error> {
        let now = chrono::Utc::now().timestamp();
        let one_month_ago = now - 30 * 24 * 60 * 60;

        // Chunk into 7-day segments to avoid large responses
        let chunk_duration = 7 * 24 * 60 * 60;
        let mut results = Vec::new();
        let mut start = one_month_ago;

        while start < now {
            let end = (start + chunk_duration).min(now);

            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Vars<'a> {
                unique_key: &'a str,
                chain_id: i32,
                options: TimeseriesOptions,
            }

            let vars = Vars {
                unique_key,
                chain_id,
                options: TimeseriesOptions {
                    start_timestamp: start,
                    end_timestamp: end,
                    interval,
                },
            };

            let response: serde_json::Value =
                self.query(queries::HISTORICAL_MARKET, vars).await?;

            if let Some(market) = response.get("marketByUniqueKey") {
                results.push(HistoricalChunk {
                    start_timestamp: start,
                    end_timestamp: end,
                    data: market.clone(),
                });
            }

            start = end;
        }

        Ok(results)
    }

    /// Fetch all vaults with pagination
    pub async fn fetch_all_vaults(
        &mut self,
        chain_ids: Vec<i32>,
    ) -> Result<Vec<serde_json::Value>, Error> {
        let mut all_vaults = Vec::new();
        let mut skip = 0;
        let page_size = 100;

        loop {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Vars {
                chain_ids: Vec<i32>,
                first: i32,
                skip: i32,
            }

            let vars = Vars {
                chain_ids: chain_ids.clone(),
                first: page_size,
                skip,
            };

            let response: serde_json::Value =
                self.query(queries::VAULT_DATA, vars).await?;
            let temp_vec = vec![];
            let items = response["vaults"]["items"].as_array().unwrap_or(&temp_vec);
            let count = items.len();

            all_vaults.extend(items.iter().cloned());

            if count < page_size as usize {
                break;
            }
            skip += page_size;
        }

        Ok(all_vaults)
    }

    /// Fetch monthly vault history
    pub async fn fetch_monthly_vault_history(
        &mut self,
        address: &str,
        chain_id: i32,
        vault_interval: Interval,
    ) -> Result<Vec<HistoricalChunk>, Error> {
        let now = chrono::Utc::now().timestamp();
        let one_month_ago = now - 30 * 24 * 60 * 60;
        let chunk_duration = 7 * 24 * 60 * 60;
        let mut results = Vec::new();
        let mut start = one_month_ago;

        while start < now {
            let end = (start + chunk_duration).min(now);

            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Vars<'a> {
                address: &'a str,
                chain_id: i32,
                options: TimeseriesOptions,
            }

            let vars = Vars {
                address,
                chain_id,
                options: TimeseriesOptions {
                    start_timestamp: start,
                    end_timestamp: end,
                    interval: vault_interval,
                },
            };

            let response: serde_json::Value =
                self.query(queries::HISTORICAL_VAULT, vars).await?;

            if let Some(vault) = response.get("vaultByAddress") {
                results.push(HistoricalChunk {
                    start_timestamp: start,
                    end_timestamp: end,
                    data: vault.clone(),
                });
            }

            start = end;
        }

        Ok(results)
    }
}
