---
id: DRAFT-00022
title: Replace stringly-typed errors with structured error types
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:01'
labels:
  - api
  - reliability
dependencies: []
documentation:
  - docs/error-handling.md
priority: high
type: feature
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Stringly errors make agent consumption and debugging painful. Introduce a proper error enum (thiserror) with stable codes that surface through both CLI JSON and MCP.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Public error type with codes (NOT_FOUND, VALIDATION, IO, INTERNAL, etc.)
- [ ] #2 CLI --json and MCP both emit the structured form
- [ ] #3 Human-readable messages remain useful
- [ ] #4 No panics or bare anyhow::Error leaks on expected failure paths
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Error codes documented
- [ ] #2 Tests assert codes for common failure cases
- [ ] #3 Clippy / thiserror hygiene clean
<!-- DOD:END -->
