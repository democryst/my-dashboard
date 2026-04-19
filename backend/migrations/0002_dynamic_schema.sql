-- 0002_dynamic_schema.sql

-- 1. Table for storing user-defined table metadata
CREATE TABLE dynamic_tables (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    config JSONB NOT NULL DEFAULT '{}'::jsonb, -- Stores field definitions and types
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Unified table for storing all dynamic record data
CREATE TABLE dynamic_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    table_id UUID NOT NULL REFERENCES dynamic_tables(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_dynamic_data_table_id ON dynamic_data (table_id);
CREATE INDEX idx_dynamic_data_timestamp ON dynamic_data (timestamp DESC);
CREATE INDEX idx_dynamic_data_payload ON dynamic_data USING GIN (payload);

-- Audit log entry for DDE operations
INSERT INTO audit_log (actor, action, resource, details)
VALUES ('system', 'SCHEMA_UPGRADE', 'database', '{"migration": "0002_dynamic_schema"}');
