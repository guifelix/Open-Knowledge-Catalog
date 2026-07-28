---
type: decision
title: "ADR-001: SQLite as Primary Storage Backend"
description: Decision to use SQLite as the primary storage backend for its portability, zero configuration, and single-file simplicity
tags: [adr, storage, sqlite, architecture, decision]
owner: felix
status: stable
---

# ADR-001: SQLite as Primary Storage Backend

**Status**: Accepted
**Date**: 2026-07-25
**Deciders**: Engineering Team

## Context

The Open Knowledge Catalog needs a storage backend for:
- Document metadata (title, type, description, tags, custom fields)
- Document content (body text, headings)
- Link graph (internal/external links, backlinks)
- Full-text search index
- Scan state (file records, content hashes, errors)
- Incremental indexing support

Requirements:
- Embedded (no separate server process)
- ACID transactions
- Concurrent reads during writes
- Full-text search capability
- Mature, well-tested, widely deployed
- Cross-platform (Linux, macOS, Windows)
- Rust ecosystem support

## Decision

Use **SQLite** (via `rusqlite`) as the primary storage backend with:
- **WAL (Write-Ahead Logging) mode** for concurrent readers
- **FTS5 virtual table** for full-text search
- **Foreign keys** for referential integrity
- **In-memory mode** for testing

## Alternatives Considered

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| **SQLite** | Embedded, ACID, WAL, FTS5, zero-config, mature | Single-writer, no horizontal scaling | ✅ **Selected** |
| PostgreSQL | Full SQL, scaling, rich types | External server, operational overhead | ❌ Overkill |
| RocksDB / Sled | Embedded, fast KV | No SQL, no FTS, manual indexing | ❌ Too low-level |
| Tantivy | Fast search, Rust-native | No SQL, separate index, no transactions | ❌ Search only |
| Redb | Embedded, ACID, Rust-native | Young, no FTS, limited ecosystem | ❌ Immature |
| SurrealDB | Embedded + server, GraphQL | Complex, young, overkill | ❌ Overkill |

## Consequences

### Positive
- **Zero operational overhead**: Single file, no server process
- **Concurrent reads**: WAL mode allows unlimited readers during writes
- **Full-text search built-in**: FTS5 with BM25, field weights, tokenizers
- **Transactions**: Atomic multi-table updates (documents + links + search)
- **Portability**: Single `.db` file, easy backup/copy/migrate
- **Testing**: In-memory mode (`file::memory:?cache=shared`) for fast isolated tests
- **Rust ecosystem**: `rusqlite` is mature, well-maintained, type-safe

### Negative
- **Single writer**: Only one write transaction at a time (mitigated by batching)
- **No horizontal scaling**: Single-node only (acceptable for local/knowledge-base use case)
- **Schema migrations**: Manual (handled by `migrations.rs` module)
- **Connection pooling**: Manual (each store gets own connection)

## Implementation Notes

```rust
// Open with WAL mode and foreign keys
let conn = Connection::open(&config.db_path)?;
conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

// Separate connections per store for isolation
let doc_conn = Connection::open(&config.db_path)?;
let search_conn = Connection::open(&config.db_path)?;
let graph_conn = Connection::open(&config.db_path)?;

// In-memory for testing (shared cache for multi-connection)
let conn = Connection::open("file::memory:?cache=shared")?;
```

## Migration Strategy

- Versioned migrations in `migrations.rs`
- `schema_version` table tracks applied version
- `run(conn)` is idempotent, safe to call on every startup
- New migrations append to the function

## Related ADRs

- ADR-002: FTS5 for Full-Text Search
- ADR-003: Trait-Based Storage Abstraction (for future backend swap)

## References

- [SQLite WAL Mode](https://www.sqlite.org/wal.html)
- [SQLite FTS5](https://www.sqlite.org/fts5.html)
- [rusqlite crate](https://github.com/rusqlite/rusqlite)