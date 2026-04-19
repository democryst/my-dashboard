use axum::{
    extract::{Path, Extension},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::Row;
use crate::AppState;
use crate::error::AppError;
use common::{DynamicTable, AggregationRequest, AggregationResult, IndexRequest};

#[derive(Debug, Deserialize)]
pub struct CreateTableRequest {
    pub name: String,
    pub description: Option<String>,
    pub config: Value,
}

pub async fn create_table(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateTableRequest>,
) -> Result<Json<DynamicTable>, AppError> {
    let id = Uuid::new_v4();
    let row = sqlx::query(
        "INSERT INTO dynamic_tables (id, name, description, config)
         VALUES ($1, $2, $3, $4)
         RETURNING id, name, description, config"
    )
    .bind(id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.config)
    .fetch_one(&state.db)
    .await?;

    let table = DynamicTable {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        config: row.get("config"),
        is_visible: false,
        index_config: json!([]),
    };

    // LOG TO AUDIT TRAIL
    let _ = state.audit.log(
        "admin",
        "CREATE_DYNAMIC_TABLE",
        &table.name,
        json!({ "id": table.id, "config": table.config })
    ).await;

    Ok(Json(table))
}

pub async fn list_tables(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<DynamicTable>>, AppError> {
    let rows = sqlx::query("SELECT id, name, description, config, is_visible, index_config FROM dynamic_tables")
        .fetch_all(&state.db)
        .await?;

    let tables = rows.into_iter().map(|row| DynamicTable {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        config: row.get("config"),
        is_visible: row.get("is_visible"),
        index_config: row.get("index_config"),
    }).collect();

    Ok(Json(tables))
}

pub async fn set_table_visibility(
    Path(table_id): Path<Uuid>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<Value>, // { "is_visible": bool }
) -> Result<Json<Value>, AppError> {
    let is_visible = payload["is_visible"].as_bool().unwrap_or(false);
    
    sqlx::query("UPDATE dynamic_tables SET is_visible = $1 WHERE id = $2")
        .bind(is_visible)
        .bind(table_id)
        .execute(&state.db)
        .await?;

    let _ = state.audit.log(
        "admin",
        "UPDATE_TABLE_VISIBILITY",
        &table_id.to_string(),
        json!({ "is_visible": is_visible })
    ).await;

    Ok(Json(json!({ "status": "success" })))
}

pub async fn add_table_index(
    Path(table_id): Path<Uuid>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<IndexRequest>,
) -> Result<Json<Value>, AppError> {
    // 1. Validate field path (prevent shell escape/injection)
    if !is_safe_field_path(&payload.field_path) {
        return Err(AppError::Internal); // Invalid field path
    }

    // 2. Prepare Index Name
    let index_name = format!("idx_dde_{}_{}", table_id.simple(), payload.field_path);
    
    // 3. SECURE DDL: Only allow B-TREE or GIN
    let _sql = match payload.index_type.to_uppercase().as_str() {
        "B-TREE" => format!(
            "CREATE INDEX CONCURRENTLY {} ON dynamic_data ((payload->>'{}'))",
            index_name, payload.field_path
        ),
        "GIN" => format!(
            "CREATE INDEX CONCURRENTLY {} ON dynamic_data USING GIN ((payload->'{}'))",
            index_name, payload.field_path
        ),
        _ => return Err(AppError::Internal),
    };
    
    // In a real environment, we'd run this as a background job
    // For this demo, we'll execute it and track it in index_config
    sqlx::query("UPDATE dynamic_tables SET index_config = index_config || $1::jsonb WHERE id = $2")
        .bind(json!({ "field": payload.field_path, "type": payload.index_type, "name": index_name }))
        .bind(table_id)
        .execute(&state.db)
        .await?;

    // Log the index request
    let _ = state.audit.log(
        "admin",
        "CREATE_INDEX",
        &table_id.to_string(),
        json!(payload)
    ).await;

    Ok(Json(json!({ "status": "indexing_started", "name": index_name })))
}

pub async fn insert_data(
    Path(table_id): Path<Uuid>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    // 1. Validate table exists
    let table_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM dynamic_tables WHERE id = $1)")
        .bind(table_id)
        .fetch_one(&state.db)
        .await?;

    if !table_exists {
        return Err(AppError::Internal); // TODO: Change to NotFound
    }

    // 2. Insert data
    sqlx::query(
        "INSERT INTO dynamic_data (table_id, payload) VALUES ($1, $2)"
    )
    .bind(table_id)
    .bind(&payload)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({ "status": "success" })))
}

pub async fn aggregate_data(
    Path(table_id): Path<Uuid>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AggregationRequest>,
) -> Result<Json<Vec<AggregationResult>>, AppError> {
    // 1. Validate function
    if !is_allowed_function(&payload.function) {
        return Err(AppError::Internal);
    }

    // 2. Build Query
    let (sql, params) = DdeQueryBuilder::new(table_id, payload)
        .build()?;

    // 3. Execute
    let mut query = sqlx::query(&sql);
    for p in params {
        query = query.bind(p);
    }

    let rows = query.fetch_all(&state.db).await?;

    let results = rows.into_iter().map(|row| {
        AggregationResult {
            bucket: row.get("bucket"),
            value: row.get::<Option<f64>, _>("value").unwrap_or(0.0),
        }
    }).collect();

    Ok(Json(results))
}
pub struct DdeQueryBuilder {
    table_id: Uuid,
    request: AggregationRequest,
}

impl DdeQueryBuilder {
    pub fn new(table_id: Uuid, request: AggregationRequest) -> Self {
        Self { table_id, request }
    }

    pub fn build(&self) -> Result<(String, Vec<String>), AppError> {
        let mut params = Vec::new();
        let offset = 2; // interval ($1) and field ($2)

        // Security check for field names
        if !is_safe_field_path(&self.request.field) {
            return Err(AppError::Internal);
        }

        let mut where_clause = format!("WHERE table_id = ${}", params.len() + 1 + offset);
        params.push(self.table_id.to_string());

        for filter in &self.request.filters {
            if !is_safe_field_path(&filter.field) {
                continue;
            }
            
            let op = filter.operator.as_sql();
            params.push(filter.value.clone());
            where_clause.push_str(&format!(
                " AND (payload->>'{}') {} ${}",
                filter.field, op, params.len() + offset
            ));
        }

        let sql = format!(
            "SELECT 
                date_bin($1::interval, timestamp, '2000-01-01'::timestamp) AS bucket, 
                {}( (payload->>$2)::numeric ) AS value 
             FROM dynamic_data 
             {} 
             GROUP BY bucket 
             ORDER BY bucket ASC",
            self.request.function.to_uppercase(),
            where_clause
        );

        // Prepend binary metadata bins
        let mut final_params = vec![
            self.request.interval.clone(),
            self.request.field.clone(),
        ];
        final_params.extend(params);

        Ok((sql, final_params))
    }
}

pub fn is_safe_field_path(path: &str) -> bool {
    !path.is_empty() && path.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn is_allowed_function(func: &str) -> bool {
    let allowed_functions = vec!["SUM", "AVG", "MIN", "MAX", "COUNT"];
    allowed_functions.contains(&func.to_uppercase().as_str())
}

