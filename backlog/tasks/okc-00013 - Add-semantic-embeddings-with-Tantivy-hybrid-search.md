---
id: OKC-00013
title: Add semantic embeddings with Tantivy/hybrid search
status: To Do
assignee: []
created_date: '2026-07-23 00:50'
updated_date: '2026-07-25 23:57'
labels:
  - future
dependencies: []
priority: medium
type: feature
ordinal: 17400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After measuring lexical retrieval failures, add Tantivy for embeddings. Pipeline: metadata filter -> FTS candidates -> graph expansion -> optional embedding rerank. Never bypass exact filters.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Tantivy index integrated alongside SQLite FTS5
- [ ] #2 Hybrid pipeline: metadata -> FTS -> graph -> optional rerank
- [ ] #3 Embeddings never bypass exact metadata filters
- [ ] #4 Chunking policy configurable
- [ ] #5 Embedding model version tracked
- [ ] #6 Embedding model is selectable: local (llama.cpp) or remote (OpenAI/Anthropic API)
- [ ] #7 Embedding cache in SQLite BLOB column avoids re-embedding unchanged documents
- [ ] #8 Embedding dimension configurable (384/768/1536)
- [ ] #9 Semantic search returns top-k results by cosine similarity with scores
<!-- AC:END -->
