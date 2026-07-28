# Competitor Assessment: hyalo

## Overview

**hyalo** (https://github.com/ractive/hyalo, https://crates.io/crates/hyalo-cli) is a Rust CLI tool for searching, filtering, bulk-editing, and reorganizing markdown knowledgebases with YAML frontmatter. It targets Obsidian vaults, Zettelkasten, documentation sites, and any folder of `.md` files. It positions itself as the tooling layer for the "LLM Wiki" pattern — enabling AI agents (specifically Claude Code) to maintain structured knowledgebases at scale.

Key characteristics:
- **Language**: Rust (multi-crate workspace: `hyalo-core`, `hyalo-cli`, `hyalo-mdlint`)
- **Architecture**: Snapshot index for fast metadata queries, BM25 inverted index for full-text search, link graph with backlink resolution
- **Distribution**: Prebuilt binaries via Homebrew, apt, dnf, AUR, Scoop, winget, Cargo
- **AI Integration**: First-class Claude Code integration via `hyalo init --claude` (installs skills, rules)
- **Profiles**: Composable schema/lint profiles (OKF, MADR, Skills, Changelog)

## Feature Comparison Matrix

| Feature | hyalo | OKC | Notes |
|---------|-------|-----|-------|
| **Full-text search** | BM25 with boolean operators, 17-language stemming, per-document language from frontmatter | SQLite FTS5 with Porter stemming, BM25 with configurable weights | hyalo: more sophisticated tokenization, language-aware; OKC: simpler but integrated with SQLite |
| **Regex search** | Yes (`--regexp`) | No | hyalo advantage |
| **Frontmatter filters** | `--property key=value`, complex value matching (equals, contains, regex, range, exists/absent) | `--types`, `--tags` only in search; `metadata` command for structured queries | hyalo: richer property filtering syntax |
| **Tag queries** | `--tag` with glob-style matching (`research*`, `!deprecated`) | `--tags` exact match | hyalo: glob/negation support |
| **Section/task/symbol queries** | `--section "## Tasks"`, `--task status=todo`, `--symbol` | No | hyalo unique capability |
| **Title filter** | `--title` with glob/regex | No | hyalo advantage |
| **Ranking quality** | BM25 with corpus statistics (N, df, avgdl), low-discriminative query warning | BM25 via FTS5, configurable column weights | Both solid; hyalo warns on poor queries |
| **Fuzzy suggestions** | Tag/property key typo detection (Damerau-Levenshtein) | No | hyalo unique |
| **Case-insensitive search** | `--case-index` mode | Not exposed | hyalo advantage |
| **Schema validation** | Required fields, type constraints (string/enum/pattern/list/date/number), filename templates, defaults, path-glob bindings, exempt globs | OKF-specific validation (required `type`, citations, reserved files), frontmatter parse errors, broken links | hyalo: general-purpose schema system; OKF: OKF-spec aligned |
| **Auto-fix** | `--fix`: insert defaults, fix enum typos, normalize dates, infer type | No | hyalo significant advantage |
| **Lint profiles** | okf, madr, skills, changelog (composable, idempotent) | OKF profile only (built-in) | hyalo: extensible profile system |
| **Rule catalog** | `lint-rules list` with stable IDs, configurable severity per rule | Hardcoded checks | hyalo: more configurable |
| **Strict mode** | `--strict` promotes warns→errors | `--json` output with exit codes | hyalo: finer control |
| **GitHub Actions output** | `--format github` for PR annotations | No | hyalo advantage |
| **Bulk property edit** | `set --property K=V` with type coercion (string/list/number/bool/date), advisory notes for schema violations | No | hyalo unique |
| **Bulk tag edit** | `set --tag`, `append --tag`, `remove --tag` | No | hyalo unique |
| **Bulk append to lists** | `append --property tags=[a,b]` | No | hyalo unique |
| **Bulk remove** | `remove --property`, `remove --tag` | No | hyalo unique |
| **File move/rename** | `mv` rewrites ALL `[[wikilinks]]` and `[markdown](links)` across vault, handles ambiguous stems, dry-run | No | hyalo significant advantage |
| **Link auto-repair** | `links fix --apply` with fuzzy matching | No | hyalo unique |
| **Link auto-link** | `links auto --apply` converts unlinked mentions to wikilinks | No | hyalo unique |
| **Broken link detection** | `find --broken-links`, `lint` rule HYALO006 | `validate` command, `scan` reports | Both have it |
| **Orphan/dead-end detection** | `find --orphan`, `find --dead-end`, `summary` views | No | hyalo advantage |
| **Summary/overview** | `summary`: file count, tags, properties, link health, task counts | `stats` command only | hyalo richer |
| **Saved views** | `[views]` in config with named filter sets | No | hyalo advantage |
| **File scaffolding** | `new --type X --file path` from schema with `TBD` placeholders | No | hyalo unique |
| **Output formats** | `text` (TTY), `json` (pipe), `--jq` filter, `--count` | JSON/text via commands | hyalo: `--jq` built-in |
| **Shell completions** | bash, zsh, fish, elvish, powershell (auto-installed by system packages) | No | hyalo advantage |
| **CI integration** | `setup-hyalo` GitHub Action, `--format github`, `--strict` | No dedicated action | hyalo advantage |
| **MCP/server mode** | No | Yes (`serve` with stdio/HTTP transports) | OKC advantage |
| **File watching** | No | Yes (`watch` command) | OKC advantage |
| **Graph traversal** | Backlinks via link graph, no multi-hop | `traverse` command with depth/relation filters | OKC advantage |
| **Architecture** | Multi-crate, snapshot index, in-memory BM25 index | Single crate, SQLite + FTS5 | Different tradeoffs |
| **Windows support** | Full (musl static, Scoop, winget) | Yes (Rust) | Both |

## Search Capability Comparison

### BM25 Implementation Details

**hyalo** (`crates/hyalo-core/src/bm25.rs`):
- Custom BM25 implementation with inverted index (`Bm25InvertedIndex`)
- Pre-tokenized corpus stored in snapshot index for fast re-scoring
- Language resolution priority: frontmatter `language` > CLI `--language` > config > English
- 17 stemming languages via `rust-stemmers` (Arabic, Danish, Dutch, English, Finnish, French, German, Greek, Hungarian, Italian, Norwegian, Portuguese, Romanian, Russian, Spanish, Swedish, Tamil, Turkish)
- Boolean query parser: `AND`/`OR`/`NOT`/phrases (`"exact phrase"`), `-term` negation
- Corpus statistics (N, df, avgdl) built over **full scoped corpus** (not just metadata-passing candidates) to ensure `--index` and no-index parity
- Section-scoped scoring: when `--section` filter active, BM25 scores only lines within matching section ranges
- Low-discriminative query detection: warns when query matches most docs with low scores
- Query-only operator detection: warns when query like `"and"` or `"or"` consumed as boolean operator

**OKC** (`src/index/search_index.rs`):
- Delegates to SQLite FTS5 with `porter unicode61` tokenizer
- BM25 weights configurable via `Bm25Config` (title, description, headings, body, concept_type)
- No stemming language selection (Porter English only)
- No boolean operators in FTS5 query (uses FTS5 syntax directly)
- No section-scoped search
- No low-discriminative query warning

### Regex Support
- **hyalo**: `--regexp` with `(?i)` case-insensitive flag, line-level matches, section-scoped
- **OKC**: Not supported

### Frontmatter Filtering
- **hyalo**: `--property key=value` supports exact, glob (`*`), regex (`/pattern/`), range (`10..20`), existence (`key!`, `!key`), list membership; composable with `--tag`, `--section`, `--task`, `--title`
- **OKC**: Search supports `--types` and `--tags` only; structured metadata queries via `metadata` command with key=value filters

### Tag/Section/Task/Symbol Queries
- **hyalo**: 
  - `--tag` with glob patterns and negation (`research*`, `!deprecated`)
  - `--section "## Tasks"` matches ATX headings, supports scope ranges
  - `--task status=todo` filters by task checkbox state
  - `--symbol` queries (experimental, for code symbols)
- **OKC**: None of these

### Ranking Quality
Both use BM25. hyalo's implementation is more transparent (exposes score in JSON output, warns on poor queries) and handles multi-language corpora. OKC's FTS5 integration is battle-tested but less configurable.

## Linting & Schema Validation

### hyalo Schema System
- **Config**: `[schema]` in `.hyalo.toml` with `default` type and named `types`
- **Constraints per property**: `type` (string/list/number/boolean/date/enum), `pattern`, `min_length`, `max_length`, `values` (for enum)
- **Required fields**: Per-type `required` array, merged with global default
- **Filename templates**: `{n}` (sequence), `{slug}` (title-derived) for `hyalo new`
- **Defaults**: Per-property default values with `$today` expansion
- **Path-glob bindings**: `[schema.bind]` ordered glob→type mappings (explicit frontmatter `type:` wins)
- **Exempt globs**: `[schema.exempt]` files skipped from validation
- **Required sections**: `required_sections = ["## Tasks", "### Notes"]` validated via heading scanner

### OKC Validation
- **OKF spec compliance**: Validates required `type` field, citation format, reserved files (`index.md`, `log.md`), augmentation guards
- **Frontmatter parsing**: Errors on invalid YAML, duplicate keys
- **Broken links**: Detected during scan and validate
- **Link health**: Reports broken link count in scan output
- **No general schema system**: Tied to OKF specification

### Auto-fix
- **hyalo**: `--fix` applies: insert missing defaults, fix enum typos (Levenshtein), normalize date formats (`2024/01/15` → `2024-01-15`), infer missing `type:` from path bindings
- **OKC**: No auto-fix

### Lint Profiles
- **hyalo**: 4 built-in profiles (okf, madr, skills, changelog), composable, idempotent, usable as ephemeral overlays (`hyalo lint --profile X` without config)
- **OKC**: Single OKF validation mode

## CLI Ergonomics & DX

### Command Structure Comparison

| hyalo | OKC | Notes |
|-------|-----|-------|
| `find` (search + filter) | `search`, `metadata` | hyalo unified; OKC split |
| `set` (bulk property/tag) | — | hyalo unique |
| `append` (list append) | — | hyalo unique |
| `remove` (property/tag removal) | — | hyalo unique |
| `mv` (move with link rewrite) | — | hyalo unique |
| `links fix/auto/lint` | `validate`, `links` | hyalo more link maintenance |
| `summary` | `stats` | hyalo richer |
| `new` (scaffold from schema) | — | hyalo unique |
| `lint` / `lint-rules` / `lint-github` | `validate` | hyalo more configurable |
| `views` (saved filters) | — | hyalo unique |
| `tags` / `tasks` / `properties` / `types` | `browse`, `get` | Different focus |
| `changelog` / `okf` / `madr` | — | hyalo domain-specific generators |
| `init` / `config` / `profiles` | Config file only | hyalo interactive setup |

### Output & Scripting
- **hyalo**: TTY-aware (`text` for terminals, `json` for pipes), `--jq` for in-process jq filtering, `--count` for scripts, `--format github` for CI annotations
- **OKC**: JSON/text per command, no built-in jq, no CI format

### Help & Documentation
- **hyalo**: Extensive `README.md`, `docs/configuration.md`, `docs/ci.md`, `docs/releasing.md`, per-command `--help`, `lint-rules list` catalog
- **OKC**: `README.md`, inline `--help`, no separate docs directory

## Bulk Operations

### Rename with Link Rewriting (`hyalo mv`)
- Rewrites **both** `[[wikilinks]]` and `[markdown](links)` across entire vault
- Handles case-insensitive filesystems via `CaseInsensitiveIndex`
- Ambiguous stem detection (NEW-3): when `foo.md` → `bar.md` and both `[[foo]]` and `[[foo/section]]` exist, skips ambiguous links with `--allow-ambiguous` override
- Batch mode: `--glob`, `--property`, `--tag` selectors for multi-file moves
- Dry-run by default, `--apply` to execute
- Index patching: renames index entry, re-scans rewritten files, updates link graph

### OKC
- No equivalent. File moves require manual link updates.

### Bulk Metadata Editing
- **hyalo**: `set --property status=done --where-tag research` (with `--dry-run`, `--validate` to reject schema violations, advisory notes for enum/date issues)
- **OKC**: No bulk edit capability

## Code Architecture Review

### hyalo (Multi-crate Workspace)
```
crates/
├── hyalo-core/       # Core logic: index, search, links, schema, filters, BM25
│   ├── bm25.rs       # 63KB - BM25 implementation, boolean query parser
│   ├── index.rs      # 71KB - VaultIndex, SnapshotIndex, scanning
│   ├── discovery.rs  # 104KB - File discovery, frontmatter parsing, link extraction
│   ├── link_graph.rs # 80KB - Link graph, backlinks, resolution
│   ├── link_rewrite.rs # 110KB - Link rewriting for mv/fix
│   ├── schema.rs     # 48KB - Schema config, validation, type merging
│   ├── filter/       # Property, tag, section, task filters + parsing
│   └── scanner/      # Multi-visitor file scanner
├── hyalo-cli/        # CLI commands, output formatting, dispatch
│   ├── commands/     # 30+ command modules
│   ├── output.rs     # 122KB - Formatting, JSON, GitHub, jq pipeline
│   └── run.rs        # 73KB - Command execution, index management
└── hyalo-mdlint/     # Markdown linting engine
    └── engine.rs     # 51KB - Rule engine, fix application
```

**Strengths:**
- Clean separation: core logic testable without CLI
- Snapshot index (`SnapshotIndex`) enables instant startup for repeat runs
- Link graph built once, reused for backlinks, orphan detection, mv rewrites
- Multi-visitor scanner: single file read feeds frontmatter, links, headings, tasks, BM25 tokens
- Boolean query parser in core (not CLI) — reusable
- Profile system in CLI with composable schema fragments

**Concerns:**
- Some very large files (link_rewrite.rs 110KB, discovery.rs 104KB, dispatch.rs 110KB) suggest modules could be split further
- CLI `dispatch.rs` and `run.rs` are large orchestration layers
- `hints.rs` (156KB) — appears to be generated or very verbose

### OKC (Single Crate, Modular)
```
src/
├── index/
│   ├── database.rs      # RepositoryIndex, SQLite connection pool
│   ├── search_index.rs  # SqliteSearchIndex (FTS5)
│   ├── graph_store.rs   # Link graph storage
│   ├── queries/         # browse, document, metadata, search, stats
│   └── validate/        # Validation checks
├── parser/              # Frontmatter, links, markdown parsing
├── scanner/             # Walker, watcher, change detection
├── service/             # OkcService facade
├── transport/           # CLI, MCP server (stdio/HTTP)
└── model/               # Data models
```

**Strengths:**
- SQLite as single source of truth (documents, FTS, links, headings, tags)
- MCP server built-in (`serve` command) for AI agent access
- File watching with incremental updates (`watch` command)
- Graph traversal with relation filtering (`traverse` command)
- Clean service layer for MCP/CLI reuse

**Concerns:**
- No snapshot index — full scan on startup (mitigated by content hashing)
- No bulk mutation operations
- Schema validation tied to OKF spec only
- No link rewriting on move
- Search limited to FTS5 capabilities

## Strengths vs OKC

| Area | hyalo Advantage |
|------|-----------------|
| **Search expressiveness** | Boolean queries, regex, section/task filters, tag globs, title filter, multi-language stemming |
| **Bulk editing** | `set`, `append`, `remove` for properties/tags with type coercion and schema advisories |
| **Link maintenance** | `mv` rewrites all links, `links fix` fuzzy-repairs, `links auto` creates wikilinks |
| **Schema system** | General-purpose, composable profiles, filename templates, path bindings, exempt globs |
| **Auto-fix** | `--fix` inserts defaults, corrects enum typos, normalizes dates, infers types |
| **CI/CD integration** | GitHub Action, `--format github`, `--strict` |
| **AI agent integration** | `hyalo init --claude` installs skills + rules for Claude Code |
| **Saved views** | Named filter sets in config |
| **File scaffolding** | `new --type` from schema with `TBD` placeholders |
| **Output flexibility** | `--jq`, `--count`, TTY-aware formatting |
| **Cross-platform distribution** | Homebrew, apt, dnf, AUR, Scoop, winget, Cargo |
| **Fuzzy typo suggestions** | Tag/property key Damerau-Levenshtein suggestions |

## Weaknesses vs OKC

| Area | OKC Advantage |
|------|---------------|
| **MCP server** | Built-in `serve` command (stdio/HTTP) for AI agent access |
| **File watching** | `watch` command with debouncing and reconciliation |
| **Graph traversal** | Multi-hop `traverse` with relation/depth limits |
| **Unified storage** | SQLite single source of truth (FTS + graph + metadata) |
| **Incremental indexing** | Content-hash-based change detection |
| **OKF specialization** | Purpose-built for OKF spec (citations, reserved files, augmentation) |
| **Architecture simplicity** | Single binary, fewer moving parts |

## OKC Improvement Opportunities

Based on hyalo's strengths, OKC should consider:

### 1. Enhanced Search (`high` priority)
- Add boolean query support (AND/OR/NOT/phrases) to FTS5 queries
- Add regex search option (`--regexp`)
- Add section-scoped search (`--section "## Tasks"`)
- Add tag glob/negation support (`--tag research*`, `--tag !deprecated`)
- Add title filter (`--title`)
- Expose BM25 score in search results (already computed, just not returned)
- Add low-discriminative query warning

### 2. Bulk Metadata Operations (`high` priority)
- Implement `set` command for bulk property/tag updates
- Add `append` for list-type properties
- Add `remove` for property/tag removal
- Include `--dry-run` and `--validate` flags
- Schema-aware advisory notes (date format, enum values)

### 3. Link Maintenance (`high` priority)
- Implement `mv` command with link rewriting (wikilinks + markdown links)
- Add `links fix` for fuzzy broken link repair
- Add `links auto` for unlinked mention → wikilink conversion
- Leverage existing link graph in `graph_store.rs`

### 4. Schema System Generalization (`medium` priority)
- Decouple schema validation from OKF spec
- Support per-type required fields, property constraints (type, enum, pattern, length)
- Add filename templates with `{n}`, `{slug}` for scaffolding
- Add path-glob → type bindings (`[schema.bind]`)
- Add exempt globs for generated/reserved files
- Add required sections validation

### 5. Auto-fix (`medium` priority)
- `--fix` flag for validation: insert missing defaults, normalize dates, fix enum typos (Levenshtein), infer missing `type:` from path bindings

### 6. Lint Profiles & Rule Catalog (`medium` priority)
- Extract validation rules into catalog with stable IDs
- Support configurable severity per rule
- Add `--strict` to promote warns→errors
- Add `--format github` for PR annotations
- Create composable profiles (OKF, MADR, Skills, Changelog, custom)

### 7. CLI Ergonomics (`medium` priority)
- Add `--jq` for in-process JSON filtering
- Add `--count` for scripting
- TTY-aware output (compact text for terminal, JSON for pipes)
- Shell completions (generate at build/install)
- `init` command for interactive config setup

### 8. Saved Views (`low` priority)
- Named filter sets in config (`[views.my-view]`) usable via `find --view my-view`

### 9. File Scaffolding (`low` priority)
- `new --type X --file path` from schema with `TBD` placeholders

### 10. AI Agent Integration (`high` priority)
- Create `okc init --claude` equivalent: install skill + rule for Claude Code
- Document MCP server as primary agent interface (already exists)

## Verdict

**hyalo is the more feature-complete CLI for markdown knowledgebase maintenance today.** Its search expressiveness, bulk editing, link rewriting, schema system, and AI-agent integration represent the state of the art for local-first markdown tooling. It is purpose-built for the "LLM Wiki" workflow where an AI agent maintains a structured vault.

**OKC has a different architectural center of gravity:** SQLite-backed unified storage with MCP server and file watching make it better suited as a **query engine and server** for AI agents that need to browse, search, and traverse a knowledge graph over a stable API. Its OKF specialization is a differentiator for OKF-compliant repositories.

**Strategic recommendation:** OKC should not try to match hyalo feature-for-feature as a CLI. Instead:
1. **Adopt hyalo's search expressiveness** (boolean, regex, section filters) — these improve the core query product
2. **Add bulk metadata mutation** (`set`/`append`/`remove`) — high leverage for AI agents doing vault maintenance
3. **Add link rewriting on move** — critical for vault reorganization
4. **Generalize schema validation** beyond OKF — enables broader adoption
5. **Build `okc init --claude`** — meet users where they are (Claude Code)
6. **Keep MCP server + file watching + graph traversal** as OKC's unique server-side strengths

The two tools are complementary: hyalo excels at **vault maintenance**; OKC excels at **vault querying and serving**. An ideal ecosystem might use hyalo for CLI maintenance and OKC's MCP server for agent access.