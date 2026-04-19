use axum::{
    routing::post,
    Router, Extension,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use backend::{init_state, background_metrics_flusher, ingest_telemetry};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = init_state().await;

    // Start background metrics flusher (ingest service computes metrics)
    let flusher_state = state.clone();
    tokio::spawn(async move {
        background_metrics_flusher(flusher_state).await;
    });

    let app = Router::new()
        .route("/health", axum::routing::get(backend::health_check))
        .route("/v1/ingest", post(ingest_telemetry))
        .route("/v1/logs", post(backend::ingest::ingest_otlp_logs))
        .route("/v1/metrics", post(backend::ingest::ingest_otlp_metrics))
        .route("/v1/seed", post(backend::seed::upload_seed_file))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(state));

    let port = std::env::var("INGEST_PORT").unwrap_or_else(|_| "8081".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    
    tracing::info!("LogStream Ingest Service listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
