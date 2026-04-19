use axum::{
    extract::{Multipart, Extension},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use crate::AppState;
use crate::error::AppError;
use uuid::Uuid;
use chrono::Utc;
use csv::ReaderBuilder;
use futures_util::StreamExt;

pub async fn upload_seed_file(
    Extension(state): Extension<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    while let Some(field) = multipart.next_field().await.map_err(|_| AppError::Internal)? {
        let name = field.name().unwrap_or("file").to_string();
        let filename = field.file_name().unwrap_or("unknown").to_string();

        if name == "file" {
            tracing::info!("Starting seed job for file: {}", filename);
            
            // 1. Create a Pending Job in DB
            let job_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO seed_jobs (id, filename, status) VALUES ($1, $2, $3)"
            )
            .bind(job_id)
            .bind(&filename)
            .bind("PROCESSING")
            .execute(&state.db)
            .await?;

            // 2. Process the stream in the background
            let db = state.db.clone();
            let stream = field.bytes().await.map_err(|_| AppError::Internal)?;
            
            tokio::spawn(async move {
                if let Err(e) = process_seed_data(job_id, &filename, stream, db).await {
                    tracing::error!("Seed job {} failed: {}", job_id, e);
                }
            });

            return Ok(Json(json!({ "status": "processing", "job_id": job_id })));
        }
    }

    Err(AppError::Internal)
}

async fn process_seed_data(
    job_id: Uuid, 
    _filename: &str, 
    data: bytes::Bytes, 
    db: sqlx::PgPool
) -> Result<(), AppError> {
    // Determine format based on extension or content
    // For now, assuming CSV
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(&data[..]);

    let mut count = 0;
    for result in reader.deserialize() {
        if let Ok(record) = result {
            let record: serde_json::Value = record;
            
            // Map CSV record to LogEntry
            let timestamp = Utc::now(); // Default if not in CSV
            
            sqlx::query(
                "INSERT INTO log_entries (id, timestamp, service_name, level, message, attributes)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(Uuid::new_v4())
            .bind(timestamp)
            .bind("seed-import")
            .bind("INFO")
            .bind("Imported from seed file")
            .bind(record)
            .execute(&db)
            .await?;

            count += 1;
            
            // Update progress every 1000 records
            if count % 1000 == 0 {
                let _ = sqlx::query("UPDATE seed_jobs SET processed_records = $1 WHERE id = $2")
                    .bind(count as i32)
                    .bind(job_id)
                    .execute(&db)
                    .await;
            }
        }
    }

    // 3. Mark as Completed
    sqlx::query("UPDATE seed_jobs SET status = 'COMPLETED', processed_records = $1, total_records = $1 WHERE id = $2")
        .bind(count as i32)
        .bind(job_id)
        .execute(&db)
        .await?;

    Ok(())
}
