# mdvault Competitor Assessment

**Date:** 2026-07-27
**Author:** Open Knowledge Catalog (OKC) team
**Subject:** [mdvault](https://github.com/agustinvalencia/mdvault) — Rust markdown vault manager
**Version assessed:** 0.7.2 (crates.io), License: MIT

---

## Overview

mdvault is a Rust CLI tool for managing opinionated markdown vaults. It provides
structured note types (daily/weekly/task/project/meeting/zettel/capture/wiki/macro),
SQLite-based indexing, Lua scripting, and a TUI dashboard. The project has 17
crates.io releases, ~300 total downloads, and uses Rust edition 2024.

A separate Python project, [markdown-vault-mcp](https://github.com/agustinvalencia/markdown-vault-mcp),
provides MCP server integration via FastMCP.

---

## Architecture

### Crate Layout

```
mdvault/
├── Cargo.toml              # workspace root, edition 2024
├── crates/
│   ├── core/               # mdvault-core library
│   │   ├── src/
│   │   │   ├── activity/       # Temporal activity tracking
│   │   │   ├── captures/       # Capture workflow system
│   │   │   ├── config/         # TOML config loader
│   │   │   ├── context/        # Context prompt generation (day/week/note/focus)
│   │   │   ├── domain/         # Trait-polymorphic note behaviors
│   │   │   │   ├── behaviors/  # Daily, Weekly, Task, Project, Meeting, Zettel, Custom
│   │   │   │   ├── creator/    # Note creation orchestration
│   │   │   │   └── traits/     # NoteBehavior trait (identity, lifecycle, prompts)
│   │   │   ├── frontmatter/    # YAML frontmatter parsing
│   │   │   ├── ids/            # ID generation
│   │   │   ├── index/          # SQLite index + search + derived data
│   │   │   ├── lint/           # Markdown linting/validation
│   │   │   ├── macros/         # Parametrized templates with Lua
│   │   │   ├── markdown_ast/   # Comrak AST wrapper
│   │   │   ├── paths/          # Path resolution
│   │   │   ├── rename/         # File renaming with link updates
│   │   │   ├── report/         # Dashboard report builder + chart data
│   │   │   ├── scripting/      # Lua sandbox with vault bindings
│   │   │   ├── templates/      # Template rendering engine
│   │   │   ├── types/          # Lua-defined note type system
│   │   │   ├── vars/           # Variable substitution
│   │   │   └── vault/          # File walker, extractor, hasher
│   └── cli/                    # CLI binary
│       └── src/
│           ├── args.rs         # Clap arg definitions
│           ├── cmd/            # 23 command modules
│           ├── completions/    # Shell completion
│           ├── logging/        # Tracing configuration
│           ├── prompt/         # Interactive prompts
│           └── tui/            # Ratatui-based TUI
```

### OKC Architecture (for comparison)

```
Open-Knowledge-Catalog/
├── Cargo.toml              # single crate, edition 2021
├── src/
│   ├── config/             # TOML config loader
│   ├── index/              # SQLite index + search + graph store
│   ├── model/              # Document/scan models
│   ├── parser/             # Pulldown-cmark + wikilink parser
│   ├── scanner/            # File discovery + scanning
│   ├── service/            # High-level service facade
│   │   ├── browse.rs
│   │   ├── documents.rs
│   │   ├── graph.rs
│   │   ├── search.rs
│   │   ├── validation.rs
│   │   └── watch.rs
│   └── transport/          # CLI + MCP
│       ├── cli.rs
│       └── mcp/
│           ├── mod.rs      # 11 MCP tools
│           ├── types.rs    # Param/output schemas
│           └── tools/      # Tool-option helpers
```

---

## Feature Inventory

### mdvault — present

| Feature | Category | Notes |
|---------|----------|-------|
| Note creation (8 types) | Creation | daily, weekly, task, project, meeting, zettel, capture, wiki, macro |
| Template system | Creation | Handlebars-like with variables |
| Capture workflow | Creation | Quick-capture with prompts |
| Task management | Productivity | List, done, cancel, status, project grouping |
| Project management | Productivity | Status, progress %, archive, velocity |
| Area reporting | Productivity | Per-period export (JSON, Markdown) |
| Daily log | Productivity | Today summary, focus mode |
| Focus mode | Productivity | Filtered view of tasks/notes |
| Context generation | AI helper | Day/week/note/focus context prompts for LLMs |
| Activity tracking | Analytics | Temporal activity, staleness detection |
| Dashboard report | Analytics | JSON dashboard, PNG charts, TUI dashboard |
| Velocity metrics | Analytics | Tasks/week over 2w/4w windows |
| Review cycles | Analytics | Review-due project tracking |
| Search | Index | SQLite FTS-based search |
| Link graph | Index | Forward/back links, orphans, broken link detection |
| Index | Index | SQLite (rusqlite) with reindex command |
| Lua scripting | Extensibility | Sandboxed mlua engine with vault bindings |
| Macro system | Extensibility | Parametrized Lua-runner workflows |
| Type system | Extensibility | Lua-defined custom note types with validation hooks |
| Lint/validation | Quality | Markdown linting, link integrity |
| TUI | Interface | Ratatui/crossterm TUI with fuzzy select |
| Shell completions | DX | Dynamic clap-complete |
| Charts | DX | PNG chart generation via charts-rs |
| Doctor | DX | System diagnostics |
| MCP server | Integration | **Separate Python project** (markdown-vault-mcp) |
| Rename | Utility | File rename with link fixing |

### OKC — present

| Feature | Category | Notes |
|---------|----------|-------|
| Scan/index | Index | SQLite with incremental updates |
| Browse | Query | Directory tree browsing |
| Get document | Query | Full document with sections |
| Get section | Query | Section by heading/anchor |
| Search | Query | Full-text FTS5 with type/tag filtering |
| Metadata query | Query | Key=value filter + field projection |
| Links/backlinks | Query | Wiki/markdown/auto links |
| Graph traversal | Query | BFS with depth limit, relation filter |
| Validate | Quality | Issue reporter (errors/warnings/infos) |
| Stats | Query | Document/link/heading/error counts |
| Watch | Sync | Inotify-based incremental reindex |
| MCP server | Integration | **Native Rust** via rmcp |
| MCP HTTP/SSE | Integration | Axum-based Streamable HTTP transport |
| JSON export | Utility | Full index JSON dump |
| Concept types | Modeling | YAML frontmatter type + tags + headings |

### Capability Gap: mdvault features absent from OKC

| Feature | Impact | Effort to Add |
|---------|--------|---------------|
| Note creation templates | High — OKC is read-only | Medium (new module) |
| Task/project management | Medium — OKC is catalog, not organizer | High (domain model) |
| Lua scripting | Medium — power user extensibility | High (embedding) |
| TUI interface | Low — OKC targets CLI + MCP | Medium (ratatui) |
| Dashboard/charts | Low — OKC targets query, not review | Medium |
| Activity tracking | Low — depends on vault mutation | Low |
| Note type support (daily, etc.) | Low — OKC is agnostic | N/A (out of scope) |
| Template rendering | Low | Medium |

### Capability Gap: OKC features absent from mdvault

| Feature | Impact | Notes |
|---------|--------|-------|
| Native Rust MCP (with HTTP/SSE) | High — mdvault needs separate Python server | OKC advantage |
| Link graph traversal (BFS with depth) | Medium | OKC advantage |
| Metadata query (filter + projection) | Medium | OKC advantage |
| Incremental file watching | Medium | mdvault has `reindex`, no watcher |
| Section extraction by anchor | Low | OKC advantage |
| JSON export for AI consumption | Low | OKC advantage |

---

## Key Dependencies

### mdvault-core

| Dependency | Purpose |
|------------|---------|
| comrak 0.50 | GFM markdown parser |
| rusqlite 0.38 | SQLite index (bundled) |
| mlua 0.11 | Lua scripting (lua54, vendored) |
| chrono 0.4 | Date/time handling |
| serde + toml + yaml | Serialization |
| walkdir 2.5 | File system walking |
| regex 1.12 | Pattern matching |
| thiserror 2.0 | Error handling |
| tracing 0.1 | Logging |

### mdvault-cli (additional)

| Dependency | Purpose |
|------------|---------|
| clap 4.5 | Arg parsing |
| ratatui 0.30 | TUI terminal UI |
| crossterm 0.29 | Terminal backend |
| dialoguer 0.12 | Interactive prompts (fuzzy select) |
| charts-rs 0.3 | PNG chart rendering |
| color-eyre 0.6 | Error reporting |
| tabled 0.20 | Table display |
| serde_yaml 0.9 | YAML config |

### OKC

| Dependency | Purpose |
|------------|---------|
| pulldown-cmark 0.x | CommonMark parser |
| rusqlite 0.38 | SQLite index (bundled) |
| rmcp 2.2.0 | MCP protocol (native Rust) |
| clap 4.x | Args |
| axum 0.x | HTTP/MCP server |
| tokio 1.x | Async runtime |
| serde + serde_json | Serialization |
| notify 8.x | File system watching |

---

## Quality Assessment

### Strengths

1. **Polymorphic note behavior** — The `domain` module uses trait-based dispatch
   (`NoteBehavior` trait with identity/lifecycle/prompts) instead of if/else chains.
   Each note type (task, project, daily, etc.) is a clean struct implementing shared
   interfaces. This makes adding new types straightforward.

2. **Extensibility by design** — Lua scripting, custom type definitions, macros, and
   template overrides form a layered extensibility model. Users start with built-in
   types, customize with Lua schemas, and eventually write full macros.

3. **Comprehensive dashboard** — The `report` module produces a rich JSON schema
   covering project velocity, activity heatmaps, staleness, review cycles, flagged
   tasks (overdue, high-priority, zombie), and progress. This rivals dedicated
   project management tooling.

4. **Edition 2024** — Uses latest Rust edition (2024), indicating active maintenance
   and modern practices.

### Weaknesses

1. **MCP is a separate Python project** — This is the biggest architectural weakness.
   The Python MCP server (`markdown-vault-mcp`) cannot leverage mdvault's Rust
   core library. It must reimplement markdown parsing and vault operations. This
   also introduces a Python runtime dependency for AI tool integration.

2. **No incremental file watching** — mdvault has a `reindex` command but no file
   system watcher. Users must manually reindex after changes. OKC has inotify-based
   incremental watching.

3. **No HTTP/SSE MCP transport** — mdvault-mcp only implements stdio transport.
   OKC supports both stdio and HTTP/SSE for remote AI assistant access.

4. **Limited graph operations** — mdvault has forward/back links and orphan detection,
   but no graph traversal with depth limits or relation filtering.

5. **Single person project** — mdvault appears maintained by one person (agustinvalencia).
   The separate Python MCP project, while functional, creates maintenance burden.

### OKC Advantages

1. **Native Rust MCP** — OKC's rmcp-based MCP server is compiled into the same
   binary, sharing all core parsing, indexing, and query logic directly.

2. **Dual MCP transport** — stdio for Claude Code, HTTP/SSE for remote AI assistants.

3. **Graph traversal** — BFS traversal with configurable depth, node limits, and
   relation filtering is unique to OKC.

4. **File watching** — Inotify-based incremental scanning for continuous index update.

5. **Section extraction** — OKC can extract specific sections by heading title or
   anchor slug, enabling targeted retrieval for AI context windows.

---

## Strategic Recommendations

### Consider integrating (lower effort, high value)
- **Template rendering** — Lightweight variable substitution for output formatting
  (handlebars or simple mustache)
- **Activity staleness scoring** — Simple last-seen tracking to identify cold documents
- **Dashboard-style report output** — Aggregate stats as JSON for AI consumption

### Consider building (medium effort)
- **Note type system** — If OKC evolves beyond read-only catalog, a type system
  (even just YAML frontmatter conventions) would match mdvault's modeling
- **Context prompt generation** — LLM-friendly context blobs (day context, note
  context) are useful for AI agents working with the vault

### Out of scope (should not build)
- **Lua scripting** — OKC's extensibility model should be MCP-based, not embedded.
  Users extend via tools, not sandboxed scripts.
- **TUI** — OKC is CLI + MCP first. A TUI adds maintenance without strategic value.
- **Task/project management** — OKC is a *catalog* for knowledge, not a task
  manager. This is a deliberate scope boundary.
- **Note creation** — OKC focuses on *reading, querying, and navigating* existing
  content. Creation is left to editors and other tools.
- **Charts/visualization** — Delegate to external tooling or frontends.

### Strategic threat assessment
mdvault is **not a direct competitive threat** to OKC. The tools have different
philosophies:
- mdvault is a *personal productivity vault manager* — create notes, manage tasks,
  track projects, run scripts
- OKC is a *knowledge catalog and query engine* — index, search, navigate, and
  serve content to AI agents via MCP

The overlap is in the "markdown knowledge base with SQLite index" foundation.
If mdvault invests significantly in their MCP capabilities (native Rust MCP,
graph traversal, HTTP transport), it could close the gap. Currently, OKC's MCP
integration is substantially more mature.

### Priority actions

1. **Maintain MCP lead** — OKC's native Rust MCP with HTTP/SSE is the strongest
   differentiator. Keep investing in tool coverage and protocol support.
2. **Extend section/context retrieval** — mdvault has "context generation" (day/week/note
   context blobs). OKC should consider adding similar context-optimized endpoints
   for AI consumption.
3. **Monitor mdvault's MCP development** — If mdvault merges MCP into the Rust
   codebase (native), reassess competitive positioning.
