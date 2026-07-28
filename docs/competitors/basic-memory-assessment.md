# Basic Memory — Competitor Assessment

## Overview

**Basic Memory** (basicmachines-co/basic-memory) is a local-first knowledge management system and MCP server that gives AI agents persistent, structured memory across sessions. Built by **Basic Machines**, licensed AGPL-3.0, it has ~3,500 GitHub stars, 239 forks, and 57K+ monthly downloads. The latest release is v0.22.1 (June 2026), written in Python 3.12+.

It positions itself as "real memory for your AI — a knowledge base you own." Knowledge lives as Markdown files that both humans and AI agents read and write, with semantic search, wikilink-based knowledge graphs, and optional cloud sync. It works with Claude Desktop, Claude Code, Codex CLI, Cursor, VS Code, ChatGPT (Custom GPTs), Obsidian, and any MCP-compatible client.

Basic Memory has three tiers: **free local open-source** (self-hosted, air-gapped), **cloud** ($15/mo locked-in pricing, cross-device sync, snapshots), and **teams** (shared workspace with real-time collaborative editing). The cloud version uses Neon Postgres and Tigris S3; the local version uses SQLite with optional Postgres.

---

## Key Features

- **Local-first Markdown knowledge base** — All knowledge stored as plain Markdown files on the user's disk. Editable in any text editor or Obsidian.
- **Semantic vector search** — FastEmbed embeddings for hybrid full-text + vector ranking. Optional cross-encoder reranking (local via FastEmbed or hosted via LiteLLM/Cohere).
- **Wikilink-based knowledge graph** — Typed relations (`pairs_well_with [[Target]]`, `requires [[Tool]]`) and observations (`[method]`, `[fact]`, `[question]`, `[resource]`) create a traversable graph.
- **MCP-native tool exposure** — 15+ MCP tools tagged with behavior hints (read-only, destructive, idempotent, open-world) for progressive agent discovery.
- **Two-way sync** — AI writes notes via MCP, humans edit in Obsidian/VS Code — `basic-memory sync` reconciles changes.
- **Multi-project management** — Isolated workspaces for different domains (work, personal, research), each with its own storage and optional cloud routing.
- **Schema inference and validation** — `schema_infer`, `schema_validate`, `schema_diff` tools let agents introspect the structure of their knowledge base.
- **Claude Code plugin** — Session-start briefings, pre-compaction checkpoints, capture output style, custom `/basic-memory:*` slash commands.
- **Importers** — Import from Claude conversation exports, ChatGPT exports, or generic memory JSON.
- **CLI** — `basic-memory` CLI for project management, config, diagnostics, health checks, reindexing, and imports. JSON output mode for scripting.
- **Auto-updates** — Background update checks for `uv tool` and Homebrew installs.
- **Telemetry** — Minimal anonymous events (cloud promo impressions, login outcomes only). Opt-out via `BASIC_MEMORY_NO_PROMOS=1`.

---

## Architecture

### Core Model

Basic Memory models knowledge as **Entities** (files) with **Observations** (typed facts) and **Relations** (wikilinks with typed predicates). Each file is a Markdown document with YAML frontmatter (title, type, permalink, tags) containing observation lists and relation links. This forms a labeled property graph where entities are nodes, relations are typed edges, and observations are entity attributes.

### Storage Backends

| Backend | Use Case | Details |
|---------|----------|---------|
| **SQLite** | Local (default) | File-based, zero-config. Stores search index and metadata alongside Markdown files on disk. FastEmbed vectors stored in SQLite. |
| **PostgreSQL** | Cloud | Hosted via Neon Postgres on cloud tier. Enables cross-device sync. |
| **Tigris S3** | Cloud file storage | Stores Markdown files for cloud sync and snapshots. |
| **Milvus** | Optional vector store | Installable extra (`basic-memory[milvus]`) for Postgres deployments that need dedicated vector storage. |

### MCP Integration

Basic Memory is an MCP server running over stdio transport (local) or HTTPS (cloud). It exposes tools in these categories:

- **Content**: `write_note`, `read_note`, `edit_note`, `move_note`, `delete_note`, `read_content`, `view_note`
- **Search & discovery**: `search_notes`, `recent_activity`, `list_directory`
- **Knowledge graph**: `build_context` (navigates `memory://` URLs)
- **Projects**: `list_memory_projects`, `list_workspaces`, `create_memory_project`, `delete_project`
- **Schema**: `schema_infer`, `schema_validate`, `schema_diff`
- **Diagnostics**: `basic_memory_diagnostics`

Every tool is annotated with MCP behavior hints (`readOnlyHint`, `destructiveHint`, `idempotentHint`, `openWorldHint`) for progressive tool discovery.

### Data Flow

