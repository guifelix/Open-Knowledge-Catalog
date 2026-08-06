---
id: DRAFT-00024
title: Decide and implement vector index storage (sqlite-vec vs lancedb vs qdrant)
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 16:32'
labels:
  - graph
  - search
  - feature
  - high-priority
dependencies: []
references:
  - src/index/search_index.rs
  - src/index/document_store.rs
  - okc-00013
documentation:
  - docs/ai-usage.md#mcp-tools
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Choose and implement vector index storage for hybrid search.

**Current:**
- OKC-00013 mentions Tantivy for embeddings but no storage decision
- Need to store embeddings alongside SQLite FTS5

**Options:**
1. **sqlite-vec** (SQLite extension): Embedded, zero-dep, SQL queries, HNSW
2. **lancedb** (embedded): Columnar, fast, Rust-native, supports filtering
3. **qdrant** (separate): Production-grade, but external dependency

**Decision criteria:**
- Embedded (no external service)
- HNSW index performance
- Metadata filtering (filter by type, tags, path)
- Rust ecosystem integration
- License compatibility

**Expected:**
- Decision doc with benchmarks
- Implementation with chosen backend
- Embedding column in document store
- Hybrid search: BM25 + vector (RRF fusion)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Decision document with benchmarks for sqlite-vec vs lancedb vs qdrant
- [ ] #2 Chosen backend implemented with embedding column
- [ ] #3 Hybrid search: BM25 + vector with RRF fusion
- [ ] #4 Metadata filtering works on vector results (type, tags, path)
- [ ] #5 Embedding model version tracked in index
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Benchmarks: index time, query latency, recall@k
- [ ] #2 Integration test: hybrid search returns relevant results
- [ ] #3 No external service dependency (embedded only)
<!-- DOD:END -->
