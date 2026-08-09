---
id: OKC-00105
title: Persist root identity and isolate multi-root repositories
status: Done
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:52'
updated_date: '2026-08-09 23:32'
labels:
  - backend
  - indexing
  - multi-root
  - data-integrity
  - mcp
dependencies: []
references:
  - src/scanner/walker.rs
  - src/index/migrations.rs
documentation:
  - docs/architecture.md
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Make multi-root catalogs preserve the identity of each configured source root. Today documents are keyed only by root-relative path, so equal paths from different roots can collide and query results cannot be scoped reliably. Introduce stable root identity, collision-safe document identity, migration behavior, root-aware links, and query filtering while preserving single-root compatibility.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each indexed document is associated with a stable root identifier that is exposed through documented service, CLI, and MCP response fields
- [ ] #2 Documents with the same relative path in different roots coexist without overwrite, ambiguity, or cross-root metadata leakage
- [ ] #3 Existing single-root indexes migrate or rebuild safely with unchanged user-visible paths
- [ ] #4 Search and metadata queries support optional root filtering, and unfiltered queries retain all-root behavior
- [ ] #5 Link resolution and graph traversal define and enforce same-root, explicit cross-root, ambiguous-target, and removed-root behavior
- [ ] #6 Statistics include per-root counts and root removal cleans up only documents owned by that root
- [ ] #7 Integration tests cover collisions, filtering, cross-root links, migration/rebuild, root removal, and single-root compatibility
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers root filtering in all query tools
- [ ] #2 Integration test: multi-root scan → filtered queries
- [ ] #3 Backward compatible: no root filter = all roots
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Implementation Plan for OKC-00105: Persist Root Identity & Isolate Multi-Root Repositories

### Phase 1: Schema & Migration (Database Layer)
1. Add `roots` table to track configured roots with stable IDs
2. Add `root_id` column to `documents` table (NOT NULL, default to 1 for backward compat)
3. Update unique constraint on `documents.path` to composite `(root_id, path)`
4. Create migration v3 in `migrations.rs` to:
   - Create `roots` table
   - Add `root_id` to `documents` with default 1
   - Migrate existing single-root data to root_id=1
   - Add composite index on `(root_id, path)`

### Phase 2: Config & Scanner (Input Layer)
5. Update `OkcConfig::roots` from `Vec<PathBuf>` to `Vec<RootConfig>` where `RootConfig { id: String, path: PathBuf }`
6. Generate stable root IDs from config (hash of path, or explicit user-provided ID)
7. Update `Scanner::discover` to accept root_id and return `FileRecord` with `root_id`
8. Update `FileRecord` to include `root_id` field

### Phase 3: Document Store & Index (Storage Layer)
9. Update `DocumentRecord` to include `root_id`
10. Update all `document_store` methods to filter/insert by root_id
11. Update `RepositoryIndex::scan` to process changes per-root
12. Update `ChangeDetector` to work with root-scoped paths
13. Update FTS `document_search` to include root_id (UNINDEXED)
14. Update `SqliteSearchIndex` queries to filter by root_id when provided

### Phase 4: Query & API Layer
15. Add optional `root_id` filter to:
    - `SearchFilters` (for search)
    - `MetadataParams.filter` (for query_metadata)
    - `BrowseParams` (for directory browsing)
    - `GetDocumentParams` (for get_document - if root_id specified, scope lookup)
16. Update `traverse_graph` and link resolution with root-aware behavior:
    - Same-root links: resolve within root
    - Cross-root links: require explicit root qualification
    - Ambiguous targets: warn and apply deterministic rule (first root wins)
    - Removed root: mark links as broken
17. Update `IndexStats` to include per-root document/error/link/heading counts

### Phase 5: MCP & CLI Exposure
18. Add `root_id` field to all MCP output types (`DocumentDetailOutput`, `SearchResultOutput`, etc.)
19. Add `root_filter` parameter to MCP tool params (optional)
20. Update CLI commands to support `--root` flag

### Phase 6: Tests & Migration Verification
21. Integration test: multi-root scan with colliding paths - verify both stored
22. Integration test: filtered queries per root
23. Integration test: cross-root link resolution behavior
24. Integration test: root removal cleanup
25. Integration test: single-root backward compatibility (no root filter = all roots)
26. Migration test: existing single-root index upgrades to multi-root schema

### Phase 7: Documentation
27. Update `docs/architecture.md` with multi-root architecture
28. Update CLI/MCP docs for root filtering
<!-- SECTION:PLAN:END -->
