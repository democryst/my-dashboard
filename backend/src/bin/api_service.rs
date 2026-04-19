use axum::{
    routing::{post, patch, get},
    Router, Extension,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use backend::init_state;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = init_state().await;

    let app = Router::new()
        .route("/health", get(backend::health_check))
        .route("/v1/tables", post(backend::dde::create_table).get(backend::dde::list_tables))
        .route("/v1/tables/:table_id/visibility", patch(backend::dde::set_table_visibility))
        .route("/v1/tables/:table_id/indexes", post(backend::dde::add_table_index))
        .route("/v1/data/:table_id", post(backend::dde::insert_data))
        .route("/v1/aggregate/:table_id", post(backend::dde::aggregate_data))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(state));

    let port = std::env::var("API_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    
    tracing::info!("LogStream API & Discovery Service listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
