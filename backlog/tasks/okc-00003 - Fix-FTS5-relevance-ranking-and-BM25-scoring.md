---
id: OKC-00003
title: Fix FTS5 relevance ranking and BM25 scoring
status: To Do
assignee: []
created_date: '2026-07-23 00:49'
updated_date: '2026-07-23 19:02'
labels:
  - search
dependencies:
  - OKC-00018
priority: high
type: feature
ordinal: 400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Current FTS5 schema uses single rank column. Need proper BM25 with field weights (title>description>headings>body) and configurable scoring.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 BM25 scoring with configurable field weights implemented
- [ ] #2 Title weight > description > headings > body
- [ ] #3 Search results ordered by relevance score
- [ ] #4 bm25_score implementation uses the Okapi BM25 formula (idf * tf * (k1+1) / (tf + k1*(1-b+b*dl/avgdl)))
- [ ] #5 BM25 parameters k1 and b are configurable per index
- [ ] #6 Benchmark: full-text search over 1000 docs completes in <50ms
- [ ] #7 Correctly handles edge cases: empty query, stop words, single-term queries
<!-- AC:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-07-23 06:50
---
Gap analysis update: The tags-filter returning empty results and total_matches equalling post-LIMIT count are now tracked as separate bug OKC-00018. This task remains focused specifically on FTS5 BM25 relevance ranking with configurable field weights.
---
<!-- COMMENTS:END -->
