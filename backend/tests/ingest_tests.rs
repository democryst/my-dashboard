use backend::ingest::record_metric;
use dashmap::DashMap;

#[test]
fn test_metric_precision_recording() {
    let metrics = DashMap::new();
    
    // Record 100ms latency
    record_metric(&metrics, "api_latency", 100.4);
    
    // Record 500ms latency
    record_metric(&metrics, "api_latency", 500.8);

    let entry = metrics.get("api_latency").unwrap();
    let hist = entry.value();
    
    assert_eq!(hist.len(), 2);
    assert!(hist.value_at_quantile(0.5) >= 100);
    assert!(hist.value_at_quantile(1.0) >= 500);
}

#[test]
fn test_metric_clamping() {
    let metrics = DashMap::new();
    
    // Record sub-millisecond latency (should be clamped to 1)
    record_metric(&metrics, "fast_api", 0.4);
    
    let entry = metrics.get("fast_api").unwrap();
    let hist = entry.value();
    
    assert_eq!(hist.value_at_quantile(1.0), 1);
}
