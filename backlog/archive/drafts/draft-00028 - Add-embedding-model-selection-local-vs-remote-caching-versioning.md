---
id: DRAFT-00028
title: 'Add embedding model selection (local vs remote, caching, versioning)'
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 16:39'
labels:
  - search
  - feature
  - medium-priority
dependencies: []
references:
  - src/index/search_index.rs
  - src/config.rs
  - okc-00013
documentation:
  - docs/ai-usage.md#mcp-tools
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Complete embedding model configuration for hybrid search.

**Current:**
- OKC-00013 mentions "embedding model selectable: local or remote" but no details

**Expected:**
- Config: embedding_model (local: BGE-small, EmbeddingGemma, all-MiniLM; remote: OpenAI, Anthropic, Cohere)
- Local: candle + tokenizers or ONNX Runtime (ort)
- Remote: API clients with retry/rate-limit
- Caching: SQLite BLOB column for embeddings, content_hash for invalidation
- Versioning: embedding_model_version tracked in index
- Dimension: configurable (384/768/1024/1536)
- Chunking policy: configurable (by heading, by tokens, by chars)

**Example config:**
```toml
[embeddings]
model = "local:bge-small"
# or model = "remote:openai:text-embedding-3-small"
dimension = 384
chunking = "by_heading"
chunk_size = 512
cache_enabled = true
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Config supports local (candle/ort) and remote (OpenAI/Anthropic) models
- [ ] #2 Embedding cache in SQLite with content_hash invalidation
- [ ] #3 Model version tracked in index metadata
- [ ] #4 Chunking policy configurable (by_heading, by_tokens, by_chars)
- [ ] #5 Dimension configurable (384/768/1024/1536)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers model selection and caching
- [ ] #2 Integration test: local and remote models work
- [ ] #3 Cache invalidation on content change
<!-- DOD:END -->
