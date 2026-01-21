use serde::Serialize;
use std::error::Error;

use axios_data::{
    onchain::{Interval, TimeseriesOptions, graphs::queries},
    sources::morpho::MorphoClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = MorphoClient::new();
    let chains = vec![1, 8453]; // Ethereum + Base

    // =========================================================================
    // TABLE 2: Market State
    // =========================================================================

    println!("=== TABLE 2: Market State ===");
    let markets = client.fetch_all_markets(chains.clone()).await?;
    println!("Fetched {} markets\n", markets.len());

    // Store market keys for historical queries
    let market_keys: Vec<String> = markets
        .iter()
        .filter_map(|m| m["uniqueKey"].as_str().map(String::from))
        .take(2) // Limit for demo
        .collect();

    // =========================================================================
    // TABLE 3: IRM State (per market)
    // =========================================================================

    println!("=== TABLE 3: IRM State ===");
    for key in &market_keys {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Vars<'a> {
            unique_key: &'a str,
            chain_id: i32,
            curve_points: i32,
        }

        let irm: serde_json::Value = client
            .query(
                queries::IRM_STATE,
                Vars {
                    unique_key: key,
                    chain_id: 1,
                    curve_points: 20,
                },
            )
            .await?;

        if let Some(market) = irm.get("marketByUniqueKey") {
            println!(
                "Market {}: borrowApy={}, supplyApy={}",
                &key[..2],
                market["state"]["borrowApy"],
                market["state"]["supplyApy"]
            );
        }
    }
    println!();

    // =========================================================================
    // TABLE 4: Derived Metrics
    // =========================================================================

    println!("=== TABLE 4: Derived Metrics ===");
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct PaginatedVars {
        chain_ids: Vec<i32>,
        first: i32,
        skip: i32,
    }

    let metrics: serde_json::Value = client
        .query(
            queries::DERIVED_METRICS,
            PaginatedVars {
                chain_ids: chains.clone(),
                first: 10,
                skip: 0,
            },
        )
        .await?;

    if let Some(items) = metrics["markets"]["items"].as_array() {
        for item in items {
            println!(
                "Market {}: utilization={}, avgNetSupplyApy={}",
                &item["uniqueKey"].as_str().unwrap_or("?")[..10],
                item["state"]["utilization"],
                item["state"]["avgNetSupplyApy"]
            );
        }
    }
    println!();

    // =========================================================================
    // TABLE 6: Oracle Data
    // =========================================================================
    println!("=== TABLE 6: Oracle Data ===");
    let oracles: serde_json::Value = client
        .query(
            queries::ORACLE_DATA,
            PaginatedVars {
                chain_ids: chains.clone(),
                first: 10,
                skip: 0,
            },
        )
        .await?;

    if let Some(items) = oracles["markets"]["items"].as_array() {
        for item in items {
            println!(
                "Market {}: oracle={}, type={:?}",
                &item["uniqueKey"].as_str().unwrap_or("?")[..10],
                item["oracleAddress"],
                item["oracle"]["type"]
            );
        }
    }
    println!();

    // =========================================================================
    // TABLE 7: Collateral Data
    // =========================================================================
    println!("=== TABLE 7: Collateral Data ===");
    let collateral: serde_json::Value = client
        .query(
            queries::COLLATERAL_DATA,
            PaginatedVars {
                chain_ids: chains.clone(),
                first: 10,
                skip: 0,
            },
        )
        .await?;

    if let Some(items) = collateral["markets"]["items"].as_array() {
        for item in items {
            if let Some(asset) = item["collateralAsset"].as_object() {
                println!(
                    "Market {}: collateral={}, lltv={}, usd={}",
                    &item["uniqueKey"].as_str().unwrap_or("?")[..10],
                    asset.get("symbol").and_then(|s| s.as_str()).unwrap_or("?"),
                    item["lltv"],
                    item["state"]["collateralAssetsUsd"]
                );
            }
        }
    }
    println!();

    // =========================================================================
    // TABLE 10: Vault Data
    // =========================================================================
    println!("=== TABLE 10: Vault Data ===");
    let vaults = client.fetch_all_vaults(chains.clone()).await?;
    println!("Fetched {} vaults", vaults.len());

    let vault_addresses: Vec<String> = vaults
        .iter()
        .filter_map(|v| v["address"].as_str().map(String::from))
        .take(3)
        .collect();

    for vault in vaults.iter().take(5) {
        println!(
            "Vault {}: {} - TVL=${}, netApy={}",
            vault["address"].as_str().unwrap_or("?"),
            vault["name"].as_str().unwrap_or("?"),
            vault["state"]["totalAssetsUsd"],
            vault["state"]["netApy"]
        );
    }
    println!();

    // =========================================================================
    // TABLE 11: Vault Strategy
    // =========================================================================
    println!("=== TABLE 11: Vault Strategy ===");
    for addr in &vault_addresses {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct VaultVars<'a> {
            address: &'a str,
            chain_id: i32,
        }

        let strategy: serde_json::Value = client
            .query(
                queries::VAULT_STRATEGY,
                VaultVars {
                    address: addr,
                    chain_id: 1,
                },
            )
            .await?;

        if let Some(vault) = strategy.get("vaultByAddress") {
            let queue_len = vault["supplyQueue"]
                .as_array()
                .map(|a| a.len())
                .unwrap_or(0);
            println!(
                "Vault {}: timelock={}, supplyQueue.len={}",
                &addr[..10],
                vault["timelock"],
                queue_len
            );
        }
    }
    println!();

    // =========================================================================
    // TABLE 15: Historical Market Data (1 month, chunked)
    // =========================================================================
    println!("=== TABLE 15: Historical Market Data ===");
    if let Some(key) = market_keys.first() {
        let history = client.fetch_monthly_history(key, 1, Interval::Hour).await?;

        let total_points: usize = history
            .iter()
            .filter_map(|chunk| {
                chunk.data["historicalState"]["supplyApy"]
                    .as_array()
                    .map(|a| a.len())
            })
            .sum();

        println!(
            "Market {}: {} chunks, ~{} hourly data points",
            &key[..10],
            history.len(),
            total_points
        );
    }
    println!();

    // =========================================================================
    // TABLE 15: Historical Vault Data (1 month)
    // =========================================================================
    println!("=== TABLE 15: Historical Vault Data ===");
    if let Some(addr) = vault_addresses.first() {
        let vault_history = client
            .fetch_monthly_vault_history(addr, 1, Interval::Hour)
            .await?;

        let total_points: usize = vault_history
            .iter()
            .filter_map(|chunk| {
                chunk.data["historicalState"]["apy"]
                    .as_array()
                    .map(|a| a.len())
            })
            .sum();

        println!(
            "Vault {}: {} chunks, aprox {} hourly data points",
            &addr[..10],
            vault_history.len(),
            total_points
        );
    }
    println!();

    // =========================================================================
    // TABLE 16: Historical Asset Price
    // =========================================================================
    println!("=== TABLE 16: Historical Asset Price ===");
    let wsteth = "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0";
    let now = chrono::Utc::now().timestamp();

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct AssetVars<'a> {
        address: &'a str,
        chain_id: i32,
        options: TimeseriesOptions,
    }

    let price_history: serde_json::Value = client
        .query(
            queries::HISTORICAL_ASSET_PRICE,
            AssetVars {
                address: wsteth,
                chain_id: 1,
                options: TimeseriesOptions {
                    start_timestamp: now - 7 * 24 * 60 * 60, // 1 week
                    end_timestamp: now,
                    interval: Interval::Hour,
                },
            },
        )
        .await?;

    if let Some(asset) = price_history.get("assetByAddress") {
        let points = asset["historicalPriceUsd"]
            .as_array()
            .map(|a| a.len())
            .unwrap_or(0);
        println!(
            "Asset {}: {} hourly price points",
            asset["symbol"].as_str().unwrap_or("?"),
            points
        );
    }

    println!("\n=== Complete ===");
    Ok(())
}