```
User/AI conversation
    → "Make a note about X"
    → MCP tool call (write_note)
    → Markdown file created on disk (~/basic-memory/)
    → SQLite index updated (FTS + embeddings)
    → Next session: search_notes → build_context → conversation continues
```

### Agent Harnesses

Beyond plain MCP, Basic Memory ships framework-specific integrations:
- **Claude Code plugin**: Session briefings, `/basic-memory:remember`, `:share`, `:status` slash commands
- **Claude Desktop**: stdio MCP config
- **Codex CLI**: TOML-based MCP config with pre-approval options
- **Cursor**: `.cursor/mcp.json`
- **VS Code**: Native MCP settings
- **ChatGPT**: Custom GPT actions (OpenAI-compatible `search`/`fetch` endpoints)
- **Obsidian**: Direct file access (no plugin needed)
- **Hermes / OpenClaw**: Dedicated plugin packages

---

## Comparison with OKF

### Knowledge Representation Model

| Dimension | Basic Memory | OKF / OKC |
|-----------|-------------|-----------|
| **Core unit** | Entity (Markdown file with YAML frontmatter) | Concept (Markdown file with YAML frontmatter) |
| **Typed relationships** | Yes — wikilinks with relation predicates (`pairs_well_with [[X]]`) | Yes — typed link references in YAML frontmatter |
| **Observations / facts** | Yes — typed observation lists in Markdown body (`[method]`, `[fact]`, etc.) | Yes — structured fields in YAML frontmatter + Markdown body |
| **Schema enforcement** | Optional — schema_infer/validate/diff tools (soft schema) | Strong — OKF v0.2 spec: required `type`, reserved filenames, citation format, trust tiers, augmentation guards |
| **Concept types** | Free-form (`type` in frontmatter) | Domain-specific types (Metric, Dataset, Dimension, Report, Decision, etc.) |
| **Progressive disclosure** | MCP tool annotations (behavior hints) | OKF spec §8: progressive-disclosure model |
| **Citations / provenance** | Not built-in (ad-hoc in frontmatter) | First-class: `sources` with id, author, usage_count, verification status, trust tiers, usage_window |
| **Reserved files** | None | `index.md`, `log.md` per directory |

Basic Memory uses a looser knowledge model: observations and relations are embedded in Markdown body text rather than structured frontmatter. OKF enforces a stricter schema with typed citations, trust tiers, reserved files, and augmentation guards. OKF's model is more rigorous for enterprise knowledge management; Basic Memory's is more accessible for ad-hoc capture.

### Storage Approach

| Dimension | Basic Memory | OKF / OKC |
|-----------|-------------|-----------|
| **Primary storage** | Markdown files on disk + SQLite index | Markdown files on disk + SQLite index (OKC) |
| **Index format** | SQLite FTS5 + FastEmbed vectors | SQLite FTS5 (BM25) — vector search planned |
| **File location** | `~/basic-memory/` (configurable per project) | Arbitrary directory (pointed at via CLI/MCP) |
| **Directory structure** | Flat or user-defined per project | Hierarchical OKF bundle structure (concepts by type/subject) |
| **Git compatibility** | Yes (plain Markdown, git-friendly) | Yes — OKF designed for git-versioned bundles |
| **Cloud backend** | Postgres + S3 (proprietary cloud tier) | None (local-first only) |
| **Vector storage** | SQLite (local) / Milvus (optional) | Not yet implemented in OKC |

Both use Markdown-on-disk with a SQLite index. Basic Memory has a more opinionated project layout (single `~/basic-memory/` default). OKC is directory-agnostic — you point it at any OKF bundle. Basic Memory adds FastEmbed vectors and optional Milvus; OKC's semantic search is still planned.

### MCP Integration

| Dimension | Basic Memory | OKF / OKC |
|-----------|-------------|-----------|
| **MCP server** | Yes — primary interface (Python, FastMCP 3.0) | Yes — primary interface (Rust, rmcp) |
| **Transport** | stdio (local) + HTTPS (cloud) | stdio + HTTP/SSE |
| **Tool count** | 15+ (content, search, graph, projects, schema, diagnostics) | 11 (browse, get, search, traverse, validate, query_metadata, links, backlinks, stats, section, scan) |
| **Tool annotations** | Yes — readOnlyHint, destructiveHint, idempotentHint, openWorldHint | Use-based categories (read-only metadata queries vs content retrieval) |
| **Write operations via MCP** | Yes — write_note, edit_note, move_note, delete_note | No — MCP is read-only; writes via CLI or direct file editing |
| **File watching** | Via `basic-memory sync` command (manual trigger) | `watch` command with fsnotify, debouncing, reconciliation |
| **Agent plugins** | Claude Code, Codex, Hermes, OpenClaw | None |

