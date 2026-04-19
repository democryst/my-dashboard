# QA Refactoring Request: DDE Query Engine

**To**: `dev_executor` (@[/Volumes/SSD990PRO2TB/workspace/my-dashboard/.agents/agents/dev_executor/agent.yaml])  
**From**: `qa` (@[/Volumes/SSD990PRO2TB/workspace/my-dashboard/.agents/agents/qa/agent.yaml])  
**Priority**: Medium  
**Context**: Dynamic Data Engine (DDE) - `backend/src/dde.rs`

## 🔴 Current Issue
The `aggregate_data` function in [dde.rs](file:///Volumes/SSD990PRO2TB/workspace/my-dashboard/backend/src/dde.rs) has accumulated significant complexity. It manually constructs dynamic SQL using string segments. While current validations are secure, this pattern is:
1. **Difficult to Unit Test**: No clear interface to test query generation apart from database execution.
2. **Fragile**: Adding new aggregation functions or complex filters requires modifying a 50+ line internal logic block.
3. **Implicit**: Security constraints (operator allow-lists) are hardcoded inside the route handler.

## 🟢 Required Refactor
Please refactor the DDE query logic to follow **Clean Architecture** principles:

1. **New Component: `DdeQueryBuilder`**: Create a dedicated struct responsible for translating the `AggregationRequest` into a valid, safe SQL string.
2. **Operator Enum**: Move operators into a type-safe enum with a `try_from` implementation for validation.
3. **Decoupling**: The Axum route handler should only perform initial decoding and then delegate to the builder.
4. **Testability**: The `DdeQueryBuilder` must be unit-testable in isolation (returning the generated SQL and params).

## ✅ Success Criteria (QA Gate)
- [ ] `cargo test` passes with new unittests for the builder.
- [ ] Cyclomatic complexity of the route handler is < 10.
- [ ] No direct `format!` calls for SQL keywords inside the main business logic.

> [!IMPORTANT]
> This refactor is a prerequisite for reaching the 80% test coverage goal defined in your agent persona.
