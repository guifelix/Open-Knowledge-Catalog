# Competitor Assessment: memcrate + ai-memory

## Overview

**memcrate** (v0.3.3, crates.io/memcrate, docs.rs/memcrate, GitHub: memcrate/memcrate) — A portable, markdown-native, locally-owned personal context vault for AI tools. Three verbs (`/save`, `/pin`, `/load`) over a defined vault shape any AI tool can read. Local-first, no cloud, no lock-in. License: MIT (code), CC0 (format spec). Author: Brad Traversy.

**ai-memory** (v0.10.0, crates.io/ai-memory, docs.rs/ai-memory, GitHub: alphaonedev/ai-memory-mcp) — A persistent memory system for AI assistants. Works with any AI that supports MCP (Claude, ChatGPT, Grok, Llama). Stores memories in local SQLite, ranks by relevance, auto-promotes important knowledge. Provides MCP server, HTTP API (92 routes), CLI (89 subcommands). License: MIT/Apache-2.0. Author: alphaonedev.

**ai-memory-core** (v0.1, crates.io/ai-memory-core) — Lightweight Rust library for AI memory with spatial metaphor (Wing/Hall/Room), pure markdown storage, token-budgeted injection, LLM extraction. Minimal dependencies.

Both target the "AI agent persistent context" use case — memcrate as a personal vault for CLI tools, ai-memory as a wiki/MCP server for agent sessions. Adjacent to OKC but focus on session-to-session context rather than knowledge base management.

---

## Context Persistence Model Comparison

| Dimension | memcrate | ai-memory | OKC | Notes |
|-----------|----------|-----------|-----|-------|
| **Persistence model** | Session rituals via three verbs over markdown vault | SQLite-backed memory store with TTL tiers (short/mid/long) + auto-promotion | Persistent knowledge graph in SQLite (documents, FTS, links, tags) | memcrite: human rituals; ai-memory: automated lifecycle; OKC: structured KB |
| **Storage format** | Plain markdown files (`Core/Context/*.md`, `Core/Sessions/*.md`) | SQLite database + optional markdown export | SQLite database (single file) | memcrate: human-readable, git-friendly; ai-memory: opaque DB; OKC: single SQLite |
| **Session-to-session flow** | `/load` reads vault → `/pin` promotes facts → `/save` writes session log | Auto-capture during session → compaction/rewrite at end → recall ranked by relevance | `watch` + incremental scan → MCP tools for search/traverse → no session concept | memcrate: explicit verbs; ai-memory: automatic; OKC: continuous indexing |
| **CLI/API design** | `memcrate init/setup/install` + tool-specific skills (`/save`, `/pin`, `/load`) | `ai-memory` CLI (89 cmds), MCP server (stdio), HTTP API (REST) | `okc` CLI (search, traverse, validate), MCP server (stdio/HTTP) | memcrate: minimal CLI, skills do the work; ai-memory: feature-rich CLI+MCP+HTTP; OKC: CLI+MCP |
| **Agent integration** | Skills for Claude Code (installed to `~/.claude/skills/`), Cursor/Aider planned | MCP primary (any MCP client), HTTP API fallback, CLI for scripts | MCP server primary, CLI secondary | memcrate: tool-specific; ai-memory: MCP-first; OKC: MCP-first |
| **Multi-user/tenancy** | Single-user vault (your sync) | Multi-user (v0.8), token auth, per-user namespaces | Single-user (SQLite file) | ai-memory ahead on multi-user |
| **Sync/portability** | Any sync (git, iCloud, Dropbox, rsync) — vault is just files | SQLite file sync; replication/federation (W-of-N quorum) in v0.7+ | SQLite file; no built-in sync | memcrate: simplest portability |
| **Architecture** | Single Rust binary (~644KB stripped), embedded vault + skills via `include_dir!` | Large multi-module crate (70+ modules), daemon + CLI + MCP + HTTP | Single crate, modular (index, parser, scanner, service, transport) | memcrate: tiny; ai-memory: enterprise-scale; OKC: mid-size |

---

## Storage Format and Data Model

