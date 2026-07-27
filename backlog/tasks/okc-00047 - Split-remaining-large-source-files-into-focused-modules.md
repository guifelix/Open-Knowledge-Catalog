---
id: OKC-00047
title: Split remaining large source files into focused modules
status: In Progress
assignee:
  - '@backend-agent'
created_date: '2026-07-26 22:47'
updated_date: '2026-07-26 22:49'
labels:
  - refactor
  - tech-debt
dependencies:
  - OKC-00031
references:
  - docs/backlog-draft-workflow.md
documentation:
  - docs/implementation-plan.md
priority: medium
type: enhancement
ordinal: 39000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Several source files have grown significantly and are strong candidates for breaking into smaller, single-responsibility modules. These remain after prior refactors (OKC-00021, OKC-00033.02) already addressed database.rs and RepositoryIndex.

Files to split (lines as of 2026-07-26):

**Tier 1 — High priority (>600 lines)**

1. **src/config.rs** (738 lines) — Config structs + defaults + env overrides + validation + CLI config creation + 15 inline tests. Split: extract Bm25Config into config/bm25.rs, extract inline tests into config/tests.rs.

2. **src/index/document_store.rs** (726 lines) — Monolithic SQLite store with every CRUD operation: documents, headings, tags, tables, code blocks, links, metadata fields. Split by entity into sub-modules: document_store/documents.rs, document_store/headings.rs, document_store/tags.rs, document_store/links.rs, document_store/tables.rs, document_store/code_blocks.rs.

3. **src/parser/links.rs** (685 lines) — LinkResolver + standalone fns + 20 inline tests. Split: extract tests into separate module, extract standalone fns (normalize_path, is_safe_path, extract_wiki_links) into parser/link_utils.rs.

4. **src/transport/mcp.rs** (643 lines) — MCP server with 27 tool param/result structs + all tool handlers in one file. Split: extract types into transport/mcp/types.rs, extract each tool handler into transport/mcp/tools/<name>.rs.

5. **src/index/queries.rs** (637 lines) — 7 query methods on RepositoryIndex (browse, get_document, get_section, search, query_metadata, recently_modified, get_stats) + helpers. Split by query domain.

**Tier 2 — Moderate priority (400-500 lines)**

6. **src/parser/markdown.rs** (499 lines) — Single parse() returning a 6-tuple. Split parsing stages: heading extraction, link extraction, section building, table parsing, code blocks.

7. **src/model/document.rs** (468 lines) — 28 structs/enums dumped in one file spanning file records, headings, tables, code blocks, links, parsed docs, search results, validation, stats. Pure types — lowest-risk split. Move into model/*.rs by domain.

8. **src/scanner/watcher.rs** (410 lines) — File watcher with event handling. Split event handler from watching loop.

9. **src/index/validate.rs** (401 lines) — Validation checks on RepositoryIndex. Split each check type into own helper.

**Guiding principles:**
- Pure code movement — no behavior changes, no logic refactors
- Each new module gets a doc comment explaining its public surface
- No circular dependencies introduced
- All existing tests pass without modification after each split
- Commit each file split separately for clean review
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 src/config.rs split — Bm25Config extracted, inline tests moved out, main config remains clean orchestrator
- [ ] #2 src/index/document_store.rs split — CRUD operations organized into entity-focused sub-modules
- [ ] #3 src/parser/links.rs split — standalone fns extracted, inline tests moved to dedicated test module
- [ ] #4 src/transport/mcp.rs split — tool types and handlers organized into focused sub-modules
- [ ] #5 src/index/queries.rs split — query methods organized by domain into sub-modules
- [ ] #6 src/parser/markdown.rs split — parse stages extracted into focused helper functions
- [ ] #7 src/model/document.rs split — domain types organized into dedicated per-domain files
- [ ] #8 src/scanner/watcher.rs split — event handling separated from watching loop
- [ ] #9 src/index/validate.rs split — validation check types extracted into focused helpers
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Each split is pure code movement — logic changes are zero
- [ ] #2 Each new module has a doc comment explaining its public API surface
- [ ] #3 No circular dependencies introduced between new modules
- [ ] #4 cargo test passes after every commit
- [ ] #5 cargo fmt --check and cargo clippy -- -D warnings pass
- [ ] #6 Each file is split in its own commit with descriptive message
<!-- DOD:END -->
