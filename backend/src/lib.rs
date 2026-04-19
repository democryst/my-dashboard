pub mod audit;
pub mod error;
pub mod ingest;
pub mod seed;
pub mod dde;

use axum::{
    response::IntoResponse,
    Json,
    extract::Extension,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use dashmap::DashMap;
use hdrhistogram::Histogram;
use chrono::Utc;
use serde_json::json;
use crate::audit::AuditService;

pub struct AppState {
    pub db: PgPool,
    pub audit: AuditService,
    pub metrics: DashMap<String, Histogram<u64>>,
}

pub async fn init_state() -> Arc<AppState> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10) // Increased for separated services
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    let audit_service = AuditService::new(pool.clone());

    Arc::new(AppState {
        db: pool,
        audit: audit_service,
        metrics: DashMap::new(),
    })
}

pub async fn background_metrics_flusher(state: Arc<AppState>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    loop {
        interval.tick().await;
        tracing::info!("Flushing aggregated metrics to database...");

        let mut metrics_to_flush = Vec::new();
        
        for mut entry in state.metrics.iter_mut() {
            let (name, hist) = entry.pair_mut();
            if hist.len() > 0 {
                let p99 = hist.value_at_quantile(0.99);
                let count = hist.len();
                let timestamp = Utc::now();

                metrics_to_flush.push((name.clone(), p99 as f64, count, timestamp));
                
                // Reset for next window
                hist.reset();
            }
        }

        for (name, p99, count, ts) in metrics_to_flush {
            let res = sqlx::query(
                "INSERT INTO metric_entries (name, value, timestamp, attributes)
                 VALUES ($1, $2, $3, $4)"
            )
            .bind(&name)
            .bind(p99)
            .bind(ts)
            .bind(json!({ "tpm": count, "p99": p99 }))
            .execute(&state.db)
            .await;

            if let Err(e) = res {
                tracing::error!("Failed to flush metric {}: {}", name, e);
            }
        }
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

pub async fn ingest_telemetry(
    Extension(_state): Extension<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    tracing::info!("Received telemetry: {:?}", payload);
    Json(json!({ "status": "received" }))
}
