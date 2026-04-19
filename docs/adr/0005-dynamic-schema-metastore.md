# ADR 0005: Dynamic Data Engine Storage Strategy

## Status
Proposed

## Context
The system needs to support "Dynamic Table Schemas" (DDE) allowing users to create custom tables and ingest arbitrary data. We need a storage strategy that is performant, secure (SOC2 compliant), and easy to manage without constant DDL migrations.

## Decision
We will use a **Unified Meta-Schema Storage (Option B)** approach in PostgreSQL.

1.  **Metadata Management**: A `dynamic_tables` table will store the schema definition (field names, types, validation rules) for each user-defined table.
2.  **Data Storage**: A single `dynamic_data` table will house all custom records using a `jsonb` column for the payload and a `table_id` foreign key for partitioning logic.
3.  **Indexing**: High-performance GIN indexes will be used on the `jsonb` column. Specific field indexes can be created dynamically if performance thresholds are exceeded.

## Consequences

### Positive
- **Security**: Zero dynamic SQL DDL (`CREATE TABLE`) at runtime, eliminating a major attack vector for SQL injection.
- **Simplicity**: No need for complex database migration logic for every user action.
- **Flexibility**: Schema evolution is handled at the application layer by updating metadata.

### Negative
- **Query Complexity**: Dynamic aggregation requires more complex JSONB manipulation in SQL (`SUM((data->>'field')::numeric)`).
- **Storage Overhead**: JSONB carries more metadata than native columns.

## Alternatives Considered

### Option A: Physical Table Generation
- **Pros**: Native performance, strict type checking.
- **Cons**: High security risk (SQL injection in `CREATE TABLE`), database bloat, and management overhead for thousands of tables.

### Option C: EAV (Entity-Attribute-Value)
- **Pros**: Purely relational.
- **Cons**: Extremely poor performance for aggregations and complex queries.