Basic Memory exposes write operations directly via MCP — agents can create, edit, and delete notes. OKC intentionally keeps MCP read-only; content creation is done via CLI or standard file operations. Basic Memory's approach enables fully autonomous agent workflows but risks agent-authored content quality issues. OKC's approach is safer for curated knowledge bases.

### Querying Capabilities

| Dimension | Basic Memory | OKF / OKC |
|-----------|-------------|-----------|
| **Full-text search** | SQLite FTS5 | SQLite FTS5 (BM25) |
| **Vector search** | FastEmbed embeddings (local, on-device) | Planned |
| **Hybrid search** | Full-text + vector (RRF fusion) | Planned |
| **Reranking** | Cross-encoder (FastEmbed or LiteLLM/Cohere) | None |
| **Graph traversal** | `build_context` — navigates `memory://` URLs | `traverse_graph` — depth-limited link traversal |
| **Structured query** | By project, tags, type (via search_notes filters) | `query_metadata` — exact match on frontmatter fields |
| **Browse hierarchy** | `list_directory` | `browse_directory` — tree view with depth control |
| **Section retrieval** | Not explicit (read_note returns full content) | `get_section` — extract specific Markdown heading section |
| **Backlinks** | Implicit via wikilink parsing | `get_backlinks` — dedicated tool |
| **Validation** | Schema diff (schema level) | `validate_repository` — 8-category structural checks |
| **Stats** | Not exposed as a tool | `get_stats` — doc count, link count, heading count |

Basic Memory leads on semantic/vector search with on-device embeddings and optional cross-encoder reranking. OKC leads on structured querying — dedicated tools for metadata queries, section extraction, backlinks, validation, and stats. Basic Memory's `build_context` provides graph-based context assembly; OKC's `traverse_graph` offers depth-limited link exploration.

### Portability / Format

| Dimension | Basic Memory | OKF / OKC |
|-----------|-------------|-----------|
| **File format** | Markdown + YAML frontmatter (proprietary conventions: observations, relations) | OKF v0.2 spec — standardized YAML frontmatter fields, citation format, reserved files |
| **Human-readable** | Yes — plain Markdown | Yes — plain Markdown |
| **Standardized** | No — Basic Memory-specific conventions | Yes — community specification at github.com/open-knowledge-format/spec |
| **Export** | Markdown files are already the source of truth | Markdown files are already the source of truth |
| **Lock-in risk** | Low — files are standard Markdown; conventions are documented | Low — files are standard Markdown; spec is open |
| **Interop** | Obsidian-native (wikilinks), MCP clients | Any tool that can read Markdown; OKF parsers in Rust, Python |

Both use Markdown-on-disk with low lock-in risk. OKF has the advantage of a formal specification with community governance — knowledge is portable across any OKF-compliant tool. Basic Memory's format is de facto defined by its implementation; while documented, it has no independent specification body.

### Agent-Readiness

| Dimension | Basic Memory | OKF / OKC |
|-----------|-------------|-----------|
| **Setup for agents** | `uvx basic-memory mcp` — single command | `okc mcp` — single command |
| **Agent plugins** | Claude Code, Codex, Hermes, OpenClaw | None |
| **Session continuity** | Claude Code plugin with session briefings and checkpoints | File watcher maintains index; no session concept |
| **Write from agent** | Yes — full CRUD via MCP | No — read-only MCP; writes via CLI |
| **Tool discovery** | Progressive (behavior hints reduce trial-and-error) | Use-based categories |
| **Agent-specific docs** | `AGENTS.md`, `CLAUDE.md`, `skills/` directory, llms-txt support | `AGENTS.md`, `AI.md` |

Basic Memory is significantly more agent-ready. Its Claude Code plugin provides session briefings, checkpoints, and custom slash commands. The agent can write notes directly. Behavior hints on tools reduce token waste. OKC is agent-accessible but takes a more cautious approach — agents can read and search but cannot write.

### Collaboration Model

| Dimension | Basic Memory | OKF / OKC |
|-----------|-------------|-----------|
| **Multi-user** | Cloud teams plan (shared workspace, real-time editing) | Single-user (SQLite file is per-user) |
| **Sync** | Cloud sync (Postgres + S3) or manual (Git, Syncthing) | Manual (Git) |
| **Permissions** | Cloud: teams with roles/invites | None (filesystem permissions) |
| **Review workflow** | Not built-in | Not built-in |
| **Conflict resolution** | Cloud: built-in (rclone-powered) | Git-native (merge conflicts) |

Basic Memory has a working teams product; OKC is single-user. This is Basic Memory's strongest differentiator for collaborative use cases.

---

## Strengths

1. **Semantic search out of the box** — FastEmbed on-device embeddings with hybrid ranking and optional cross-encoder reranking provide far better retrieval quality than BM25 alone. OKC does not have vector search yet.

