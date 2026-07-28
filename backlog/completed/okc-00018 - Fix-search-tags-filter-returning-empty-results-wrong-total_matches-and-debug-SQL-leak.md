---
id: OKC-00018
title: >-
  Fix search tags filter returning empty results, wrong total_matches, and debug
  SQL leak
status: Done
assignee:
  - '@felix'
created_date: '2026-07-23 06:49'
updated_date: '2026-07-23 07:01'
labels: []
dependencies: []
references:
  - 'src/index/database.rs:653'
  - 'src/index/database.rs:686-692'
priority: high
type: bug
ordinal: 18000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Three related search bugs identified in code review:

1. `search` with `--tags` filter always returns empty results because the tags join uses an incorrect column reference (database.rs:686-692).
2. `total_matches` in search results equals `results.len()` (post-LIMIT count) instead of the true total row count before LIMIT was applied.
3. Debug SQL printed to stderr on every search via `eprintln!("DEBUG SQL: {}", full_sql)` at database.rs:653, leaking internal schema to users and filling stderr.

Found during code review gap analysis against design doc doc1.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 search --tags <name> returns correct results matching the tag filter
- [ ] #2 total_matches reflects pre-LIMIT count (actual total matching documents)
- [ ] #3 No debug SQL printed to stderr in production search path
- [ ] #4 Existing FTS5 integration tests pass after fix
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Manual testing with tagged and untagged concepts confirms correct counts
<!-- DOD:END -->
