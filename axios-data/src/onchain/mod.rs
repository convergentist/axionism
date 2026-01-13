use serde::{Deserialize, Serialize};
pub mod graphs;

#[derive(Debug, Serialize)]
pub struct GraphQLRequest<V: Serialize> {
    pub query: &'static str,
    pub variables: V,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesOptions {
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub interval: Interval,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Interval {
    Minute,
    FiveMinutes,
    FifteenMinutes,
    HalfHour,
    Hour,
    Day,
    Week,
    Month,
}

#[derive(Debug, Deserialize)]
pub struct TimeseriesPoint {
    pub x: i64, // timestamp
    pub y: f64, // value
}

#[derive(Debug)]
pub struct HistoricalChunk {
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub data: serde_json::Value,
}
