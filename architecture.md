# Architecture Design: LogStream Observability Portal

This document defines the high-level architecture of LogStream, mapping all P0 requirements to system components and documenting failure strategies.

---

## 1. System Overview
LogStream is composed of four primary layers:
1.  **Ingestion Engine (Hot Path)**: A high-throughput Rust service (`ingest_service`) handling OTLP streams and multi-GB seeding.
2.  **Web & Analytics API (Cold Path)**: A responsive Rust service (`api_service`) serving the discovery portal, administration logic, and DDE queries.
3.  **Analytics & Storage**: A PostgreSQL 16 database utilizing JSONB for flexible logs and relational tables for aggregated metrics.
4.  **Visualization Portal**: A unified Leptos (WASM) frontend providing a "Lens" drag-and-drop experience and interactive DDE exploration.

---

## 2. Component Mapping

| Requirement | Component | Technology | Rationale |
| :--- | :--- | :--- | :--- |
| **Hot/Cold Path Separation** | Microservices | Rust / Decoupled Binaries | Prevents ingest bursts from impacting UI availability. |
| **P99 Metric Precision (+/- 1ms)** | Ingest Aggregator | Rust / `HdrHistogram` | Sub-millisecond recording at scale. |
| **Self-Service Discovery** | DDE Service | JSONB / Dynamic Indexing | Allows no-code exploration of arbitrary datasets. |
| **SOC2 / OWASP Compliance** | Security Layer | Cryptographic Audit Chaining | Tamper-evident evidence for auditors. |
| **Injection Guard** | DDE Parser | Regex / Operator Allow-list | Prevents SQL/DDL injection in dynamic query builder. |

---

## 3. Data Flow
1.  **OTLP Ingest**: `otel-collector` -> `ingest_service:8081` -> `HdrHistogram` -> `Postgres`.
2.  **Self-Service Discovery**: `Leptos UI` -> `api_service:8080` -> `Postgres (Dynamic Tables)`.
3.  **Query Path**: `Leptos UI` -> `api_service:8080` -> `Postgres (JSONB Aggregation)`.

---

## 4. Failure Scenarios & Recovery

### A. Database Saturation during 1GB Ingest
- **Strategy**: The `ingest_service` uses **Backpressure**. It monitors DB write latency; if latency exceeds 500ms, the parser throttled the stream.
- **Isolation**: High ingest latency in `ingest_service` does not impact the responsiveness of `api_service`.

### B. OTLP Stream Spike
- **Strategy**: **In-memory buffering**. The `ingest_service` buffers observations in fixed-size circular buffers. If the buffer is full, it returns `429 Too Many Requests`.

---

## 5. Security Gates
- **OWASP**: All API inputs are validated. DDE queries are restricted to hardcoded aggregate functions and validated operators.
- **CVE**: CI/CD pipeline integrated with `cargo-audit` to block vulnerabilities.
- **Audit**: All DDE mutations (visibility, indexing) are cryptographically chained in the audit trail.
