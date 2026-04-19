# Project Requirements: LogStream Observability Portal

LogStream is a high-performance, system-level log aggregation and telemetry visualization platform, designed for real-time observability, forensic analysis, and advanced data exploration.

---

## 1. Product Vision
A centralized, high-throughput observability engine that combines real-time OpenTelemetry ingestion with flexible batch data exploration. Optimized for security and performance using a pure Rust stack.

---

## 2. Functional Requirements

### 📊 Advanced Visualization Suite (LogStream Lens)
- **[P0] Drag-and-Drop Builder**: Interface for creating area, bar, line, metric, and table visualizations.
    - **AC 1**: Users can drag a numeric field from the schema sidebar to the Y-axis and a time field to the X-axis to generate a Chart in < 500ms.
- **[P0] Time-Series Visual Builder (TSVB)**: High-frequency analysis with millisecond precision.
    - **AC 1**: Support for bucketing data into 1ms, 10ms, and 100ms intervals.
- **[P1] Geospatial Analytics**:
    - Coordinate Maps and Heatmaps for location-based data points.
- **[P1] Custom Visualizations**: Support for Vega/Vega-Lite grammars.
- **[P0] Interactive Dashboards**: Combined panels with global cross-filtering.
    - **AC 1**: Changing a filter on one panel must update all other panels in the dashboard within < 200ms.

### 📥 Data Ingestion & Seeding
- **[P0] OTLP Native Ingestion**: Support for OpenTelemetry Protocol (OTLP).
    - **AC 1**: Accurately calculate P99 latency with an error margin of **+/- 1ms**.
- **[P0] Batch Seeding Jobs**:
    - **CSV/JSON Import**: Dedicated interface to upload datasets up to **1GB** at a time.
    - **Data Splitting Tool**: Provide a utility for users to split files larger than 1GB.
    - **AC 1**: 1GB datasets must be fully ingested and indexed within < 5 minutes.
- **[P0] Real-Time Log Exploration**: "Discover" mode with sub-second tailing.

### 🧩 Dynamic Data Engine (DDE)
- **[P0] Dynamic Table Schema**: API to create and manage virtual data tables with typed fields.
- **[P0] Admin Management Console**: Manual interface to manage table lifecycles and metadata.
    - **AC 1**: Admins can toggle table visibility (`is_visible`) to control end-user access.
    - **AC 2**: Admins can manually request performance indexes (B-TREE/GIN) on field paths.
- **[P0] Table Discovery Portal**: Secure interface for users to browse published datasets.
    - **AC 1**: Non-admin users can only view and query tables marked as `is_visible`.
- **[P0] Custom Data Ingestion**: Dedicated endpoint `/v1/data/{table_id}` for inserting arbitrary JSON objects.
- **[P0] Self-Service Query Builder**: No-code interface allowing users to:
    - Select a dynamic table and field.
    - Apply structured filters (Field, Operator, Value).
    - Render real-time charts updating as new data is inserted.

### 🏗️ Data Architecture
- **Service Decoupling**: Separation of Ingest (Hot Path) and Web API (Cold Path) for resource isolation.
- **Flexible Schema Engine**: Structured storage in PostgreSQL using JSONB.
- **Aggregation Pipeline**: In-memory HdrHistogram aggregation (P99/TPM).
- **Indexing Strategy**: Time-bucketed and managed DDE indexes.

### 🔐 Identity & Security
- **[P0] Security Compliance**:
    - **SOC2 & OWASP**: The system must be designed to pass SOC2 Type I and OWASP Top 10 security audits.
    - **CVE Vulnerability Management**:
        - **Critical/High**: 0 open vulnerabilities allowed.
        - **Medium**: Maximum 5% of total findings allowed.
- **[P0] Rust-Native Safety**: Zero memory-safety vulnerabilities.
- **[P0] RBAC**: Granular permissions (ADMIN, OPERATOR, VIEWER).
- **[P0] Cryptographic Audit Trail**: Every administrative action must be cryptographically chained via SHA-256 for non-repudiation.

### ✅ Quality & Hardening
- **[P0] Automated Test Suite**: Critical paths (Audit, DDE, Ingest) must have > 80% integration test coverage.
    - **AC 1**: Tests must be separated from source code in the `tests/` directory.
- **[P0] Safe DDL Generation**: Dynamic SQL must be abstracted via a `QueryBuilder` and strictly bound to type-safe operators to prevent injection.

---

## 3. Non-Functional Requirements

- **Exploration Performance**: Sub-second response times for complex analytical queries.
- **Scalability**: Handle millions of log events per minute.
- **Surgical Code Organization**: Modular architecture (Shared `common` crate, decoupled services) following a "Simplicity First" approach.

---

## 4. Technical Requirements

- **Backend**: Rust (Axum, Tokio, SQLx).
- **Frontend**: Rust (Leptos + WASM).
- **Shared**: `common` crate for type-safe models and operators.
- **Database**: PostgreSQL (with JSONB for dynamic schema).
- **Testing**: Standalone integration tests in `tests/` directory.
- **Environment**: Containerized Docker workflow (Decoupled `ingest-service` and `api-service`).

---

## 5. Success Criteria
- [x] STAGE 1: Requirement Analysis complete with measurable ACs.
- [x] STAGE 2: Architecture Design (ADRs) complete.
- [x] STAGE 3: Infrastructure Provisioning (Docker-compose) complete.
- [x] STAGE 4: Feature Implementation (DDE & Decoupling) complete.
- [x] STAGE 5: Hardening & Testing (80% critical path coverage) complete.
- [x] STAGE 6: Engine Refactoring (DdeQueryBuilder) complete.
- [ ] Successful ingestion and visualization of 1GB+ logs from a single seed job.
- [x] Accurate Millisecond-level P99 latency tracking (+/- 1ms margin).
- [ ] 0 Critical/High CVE findings in the Rust/PostgreSQL stack.
