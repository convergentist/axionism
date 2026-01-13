use serde::Deserialize;
use std::{error::Error, fs};
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub exchange: Vec<ExchangeConfig>,
    pub credentials: Vec<CredentialsConfig>,
    pub feeds: Vec<FeedsConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExchangeConfig {
    pub name: String,
    pub category: String,
    pub base_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CredentialsConfig {
    pub key: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FeedsConfig {
    pub wss: Vec<WssConfig>,
    pub rest: Vec<RestConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WssConfig {
    pub api_version: String,
    pub public_url: String,
    pub private_url: String,
    pub testnet: String,
    pub recv_window: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RestConfig {
    pub api_version: String,
    pub base_url: String,
    pub interval_seconds: u64,
    pub retry_attempts: u64,
    pub timeout_seconds: u64,
}

impl Config {
    pub fn from_toml(file_route: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file_route)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
