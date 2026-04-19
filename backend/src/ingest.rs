use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::AppState;
use crate::error::AppError;
use uuid::Uuid;
use chrono::Utc;

pub async fn ingest_otlp_logs(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, AppError> {
    // Basic OTLP Log mapping logic
    // Expecting OTLP JSON structure: { "resourceLogs": [ ... ] }
    
    let mut entries_inserted = 0;

    if let Some(resource_logs) = payload.get("resourceLogs").and_then(|v| v.as_array()) {
        for res_log in resource_logs {
            let service_name = res_log
                .get("resource")
                .and_then(|r| r.get("attributes"))
                .and_then(|a| a.as_array())
                .and_then(|a| a.iter().find(|attr| attr["key"] == "service.name"))
                .and_then(|s| s["value"]["stringValue"].as_str())
                .unwrap_or("unknown-service")
                .to_string();

            if let Some(scope_logs) = res_log.get("scopeLogs").and_then(|v| v.as_array()) {
                for scope_log in scope_logs {
                    if let Some(log_records) = scope_log.get("logRecords").and_then(|v| v.as_array()) {
                        for record in log_records {
                            let message = record.get("body").and_then(|b| b["stringValue"].as_str()).unwrap_or("");
                            let level = record.get("severityText").and_then(|s| s.as_str()).unwrap_or("INFO");
                            let timestamp_nanos = record.get("timeUnixNano").and_then(|t| t.as_str()).and_then(|s| s.parse::<i64>().ok()).unwrap_or_else(|| Utc::now().timestamp_nanos_opt().unwrap_or(0));
                            
                            // Convert nanos to DateTime
                            let timestamp = chrono::DateTime::from_timestamp(timestamp_nanos / 1_000_000_000, (timestamp_nanos % 1_000_000_000) as u32).unwrap_or_default();

                            let attributes = record.get("attributes").cloned().unwrap_or(json!([]));

                            // Insert into Postgres
                            sqlx::query(
                                "INSERT INTO log_entries (id, timestamp, service_name, level, message, attributes)
                                 VALUES ($1, $2, $3, $4, $5, $6)"
                            )
                            .bind(Uuid::new_v4())
                            .bind(timestamp)
                            .bind(&service_name)
                            .bind(level)
                            .bind(message)
                            .bind(attributes)
                            .execute(&state.db)
                            .await?;

                            entries_inserted += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(Json(json!({ 
        "status": "success", 
        "processed": entries_inserted 
    })))
}

pub async fn ingest_otlp_metrics(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, AppError> {
    // Basic OTLP Metrics mapping logic
    // Expecting OTLP JSON structure: { "resourceMetrics": [ ... ] }

    if let Some(resource_metrics) = payload.get("resourceMetrics").and_then(|v| v.as_array()) {
        for res_metric in resource_metrics {
            if let Some(scope_metrics) = res_metric.get("scopeMetrics").and_then(|v| v.as_array()) {
                for scope_metric in scope_metrics {
                    if let Some(metrics) = scope_metric.get("metrics").and_then(|v| v.as_array()) {
                        for metric in metrics {
                            let name = metric.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                            
                            // Handling Gauge or Sum (simple numeric values)
                            if let Some(datapoints) = metric.get("gauge").or(metric.get("sum")).and_then(|g| g.get("dataPoints")).and_then(|d| d.as_array()) {
                                for dp in datapoints {
                                    if let Some(val) = dp.get("asDouble").and_then(|v| v.as_f64()).or_else(|| dp.get("asInt").and_then(|v| v.as_i64()).map(|i| i as f64)) {
                                        // Update HdrHistogram for this metric name
                                        // Histogram stores u64, so we convert ms to u64
                                        let mut hist = state.metrics.entry(name.to_string()).or_insert_with(|| {
                                            hdrhistogram::Histogram::<u64>::new_with_bounds(1, 60000, 3).unwrap()
                                        });

                                        // Record value (clamping to 1ms minimum for the histogram)
                                        hist.record(val.max(1.0) as u64).ok();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(Json(json!({ "status": "success" })))
}
