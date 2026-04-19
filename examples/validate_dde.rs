use serde_json::{json, Value};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

#[derive(Debug)]
struct DynamicRecord {
    table_id: String,
    timestamp: DateTime<Utc>,
    payload: Value,
}

fn simulate_aggregation(
    data: &[DynamicRecord],
    field: &str,
    function: &str,
    interval_mins: i64,
) -> HashMap<i64, f64> {
    let mut buckets: HashMap<i64, Vec<f64>> = HashMap::new();

    for record in data {
        // Use a fixed epoch start for this test to ensure alignment
        let ts_sec = record.timestamp.timestamp();
        let bucket_id = ts_sec / (interval_mins * 60);

        if let Some(val) = record.payload.get(field).and_then(|v| v.as_f64()) {
            buckets.entry(bucket_id).or_default().push(val);
        }
    }

    let mut results = HashMap::new();
    for (bucket_id, values) in buckets {
        let agg_val = match function.to_uppercase().as_str() {
            "SUM" => values.iter().sum(),
            "AVG" => values.iter().sum::<f64>() / values.len() as f64,
            "MAX" => values.iter().fold(f64::MIN, |a, &b| a.max(b)),
            "MIN" => values.iter().fold(f64::MAX, |a, &b| a.min(b)),
            _ => 0.0,
        };
        results.insert(bucket_id, agg_val);
    }

    results
}

fn main() {
    println!("--- LOGSTREAM DDE VALIDATION SIMULATION ---");

    // 1. DATA SEEDING (Simulated 10,000 records)
    println!("\n[1/3] Seeding 10,000 simulated IoT records...");
    let mut data = Vec::new();
    // Use an aligned start time (divisible by 300s for 5m buckets)
    let start_ts = 1713500100; // Aligned point
    let start_time = DateTime::from_timestamp(start_ts, 0).unwrap().with_timezone(&Utc);

    for i in 0..10000 {
        data.push(DynamicRecord {
            table_id: "iot_sensors".to_string(),
            timestamp: start_time + chrono::Duration::seconds(i * 2), // Every 2 seconds
            payload: json!({
                "temp": 20.0 + (i as f64 * 0.01),
                "humidity": 45.0 + (i as f64 * 0.005)
            }),
        });
    }
    println!("✅ Generated {} records spanning {} hours.", data.len(), (10000 * 2) / 3600);

    // 2. AGGREGATION EXECUTION
    println!("\n[2/3] Executing 5-minute AVG aggregation on field 'temp'...");
    let results = simulate_aggregation(&data, "temp", "AVG", 5);

    println!("✅ Computed {} time buckets.", results.len());

    // 3. RESULT VERIFICATION
    println!("\n[3/3] Verifying mathematical accuracy...");
    
    // Check first bucket (aligned to 5 mins)
    let first_bucket_id = start_ts / (5 * 60);
    if let Some(&avg_val) = results.get(&first_bucket_id) {
        println!("   Bucket {}: AVG Temp = {:.4}", first_bucket_id, avg_val);
        
        // Manual check for first 5 mins (150 records at 2s interval)
        let expected_avg: f64 = (0..150).map(|i| 20.0 + (i as f64 * 0.01)).sum::<f64>() / 150.0;
        if (avg_val - expected_avg).abs() < 0.0001 {
            println!("   ✅ Result matches theoretical expectation ({:.4} vs {:.4})", avg_val, expected_avg);
            println!("\n✅ DDE VALIDATION SUCCESSFUL: Dynamic aggregation logic verified.");
        } else {
            println!("   ❌ Result mismatch! Expected {:.4}", expected_avg);
            std::process::exit(1);
        }
    } else {
        println!("   ❌ Could not find bucket {} for verification!", first_bucket_id);
        std::process::exit(1);
    }
}
