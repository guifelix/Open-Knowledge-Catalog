---
id: OKC-00006
title: Add input validation and size limits enforcement
status: To Do
assignee: []
created_date: '2026-07-23 00:49'
updated_date: '2026-07-23 19:02'
labels:
  - safety
dependencies:
  - OKC-00004
references:
  - src/scanner/walker.rs
  - src/scanner/frontmatter.rs
  - 'src/index/database.rs:897'
priority: high
type: feature
ordinal: 3400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Enforce all configured limits: max_file_size, max_front_matter_size, max_response_chars, max_graph_depth, max_graph_nodes, max_scan_results. Return structured errors when exceeded.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All limits checked before processing
- [ ] #2 Structured error responses with limit name and value
- [ ] #3 CLI shows helpful error messages
- [ ] #4 MCP returns standardized error codes
- [ ] #5 All user-supplied paths are sandboxed — no path traversal via ../ beyond bundle root
- [ ] #6 Filter values are escaped/validated before SQL construction
- [ ] #7 JSON output for validation errors (not just eprintln!) so MCP clients get machine-readable errors
- [ ] #8 Resource limits (max file size, max bundle depth, max tag count) return structured errors instead of panics
<!-- AC:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-07-23 06:50
---
Gap analysis finding: Most resource limits ARE already enforced at scan time (max_file_size in walker.rs, max_front_matter_size in frontmatter.rs, max_graph_depth and max_graph_nodes in config.rs -> enforced in database.rs:897). The gaps are:

1. Post-scan re-validation: validate tool does not re-check max_front_matter_size or max_file_size against stored documents (doc1 requires re-validation on extract, not just scan time).
2. Structured error responses: errors from limit violations are simple strings, not structured with limit name/value as doc1 specifies.
3. MCP error codes: no MCP transport exists yet to add these to (implied by OKC-00004 dependency).

Consider adjusting scope to focus on structured error responses and re-validation checks.
---
<!-- COMMENTS:END -->
