# ADR 0006: DDE Admin Controls & User Discovery

## Status
Proposed

## Context
The current Dynamic Data Engine (DDE) allows programmatic creation of tables and ingestion of JSON data. However, there is no administrative layer to manage table visibility, nor is there a way for non-admin users to discover and query these datasets. Additionally, performance tuning (indexing) is currently fixed to a global GIN index.

## Decision
1.  **Metastore Expansion**: The `dynamic_tables` table will be extended with `is_visible` (Boolean) and `index_definitions` (JSONB) fields.
2.  **Access Control**: 
    - APIs for table creation, visibility toggling, and indexing will be restricted to the `ADMIN` role.
    - End-users will only be able to list and query tables where `is_visible = true`.
3.  **Dynamic Indexing Implementation**: To avoid arbitrary DDL injection, we will implement a "Managed Indexer".
    - Admins "request" an index on a specific JSONB path (e.g., `payload->>'user_id'`).
    - The backend translates this into a pre-validated `CREATE INDEX CONCURRENTLY` statement using a templated approach that enforces strict naming and structure.
4.  **Web-Based Query Parser**:
    - We will implement a structured query object for the web: `{ field: "status", op: "=", value: "ERROR" }`.
    - This will be translated by the backend into a parameterized SQL `WHERE` clause using JSONB extraction operators.

## Consequences
- **Security**: The "Managed Indexer" introduces a controlled way to perform DDL at runtime. This must be audited and rate-limited.
- **Performance**: Fine-grained indexes will significantly improve query performance for specific high-traffic fields in the JSONB payload.
- **Usability**: Non-technical users gain the ability to explore datasets without needing to know the underlying table IDs.
