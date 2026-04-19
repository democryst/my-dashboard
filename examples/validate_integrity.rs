use sha2::{Sha256, Digest};
use hex;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
struct AuditLog {
    actor: String,
    action: String,
    resource: String,
    details: Value,
    prev_row_hash: String,
    curr_row_hash: String,
}

fn calculate_hash(prev_hash: &str, actor: &str, action: &str, resource: &str, details: &Value) -> String {
    let content = format!(
        "{}{}{}{}{}",
        prev_hash,
        actor,
        action,
        resource,
        details.to_string()
    );
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}

fn main() {
    println!("--- LOGSTREAM SECURITY VALIDATION SIMULATION ---");

    // 1. SEEDING VALID DATA
    println!("\n[1/3] Seeding valid cryptographic chain...");
    let mut chain: Vec<AuditLog> = Vec::new();

    // Block 0 (Genesis)
    let genesis_prev = "0".repeat(64);
    let details0 = json!({"ip": "127.0.0.1"});
    let hash0 = calculate_hash(&genesis_prev, "admin", "LOGIN", "system", &details0);
    chain.push(AuditLog {
        actor: "admin".to_string(),
        action: "LOGIN".to_string(),
        resource: "system".to_string(),
        details: details0,
        prev_row_hash: genesis_prev,
        curr_row_hash: hash0,
    });

    // Block 1 (Linked to 0)
    let details1 = json!({"file": "config.yaml"});
    let hash1 = calculate_hash(&chain[0].curr_row_hash, "admin", "UPDATE", "config", &details1);
    chain.push(AuditLog {
        actor: "admin".to_string(),
        action: "UPDATE".to_string(),
        resource: "config".to_string(),
        details: details1,
        prev_row_hash: chain[0].curr_row_hash.clone(),
        curr_row_hash: hash1,
    });

    println!("✅ Seeded {} valid blocks.", chain.len());

    // 2. VERIFYING VALID CHAIN
    println!("\n[2/3] Verifying integrity of untouched chain...");
    let mut expected_prev = "0".repeat(64);
    let mut success = true;

    for (i, log) in chain.iter().enumerate() {
        let calc = calculate_hash(&log.prev_row_hash, &log.actor, &log.action, &log.resource, &log.details);
        if log.prev_row_hash != expected_prev || log.curr_row_hash != calc {
            println!("❌ Block {} failed verification!", i);
            success = false;
            break;
        }
        expected_prev = log.curr_row_hash.clone();
    }

    if success {
        println!("✅ Chain integrity verified! All hashes align.");
    }

    // 3. SIMULATING TAMPERING
    println!("\n[3/3] Simulating unauthorized data alteration...");
    println!("Modifying 'actor' in Block 0 from 'admin' to 'hacker'...");
    
    // DELIBERATE TAMPERING
    chain[0].actor = "hacker".to_string();

    println!("Re-running verification...");
    let mut expected_prev_tamper = "0".repeat(64);
    let mut tamper_detected = false;

    for (i, log) in chain.iter().enumerate() {
        let calc = calculate_hash(&log.prev_row_hash, &log.actor, &log.action, &log.resource, &log.details);
        if log.prev_row_hash != expected_prev_tamper || log.curr_row_hash != calc {
            println!("🚨 BREACH DETECTED: Integrity violation at Block {}!", i);
            println!("   Reason: Data hash mismatch. The record has been altered after signing.");
            tamper_detected = true;
            break;
        }
        expected_prev_tamper = log.curr_row_hash.clone();
    }

    if tamper_detected {
        println!("\n✅ SECURITY VALIDATION PASSED: The system successfully detected the tamper attempt.");
    } else {
        println!("\n❌ SECURITY VALIDATION FAILED: The system failed to detect the modification.");
    }
}
