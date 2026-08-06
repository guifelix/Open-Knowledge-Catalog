---
id: DRAFT-00025
title: Implement hybrid fusion algorithm (RRF vs weighted) for BM25 + vector search
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 16:33'
labels:
  - search
  - feature
  - high-priority
dependencies: []
references:
  - src/index/search_index.rs
  - src/config.rs
  - okc-00013
documentation:
  - docs/ai-usage.md#mcp-tools
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement fusion algorithm for hybrid BM25 + vector search.

**Current:**
- OKC-00013 mentions "hybrid pipeline" but no fusion method specified

**Options:**
1. **RRF (Reciprocal Rank Fusion)**: k=60, rank-based, no score calibration needed
2. **Weighted score fusion**: α·BM25 + β·cosine, requires score normalization
3. **Hybrid**: RRF for top-k, then weighted rerank

**Expected:**
- Configurable fusion method in config
- RRF as default (robust, no calibration)
- Weighted fusion as option with score normalization
- Per-query override via MCP tool
- Benchmarks comparing methods
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 RRF fusion implemented as default (k=60)
- [ ] #2 Weighted fusion option with score normalization
- [ ] #3 Configurable via config file and MCP tool override
- [ ] #4 Benchmarks: recall@k, latency for each method
- [ ] #5 Per-query fusion method override in search tool
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers RRF and weighted fusion
- [ ] #2 Integration test: hybrid search with both methods
- [ ] #3 Benchmarks documented
<!-- DOD:END -->
