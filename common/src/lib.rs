use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub service_name: String,
    pub level: String,
    pub message: String,
    pub attributes: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricEntry {
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub attributes: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggregatedMetrics {
    pub p99_latency: f64,
    pub tpm: f64,
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicTable {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub config: serde_json::Value,
    pub is_visible: bool,
    pub index_config: serde_json::Value, // Array of field mappings
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryFilter {
    pub field: String,
    pub operator: Operator,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    #[serde(rename = "=")]
    Eq,
    #[serde(rename = ">")]
    Gt,
    #[serde(rename = "<")]
    Lt,
    #[serde(rename = ">=")]
    Gte,
    #[serde(rename = "<=")]
    Lte,
}

impl Operator {
    pub fn as_sql(&self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::Gt => ">",
            Self::Lt => "<",
            Self::Gte => ">=",
            Self::Lte => "<=",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggregationRequest {
    pub field: String,
    pub function: String, // SUM, AVG, MIN, MAX, COUNT
    pub interval: String, // 5 minutes, 1 hour, etc.
    pub filters: Vec<QueryFilter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggregationResult {
    pub bucket: DateTime<Utc>,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexRequest {
    pub field_path: String,
    pub index_type: String, // B-TREE, GIN
}