### memcrate: Markdown Vault Structure
```
~/vault/
├── .memcrate                    # Marker file (git-style upward discovery)
├── Core/
│   ├── Context/
│   │   ├── Profile.md           # Stable: who you are, tools, anti-goals
│   │   ├── Projects.md          # Every project with status, stack
│   │   └── Current State.md     # Living: this week's focus, deadlines
│   └── Sessions/                # /save writes session logs here
│       └── YYYY-MM-DD-slug.md
└── (optional) Projects/, Daily/, Tasks/, Inbox/
```
- **Four canonical files** in `Core/Context/` define the verb surface
- Section guidance inline (AI knows where `/pin` writes)
- Human-editable, Obsidian-compatible, git-trackable
- Format spec in `docs/` (CC0 — anyone can implement)

### ai-memory: SQLite Schema + Memory Model
- **Memory table**: `id`, `namespace`, `title`, `content`, `tier` (short/mid/long), `created_at`, `updated_at`, `access_count`, `embedding` (optional HNSW vector index)
- **Tiered TTL**: Short (1hr extend), Mid (1day extend), Long (permanent) — auto-promotion on access threshold
- **Namespaces**: Hierarchical (`alphaone/engineering/platform/team/squad/pod/role/agent` — 8 levels max)
- **Entity memories**: Separate `ENTITY_KIND` marker for deduplication (title+namespace)
- **Knowledge graph**: `memory_links` table for relationships
- **Audit trail**: `signed_events` append-only chain (v0.7+)
- **Vector search**: HNSW index over embeddings (optional feature)
- **Export**: Can dump to markdown wiki (`ai-memory export`)

### ai-memory-core: Spatial Markdown
- Single `memories.md` file with spatial paths: `Wing/Hall/Room`
- Wings: `User`, `Project`, `Team` (extensible via macro)
- Halls: `Preferences`, `ProjectKnowledge`, `Decisions`, `WorkflowPatterns`, `CodingStandards`
- MD5 fingerprint per entry + access count metadata (sidecar JSON)
- Token-budgeted injection (2000 tokens) into system prompt

### OKC: Unified SQLite Knowledge Graph
- Documents table (path, hash, frontmatter JSON, content)
- FTS5 virtual table for BM25 search (title, description, headings, body, concept_type weights)
- Link graph table (source, target, link_type)
- Tags, headings, concepts tables
- OKF-specific: citations, reserved files (`index.md`, `log.md`), augmentation guards
- Single SQLite file = complete KB state

---

## CLI Design and DX Comparison

| Aspect | memcrate | ai-memory | OKC |
|--------|----------|-----------|-----|
| **Primary verbs** | `init`, `setup`, `install <tool>` | 89 subcommands across MCP, HTTP, CLI, admin | `search`, `traverse`, `validate`, `watch`, `serve`, `stats`, `browse`, `get` |
| **Session verbs** | `/save`, `/pin`, `/load` (installed as AI tool skills) | Auto-capture; `ai-memory recall`, `ai-memory save`, `ai-memory expand` | No session concept; continuous |
| **Config** | Auto-discovery (`.memcrate` marker, `~/vault` fallback) | `.ai-memory.toml` with extensive sections | `okc.toml` (index path, watch, MCP config) |
| **Install** | `curl ... install.sh \| sh` or `cargo install memcrate` (prebuilt binaries Linux/macOS/Win) | `cargo install ai-memory` (or git) | `cargo install okc` |
| **Shell completions** | No | No (planned) | No |
| **CI integration** | No | No | No |
| **Output formats** | Human text (skills) | Text, JSON (CLI), SSE (approvals) | JSON, text |
| **Help system** | Inline skill docs, `memcrate --help` | Per-command `--help`, extensive docs/ | `--help` per command |
| **Skill/Rule install** | `memcrate install claude-code` → `~/.claude/skills/` | MCP auto-config per client (Cursor, Claude Desktop, etc.) | MCP server (`okc serve`) |
| **DX philosophy** | "Three verbs. One vault. Any tool." — minimal, ritualistic | "Install once, every AI remembers forever" — feature-complete | "Query engine + MCP server for AI agents" — server-first |

**memcrate DX insight**: The verb trio (`/save`, `/pin`, `/load`) becomes muscle memory. The CLI only scaffolds; the AI tool does the work via skills. Brilliant separation: CLI = setup, AI = operation.

