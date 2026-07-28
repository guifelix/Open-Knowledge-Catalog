---
type: decision
title: "ADR-003: Trait-Based Storage Abstraction"
description: Decision to use Rust traits for storage abstraction, enabling interchangeable backends
tags: [adr, traits, abstraction, architecture, decision]
owner: felix
status: stable
---

# ADR 003: Trait-Based Storage Abstraction

**Status**: Accepted
**Date**: 2026-07-25
**Deciders**: Architecture team
**Technical Story**: OKC-00033.05

## Context

The storage layer needs to support:
- Current: SQLite with FTS5 and graph tables
- Future: PostgreSQL, Tantivy, or other backends
- Testing: In-memory SQLite, mock implementations
- Clear separation between service logic and storage implementation

## Decision

Define **storage traits** (`DocumentStore`, `SearchIndex`, `GraphStore`) in `index/traits.rs` and implement them for SQLite. The service layer depends only on traits, not concrete types.

## Alternatives Considered

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| **Trait abstraction** | Swappable backends, testable, clean architecture | Slight indirection, trait object overhead | ✅ **Selected** |
| Concrete types only | Simple, zero overhead | Hard to test, locked to SQLite | ❌ Rejected |
| Generics on service | Zero-cost abstraction | Leaks storage types to service, complex | ❌ Rejected |
| Repository pattern (struct with methods) | Familiar | Still couples to concrete impl | ❌ Rejected |

## Consequences

### Positive
- **Backend swap**: SQLite → PostgreSQL/Tantivy without changing service layer
- **Testing**: Mock implementations, in-memory SQLite for unit tests
- **Separation of concerns**: Service logic independent of storage details
- **Parallel development**: Storage and service can evolve independently
- **Feature flags**: Enable/disable graph store, search backend at compile time

### Negative
- **Trait object overhead**: Dynamic dispatch (mitigated: `Box<dyn Trait>` or generics)
- **Trait design complexity**: Must anticipate all backend capabilities
- **Leaky abstractions**: SQLite-specific features may not map cleanly

## Implementation

### Traits (`index/traits.rs`)

```rust
use async_trait::async_trait;  // If async needed later

pub trait DocumentStore: Send + Sync {
    fn upsert_document(&self, doc: &DocumentRecord) -> Result<()>;
    fn get_document(&self, path: &str) -> Result<Option<DocumentRecord>>;
    fn delete_document(&self, path: &str) -> Result<()>;
    fn list_documents(&self, parent_path: &str) -> Result<Vec<DocumentRecord>>;
    // Tags, headings, links, metadata, errors...
}

pub trait SearchIndex: Send + Sync {
    fn index_document(&self, doc: &SearchableDocument) -> Result<()>;
    fn remove_document(&self, path: &str) -> Result<()>;
    fn search(&self, query: &SearchQuery) -> Result<SearchResponse>;
}

pub trait GraphStore: Send + Sync {
    fn store_links(&self, source: &str, links: &[Link]) -> Result<()>;
    fn remove_links(&self, source: &str) -> Result<()>;
    fn get_links(&self, path: &str) -> Result<Vec<GraphEdge>>;
    fn get_backlinks(&self, path: &str, limit: usize) -> Result<Vec<GraphEdge>>;
    fn traverse(&self, params: &TraverseParams) -> Result<TraverseResponse>;
}
```

### RepositoryIndex Composition

```rust
pub struct RepositoryIndex {
    pub(crate) conn: Connection,
    pub(crate) document_store: Box<dyn DocumentStore>,
    pub(crate) search_index: Box<dyn SearchIndex>,
    pub(crate) graph_store: Option<Box<dyn GraphStore>>,
    pub(crate) config: OkcConfig,
}

impl RepositoryIndex {
    pub fn open(config: &OkcConfig) -> Result<Self> {
        let conn = Connection::open(&config.db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        let doc_conn = Connection::open(&config.db_path)?;
        let search_conn = Connection::open(&config.db_path)?;
        let graph_conn = Connection::open(&config.db_path)?;

        Ok(Self {
            conn,
            document_store: Box::new(SqliteDocumentStore::new(doc_conn)),
            search_index: Box::new(SqliteSearchIndex::new(search_conn)),
            graph_store: Some(Box::new(SqliteGraphStore::new(graph_conn))),
            config: config.clone(),
        })
    }
}
```

### Service Layer Usage

```rust
pub struct OkcService {
    index: RepositoryIndex,  // Uses trait methods internally
}

impl OkcService {
    pub fn search(&self, query: &str, ...) -> Result<SearchResponse> {
        self.index.search_index.search(&SearchQuery { ... })
    }

    pub fn traverse(&self, start: &str, ...) -> Result<TraverseResponse> {
        if let Some(ref gs) = self.index.graph_store {
            gs.traverse(&TraverseParams { ... })
        } else {
            Err(anyhow!("Graph store not available"))
        }
    }
}
```

## Testing Benefits

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::traits::{DocumentStore, SearchIndex, GraphStore};

    // Mock implementations for unit tests
    struct MockDocumentStore { ... }
    impl DocumentStore for MockDocumentStore { ... }

    struct MockSearchIndex { ... }
    impl SearchIndex for MockSearchIndex { ... }

    #[test]
    fn test_service_search_delegates_to_index() {
        let mock_index = MockSearchIndex::new();
        let service = OkcService { index: mock_index };
        // Test service logic without real database
    }
}
```

## Future Backend Implementation

```rust
// PostgreSQL implementation
pub struct PostgresDocumentStore { pool: PgPool }
impl DocumentStore for PostgresDocumentStore { ... }

// Tantivy search implementation
pub struct TantivySearchIndex { index: tantivy::Index }
impl SearchIndex for TantivySearchIndex { ... }

// In-memory for testing
pub struct MemoryDocumentStore { docs: HashMap<String, DocumentRecord> }
impl DocumentStore for MemoryDocumentStore { ... }
```

## Related ADRs

- ADR-001: SQLite as Primary Storage Backend
- ADR-002: FTS5 for Full-Text Search
- ADR-004: MCP Transport for AI Integration

## References

- [Rust Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Repository Pattern](https://martinfowler.com/eaaCatalog/repository.html)