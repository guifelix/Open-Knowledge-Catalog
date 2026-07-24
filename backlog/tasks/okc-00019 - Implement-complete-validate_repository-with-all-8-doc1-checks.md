---
id: OKC-00019
title: Implement complete validate_repository with all 8 doc1 checks
status: Done
assignee: []
created_date: '2026-07-23 06:49'
updated_date: '2026-07-24 00:22'
labels:
  - completeness
dependencies:
  - OKC-00020
references:
  - 'src/index/database.rs:1008-1078'
  - docs/doc1.md
priority: high
type: feature
ordinal: 21000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Current `validate_repository` in database.rs:1008-1078 only covers 3 of 8 required checks from design doc doc1.md:

Currently implemented:
- Broken links (link_exists check via graph_store)
- Scan errors (scan_errors table)
- Missing index files (config check)

Missing (need implementation):
- Invalid YAML front matter (re-check on extract, not just scan time)
- Missing required metadata fields (title, id, type according to OKF schema)
- Duplicate concept identifiers across the repository
- Unsupported file encoding (re-check stored path encoding)
- Oversized front matter (re-check against max_front_matter_size)

The validate tool should return a structured report enumerating all issues found, grouped by severity.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 validate tool checks all 8 conditions from doc1
- [ ] #2 Results grouped by severity (error, warning, info)
- [ ] #3 Each validation issue includes file path and description
- [ ] #4 Exit code 0 when no errors found, non-zero when errors exist
- [ ] #5 Existing tests continue to pass
- [ ] #6 okf validate checks: (1) all links resolve, (2) no broken frontmatter, (3) no circular refs, (4) no duplicate content, (5) size limits honored, (6) valid YAML, (7) valid WikiLinks, (8) valid Markdown links
- [ ] #7 All 8 validation checks from doc1 spec are implemented (current: only 3/8)
- [ ] #8 Validation report is structured JSON for MCP agent consumption
- [ ] #9 Validation errors include file path, check name, and human-readable explanation
- [ ] #10 Validation is incremental: re-validate only changed files when previous report exists
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Manual validation run against known-bad fixture repository confirms all 8 checks produce correct results
<!-- DOD:END -->