**ai-memory DX insight**: Overwhelming CLI surface (89 commands). MCP is the intended primary interface; CLI is for operators. Multi-user, TLS, replication, audit — enterprise features that complicate the happy path.

**OKC DX insight**: Clean CLI for KB operations, MCP server for agents. No session verbs — agents query the KB continuously via MCP.

---

## Integration with AI Agent Tooling

### memcrate
- **Claude Code**: First-class via `memcrate install claude-code` → drops `/save`, `/load`, `/pin` skill files to `~/.claude/skills/`
- **Cursor**: Planned (rules-based)
- **Aider**: Planned
- **Claude Desktop**: Planned
- **MCP**: Not yet (format is open, anyone can build)
- **Mechanism**: Skills read/write the vault directly; vault discovery via `.memcrate` marker

### ai-memory
- **MCP Server**: Primary integration (`stdio` transport). Exposes tools: `memory_save`, `memory_recall`, `memory_search`, `memory_expand`, `memory_reembed`, `recover_previous_session`, etc.
- **HTTP API**: 92 routes / 78 unique paths on localhost. For non-MCP clients.
- **CLI**: 89 subcommands for scripting/automation.
- **Clients tested**: Claude Desktop, Cursor, Gemini CLI, Antigravity CLI, OpenClaw, OMP, VS Code Copilot (via MCP)
- **Multi-user**: Bearer tokens, per-user namespaces (v0.8)

### OKC
- **MCP Server**: `okc serve` (stdio + HTTP transports). Tools: `search`, `traverse`, `browse`, `get`, `validate`, `stats`.
- **CLI**: `okc search`, `okc traverse`, `okc validate`, `okc watch`, etc.
- **No session memory** — agents query the KB as a reference, not a session log.

---

## Boundary Analysis: Context Memory vs Knowledge Base vs OKC

```
┌─────────────────────────────────────────────────────────────────────┐
│                        SESSION CONTEXT MEMORY                        │
│  (memcrate, ai-memory, ai-memory-core, CPR, mem0, Letta, etc.)      │
│  Purpose: "Where was I? What did I decide? What's next?"            │
│  Lifetime: Session → Session (hours to weeks)                       │
│  Write pattern: Append-only logs, auto-capture, promotion           │
│  Read pattern: Recall ranked by relevance, inject into prompt       │
│  Structure: Flat or lightly structured (tiers, namespaces)          │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼ Promotion / Extraction
┌─────────────────────────────────────────────────────────────────────┐
│                      STRUCTURED KNOWLEDGE BASE                       │
│  (OKC, hyalo, Obsidian, Notion, GitBook, etc.)                      │
│  Purpose: "What is the architecture? What are the APIs? Decisions?" │
│  Lifetime: Project → Forever (years)                                 │
│  Write pattern: Curated, reviewed, versioned                        │
│  Read pattern: Search, traverse, browse, structured query           │
│  Structure: Rich (frontmatter, links, tags, types, citations)       │
└─────────────────────────────────────────────────────────────────────┘
```

### Where the boundary lives

| Concern | Session Context Memory | Knowledge Base (OKC) |
|---------|------------------------|----------------------|
| **Primary writer** | AI agent (auto) or human (ritual) | Human (curated) or AI (with review) |
| **Primary reader** | AI agent (prompt injection) | Human + AI agent (MCP query) |
| **Mutation rate** | High (every session) | Low (deliberate edits) |
| **Schema** | Minimal (tiers, namespaces) | Rich (OKF: type, citations, links) |
| **Search** | Semantic/keyword recall | BM25 + graph traversal + filters |
| **Sync** | Personal (git, cloud) | Team (git, shared DB) |
| **Garbage collection** | TTL + promotion | Manual archival |

### memcrate boundary
- **Stops at**: `Core/Context/` (Profile, Projects, Current State) — this IS the KB-lite
- **Session logs**: `Core/Sessions/` — pure context memory
- **Optional folders**: `Projects/`, `Daily/`, `Tasks/` — user's personal OS
- **No promotion pipeline** — human decides what to `/pin` from session → Context

### ai-memory boundary
- **Short/Mid tier** = session context (TTL, auto-promote)
- **Long tier** = promoted knowledge (permanent)
- **Namespaces** allow project scoping but no formal KB structure
- **Export to wiki** → markdown files → could feed OKC

