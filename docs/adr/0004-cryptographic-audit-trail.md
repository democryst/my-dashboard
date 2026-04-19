# ADR-004: Cryptographic Audit Trail (SOC2 Compliance)

## Status
Proposed (Stage 2: Architecture Design)

## Context
LogStream must pass SOC2 Type I / OWASP security audits. A critical requirement is an immutable, tamper-evident audit trail for all administrative and data-altering actions.

## Decision
We will implement an **Immutability & Integrity** layer for the audit log table.

1. **Append-Only Enforcement**: We will use PostgreSQL Rules (`DO INSTEAD NOTHING`) on the `audit_log` table to prevent `UPDATE` and `DELETE` operations.
2. **Cryptographic Chaining**: Each row will contain a `prev_row_hash` field. Upon insertion, the Rust application will calculate a SHA-256 hash that includes the current row content PLUS the hash of the previous row.
3. **Verification**: We will provide a CLI utility/background job to periodically verify the chain's integrity. Any break in the chain indicates unauthorized tamper attempts.

## Consequences
- **Sequential Ingestion**: Ingesting audit logs becomes sequential (one by one per service) to ensure the hash chain remains consistent.
- **Audit Overhead**: Calculating SHA-256 on every audit write adds a minor CPU cost, which is acceptable for security compliance.
