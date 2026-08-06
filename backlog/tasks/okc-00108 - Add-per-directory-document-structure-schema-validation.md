---
id: OKC-00108
title: Add optional per-directory document schema validation
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 15:51'
updated_date: '2026-08-06 19:54'
labels:
  - cli
  - mcp
  - schema
  - validation
  - markdown
dependencies:
  - OKC-00107
references:
  - src/index/validate.rs
  - src/scanner/walker.rs
documentation:
  - docs/ai-usage.md
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Allow repositories to declare optional document-structure schemas that vary by directory while remaining separate from Markdown style linting and baseline OKF conformance. Reuse the layered-configuration resolver from OKC-00107, define deterministic inheritance, and surface schema violations through existing validation outputs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A versioned repository schema can define document-structure rules and nearest-directory overrides for a subtree
- [ ] #2 Inheritance and replacement behavior is documented for scalars, lists, maps, removed rules, and invalid child overrides
- [ ] #3 Rules cover required or forbidden headings and measurable section constraints without duplicating Markdown style rules
- [ ] #4 Schema parse errors and document violations have documented severities and do not silently prevent unrelated documents from being indexed
- [ ] #5 Existing validate output reports structured schema diagnostics with rule identifier, path, line, and column when available
- [ ] #6 Per-document suppression uses a schema-specific namespace and rejects unknown rule identifiers
- [ ] #7 Resolved schemas are cached and invalidated when any contributing schema file changes
- [ ] #8 Tests cover global and nested schemas, merge edge cases, invalid schemas, suppression, cache invalidation, and repositories with no schema
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers schema parsing, resolution, and validation rules
- [ ] #2 Integration test: global schema + directory override → correct validation
- [ ] #3 MCP tool returns structured violations with line/column
- [ ] #4 Inheritance works: global + parent + current directory override
- [ ] #5 Schema suppression uses the documented schema-specific namespace
- [ ] #6 Validation performance is measured on representative nested-schema fixtures and does not regress normal scans materially
<!-- DOD:END -->
