# Module Dependency Diagram

## Overview

This document shows the module dependencies in the Open Knowledge Catalog crate. Arrows indicate "depends on" (uses types/functions from).

```mermaid
graph TD
    MAIN["main.rs\n(CLI entry point)"]
    
    subgraph TRANSPORT["transport/"]
        CLI["cli.rs\n(clap commands)"]
        MCP["mcp.rs\n(MCP server)"]
    end
    
    subgraph SERVICE["service/"]
        OKCSERVICE["OkcService (Facade)"]
        BROWSE["Browse"]
        DOCS["Documents"]
        GRAPH["Graph"]
        SEARCH["Search"]
        VALIDATE["Validate"]
        WATCH["Watch"]
    end
    
    subgraph INDEX["index/"]
        REPOINDEX["RepositoryIndex"]
        DOCSTORE["Document Store\n(SqliteDocumentStore)"]
        SEARCHIDX["Search Index\n(SqliteSearchIndex)"]
        GRAPHSTORE["Graph Store\n(SqliteGraphStore)"]
    end
    
    subgraph INDEX_SUPPORT["index/ (support modules)"]
        DATABASE["database.rs\n(scan, process)"]
        QUERIES["queries.rs\n(search, browse, get, etc)"]
        VALIDATE_IDX["validate.rs\n(8 checks)"]
        PARSER_IDX["parser.rs (index)\n(process_changes, DocumentParser)"]
        TRAITS["traits.rs\n(DocStore, SearchIdx, GraphStore)"]
        MIGRATIONS["migrations.rs\n(schema v1)"]
        EXPORT["export.rs\n(JSON)"]
        GRAPH_IDX["graph.rs / graph_store.rs\n(traversal algos)"]
    end
    
    subgraph PARSER["parser/"]
        FRONTMATTER["frontmatter.rs\n(YAML bounds)"]
        YAML["yaml.rs\n(saphyr)"]
        MARKDOWN["markdown.rs\n(pulldown-cmark)"]
        LINKS["links.rs\n(wiki-links, URLs, resolution)"]
    end
    
    subgraph SCANNER["scanner/"]
        WALKER["walker.rs\n(parallel walk, ignore)"]
        CHANGES["changes.rs\n(diff: add/mod/del)"]
        WATCHER["watcher.rs\n(notify, debounce, reconcile)"]
    end
    
    subgraph MODEL["model/"]
        DOCUMENT["document.rs\n(FileRecord, FrontMatter,\nSearchResult, Link, etc)"]
        DIRECTORY["directory.rs\n(BrowseResp, DirNode, DirDoc)"]
        GRAPH_MODEL["graph.rs\n(GraphEdge, TraverseNode,\nTraverseResp)"]
    end
    
    CONFIG["config.rs\n(OkcConfig - figment)"]
    
    MAIN --> TRANSPORT
    TRANSPORT --> SERVICE
    SERVICE --> INDEX
    INDEX --> INDEX_SUPPORT
    INDEX_SUPPORT --> PARSER
    INDEX_SUPPORT --> SCANNER
    INDEX_SUPPORT --> MODEL
    PARSER --> MODEL
    SCANNER --> CONFIG
    SCANNER --> MODEL
    INDEX --> CONFIG
    INDEX --> MODEL
    INDEX --> PARSER
    INDEX --> SCANNER
    SERVICE --> CONFIG
    SERVICE --> MODEL
    TRANSPORT --> CONFIG
    TRANSPORT --> MODEL
    MAIN --> CONFIG
    MAIN --> MODEL
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

```mermaid
graph LR
    TRAITS["index/traits.rs"]
    TRAITS --> DOCSTORE_TRAIT["DocumentStore"]
    TRAITS --> SEARCHIDX_TRAIT["SearchIndex"]
    TRAITS --> GRAPHSTORE_TRAIT["GraphStore"]
    
    DOCSTORE_TRAIT --> SQLITE_DOC["SqliteDocumentStore"]
    DOCSTORE_TRAIT --> MOCK_DOC["MockDocumentStore"]
    DOCSTORE_TRAIT --> MEMORY_DOC["MemoryDocumentStore"]
    
    SEARCHIDX_TRAIT --> SQLITE_SEARCH["SqliteSearchIndex"]
    SEARCHIDX_TRAIT --> TANTIVY_SEARCH["TantivySearchIndex"]
    SEARCHIDX_TRAIT --> MOCK_SEARCH["MockSearchIndex"]
    
    GRAPHSTORE_TRAIT --> SQLITE_GRAPH["SqliteGraphStore"]
    GRAPHSTORE_TRAIT --> MOCK_GRAPH["MockGraphStore"]
    GRAPHSTORE_TRAIT --> MEMORY_GRAPH["MemoryGraphStore"]
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