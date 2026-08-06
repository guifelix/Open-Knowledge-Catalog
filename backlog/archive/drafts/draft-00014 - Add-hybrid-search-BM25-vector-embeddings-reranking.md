---
id: DRAFT-00014
title: 'Add hybrid search: BM25 + vector embeddings + reranking'
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:50'
labels:
  - mcp
  - backend
  - feature
  - medium-priority
dependencies: []
references:
  - src/index/search_index.rs
  - src/config.rs
  - vagus
  - relay-knowledge implementations
documentation:
  - docs/ai-usage.md#mcp-tools
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Only BM25 keyword search exists. No semantic understanding.

**Current:**
- search("how do I configure MCP") → matches "configure" "MCP" but misses intent
- No embeddings, no vector search, no reranking

**Expected (like vagus, relay-knowledge, semantic-memory-mcp):**
- Generate embeddings for documents (local: EmbeddingGemma, BGE, or API)
- Hybrid retrieval: BM25 + vector (RRF fusion)
- Cross-encoder reranking for top-K
- HyDE query expansion
- Configurable weights per use case
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Embeddings generated for all documents on scan (configurable model)
- [ ] #2 Hybrid search: BM25 + vector with RRF fusion
- [ ] #3 Cross-encoder reranking for top-20 results
- [ ] #4 HyDE query expansion for short queries
- [ ] #5 Configurable: bm25_weight, vector_weight, rerank_top_k
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers embedding generation and hybrid scoring
- [ ] #2 Integration test: semantic query returns relevant results without exact keywords
- [ ] #3 Local embedding model (no API dependency by default)
<!-- DOD:END -->
