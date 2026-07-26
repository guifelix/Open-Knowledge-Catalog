---
type: Documentation
title: Roadmap
description: Phase status and known issues tracked in backlog
tags:
  - roadmap
  - planning
  - status
owner: Engineering Team
status: published
---

# Roadmap

## Phase 1 (Current) - Minimal Repository Reader
- ✅ Filesystem traversal with ignore support
- ✅ Front-matter extraction & YAML parsing
- ✅ Normalized document records
- ✅ Basic CLI output

## Phase 2 - Markdown Structure
- ✅ Heading extraction
- ✅ Internal link extraction & resolution
- ✅ Broken link detection
- ✅ Directory tree construction

## Phase 3 - Persistent Index
- ✅ SQLite schema with FTS5
- ✅ Incremental file updates
- ✅ Metadata indexes
- ✅ Deleted file handling

## Phase 4 - AI-Facing Operations
- ✅ `browse_directory`
- ✅ `get_document` / `get_section`
- ✅ `search_documents`
- ✅ `query_metadata`
- ✅ `get_links` / `get_backlinks`
- ✅ `traverse_graph`
- ✅ `validate_repository`
- 🔲 MCP server transport

## Phase 5 - Continuous Updates
- ✅ Filesystem watcher (`notify`)
- ✅ Debounced updates
- ✅ Partial graph rebuilding
- ✅ Index health reporting

## Phase 6 - Advanced Retrieval (Future)
*Only add after measuring real retrieval failures:*
- Fuzzy filename matching
- Trigram search
- Semantic embeddings
- Reranking
- Generated directory summaries
- PageIndex-style hierarchical reasoning
- Relationship extraction from custom metadata

## Current Status

| Phase | Status | Completion |
|-------|--------|------------|
| 1 | ✅ Done | 100% |
| 2 | ✅ Done | 100% |
| 3 | ✅ Done | 100% |
| 4 | ✅ Done | 100% |
| 5 | ✅ Done | 100% |
| 6 | 🔮 Future | 0% |

## Known Issues (Tracked in Backlog)

- **OKC-00001**: SQL injection in `query_metadata`
- **OKC-00002**: Thread safety (RefCell → Mutex/RwLock)
- **OKC-00003**: FTS5 BM25 relevance ranking
- **OKC-00005**: Filesystem watcher
- **OKC-00006**: Input validation & size limits
- **OKC-00007**: Link resolution edge cases
- **OKC-00008**: YAML aliases/anchors/merge keys
- **OKC-00014**: Trait interfaces for storage backends
- **OKC-00022**: JSON output mode for CLI
- **OKC-00026**: Structured error types
- **OKC-00033**: MAANG-level code quality remediation (CI, path traversal, unwrap, docs, etc.)

See `backlog/tasks/` for full task list.