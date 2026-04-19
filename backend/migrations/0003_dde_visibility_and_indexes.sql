-- 0003_dde_visibility_and_indexes.sql

-- 1. Extend dynamic_tables metadata
ALTER TABLE dynamic_tables 
ADD COLUMN is_visible BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN index_config JSONB NOT NULL DEFAULT '[]'::jsonb; -- Array of { "field": "...", "type": "B-TREE|GIN" }

-- 2. Create a log entry for the update
INSERT INTO audit_log (actor, action, resource, details)
VALUES ('system', 'SCHEMA_UPGRADE', 'dynamic_tables', '{"migration": "0003_dde_visibility_and_indexes", "added": ["is_visible", "index_config"]}');
