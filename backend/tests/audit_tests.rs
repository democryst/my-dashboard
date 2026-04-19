use backend::audit::calculate_row_hash;
use serde_json::json;

#[test]
fn test_calculate_row_hash_determinism() {
    let prev = "0".repeat(64);
    let details = json!({"path": "/api/v1/test"});
    let h1 = calculate_row_hash(&prev, "user1", "CREATE", "table", &details);
    let h2 = calculate_row_hash(&prev, "user1", "CREATE", "table", &details);
    assert_eq!(h1, h2);
}

#[test]
fn test_hash_chain_sensitivity() {
    let prev = "0".repeat(64);
    let details1 = json!({"ip": "127.0.0.1"});
    let details2 = json!({"ip": "127.0.0.2"}); // Only one character difference
    
    let h1 = calculate_row_hash(&prev, "admin", "LOGIN", "system", &details1);
    let h2 = calculate_row_hash(&prev, "admin", "LOGIN", "system", &details2);
    
    assert_ne!(h1, h2, "Hash must change when details are altered");
}

#[test]
fn test_actor_sensitivity() {
    let prev = "0".repeat(64);
    let details = json!({});
    let h1 = calculate_row_hash(&prev, "admin", "LOGIN", "system", &details);
    let h2 = calculate_row_hash(&prev, "hacker", "LOGIN", "system", &details);
    assert_ne!(h1, h2);
}
