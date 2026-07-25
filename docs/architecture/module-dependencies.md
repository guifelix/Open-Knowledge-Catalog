# Module Dependency Diagram

## Overview

This document shows the module dependencies in the Open Knowledge Catalog crate. Arrows indicate "depends on" (uses types/functions from).

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                    main.rs                                       │
│                              (CLI entry point)                                   │
└─────────────────────────────────────┬───────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                  transport/                                      │
│  ┌─────────────────────┐    ┌─────────────────────┐                             │
│  │       cli.rs        │    │       mcp.rs        │                             │
│  │  (clap commands)    │    │  (MCP server)       │                             │
│  └──────────┬──────────┘    └──────────┬──────────┘                             │
└─────────────┼──────────────────────────┼────────────────────────────────────────┘
              │                          │
              ▼                          ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                  service/                                        │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                            OkcService (Facade)                              │ │
│  │  ┌─────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ ┌──────────┐ ┌──────────┐  │ │
│  │  │ Browse  │ │ Documents│ │ Graph  │ │ Search │ │ Validate │ │  Watch   │  │ │
│  │  └────┬────┘ └────┬─────┘ └────┬───┘ └────┬───┘ └────┬─────┘ └────┬─────┘  │ │
│  └───────┼───────────┼────────────┼───────────┼───────────┼────────────┼────────┘ │
└──────────┼───────────┼────────────┼───────────┼───────────┼────────────┼──────────┘
           │           │            │           │           │            │
           ▼           ▼            ▼           ▼           ▼            ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                    index/                                        │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                          RepositoryIndex                                     │ │
│  │  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────────────────┐   │ │
│  │  │ Document Store  │ │  Search Index   │ │       Graph Store           │   │ │
│  │  │ (SqliteDocStore)│ │(SqliteSearchIdx)│ │      (SqliteGraphStore)     │   │ │
│  │  └────────┬────────┘ └────────┬────────┘ └──────────────┬──────────────┘   │ │
│  └───────────┼───────────────────┼─────────────────────────┼───────────────────┘ │
└──────────────┼───────────────────┼─────────────────────────┼──────────────────────┘
               │                   │                         │
               ▼                   ▼                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              index/ (support modules)                            │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌────────────────────────┐  │
│  │  database.rs │ │  queries.rs  │ │  validate.rs │ │  parser.rs (index)     │  │
│  │  (scan,      │ │  (search,    │ │  (8 checks)  │ │  (process_changes,    │  │
│  │   process)   │ │   browse,    │ │              │ │   DocumentParser)      │  │
│  │              │ │   get, etc)  │ │              │ │                        │  │
│  └──────────────┘ └──────────────┘ └──────────────┘ └────────────────────────┘  │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌────────────────────────┐  │
│  │  traits.rs   │ │ migrations.rs│ │  export.rs   │ │  graph.rs / graph_store│  │
│  │  (DocStore,  │ │  (schema v1) │ │  (JSON)      │ │  (traversal algos)     │  │
│  │   SearchIdx, │ │              │ │              │ │                        │  │
│  │   GraphStore)│ │              │ │              │ │                        │  │
│  └──────────────┘ └──────────────┘ └──────────────┘ └────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
               │                   │                         │
               ▼                   ▼                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                  parser/                                         │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌────────────────────────┐  │
│  │ frontmatter.rs│ │   yaml.rs    │ │ markdown.rs  │ │      links.rs        │  │
│  │ (YAML bounds) │ │ (saphyr)     │ │(pulldown-    │ │  (wiki-links, URLs,  │  │
│  │               │ │              │ │  cmark)      │ │   resolution)        │  │
│  └──────────────┘ └──────────────┘ └──────────────┘ └────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
               │                   │                         │
               ▼                   ▼                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                  scanner/                                        │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐                             │
│  │  walker.rs   │ │  changes.rs  │ │  watcher.rs  │                             │
│  │ (parallel    │ │ (diff: add/  │ │ (notify,     │                             │
│  │  walk,      │ │  mod/del)    │ │  debounce,   │                             │
│  │  ignore)     │ │              │ │  reconcile)  │                             │
│  └──────────────┘ └──────────────┘ └──────────────┘                             │
└─────────────────────────────────────────────────────────────────────────────────┘
               │                   │                         │
               ▼                   ▼                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                  model/                                          │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐                             │
│  │ document.rs  │ │ directory.rs │ │   graph.rs   │                             │
│  │ (FileRecord, │ │ (BrowseResp, │ │ (GraphEdge,  │                             │
│  │  FrontMatter,│ │  DirNode,    │ │  TraverseNode,│                             │
│  │  SearchResult│ │  DirDoc)     │ │  TraverseResp)│                             │
│  │  Link, etc)  │ │              │ │              │                             │
│  └──────────────┘ └──────────────┘ └──────────────┘                             │
└─────────────────────────────────────────────────────────────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                  config.rs                                       │
│                           (OkcConfig - figment)                                  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Dependency Matrix

| Module | config | model | parser | scanner | index | service | transport |
|--------|--------|-------|--------|---------|-------|---------|-----------|
| **config** | - | - | - | - | - | - | - |
| **model** | - | - | - | - | - | - | - |
| **parser** | - | ✓ | - | - | - | - | - |
| **scanner** | ✓ | ✓ | - | - | - | - | - |
| **index** | ✓ | ✓ | ✓ | ✓ | - | - | - |
| **service** | ✓ | ✓ | - | - | ✓ | - | - |
| **transport** | ✓ | ✓ | - | - | - | ✓ | - |
| **main** | ✓ | ✓ | - | - | - | ✓ | ✓ |

## Key Dependency Rules

1. **config** - No dependencies (leaf)
2. **model** - No dependencies (leaf, pure data types)
3. **parser** - Depends on `model` (outputs `ParsedDocument` with model types)
4. **scanner** - Depends on `config` (exclude patterns, limits), `model` (`FileRecord`)
5. **index** - Depends on `config`, `model`, `parser`, `scanner` (orchestrates scan)
6. **service** - Depends on `config`, `model`, `index` (facade over RepositoryIndex)
7. **transport** - Depends on `config`, `model`, `service` (CLI/MCP expose service)
8. **main** - Depends on all (composition root)

## Trait Boundaries (for Testing/Swapping)

```
index/traits.rs
├── DocumentStore  →  SqliteDocumentStore, MockDocumentStore, MemoryDocumentStore
├── SearchIndex    →  SqliteSearchIndex,  TantivySearchIndex,  MockSearchIndex
└── GraphStore     →  SqliteGraphStore,   MockGraphStore,      MemoryGraphStore
```

Service layer uses `RepositoryIndex` which composes trait objects:
```rust
pub struct RepositoryIndex {
    document_store: Box<dyn DocumentStore>,
    search_index: Box<dyn SearchIndex>,
    graph_store: Option<Box<dyn GraphStore>>,
    ...
}
```

## Circular Dependency Prevention

- **No cycles**: Dependency graph is a DAG (topological order: config → model → parser/scanner → index → service → transport → main)
- **Traits in index/**: Break implementation cycles (index defines traits, implements them)
- **Parser in index/**: `index/parser.rs` is distinct from `parser/` module - it's the orchestration layer that uses `parser/` modules

## Feature Flags (Future)

```toml
[features]
default = ["sqlite", "mcp"]
sqlite = ["rusqlite", "sqlite-fts5"]
postgres = ["sqlx", "postgres"]
tantivy = ["tantivy"]
mcp = ["rmcp", "tokio"]
cli = ["clap"]
watch = ["notify"]
```