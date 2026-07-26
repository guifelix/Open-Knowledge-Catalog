---
id: DRAFT-00001
title: Fix FTS5 BM25 Relevance Ranking with Configurable Field Weights
status: Draft
assignee: []
created_date: '2026-07-25 19:15'
labels: 
    - search
    - core
    - performance
priority: high
type: bug
dependencies:
  - OKC-00003
  - OKC-00018
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve full-text search relevance for technical and structured knowledge bases by implementing proper Okapi BM25 scoring with field-specific weighting (title > description/headings > body). This addresses suboptimal ranking that affects agent retrieval quality in concept lookup and semantic-like queries.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->

- [ ] #1 Implement BM25 scoring formula (IDF * TF * (k1+1) / (TF + k1*(1-b+b*dl/avgdl))) with configurable k1/b parameters
- [ ] #2 Apply field weights: title (highest), description/headings, body (lowest)
- [ ] #3 Search results ordered by relevance score; expose score in outputs
- [ ] #4 Handle edge cases: empty queries, stop words, single-term, technical tokens
- [ ] #5 Tag / type / path filters compose correctly with full-text search
- [ ] #6 Known bugs (empty results, wrong total_matches, debug SQL leak) are fixed
- [ ] #7 Small gold-set of queries has expected ranking
- [ ] #8 Update tests and benchmarks (performance <50ms on 1000+ docs)

<!-- AC:END -->

## Definition of Done

<!-- DOD:BEGIN -->

- [ ] #1 All unit + property tests pass
- [ ] #2 Documentation updated in README and docs/
- [ ] #3 Backlog task finalized with implementation notes
- [ ] #4 No SQL or internal debug strings in user-facing output

<!-- DOD:END -->

## Implementation Plan

1. Research and implement BM25 in SQLite FTS5 context (custom rank function or auxiliary columns)
2. Extend query builders in database.rs for field weighting
3. Add configuration options and update CLI/MCP outputs
4. Benchmark and validate on technical fixtures
