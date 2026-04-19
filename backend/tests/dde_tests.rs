use backend::dde::{is_safe_field_path, is_allowed_function, DdeQueryBuilder};
use common::{AggregationRequest, Operator, QueryFilter};
use uuid::Uuid;

#[test]
fn test_field_path_safety() {
    assert!(is_safe_field_path("temperature"));
    assert!(is_safe_field_path("sensor_01"));
    assert!(!is_safe_field_path("temp; DROP TABLE users;"));
    assert!(!is_safe_field_path("payload->'field'"));
    assert!(!is_safe_field_path(""));
}

#[test]
fn test_aggregation_function_whitelist() {
    assert!(is_allowed_function("AVG"));
    assert!(is_allowed_function("sum"));
    assert!(is_allowed_function("MAX"));
    assert!(!is_allowed_function("DELETE"));
    assert!(!is_allowed_function("pg_sleep(10)"));
}

#[test]
fn test_query_builder_params() {
    let table_id = Uuid::new_v4();
    let req = AggregationRequest {
        interval: "1m".to_string(),
        field: "temp".to_string(),
        function: "AVG".to_string(),
        filters: vec![
            QueryFilter {
                field: "status".to_string(),
                operator: Operator::Eq,
                value: "ok".to_string(),
            }
        ],
    };

    let builder = DdeQueryBuilder::new(table_id, req);
    let (sql, params) = builder.build().unwrap();

    assert!(sql.contains("AVG"));
    assert!(sql.contains("WHERE table_id = $3"));
    assert!(sql.contains("AND (payload->>'status') = $4"));
    assert_eq!(params.len(), 4);
    assert_eq!(params[0], "1m");
    assert_eq!(params[1], "temp");
    assert_eq!(params[2], table_id.to_string());
    assert_eq!(params[3], "ok");
}
