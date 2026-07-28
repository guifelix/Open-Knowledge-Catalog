# Arkouda Memstead — Competitor Assessment

## Note on Naming

"Arkouda" and "Memstead" are separate projects with distinct origins. **Arkouda** (Bears-R-Us/arkouda) is a Chapel-backed HPC data analytics platform (NumPy-like API at supercomputing scale) — unrelated to agent memory. **Memstead** is the MCP-based typed knowledge graph tool for AI agents. The compound "Arkouda Memstead" does not refer to a single project. This assessment focuses on **Memstead** (GitHub: memstead/memstead), as it is the directly relevant competitor to OKF/OKC. The only plausible connection is that Memstead's builder, Björn Bösenberg, built it partially via AI orchestration — but "Arkouda" as a separate HPC project has no relation.

## Overview

**Memstead** (memstead.io, GitHub: memstead/memstead) is a schema-validated, typed knowledge graph engine for AI agents, stored as plain Markdown in a git repository you own. It gives agents a durable, typed model of a project — entities, relationships, and metadata — written through MCP tools and enforced by a Rust engine against a user-pinned schema.

Built by **Björn Bösenberg**, a Berlin-based full-stack developer of ~25 years, Memstead was itself built as an AI-orchestration project (~138K lines of Rust, ~3,100 agent commits against 4 human ones, across ~4.5 calendar months of part-time work). It is dual-licensed under MIT OR Apache-2.0.

Memstead has three components: a **Rust engine** (schema layer, in-memory store, storage backends), a **CLI** (`memstead`), and an **MCP server** (`memstead-mcp`). It also has a closed-source **registry** at memstead.io (for publishing/sharing `.mem` packages) and a native macOS app.

The core thesis: *correctness enforced at boundaries replaces trust in the author*. Every write to a mem is validated against a pinned schema before it touches disk. The engine is in Rust — the compiler stands in for human code review.

## Key Features

- **Schema-validated knowledge graphs**: Pinned schemas define entity types, sections, relationships, and metadata fields. Every mutation is validated against the schema before it reaches disk. The `default` schema ships with 10 general-purpose types (`concept`, `assertion`, `memo`, `spec`, `inquiry`, etc.).

- **Markdown-in-git storage**: Entities are plain Markdown files with YAML frontmatter — human-readable, diffable, git-trackable, reviewable. No database, no vendor lock-in.

- **MCP-native**: `memstead-mcp` exposes `memstead_*` and `workspace_*` tools over stdio. Works with any MCP-compatible agent (Claude Code, Codex, Gemini CLI, Cursor). Claude Code has a dedicated plugin (skills + guard hooks).

- **CLI parity**: The `memstead` CLI mirrors nearly every MCP tool — parity matrix documented. Operations: `create`, `search`, `status`, `type` (list schema types), `export`, `publish`, `install`.

- **Multi-mem workspaces**: A workspace can mount multiple mems simultaneously, each with its own schema. Cross-mem edges (relationships between entities in different mems) are supported.

- **Two storage backends**: Folder-backed (simple file-per-entity in `.memstead/`) and git-branch-backed (each mutation is a git commit). Both are replaceable via a storage backend trait.

- **Registry for sharing**: `memstead publish` packages a mem into a `.mem` file and publishes it to memstead.io. `memstead install scope/name` installs from the registry. Trust boundary: foreign schemas are served structure-only (prose withheld), foreign entities are tagged with origin.

- **Provenance / mutation log**: Every entity mutation records who, when, what, and optionally why — an append-only structured log.

- **Export/import**: `memstead export --format mem -o my.mem` packages a mem for distribution or backup.

- **Dual-license**: MIT OR Apache-2.0 (MIT only for the Claude Code plugin).

## Architecture

```
Schema (.memstead/schemas/<name>@<version>/) 
  ├── types (entity type definitions)
  ├── sections (required/optional per type)
  ├── metadata (valid fields)
  ├── relationships (allowed types between types)
  └── write rules (creation/update constraints)
        ↓
Engine (Rust)
  ├── Parse (markdown + frontmatter → in-memory graph)
  ├── Validate (against pinned schema)
  ├── In-memory store (graph operations)
  └── Write-through (to storage backend)
        ↓
Surface layers:
  ├── MCP server (memstead_* + workspace_* tools over stdio)
  ├── CLI (memstead command)
  ├── UniFFI (programmatic API for native apps)
  └── WASM (browser surface)
```

