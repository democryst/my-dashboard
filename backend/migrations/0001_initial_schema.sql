-- 0001_initial_schema.sql

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Log Entries Table (JSONB for attributes)
CREATE TABLE log_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    timestamp TIMESTAMPTZ NOT NULL,
    service_name TEXT NOT NULL,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    attributes JSONB NOT NULL DEFAULT '{}'::jsonb
);

-- Index for time-range queries
CREATE INDEX idx_logs_timestamp ON log_entries (timestamp DESC);
-- GIN index for high-performance JSON attribute filtering
CREATE INDEX idx_logs_attributes ON log_entries USING GIN (attributes);

-- 2. Metric Entries Table (Aggregated)
CREATE TABLE metric_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    attributes JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX idx_metrics_timestamp ON metric_entries (timestamp DESC);

-- 3. Audit Log Table (SOC2 Immutable)
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    actor TEXT NOT NULL,
    action TEXT NOT NULL,
    resource TEXT NOT NULL,
    details JSONB NOT NULL DEFAULT '{}'::jsonb,
    prev_row_hash TEXT, -- SHA-256 Chaining
    curr_row_hash TEXT  -- Hash of this row + prev_row_hash
);

-- Deny UPDATE and DELETE on audit_log for immutability
CREATE RULE protect_audit_log_update AS ON UPDATE TO audit_log DO INSTEAD NOTHING;
CREATE RULE protect_audit_log_delete AS ON DELETE TO audit_log DO INSTEAD NOTHING;

-- 4. Seed Jobs Tracking
CREATE TABLE seed_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    filename TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'PROCESSING', 'COMPLETED', 'FAILED')),
    total_records INTEGER,
    processed_records INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT
);