2. **Full CRUD from MCP** — Agents can write, edit, and delete notes through MCP tools. This enables truly autonomous knowledge base maintenance without human file editing.

3. **Agent plugins** — The Claude Code plugin (session briefings, checkpoints, slash commands) is a polished integration that OKC lacks entirely. Agent-first design shows throughout.

4. **Cloud + teams** — Working cloud offering with cross-device sync, snapshots, and team collaboration. OKC has no equivalent — it remains local and single-user.

5. **MCP tool annotations** — Behavior hints (`readOnlyHint`, `destructiveHint`) let agents discover capabilities progressively without trial-and-error. OKC does not use these.

6. **Multi-project routing** — Per-project cloud/local routing lets users mix privacy-sensitive projects (local) with shared ones (cloud). OKC has no project abstraction — it points at one directory.

7. **Schema introspection** — `schema_infer`, `schema_validate`, `schema_diff` give agents awareness of their knowledge base's structure. OKC validates against a fixed spec but has no introspective tools.

8. **Ecosystem maturity** — 3.5k stars, 57K monthly downloads, active community on Discord, dedicated docs site, multiple third-party tutorials. Significantly more adoption than OKC.

---

## Weaknesses

1. **Format lock-in** — Basic Memory's observation/relation conventions are implementation-defined, not standardized. There is no formal specification body. Interoperability depends on Basic Memory's parser.

2. **No provenance/citations** — Unlike OKF, Basic Memory has no built-in support for knowledge provenance (trust tiers, source citation, verification status). Useful for personal notes but insufficient for enterprise knowledge where auditability matters.

3. **Python dependency** — Requires Python 3.12+ and `uv`. This is a heavier dependency than OKC's single Rust binary. Cloud tier removes this requirement but introduces a subscription cost.

4. **AGPL-3.0 license** — Restrictive for commercial embedding. Any organization that distributes modifications must open-source their entire work. OKC is Apache-2.0, which is more business-friendly.

5. **No validation tools** — No structural validation (broken links, malformed YAML, circular references). OKC's `validate_repository` with 8-category checks provides confidence in knowledge base integrity.

6. **No link graph analysis** — While Basic Memory has wikilinks, it lacks dedicated backlink traversal, circular-reference detection, or orphan detection. OKC provides `get_links`, `get_backlinks`, and link validation.

7. **No section-level retrieval** — `read_note` returns full content. OKC's `get_section` can extract a specific heading section, which is more efficient for large documents.

8. **No Git-native versioning** — Basic Memory relies on its sync mechanism for versioning. OKF bundles are designed for Git — diffs, history, branching are native.

---

## Threat Level

**MEDIUM-HIGH**

Basic Memory is the most significant competitor in the AI-agent-knowledge space. It has more stars, more downloads, more integrations, and a working cloud/teams product that OKC lacks entirely. Its semantic search capabilities and Claude Code plugin are clearly differentiated features that OKC does not match.

However, it is not a direct substitute for OKF/OKC in several key ways:

- **Use case overlap is partial**: Basic Memory targets individual knowledge workers who want AI-assisted note-taking and context persistence. OKF targets structured, typed, portable knowledge bundles for enterprise and technical documentation.
- **Format philosophy differs**: OKF's formal specification with citations, trust tiers, and reserved files makes it suitable for auditable knowledge. Basic Memory's looser model is better for personal capture.
- **License barrier**: AGPL-3.0 is a significant adoption barrier for commercial users who need to embed the technology.

Basic Memory threatens OKC's adoption in the **personal knowledge management** and **AI agent memory** space. OKC's differentiation must come from its **structured knowledge model**, **provenance tracking**, **validation**, **Rust performance**, and **Apache-2.0 licensing**.

---

## Notes

- Basic Memory's creator (Basic Machines) appears to be a well-funded startup. The project has a dedicated team, a pricing page, a cloud product, and a marketing site. OKC is a single-developer open-source project. This asymmetry in resources means Basic Memory will likely outpace OKC on features, integrations, and polish.
- The Claude Code plugin is particularly well-designed — session briefings at conversation start, automatic checkpoints before compaction, and `/basic-memory:remember` slash commands. This is the kind of integration that creates habit and lock-in.
- Basic Memory's knowledge format (observations + relations in Markdown) is conceptually similar to Zettelkasten and shares DNA with Obsidian's graph view. OKF's format (typed concepts with citations) is more structured for engineering knowledge.
- The cloud pivot (adding $15/mo Teams plan) suggests Basic Machines sees the real value in collaborative knowledge management rather than personal note-taking. This is a different market from OKC's focus on portable knowledge bundles.
- For OKC: the most impactful response would be (1) implementing vector search with on-device embeddings, (2) adding write tools to the MCP server, (3) building a Claude Code plugin or equivalent, and (4) publishing agent skill files that match Basic Memory's developer experience.