### Storage Backends

1. **Folder backend**: Each entity is one `.md` file in a folder hierarchy under `.memstead/mems/<mem-name>/entities/`. Schemas live under `.memstead/schemas/<name>@<version>/`. Simple, transparent, portable.

2. **Git-branch backend**: Each mutation becomes a git commit. Provides full provenance, branching, and merge semantics. More complex but enables collaboration workflows.

### MCP Integration

- `memstead-mcp` walks up from cwd looking for `.memstead/workspace.toml`
- MCP tools: `memstead_create`, `memstead_search`, `memstead_status`, `memstead_type`, `memstead_overview`, `memstead_schema`, `memstead_entity`, `workspace_*` tools
- Claude Code plugin: installed via `/plugin marketplace add memstead/memstead`, runs `/setup` skill to configure workspace + MCP wiring
- Any other agent: configure MCP client with point to `memstead-mcp` binary

### What's in the Repository

| Folder | Content |
|--------|---------|
| `crates/` | Rust engine — schema, store, backends, CLI, MCP server, WASM |
| `plugins/claude-code/` | Claude Code plugin (skills + guard hooks), no npm deps |
| `docs/` | Diátaxis-structured documentation (tutorial, how-to, reference, explanation) |
| `examples/` | Example schemas (`agent-program`, `reimpl-source`/`reimpl-target`) |

## Comparison with OKF

### Knowledge Representation Model

| Dimension | Memstead | OKC / OKF |
|-----------|----------|-----------|
| **Core unit** | **Entity** — typed, schema-validated markdown document in a mem | **Document** — YAML-frontmattered markdown file in a directory hierarchy |
| **Type system** | Schema-pinned: types define sections, relationships, metadata fields. Schema is versioned (`default@1.0.0`). Types are extensible via custom schemas. | Concept types via `type:` frontmatter field. No formal schema — any YAML key is valid. Looser, more flexible. |
| **Relationships** | Typed relationships between entities, defined in schema. Cross-mem edges supported between different mems. | Link graph extracted from markdown links. Link types are implicit (not typed). Backlinks, traversal, graph edges as first-class operations. |
| **Sections** | Schema-defined per entity type (required + optional). Sections are typed, ordered, validated. | Headings parsed from markdown body. No schema-level section constraints. |
| **Metadata** | Schema-defined fields with types, required/optional. | Arbitrary YAML frontmatter — any key-value pairs. No type enforcement. |
| **Modal flavors** | Knowledge, planning, inquiry, spec, hybrid — determined by pinned schema | Single mode: knowledge catalog. No planning/spec modes. |

**OKC advantage**: More flexible — any YAML frontmatter is accepted. Better for heterogeneous data where schema rigidity would be overhead. Link graph is a first-class feature with traversal operations.

**Memstead advantage**: Schema enforcement prevents structural drift. Typed relationships are more expressive. Versioned schemas enable schema evolution. Modal flavors support non-knowledge use cases (planning, specs).

### Storage Approach

| Dimension | Memstead | OKC |
|-----------|----------|-----|
| **On-disk format** | Markdown entities in `.memstead/mems/<name>/entities/` | Markdown files in any directory under roots (user-controlled) |
| **Index** | In-memory graph (optional git-backed store) | SQLite (rusqlite + FTS5 for BM25 search) |
| **Schema location** | `.memstead/schemas/<name>@<version>/` — versioned, portable | No schema directory — conventions only |
| **Source of truth** | Markdown files (git repo) | Markdown files (filesystem) |
| **Portability** | `.mem` archive for distribution, git repo for collaboration | Filesystem directory — rsync, git, cloud sync |
| **Database dependency** | None by default (optional git-backend for provenance) | SQLite required for index (rebuildable from files) |
| **Content addressing** | Git-style (commit hashes in git backend) | Blake3 content hashing for incremental scan |

**OKC advantage**: No `.memstead/` hidden directory needed — documents live wherever the user puts them. SQLite provides persistence and FTS5 across restarts. Git-agnostic — works with any file sync.

**Memstead advantage**: Schema versioning baked into storage layout. Git-backend provides provenance natively. No external database — the filesystem *is* the database.

### MCP Integration

