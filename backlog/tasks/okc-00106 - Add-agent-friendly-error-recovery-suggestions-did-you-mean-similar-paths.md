---
id: OKC-00106
title: Add bounded recovery hints to structured agent errors
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:52'
updated_date: '2026-08-06 19:53'
labels:
  - mcp
  - backend
  - enhancement
  - agent-ux
  - errors
dependencies:
  - OKC-00026
references:
  - src/transport/mcp/mod.rs
  - src/index/queries.rs
documentation:
  - docs/ai-usage.md
priority: medium
type: enhancement
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Extend the structured error contract from OKC-00026 with deterministic recovery hints that help agents correct missing or invalid repository paths. Suggestions must be bounded, scoped to configured roots, and safe to expose; this task does not redefine the base error taxonomy.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Not-found errors can include a bounded ranked list of similar repository paths and one documented did-you-mean candidate
- [ ] #2 Invalid-path errors include concise path rules and examples without exposing paths outside configured roots
- [ ] #3 Parent-directory context is returned only when the parent is valid, in scope, and within configured result limits
- [ ] #4 Suggestion ranking is deterministic, has a documented maximum candidate count, and remains responsive on large indexes
- [ ] #5 Errors without useful recovery information retain the base structured error shape without empty or misleading hints
- [ ] #6 Unit and packaged-MCP tests cover ranking, ties, no-match cases, traversal attempts, disclosure boundaries, and response limits
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers error suggestion generation
- [ ] #2 Integration test: agent recovers from NOT_FOUND using suggestions
- [ ] #3 Consistent error schema across all 11 tools
<!-- DOD:END -->
