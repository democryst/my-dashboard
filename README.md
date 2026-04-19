# LogStream: High-Performance Rust Log Aggregator

LogStream is a observability platform built entirely in Rust. It provides high-precision metric aggregation, OTLP ingestion, and a SOC2-compliant cryptographic audit trail.

---

## 🚀 Key Features

- **OTLP Native**: Ingest logs and metrics via OpenTelemetry gRPC/HTTP.
- **Service Decoupling**: Separated high-throughput **Ingest Service** from the responsive **API & Analytics Service**.
- **Dynamic Data Engine (DDE)**: Self-service data exploration with visual query building and managed indexing.
- **SOC2 Audit Trail**: Cryptographically chained SHA-256 logs for non-repudiable auditing.
- **Tutorial**: Learn how to build your first dashboard in the **[DDE Walkthrough Guide](./docs/DDE_GUIDE.md)**.
- **Examples**: Explore pre-built datasets and security validation scripts in the **[Examples Suite](./docs/EXAMPLES.md)**.

---

## 🛠 Tech Stack

- **Backend**: Rust (Axum, SQLx, Tokio, HdrHistogram).
- **Frontend**: Rust (Leptos, WASM, Chartistry).
- **Storage**: PostgreSQL 16 (JSONB for logs, GIN indexing).
- **Infrastructure**: Docker Compose (Decoupled Microservices).

---

## 🏃 Getting Started
For a detailed step-by-step guide on prerequisites and environment setup, see the **[Installation & Setup Guide](./docs/installation.md)**.

### 1. Provision Infrastructure
Everything is containerized. Start the database, collector, and both backend services:
```bash
make infra-up
```

### 2. Manual Execution (Development)
Use the **Makefile** for common dev commands:
- `make ingest`: Start Ingest Service (8081).
- `make api`: Start Web API Service (8080).
- `make frontend`: Start WASM Dashboard (3000).
- `make verify`: Run security audit verification.

### 3. Verify Security
To verify the cryptographic integrity of the audit logs:
```bash
cd backend
cargo run --bin verify_audit
```

---

## 📊 Service Endpoints

### Ingest Service (Port 8081)
- **OTLP Ingest**: `POST /v1/ingest`
- **OTLP Logs**: `POST /v1/logs`
- **OTLP Metrics**: `POST /v1/metrics`
- **Batch Seeding**: `POST /v1/seed`

### Web API Service (Port 8080)
- **Table Management**: `GET/POST /v1/tables`
- **DDE Aggregation**: `POST /v1/aggregate/:table_id`
- **Visibility Control**: `PATCH /v1/tables/:id/visibility`

---

## 🔐 Compliance & Security
LogStream was designed for **SOC2 Type I** compliance. 

1. **Audit Immutability**: PostgreSQL rules (`protect_audit_log_update`) prevent alteration of history.
2. **Hash Chaining**: Each audit entry is linked to the previous via SHA-256 signatures.
3. **Injection Guard**: Strict allow-list validation for dynamic DDE queries and alphanumeric-only index paths.
4. **Memory Safety**: Built entirely in memory-safe Rust to eliminate Critical/High CVEs.

---

## 📄 License
MIT License. Created with Antigravity.
