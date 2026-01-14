use crate::sources::morpho::responses::base::{Asset, PageInfo};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Deserializes a field that can be either a JSON string or number into String
fn string_or_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<Value> = Option::deserialize(deserializer)?;
    Ok(opt.map(|v| match v {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        _ => v.to_string(),
    }))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationMarket {
    pub unique_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub market: Option<AllocationMarket>,
    #[serde(deserialize_with = "string_or_number", default)]
    pub supply_assets: Option<String>,
    pub supply_assets_usd: Option<f64>,
    #[serde(deserialize_with = "string_or_number", default)]
    pub supply_cap: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultState {
    #[serde(deserialize_with = "string_or_number", default)]
    pub total_assets: Option<String>,
    pub total_assets_usd: Option<f64>,
    #[serde(deserialize_with = "string_or_number", default)]
    pub total_supply: Option<String>,
    pub fee: Option<f64>,
    pub apy: Option<f64>,
    pub net_apy: Option<f64>,
    pub allocation: Option<Vec<Allocation>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub id: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultItem {
    pub address: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub chain: Option<Chain>,
    pub asset: Option<Asset>,
    pub state: Option<VaultState>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultsResult {
    pub items: Vec<VaultItem>,
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VaultStateResponse {
    pub vaults: VaultsResult,
}
