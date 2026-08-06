---
id: DRAFT-00022
title: Add contradiction detection for typed relations
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 16:31'
labels:
  - graph
  - feature
  - high-priority
dependencies: []
references:
  - src/index/validate.rs
  - src/index/graph_store.rs
  - src/transport/mcp.rs
documentation:
  - docs/ai-usage.md#relationship-reasoning
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add contradiction detection to validate() and graph traversal.

**Current:**
- No contradiction detection
- Agents cannot find conflicting information

**Expected:**
- Detect: A supports X, B contradicts X → flag conflict
- Detect: A supersedes B, but both active → flag
- Detect: Circular supports/contradicts chains
- MCP tool: find_contradictions(path) returns conflicting concepts
- validate() includes contradiction warnings

**Relation types involved:**
- supports vs contradicts
- supersedes vs active
- derived_from cycles
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 validate() returns contradiction warnings with source paths
- [ ] #2 MCP tool find_contradictions(path) returns conflicting concepts
- [ ] #3 Detects: supports vs contradicts on same target
- [ ] #4 Detects: supersedes chain with both active
- [ ] #5 Detects: derived_from cycles
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers contradiction detection patterns
- [ ] #2 Integration test: validate() flags known contradictions in fixtures
- [ ] #3 MCP tool returns structured contradictions with evidence
<!-- DOD:END -->
