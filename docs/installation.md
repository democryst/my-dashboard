# Installation & Setup Guide

This guide ensures your local environment is correctly configured for developing, testing, and running LogStream.

## 📋 Prerequisites

Ensure the following tools are installed on your system:

| Tool | Version | Purpose |
| :--- | :--- | :--- |
| **Rust** | 1.80+ | Core language for backend and frontend. |
| **Docker** | Latest | Database (Postgres) and OTel Collector runtime. |
| **Trunk** | Latest | Build tool and dev server for the WASM frontend. |
| **SQLx CLI** | Latest | Database migration management. |

### 1. Install Rust Targets
The frontend requires the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

### 2. Install CLI Tools
```bash
cargo install trunk
cargo install sqlx-cli --no-default-features --features postgres
```

---

## 🚀 Step-by-Step Setup

### 1. Start Infrastructure
Start the database and OpenTelemetry collector using Docker Compose:
```bash
docker-compose up -d
```
> [!NOTE]
> This will start PostgreSQL on port `5432` and the OTel collector on ports `4317/4318`.

### 2. Database Migrations
Initialize the schema in the local database:
```bash
cd backend
export DATABASE_URL=postgres://postgres:password@localhost:5432/logstream
sqlx database setup
```

### 3. Start Backend Services
LogStream uses a decoupled architecture. You must start both the Ingest and API services.

**Terminal 1: Ingest Service (Port 8081)**
```bash
cd backend
export DATABASE_URL=postgres://postgres:password@localhost:5432/logstream
export INGEST_PORT=8081
cargo run --bin ingest_service
```

**Terminal 2: Web API Service (Port 8080)**
```bash
cd backend
export DATABASE_URL=postgres://postgres:password@localhost:5432/logstream
export API_PORT=8080
cargo run --bin api_service
```

### 4. Start Frontend Dashboard
Build and serve the WASM dashboard locally:
```bash
cd frontend
trunk serve
```
> [!TIP]
> Once finished, the dashboard will be available at `http://localhost:3000`.

---

## 🛠 Troubleshooting

### "Database Connection Refused"
- Ensure `logstream-db` container is running: `docker ps`.
- Check if port `5432` is occupied by another Postgres instance.

### "WASM build failed"
- Verify the target is installed: `rustup target list --installed`.
- Ensure `Trunk` is up to date.

### "OTLP Export Failed"
- Check `logstream-otel` container logs: `docker logs logstream-otel`.
- Ensure `ingest-service` is running on port `8081`.