### OKC boundary
- **No session concept** — continuous indexing
- **Agents query** via MCP for facts, not "where was I"
- **Could ingest** promoted memories from ai-memory/wiki export as documents

---

## OKC Improvement Opportunities

### 1. Session Context Awareness (`high` priority)
OKC has no concept of "current session." Consider:
- **MCP tool**: `okc_session_start` / `okc_session_end` — writes session log to KB (new `session_logs/` area)
- **MCP tool**: `okc_recall_session` — returns last N session summaries for context injection
- **Watch mode enhancement**: Track active session, auto-log file touches/commands
- **Rationale**: Bridges OKC from pure KB to session-aware assistant

### 2. Tiered Memory Model (`medium` priority)
Adopt ai-memory's tiered TTL concept for session-derived content:
- `session_logs/` — raw, TTL 30 days
- `session_insights/` — promoted, reviewed, permanent
- `knowledge/` — curated KB (current OKC)
- **Auto-promotion**: Access count + LLM extraction → promote to insights

### 3. Skill/Rule Install for Claude Code (`high` priority)
- `okc install claude-code` → drops MCP config + skill for session verbs
- Memcrate proves this pattern: `memcrate install claude-code` works beautifully
- OKC's MCP server is superior to skills for query, but skills win for session rituals

### 4. Simplified Vault Bootstrapping (`medium` priority)
- `okc init` → scaffolds KB structure + `okc.toml` + `.okc` marker
- `okc setup` → interactive wizard for project metadata (like `memcrate setup`)
- Auto-discovery via `.okc` marker (git-style upward walk)

### 5. Markdown Export/Import for Interop (`low` priority)
- `okc export --format markdown` → memcrate/ai-memory wiki compatible
- `okc import --from markdown` → ingest promoted memories
- Enables OKC as the "promotion target" for session memory tools

### 6. MCP Tool for "Load Context" (`high` priority)
Single MCP tool that returns:
```json
{
  "profile": "Profile.md content",
  "projects": "Projects.md content",
  "current_state": "Current State.md content",
  "recent_sessions": ["last 3 session logs"],
  "open_questions": "extracted from recent sessions"
}
```
This is what memcrate's `/load` does — OKC can serve it via MCP.

---

## Verdict

**memcrate** is the **best-in-class session context vault for individual developers**. Its brilliance:
- Three verbs become muscle memory (`/save`, `/pin`, `/load`)
- Markdown vault = human-readable, git-trackable, Obsidian-openable
- Skills model: CLI sets up, AI tool operates — clean separation
- Tiny binary (644KB), zero dependencies, CC0 format spec
- **Weakness**: No MCP server, no multi-user, no search beyond grep, single-tool skills only (Claude Code today)

**ai-memory** is the **most feature-complete MCP memory server**. Its strength:
- MCP-first (works with any MCP client)
- Tiered memory with auto-promotion (smart lifecycle)
- Multi-user, TLS, replication, audit — enterprise-ready
- HTTP API for non-MCP integration
- **Weakness**: Overwhelming complexity (70+ modules, 89 CLI cmds), opaque SQLite, no human-readable vault, steep learning curve

**OKC occupies a different niche**: **Structured Knowledge Base + MCP Query Server**.
- Not a session memory tool — no session verbs, no TTL tiers
- Excels at: structured KB (OKF), graph traversal, continuous indexing, MCP serving
- **Gap**: No session context bridge, no skill install, no tiered memory

**Strategic positioning**: OKC should NOT become a session memory tool. Instead:
1. **Add session context MCP tools** (`session_start`, `session_end`, `recall_session`) — makes OKC session-aware
2. **Build `okc install claude-code`** — meet users in their tool
3. **Define promotion path**: session logs → insights → KB documents
4. **Keep KB strengths**: OKF, graph traversal, MCP server, file watching

**The ecosystem**: memcrate/ai-memory handle *session context*; OKC handles *project knowledge*. They're complementary. An ideal workflow: memcrate for daily session rituals → promoted insights exported to OKC KB → agents query OKC via MCP for facts.

---

## Assessment Completeness

- [x] #1 Context persistence model comparison
- [x] #2 Storage format and data model review
- [x] #3 CLI design and DX comparison vs OKC
- [x] #4 Boundary analysis: context memory vs knowledge base vs OKC