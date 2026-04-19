# ADR-002: Telemetry Aggregation (HdrHistogram)

## Status
Proposed (Stage 2: Architecture Design)

## Context
Requirements state that P99 latency must be tracked with a precision of **+/- 1ms**. Standard averaging or simple bucketing is insufficient for high-dynamic range tail latencies.

## Decision
We will use the **`HdrHistogram`** (High Dynamic Range Histogram) library within the Rust backend to calculate p99 and other percentiles.

1. **Precision**: HdrHistograms are designed to maintain a fixed relative error across a wide range of values (e.g., 1ms to 60s).
2. **Performance**: Ingestion-time recording is O(1) and extremely fast, suitable for OTLP streams.
3. **Aggregation**: We will aggregate observations in-memory in 1-minute buckets before flushing the calculated percentiles to PostgreSQL for long-term storage and visualization.

## Consequences
- **Memory Usage**: Each histogram has a fixed memory footprint based on its configuration (max value and precision), which is manageable for a dedicated log aggregator.
- **Complexity**: Requires careful configuration of the histogram range to avoid "out of range" errors for long-tail outliers.