| Dimension | Memstead | OKC |
|-----------|----------|-----|
| **MCP transport** | stdio (primary) | stdio + HTTP/SSE |
| **Number of tools** | ~8 core + workspace tools | 11 tools |
| **Tool surface** | Entity CRUD, search, schema inspection, workspace management | Scan, browse, get, section, search, metadata query, links, backlinks, traverse, stats, validate |
| **Search capability** | Exact/structural (content match, type/metadata filters). **No semantic/vector search.** | BM25 full-text search (FTS5), structured metadata queries (`query_metadata`), graph traversal |
| **Agent discovery** | Schema-driven — agent sees entity types, relationships | Browse-driven — agent explores directory hierarchy, then searches |
| **Streaming** | Not mentioned | HTTP/SSE transport for streaming |
| **Registry** | memstead.io — publish/install `.mem` packages | No registry |

**OKC advantage**: Richer search (FTS5 BM25), structured metadata queries, graph traversal. HTTP/SSE transport for remote agents. More diverse tool surface.

**Memstead advantage**: Schema-driven discovery is more structured for agents. Registry enables sharing. Entity CRUD is more focused.

### Querying Capabilities

| Dimension | Memstead | OKC |
|-----------|----------|-----|
| **Full-text search** | Exact/structural match only. **No BM25, no vector search.** | BM25 via FTS5 with configurable weights (title, description, headings, body, concept_type). |
| **Metadata queries** | Schema-defined fields filterable | `query_metadata` with filter/select/limit on any frontmatter field |
| **Graph queries** | Relationship traversal within and across mems | `traverse` (depth-limited), `links`, `backlinks`, `get_neighbors` |
| **Section extraction** | Via entity read (schema-defined sections) | `get_section(path, heading)` — extract by heading name |
| **Browse** | Schema/type listing | `browse` — directory hierarchy, `browse_directory` MCP tool |
| **Validation** | Schema validation on every write | `validate` — 8-category structural validation (broken links, malformed YAML, etc.) |
| **Statistics** | Entity counts, type distribution, edge counts | `stats` — full repository statistics |

**OKC advantage**: Significantly stronger search. BM25 relevance ranking is mature. `query_metadata` with arbitrary field filtering. Graph traversal with depth limits. Repository-wide validation.

**Memstead advantage**: Schema validation on every write guarantees queryable data. Relationship queries with typed edges are more expressive. Cross-mem queries enable federation. But search is deliberately limited ("exact and structural") — a stated omission.

### Portability / Format

| Dimension | Memstead | OKC |
|-----------|----------|-----|
| **Format specification** | `.mem` archive format + workspace layout defined in code | OKF spec (markdown + YAML frontmatter conventions) |
| **Standardization** | Proprietary but open-source | References Google Cloud Platform's OKF spec |
| **Human readability** | Markdown entities — openable, editable | Markdown files — openable, editable |
| **Tooling independence** | Requires Memstead engine to write (schema validation) | Any text editor — no engine required |
| **Distribution** | `.mem` archives + memstead.io registry | Filesystem directories (git, rsync, cloud) |
| **Versioning** | Git commits (git backend) + schema version tags | Git (user-managed) |
| **Zero-dependency reading** | Entity files are plain markdown — readable without the engine | Identical — plain markdown files |

**OKC advantage**: The OKF spec is an open standard with reference implementations. Zero tooling dependency for reading/editing — any text editor works. Memstead requires its engine to write (schema validation enforces this).

**Memstead advantage**: `.mem` archive format enables distribution as single files. Registry enables discoverability. Schema versioning is built-in.

### Agent-Readiness

| Dimension | Memstead | OKC |
|-----------|----------|-----|
| **Agent writes** | Schema-validated writes — agent cannot produce structurally invalid data | Free-form — agent writes any valid markdown. Validation is separate (`okc validate`). |
| **Agent reads** | Schema-guided — agent knows entity types, sections, relationships | Browse + search — agent explores and discovers |
| **Session persistence** | Multi-session — entities persist across sessions via git | Multi-session — persistent SQLite index |
| **Context injection** | Not designed for prompt injection (no "load context" capability) | Not designed for prompt injection — agents query via MCP |
| **Provenance** | Mutation log — every change tracked with who, when, what, why | No mutation log — git history for file-level tracking |
| **Schema guidance** | Agent learns entity vocabulary from schema (auto-discovered via MCP) | Agent learns conventions from browsing documents — no formal vocabulary |
| **Foreign content trust** | Origin tags, instruction prose withheld from foreign schemas | Not addressed |

