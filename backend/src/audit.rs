use sqlx::PgPool;
use sha2::{Sha256, Digest};
use hex;
// use chrono::Utc;
use serde_json::Value;

#[derive(Clone)]
pub struct AuditService {
    db: PgPool,
}

impl AuditService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn log(
        &self,
        actor: &str,
        action: &str,
        resource: &str,
        details: Value,
    ) -> Result<(), sqlx::Error> {
        // 1. Get the last hash in the chain
        let last_hash: Option<String> = sqlx::query_scalar(
            "SELECT curr_row_hash FROM audit_log ORDER BY timestamp DESC LIMIT 1"
        )
        .fetch_optional(&self.db)
        .await?;

        let prev_hash = last_hash.unwrap_or_else(|| "0".repeat(64));

        // 2. Construct content for hashing
        let content = format!(
            "{}{}{}{}{}",
            prev_hash,
            actor,
            action,
            resource,
            details.to_string()
        );

        // 3. Calculate current row hash
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let curr_hash = hex::encode(hasher.finalize());

        // 4. Insert into DB
        sqlx::query(
            "INSERT INTO audit_log (actor, action, resource, details, prev_row_hash, curr_row_hash)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(actor)
        .bind(action)
        .bind(resource)
        .bind(details)
        .bind(&prev_hash)
        .bind(&curr_hash)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
