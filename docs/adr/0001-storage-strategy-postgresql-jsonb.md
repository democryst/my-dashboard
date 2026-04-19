# ADR-001: Storage Strategy (PostgreSQL JSONB + Native Partitioning)

## Status
Proposed (Stage 2: Architecture Design)

## Context
LogStream must ingest high volumes (>1GB/job) of heterogeneous log data from both OpenTelemetry (OTLP) and batch seeding (CSV/JSON). Attributes vary widely between services and datasets.

## Decision
We will use **PostgreSQL 16+** as the primary storage engine, utilizing the **JSONB** data type for log attributes.

1. **Schema Flexibility**: JSONB allows us to store varying metadata without static column definition.
2. **Indexing**: We will utilize **GIN (Generalized Inverted Index)** on the attributes field to ensure sub-second response times for complex property filters.
3. **Partitioning**: We will implement **Native Range Partitioning** based on the `timestamp` column (e.g., daily partitions) to ensure efficient data retention and query performance on 1GB+ datasets.

## Consequences
- **Storage Overhead**: JSONB has a slight storage overhead compared to raw columns, but this is offset by GIN search performance.
- **Complexity**: Managing partitions requires manual logic or `pg_partman` integration.
- **Portability**: Stays within the standard PostgreSQL ecosystem, avoiding specialized time-series extensions for the initial phase.
