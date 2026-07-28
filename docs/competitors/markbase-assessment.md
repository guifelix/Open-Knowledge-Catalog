# Competitor Assessment: markbase

## Overview

**markbase** (https://crates.io/crates/markbase, https://github.com/flyisland/markbase) is an Obsidian-compatible CLI tool built for agent-driven Markdown vault workflows. It centers on a three-way contract: **Human (intent) → Agent (fill templates) → markbase (enforce schema)**.

Key characteristics:
- Written in Rust (requires 1.85+)
- Uses **DuckDB** as a rebuildable derived index (source of truth = Markdown files)
- Template system with `_schema` frontmatter block defining structure, required fields, types, enums, link targets
- `note verify` enforces schema compliance after creation/edits
- Obsidian-compatible: wikilinks `[[note]]`, embeds `![[note]]`, `.base` views, frontmatter
- Web server (`web serve`) for browser inspection
- Source evidence attachment workflow for evidence-based notes

---

## Template System Design Comparison

| Aspect | markbase | OKC | Notes |
|--------|----------|-----|-------|
| **Schema location** | In template's `_schema` frontmatter key | No formal schema; implicit via `concept_type` conventions | markbase embeds schema in template; OKC uses loose conventions |
| **Field types** | `text`, `number`, `boolean`, `date`, `datetime`, `list` | Frontmatter parsed as YAML → `serde_json::Value`; no type enforcement | markbase validates types; OKC accepts any valid YAML |
| **Required fields** | `_schema.required: [field1, field2]` | Hardcoded `title` and `type` in validation | markbase is template-driven; OKC is global-constant |
| **Enum constraints** | `_schema.properties.field.enum: [A, B]` | Not supported | markbase enables closed-world validation |
| **Link target types** | `_schema.properties.field.format: "link"`, `target: "company"` | Links extracted but not validated against target type | markbase verifies `[[Acme]]` points to a `type: company` note |
| **Filename patterns** | `_schema.filename.pattern: "{{timestamp}}_{{name}}"` | Not supported | markbase supports generated filenames with template variables |
| **Location enforcement** | `_schema.location: "customers/"` → verified on `note verify` | Documents indexed anywhere under roots; no enforced location | markbase binds template → directory |
| **Agent-writable sections** | Template body declares writable sections (implicitly via `create` block) | No concept; any frontmatter/body writable | markbase separates `create` (agent-filled) from template outer FM |
| **Multi-template composition** | `templates: ["[[t1]]", "[[t2]]"]` merged during verify | Single `concept_type` per document | markbase supports schema composition; OKC is single-type |
| **Default values** | `_schema.create.field: default` → injected at creation | Not supported | markbase distinguishes template-time vs create-time defaults |
| **Verification command** | `markbase note verify <name>` | `okc validate` (repo-wide, not per-note) | markbase is per-note + template-specific; OKC is global |

**Key markbase template example** (from README):
```yaml
---
type: company
owner: Alice
_schema:
  location: customers/
  create:
    type: company
    tags: []
---
# {{name}}
```

**OKC equivalent** (convention, not enforced):
```yaml
---
title: Acme
type: company
tags: [customer]
owner: "[[Alice]]"
---
# Acme
```

---

## DuckDB Integration Patterns

### markbase
- **Purpose**: Derived index for fast queries, link resolution, rendering
- **Schema**: Single `notes` table with columns: `path`, `folder`, `name`, `ext`, `size`, `ctime`, `mtime`, `tags[]`, `links[]`, `backlinks[]`, `embeds[]`, `properties JSON`
- **Indexing**: Incremental via `scanner.rs`; computes mtime/size comparison; rebuildable from filesystem
- **Queries**: Custom expression language → SQL translation (`query/translator.rs`) + raw SQL passthrough
- **Security**: SELECT-only; multi-statement rejection
- **Web integration**: Per-request DB handle + index refresh before route resolution

### OKC
- **Backend**: SQLite (via `rusqlite`) for document metadata + Tantivy for full-text search
- **Schema**: `documents` table with `path`, `parent_path`, `title`, `concept_type`, `description`, `tags`, `content_hash`, `frontmatter_json`, `file_size`, `modified_at`
- **Indexing**: `scanner/watcher` with notify-based incremental updates; full rebuild via `scan`
- **Queries**: Structured metadata queries via `query_metadata` (filter/select/limit); full-text via Tantivy
- **Graph**: Separate `graph_store` (SQLite) for link traversal, backlinks, traversal

### Comparison

| Dimension | markbase (DuckDB) | OKC (SQLite + Tantivy) |
|-----------|-------------------|------------------------|
| **Analytical queries** | Excellent (columnar, vectorized) | Limited (row-oriented SQLite) |
| **Full-text search** | Basic (LIKE, array contains) | Excellent (Tantivy/BM25) |
| **Incremental index** | mtime+size comparison | notify watcher + debounced reconcile |
| **Schema evolution** | `ON CONFLICT` upsert; additive columns | Migrations via `rusqlite` |
| **Embedding/vector** | Not present | Not present (planned via Tantivy HNSW) |
| **Concurrency** | Single-writer (CLI) | Single-writer (service) |
| **Rebuildability** | Explicit design goal | Supported via full scan |

**Assessment**: DuckDB is superior for analytical workloads (aggregations, column scans, complex filters). OKC's Tantivy integration wins for relevance-ranked search. markbase's `query` subsystem (expression→SQL) is more flexible for agent-driven ad-hoc queries; OKC's `query_metadata` is more structured but less expressive.

---

## Three-Way Contract (Human→Agent→System)

### markbase's Model
```
Human: "Record Acme as customer, owner Alice"
  → Agent: `markbase template list` → picks `company_customer`
  → Agent: `markbase note new acme --template company_customer`
  → Agent: fills `create` block fields (type, tags, description)
  → markbase: creates note with `templates: ["[[company_customer]]"]`
  → Agent: `markbase note verify acme` → passes/fails
  → Human: inspects via `note render` or `web serve`
```

### Enforcement Mechanisms
1. **Creation-time**: `creator.rs` only writes `_schema.create` fields + injects `templates` link
2. **Verification-time**: `verifier.rs` checks:
   - Global `description` non-empty
   - `templates` array present with valid wikilinks
   - All `_schema.required` fields present + non-empty
   - Field types match (`text`/`number`/`boolean`/`date`/`datetime`/`list`)
   - Enum values allowed
   - `format: link` fields → target exists + has correct `type`
   - Embedded `.base` files exist
   - Note location matches `_schema.location`

### Applicability to OKC
| markbase Feature | OKC Status | Adoption Value |
|------------------|------------|----------------|
| Per-template schema | ❌ | High: enables agent-friendly structured notes |
| Required field enforcement | Partial (global only) | High: catches missing metadata early |
| Type validation | ❌ | Medium: prevents `type: 123` errors |
| Enum constraints | ❌ | Medium: enforces controlled vocabularies |
| Link target type checking | ❌ | High: ensures `owner: [[Person]]` not `[[Company]]` |
| Location enforcement | ❌ | Low-Medium: organizational convention |
| Embedded view (`.base`) verification | ❌ | Low: OKC has no `.base` equivalent |
| Agent-facing `verify` command | ❌ (repo-wide only) | High: enables CI for agent output |

**Recommendation**: OKC should adopt a **template-driven schema system** for agent-written notes. This is the highest-leverage feature from markbase.

---

## Obsidian Compatibility Layer

| Feature | markbase | OKC |
|---------|----------|-----|
| Wikilinks `[[note]]` | Full (parse, normalize, resolve) | Full (parse, extract, resolve) |
| Wikilink aliases `[[note\|alias]]` | Yes | Yes |
| Wikilink headers `[[note#Heading]]` | Yes (stripped for resolution) | Yes |
| Wikilink blocks `[[note#^block]]` | Yes | Not explicitly tested |
| Embeds `![[note]]` | Full (render expansion) | Extracted as links |
| Embed sizing `![[img.png\|200]]` | Parsed | Not handled |
| `.base` files | First-class (rendered, verified) | Not supported |
| Frontmatter links | Parsed from YAML strings | Extracted via YAML parsing |
| Tags `#tag` | Extracted, normalized (lowercase) | Extracted via frontmatter `tags` array |
| Vault path conventions | Name-based identity (unique stem) | Path-based identity |
| Callouts `[!type]` | Preserved in render | Not special-cased |

**Gap**: OKC uses **path-based** identity (`metrics/revenue.md`), markbase uses **name-based** (`revenue` unique across vault). This is a fundamental architectural difference. markbase's approach matches Obsidian; OKC's matches traditional file systems.

---

## Code Architecture Review

### markbase (single binary, ~15 modules)
```
main.rs (CLI, orchestration)
├── db.rs (DuckDB schema, upsert, query)
├── scanner.rs (vault traversal, incremental index)
├── extractor.rs (frontmatter + body parsing)
├── link_syntax.rs (wikilink/embed tokenization)
├── template.rs (template load, normalize, render)
├── creator.rs (note creation from template)
├── verifier.rs (schema validation)
├── renamer.rs (rename + link rewrite)
├── resolver.rs (name → note resolution)
├── name_validator.rs (path-free name rules)
├── query/ (expression → SQL translation)
├── renderer/ (.base expansion, filter translation)
├── web/ (HTTP server, route resolution)
├── attachment.rs (source evidence workflow)
├── describe.rs (template inspection)
└── output.rs (table/JSON formatting)
```

**Strengths**:
- Clear layering (ARCHITECTURE.md documents invariants)
- Shared logic centralized (`link_syntax.rs`, `name_validator.rs`)
- Stateless extractors/parsers (`extractor.rs`, `template.rs`)
- Query translation separate from execution
- Web reuses renderer (no duplicate logic)

**Weaknesses**:
- `main.rs` is large (3300 lines) — orchestrates everything
- `verifier.rs` is 2700 lines — does too much (could split per check type)
- No plugin/extension system

### OKC (library + binary, more modules)
```
main.rs (CLI, MCP server)
├── config.rs
├── service.rs (orchestration)
├── transport/cli.rs (commands)
├── transport/mcp/ (MCP server)
├── scanner/ (watcher, incremental)
├── index/ (database, validate, queries)
├── parser/ (frontmatter, YAML, markdown, links)
├── model/ (document, validation types)
└── service/ (search, browse, graph)
```

**Strengths**:
- Cleaner separation: transport (CLI/MCP) vs service vs index vs parser
- Graph traversal as first-class feature
- MCP server built-in (agent-native)
- Watcher-based incremental indexing

**Weaknesses**:
- Validation scattered: `index/validate/checks.rs` + `model/document/validation.rs` + `service/validation.rs`
- No template/schema concept
- Link resolution less sophisticated (no block/heading anchors in resolution)

---

## Strengths vs OKC

| Area | markbase Advantage |
|------|-------------------|
| **Agent-facing schema enforcement** | `note verify` + template `_schema` = contract agents can't drift from |
| **Template-driven creation** | `note new --template` injects defaults, location, filename pattern |
| **Link target type checking** | Verifies `owner: [[Person]]` actually points to `type: person` |
| **Analytical query power** | DuckDB columnar + expression language → SQL |
| **Obsidian parity** | `.base` render, wikilink fidelity, name-based identity |
| **Source evidence workflow** | `source attach/verify/rerender` for auditable notes |
| **Web delivery** | `web serve` + `web get` for headless inspection |
| **Single binary simplicity** | No external services; DuckDB embedded |

---

## Weaknesses vs OKC

| Area | OKC Advantage |
|------|---------------|
| **Full-text search** | Tantivy BM25 + snippets; markbase uses `LIKE`/array ops |
| **Graph traversal** | `traverse`, `backlinks`, relations as edges; markbase only stores `links[]`/`backlinks[]` |
| **MCP server** | Built-in `serve --transport stdio/http`; markbase is CLI-only |
| **Incremental indexing** | notify watcher + debounced reconcile; markbase re-scans on command |
| **Multi-root vaults** | `roots: [path1, path2]` config; markbase single `MARKBASE_BASE_DIR` |
| **Structured metadata queries** | `query_metadata` with filters/select/limit; markbase requires SQL knowledge |
| **Path-based identity** | Works with nested folders naturally; markbase requires unique stems |
| **Section extraction** | `get_section(path, heading)`; markbase renders whole note |

---

## OKC Improvement Opportunities

### 1. Add Template-Driven Schema Validation (High Priority)
**Why**: markbase proves agents need a contract. OKC's `validate` only checks global `title`/`type`.

**Design sketch**:
```yaml
# templates/company.md
---
_schema:
  description: "Customer company"
  location: entities/company/
  required: [title, type, description, owner]
  properties:
    title: { type: text }
    type: { type: text, enum: [company] }
    description: { type: text }
    owner: 
      type: text
      format: link
      target: person
    tags:
      type: list
      items: { type: text }
  create:
    type: company
    tags: []
---
# {{title}}

Owner: {{owner}}
```

**Implementation**:
- New `templates/` directory under roots
- `okc template list|describe|create` commands
- Extend `validate` to accept `--template` or read `templates` frontmatter array
- Reuse `model/document/validation.rs` types; add schema parsing

### 2. Adopt DuckDB for Analytical Queries (Medium)
**Why**: OKC's SQLite metadata queries are row-oriented; DuckDB excels at aggregations, column scans.

**Approach**: Dual-engine — SQLite for document storage + Tantivy for FTS, optional DuckDB for analytical views (or migrate metadata to DuckDB). markbase shows DuckDB works well for `tags[]`, `links[]` array columns.

### 3. Link Target Type Validation (High)
**Why**: Prevents `owner: [[Acme]]` where Acme is a `company` not `person`.

**Implementation**: During `validate`, for each frontmatter link field with known target type (from template schema or convention), resolve and check target's `type`.

### 4. Name-Based Identity Option (Low)
**Why**: Obsidian compatibility. Could add `--name-based` flag for vaults that follow Obsidian conventions.

### 5. Per-Note Verification Command (High)
**Why**: `okc validate` is repo-wide. Agents need `okc verify <path>` for CI on their output.

### 6. Embedded View Rendering (`.base` equivalent) (Low)
**Why**: markbase's `.base` renders related notes inline. OKC could add `okc render --expand-links <path>`.

### 7. Filename Pattern Generation (Medium)
**Why**: `{{timestamp}}_{{name}}` useful for meeting notes, logs.

---

## Verdict

**markbase is a specialized tool for agent-structured Obsidian vaults**. Its template→schema→verify pipeline is the standout innovation — it turns "agent writes Markdown" from a hope into a verifiable contract.

**OKC is a broader knowledge catalog** with stronger search, graph traversal, and MCP integration. It lacks the schema enforcement layer that makes agent collaboration reliable.

**Strategic recommendation**: 
1. **Adopt markbase's template schema model** as an optional layer in OKC (templates dir, `_schema` frontmatter, per-note verify)
2. **Add link target type checking** to validation (leverages existing graph store)
3. **Expose per-note `verify` command** for agent CI
4. **Keep Tantivy + SQLite** for search/graph; consider DuckDB only if analytical query demand grows

The three-way contract (Human→Agent→System) is the right abstraction for AI-era knowledge bases. OKC should implement it.