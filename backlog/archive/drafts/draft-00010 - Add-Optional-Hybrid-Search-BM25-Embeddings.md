---
id: DRAFT-00010
title: Add Optional Hybrid Search (BM25 + Embeddings)
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 19:57'
labels:
  - search
  - ai
  - performance
dependencies:
  - OKC-00013
  - OKC-00003
documentation:
  - docs/hybrid-search.md
priority: high
type: feature
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Introduce optional hybrid search combining improved BM25 with lightweight embeddings. This significantly boosts retrieval quality for technical knowledge without making embeddings mandatory.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Optional embedding support (e.g., via candle or ONNX sentence-transformers)
- [ ] #2 Hybrid ranking (BM25 + cosine similarity) with configurable weights
- [ ] #3 Field-aware embedding (headings prioritized)
- [ ] #4 Configurable embedder model and index rebuild option
- [ ] #5 Performance benchmarks and fallback to pure lexical
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Optional dependency handling (feature flag)
- [ ] #2 Documentation and examples for enabling hybrid search
- [ ] #3 Tests for hybrid vs lexical results
<!-- DOD:END -->