**OKC advantage**: Lower friction for agent adoption — agents can write any markdown naturally without schema training. MCP tools are query-focused (search, browse, get) rather than CRUD-focused. Better for read-heavy agent workflows.

**Memstead advantage**: Schema validation acts as a "type system" for agent knowledge — prevents drift. Mutation log provides accountability. Foreign content trust model is more sophisticated. Better for write-heavy agent workflows where schema discipline matters.

### Collaboration Model

| Dimension | Memstead | OKC |
|-----------|----------|-----|
| **Multi-agent** | Multiple agents can read/write the same workspace via MCP | Multiple agents can query the same index via MCP |
| **Multi-user** | Git-based collaboration (branch, review, merge) | File-based (git, shared drive) |
| **Registry** | memstead.io — publish, discover, install `.mem` packages | No registry |
| **Permissions** | Cross-mem permission tables in workspace store | No permission model (filesystem-based) |
| **Review workflow** | Git PRs for entity changes (git backend) | Standard git workflows |

**OKC advantage**: Simpler model — filesystem + git is universal. No schema bureaucracy for small teams.

**Memstead advantage**: Registry enables knowledge reuse across projects. Permission model enables controlled sharing. Git backend provides provenance and review.

## Strengths

1. **Schema enforcement is a killer feature**: The engine validates every write against the pinned schema. This guarantees structural consistency — an agent cannot accidentally write a `concept` entity missing its `definition` section, or create a relationship to a non-existent type. This is the single strongest differentiator from OKC.

2. **Typed, versioned schemas**: Schemas are versioned (`default@1.0.0`), portable, and drive all engine behaviour. There are zero hardcoded field names. A different schema = different graph without code changes. Versioning enables migration paths.

3. **Typed relationships**: Relationships between entities are schema-defined and typed. Cross-mem edges between different mems in the same workspace. More expressive than OKC's implicit link graph.

4. **Multi-mem workspaces**: A single workspace can mount multiple independent knowledge graphs, each with its own schema. Cross-mem edges bridge them. This enables a modular knowledge architecture — e.g., one mem for project specs, another for ADRs, another for domain concepts.

5. **Provenance / mutation log**: Every change is recorded — who, when, what, why. This is significant for auditability and collaborative workflows. OKC has no equivalent.

6. **Registry for sharing**: The memstead.io registry and `.mem` archive format enable distribution and reuse. Publishing a typed knowledge graph as a versioned package is novel.

7. **Rust performance**: The engine is compiled Rust — fast startup, low memory, single binary. No runtime dependencies beyond the filesystem.

8. **Built on its own dogfood**: Memstead tracks its own project knowledge as live Memstead mems, in the open. This builds credibility and surfaces real usage patterns.

9. **Well-documented concepts**: The 14-term glossary (CONCEPTS.md) is clean, precise, and well-linked. Diátaxis documentation structure is professional.

10. **Foreign content trust model**: Origin tags and instruction-prose separation for third-party mems is thoughtful — addresses a real security concern in agent ecosystems.

## Weaknesses

1. **No semantic / vector search**: Stated explicitly: "No semantic/embedding search." `memstead_search` is exact and structural (content match, type/metadata filters). No BM25, no embeddings, no ranking. In an era where vector search is table stakes for knowledge management, this is a significant gap. OKC has FTS5 with BM25.

2. **No one-shot import**: "Nothing turns a folder of notes into a mem in a single command." Every entity must enter through a schema-validated write. Bulk ingestion requires the projection/pipeline system, which is still emergent. OKC scans entire directory trees in one command.

3. **No built-in visualization**: The graph is queryable (status, overview, relations) but ships no renderer. OKC has no visualizer either, so this is a tie — but both are behind offerings like Obsidian.

4. **Windows is untested**: "Developed and CI-tested on macOS and Linux. Release archives include a Windows build, but no Windows CI gate exists yet — expect rough edges, path handling especially."

5. **Pre-1.0**: APIs, schemas, file formats, CLI flags, and MCP tool wire shapes may change without notice. "Not yet stable." OKC is also pre-1.0 but has been stable across releases.

6. **Single builder**: Björn Bösenberg is the sole developer. Bus factor is a concern. The project's scale (~138K lines Rust) for a pre-1.0 solo project is ambitious.

