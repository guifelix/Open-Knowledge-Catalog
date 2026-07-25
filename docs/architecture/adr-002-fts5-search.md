# ADR 002: FTS5 for Full-Text Search

**Status**: Accepted
**Date**: 2026-07-25
**Deciders**: Architecture team
**Technical Story**: OKC-00033.05

## Context

The knowledge catalog requires full-text search across:
- Document titles
- Descriptions
- Heading text
- Body content
- Concept types

Requirements:
- Sub-100ms query latency for typical corpora (<100k docs)
- Relevance ranking (BM25)
- Field-weighted search (title > description > headings > body)
- Incremental indexing (add/update/delete single documents)
- Unicode support
- No external dependencies (embedded)

## Decision

Use **SQLite FTS5** (Full-Text Search version 5) as the built-in search engine.

## Alternatives Considered

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| **SQLite FTS5** | Embedded, zero-dep, BM25, field weights, incremental, Unicode, transactions | Single-writer, no distributed scaling | ✅ **Selected** |
| Tantivy | Fast, Rust-native, BM25, field weights, incremental | External dep, separate index files, more complex | ❌ Future option |
| Meilisearch | HTTP API, typo tolerance, faceting | External service, resource heavy | ❌ Overkill |
| Elasticsearch/Opensearch | Distributed, rich features | Heavy, external, operational burden | ❌ Overkill |
| PostgreSQL tsvector | ACID, SQL integration | Weaker ranking, no field weights in BM25 | ❌ Weaker relevance |
| Custom inverted index | Full control | Reinventing wheel, maintenance burden | ❌ Not worth it |

## Consequences

### Positive
- **Zero infrastructure**: Runs in same SQLite process
- **Transactional**: Search index updates in same transaction as document store
- **BM25 ranking**: Industry-standard relevance with `rank` function
- **Field weights**: `title^10 description^5 headings^2 body^1` via `bm25()` parameters
- **Incremental**: `INSERT`/`UPDATE`/`DELETE` on virtual table
- **Unicode**: `tokenize='porter unicode61'` for stemming + Unicode
- **Prefix search**: `term*` for autocomplete
- **Phrase search**: `"exact phrase"` support
- **NEAR queries**: `term1 NEAR/5 term2` for proximity
- **Column filtering**: Search specific fields only
- **Mature**: SQLite FTS5 is battle-tested (since 2015)

### Negative
- **Single writer**: Only one indexing transaction at a time (mitigated by batching)
- **No distributed search**: Single-node only (acceptable for local knowledge base)
- **Limited analyzers**: Porter stemmer only (no custom analyzers without compile-time extension)
- **No faceting**: Must implement via metadata queries
- **Schema changes**: FTS5 virtual table recreation required for schema changes

## Implementation

### Virtual Table Definition

```sql
CREATE VIRTUAL TABLE document_search USING fts5(
    path,
    title,
    description,
    headings,
    body,
    concept_type,
    tokenize='porter unicode61'
);
```

### Indexing a Document

```rust
let searchable = SearchableDocument {
    path: doc.path.clone(),
    title: doc.title.clone(),
    description: doc.description.clone(),
    headings: doc.headings.iter().map(|h| h.title.clone()).collect::<Vec<_>>().join(" "),
    body: doc.body_text.clone(),
    concept_type: doc.concept_type.clone(),
};

self.search_index.index_document(&searchable)?;
```

### Search Query with BM25 Weights

```sql
SELECT
    path,
    title,
    description,
    concept_type,
    bm25(document_search, 10.0, 5.0, 2.0, 1.0, 0.0) AS rank
FROM document_search
WHERE document_search MATCH ?
ORDER BY rank
LIMIT ?;
```

Weights: `title=10.0`, `description=5.0`, `headings=2.0`, `body=1.0`, `concept_type=0.0`

### Search Query Features

| Feature | Syntax | Example |
|---------|--------|---------|
| Term | `term` | `rust` |
| Phrase | `"phrase"` | `"machine learning"` |
| Prefix | `prefix*` | `prog*` |
| NEAR | `t1 NEAR/n t2` | `async NEAR/5 await` |
| Column | `col:term` | `title:rust` |
| NOT | `-term` | `rust -async` |
| AND (implicit) | `t1 t2` | `rust async` |
| OR | `t1 OR t2` | `rust OR go` |

### Incremental Updates

```rust
// Add/Update
INSERT INTO document_search(rowid, path, title, description, headings, body, concept_type)
VALUES (?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(rowid) DO UPDATE SET
    title=excluded.title,
    description=excluded.description,
    headings=excluded.headings,
    body=excluded.body,
    concept_type=excluded.concept_type;

// Delete
DELETE FROM document_search WHERE path = ?;
```

## Performance Characteristics

| Operation | Complexity | Typical Latency |
|-----------|------------|-----------------|
| Index 1 doc | O(doc size) | <5ms |
| Search (10k docs) | O(log n) | <20ms |
| Search (100k docs) | O(log n) | <50ms |
| Delete 1 doc | O(log n) | <5ms |

## Migration Path to Tantivy (Future)

If scaling beyond SQLite FTS5 limits:

1. Implement `SearchIndex` trait for Tantivy (ADR-003)
2. Keep SQLite for metadata, move search to Tantivy
3. Dual-write during transition
4. Feature flag to switch backends

## Related ADRs

- ADR-001: SQLite as Primary Storage Backend
- ADR-003: Trait-Based Storage Abstraction
- ADR-004: MCP Transport for AI Integration

## References

- [SQLite FTS5 Documentation](https://www.sqlite.org/fts5.html)
- [FTS5 BM25 Ranking](https://www.sqlite.org/fts5.html#bm25)
- [FTS5 Tokenizers](https://www.sqlite.org/fts5.html#tokenizers)
- [rusqlite FTS5 Examples](https://github.com/rusqlite/rusqlite/blob/master/examples/fts5.rs)