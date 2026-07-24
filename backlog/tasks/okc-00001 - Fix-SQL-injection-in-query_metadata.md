---
id: OKC-00001
title: Fix SQL injection in query_metadata
status: Done
assignee: []
created_date: '2026-07-23 00:49'
updated_date: '2026-07-24 21:58'
labels:
  - security
  - critical
dependencies: []
references:
  - src/index/database.rs
priority: high
type: bug
ordinal: 100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The query_metadata function interpolates table names directly into SQL, allowing injection via filter keys. Must use parameterized queries with dynamic JOIN aliases.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 query_metadata builds SQL via string interpolation — no user-supplied values reach SQL without parameterization
- [ ] #2 All SQL in src/index/database.rs uses rusqlite params (?, :name, or $NN) — verified by grep for format! or + in SQL strings
- [ ] #3 Fuzz test with malicious filter keys (single quotes, semicolons, UNION) does not alter query structure
- [ ] #4 Clippy safety lint pass: no unsafe SQL construction patterns remain
<!-- AC:END -->