7. **Schema overhead**: Schema-first design adds friction. Before you can write knowledge, you need a schema. If the built-in `default` schema doesn't fit, you must author a custom schema. OKC's free-form approach lets you start writing immediately.

8. **Hidden directory overhead**: `.memstead/` is required with workspace, schemas, entities. Users must opt into this structure. OKC can index any existing markdown directory without imposing a structure.

9. **Closed-source components**: The registry and native macOS app are closed-source. The open engine cannot fully duplicate the ecosystem.

10. **No session context**: Like OKC, Memstead is a knowledge graph, not a session memory tool. It doesn't solve the "where was I?" problem that tools like memcrate and ai-memory target.

## Threat Level

**Medium**. 

Here's why:

- **Memstead and OKC/OKF target adjacent but different niches**. Memstead is a schema-validated knowledge graph engine with CRUD MCP tools. OKC is a query-focused knowledge catalog with rich search and traversal. A user choosing between them would be deciding on workflow: "I need schema-disciplined entity writing" (Memstead) vs "I need searchable knowledge from existing files" (OKC).

- **Schema enforcement is Memstead's killer advantage that OKC cannot easily match**. Adopting schema validation in OKC would be a major architectural change. Memstead's approach is more agent-memory-aligned — it treats knowledge as structured data that agents *write*, not read-only reference docs that agents *query*.

- **But search is OKC's counter-punch**. Memstead's deliberate lack of semantic/vector search is a real limitation. If agents need to find knowledge by relevance rather than by structural query, OKC wins. BM25 + planned vector search in OKC is a material advantage.

- **The registry angle is interesting but early**. Memstead's `.mem` package registry is novel but has no traction yet (pre-1.0). OKC could adopt a similar distribution model via OKF bundles.

- **Memstead is not a direct threat to OKC's core use case** — OKC is a knowledge catalog for existing markdown collections; Memstead is a structured knowledge engine for agent-written content. They could complement each other (a user could maintain both a Memstead workspace and an OKC-indexed knowledge base), but they compete for the "how agents manage persistent knowledge" mindshare.

## Notes

- **The Arkouda confusion is harmless but revealing**: The name "Arkouda" refers to an unrelated HPC analytics platform. The user likely combined both names from related searches. This document focuses on Memstead as the relevant competitor.

- **Memstead is philosophically aligned with OKC in one key way**: Both believe knowledge should be plain markdown in a git repo you own, agent-readable and human-readable, with no vendor lock-in. This is a shared conviction against cloud-only, opaque-storage competitors (mem0, Zep, Letta).

- **Where they diverge**: Memstead believes agent writes should be schema-validated (error-prevention). OKC believes agent reads should be like a library card catalog (discovery-oriented). These are complementary philosophies.

- **Memstead's relationship typing is more advanced** than OKC's link graph. OKC extracts links from any `[[wikilink]]` or `/path.md` reference, but doesn't type them. Memstead's schema defines allowed relationship types between entity kinds — `depends_on`, `implements`, `contradicts`, etc. This enables richer semantic queries.

- **Memstead's pipeline/projection system** (medium, facet, projection) is relevant to OKC's roadmap. It's a mechanism for populating mem content from external sources (codebases, docs trees) — similar to OKC's scanner but more structured. Worth monitoring.

- **The Claude Code plugin is well-designed**: skills for `/setup`, `/ingest`, `/sync` + guard hooks for schema enforcement during agent sessions. OKC could benefit from similar guard hooks.

- **Memstead was built by AI orchestration** — ~3,100 agent commits vs 4 human commits. This is a data point about the project's development methodology, but also raises questions about code quality and maintainability at 138K Rust lines.

- **Recommended actions for OKC**:
  1. **Consider a lightweight schema layer** — not as rigid as Memstead's, but enough to guide agents (validated `type` values, required frontmatter fields per type). See markbase assessment for prior analysis.
  2. **Add typed relationship support** — Memstead's typed edges are a clear step beyond OKC's implicit link graph. Even optional type metadata on links would be an improvement.
  3. **Monitor Memstead's registry model** — if `.mem` packages gain traction, OKC should consider OKF bundle distribution as a comparable mechanism.
  4. **Don't compete on schema discipline** — OKC's free-form approach is a feature for many users. Schema rigidity is Memstead's bet, not OKC's.
