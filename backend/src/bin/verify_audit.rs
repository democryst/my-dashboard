use sqlx::{postgres::PgPoolOptions, Row};
use sha2::{Sha256, Digest};
use hex;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;

    println!("Starting cryptographic audit chain verification...");

    // Fetch all logs in order of creation
    let logs = sqlx::query(
        "SELECT actor, action, resource, details, prev_row_hash, curr_row_hash 
         FROM audit_log ORDER BY timestamp ASC"
    )
    .fetch_all(&pool)
    .await?;

    let mut expected_prev_hash = "0".repeat(64);
    let mut verified_count = 0;

    for (i, log) in logs.iter().enumerate() {
        let actor: String = log.get("actor");
        let action: String = log.get("action");
        let resource: String = log.get("resource");
        let details: Value = log.get("details");
        let prev_hash_in_db = log.get::<Option<String>, _>("prev_row_hash")
            .unwrap_or_else(|| "0".repeat(64));
        let curr_row_hash: Option<String> = log.get("curr_row_hash");

        // 1. Verify connection to previous block
        if prev_hash_in_db != expected_prev_hash {
            println!("❌ Chain broken at row {}! Expected prev_hash {}, found {}", 
                i, expected_prev_hash, prev_hash_in_db);
            std::process::exit(1);
        }

        // 2. Re-calculate current hash
        let content = format!(
            "{}{}{}{}{}",
            prev_hash_in_db,
            actor,
            action,
            resource,
            details.to_string()
        );

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let calculated_hash = hex::encode(hasher.finalize());

        // 3. Verify current hash
        if Some(calculated_hash.clone()) != curr_row_hash {
            println!("❌ Data integrity violation at row {}! Calculated {}, found {:?}", 
                i, calculated_hash, curr_row_hash);
            std::process::exit(1);
        }

        expected_prev_hash = calculated_hash;
        verified_count += 1;
    }

    println!("✅ Verification successful! {} logs verified with 0 integrity violations.", verified_count);
    Ok(())
}
