---
id: DRAFT-00015
title: Fix SQL injection in query_metadata
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 19:59'
labels:
  - security
  - correctness
  - p0
dependencies: []
documentation:
  - docs/security/sql-injection-fix.md
priority: high
type: bug
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Close the SQL injection vulnerability in query_metadata (tracked as OKC-00001). Structured metadata filters must never concatenate user-controlled strings into SQL. This is a hard blocker for any production or agent-facing use.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All metadata filters use parameterized queries / prepared statements only
- [ ] #2 Fuzz or property-based tests cover malicious filter values (quotes, comments, unions, etc.)
- [ ] #3 No raw string interpolation remains in any query path that accepts user input
- [ ] #4 Existing legitimate queries continue to return correct results
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Tests pass (unit + integration + fuzz)
- [ ] #2 Security note added to CHANGELOG / docs
- [ ] #3 Code review confirms no remaining injection surface
<!-- DOD:END -->

## Implementation Plan
<!-- SECTION:PLAN:BEGIN -->
1. **Audit all query paths**: Search codebase for `query_metadata` and related metadata filter functions; identify every location where user input is interpolated into SQL strings
2. **Replace with parameterized queries**: Use `sqlx::query` / `sqlx::query_as` with bind parameters for all filter values; ensure `LIKE` patterns are built in Rust and passed as parameters
3. **Add input validation layer**: Validate filter keys against allowed metadata fields; reject unknown keys before query construction
4. **Fuzz testing**: Add proptest/fuzz targets generating malicious filter values (SQL comments, unions, quotes, semicolons); run in CI
5. **Regression tests**: Add integration tests for all legitimate query patterns (equality, range, LIKE, IN, IS NULL) to ensure no behavior change
6. **Documentation**: Update `docs/security/sql-injection-fix.md` with before/after examples and parameterization patterns
<!-- SECTION:PLAN:END -->
