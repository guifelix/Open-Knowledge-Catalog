# Competitor Assessment: mcp-knowledge-base (TF-IDF MCP)

## Overview

**mcp-knowledge-base** (crates.io: `mcp-knowledge-base`, v1.2.0, Apache-2.0) is a Rust-based MCP server that provides a knowledge base with TF-IDF search, feedback loops, gap detection, versioning, and draft/publish workflow. Built for ADK-Rust Enterprise ecosystem. It exposes 9 MCP tools for managing articles, policies, and known issues.

**OKC (Open Knowledge Catalog)** is a local-first tool that transforms a filesystem-based collection of Markdown documents with YAML front matter (OKF format) into a structured, searchable knowledge base with CLI, MCP server, and filesystem watcher. Uses SQLite + FTS5 with BM25 ranking.

Both are direct MCP-level competitors — they expose knowledge operations as MCP tools for AI agents.

---

## MCP Server Capability Inventory

| Tool/Resource | mcp-knowledge-base | OKC | Notes |
|---------------|-------------------|-----|-------|
| **Search** | `search_articles` — TF-IDF, category/audience filter, gap tracking | `search_documents` — BM25 (FTS5), path/type/tag filters | OKC has more filters (path, type, tags); mcp-kb has gap tracking |
| **Get Document** | `get_article` — full article with body, stats, version | `get_document` — metadata, headings, body, sections | OKC supports partial retrieval (headings, sections) |
| **Related** | `list_related_articles` — shared tags + category | `traverse_graph` — link-based graph traversal | Different paradigms: tag-based vs link-based |
| **Create** | `create_article_draft` — draft with author, tags, audience | N/A (CLI-only `okc scan` + manual file creation) | mcp-kb has MCP write tools; OKC is read-only via MCP |
| **Update** | `suggest_article_update` — new draft version, no mutation | N/A | mcp-kb has versioned updates via MCP |
| **Publish** | `publish_article` — requires reviewer, makes searchable | N/A | Governance workflow unique to mcp-kb |
| **Feedback** | `record_article_feedback` — helpful/not helpful, affects ranking | N/A | mcp-kb has explicit feedback loop |
| **List/Browse** | `list_articles` — filter by category/status | `browse_directory` — tree view with summaries | OKC has hierarchical browsing; mcp-kb has flat filtered list |
| **Gaps** | `get_article_gaps` — failed queries ranked by frequency | N/A | mcp-kb unique: gap detection from failed searches |
| **Validate** | N/A | `validate_repository` — 8-category structural checks | OKC unique: filesystem-level validation |
| **Stats** | N/A | `get_stats` — doc count, link count, heading count | OKC provides index statistics |
| **Sections** | N/A | `get_section` — extract specific Markdown section | OKC unique: section-level retrieval |
| **Links** | N/A | `get_links`, `get_backlinks` — link navigation | OKC unique: link graph |
| **Metadata Query** | N/A | `query_metadata` — exact structured filtering | OKC unique: structured front-matter queries |
| **Scan/Index** | N/A (in-memory) | `scan` — incremental index with content hashing | OKC unique: persistent incremental indexing |
| **Watch** | N/A | `watch` — fsnotify with debouncing/reconciliation | OKC unique: live filesystem sync |

**Transport**: Both use `rmcp` with stdio transport. OKC adds HTTP/SSE transport.

---

## Search Quality Comparison

### TF-IDF (mcp-knowledge-base) vs BM25 (OKC)

| Aspect | mcp-knowledge-base (TF-IDF) | OKC (BM25 via FTS5) |
|--------|----------------------------|---------------------|
| **Algorithm** | Custom TF-IDF in Rust: `score += (N/df) * log + 1` + boosts | SQLite FTS5 built-in BM25 (industry standard) |
| **Term Frequency** | Binary presence (contains check), no saturation | BM25: saturated TF with k1=1.2 (diminishing returns) |
| **Document Length** | No normalization (raw counts) | BM25: length normalization with b=0.75 |
| **Field Weighting** | Hardcoded: title +3, tags +2, body +1 | Configurable: title=10, description=5, headings=2, body=1 |
| **Stop Words** | Hardcoded 32-word list | FTS5 tokenizer (porter/unicode61) + custom stop words |
| **Scoring Boosts** | Helpfulness, freshness, staleness, expiration, feedback penalties | Pure BM25; no business-logic boosts in index |
| **Category/Audience Filter** | Pre-filter before scoring | SQL WHERE clause before FTS5 MATCH |
| **Gap Tracking** | Yes — failed queries tracked, ranked by frequency | No |
| **Persistence** | In-memory HashMap (lost on restart) | SQLite FTS5 index (persistent, incremental) |
| **Scalability** | O(N) scan of all articles per query | FTS5 inverted index — sub-linear |

**Key Differences:**

- **TF-IDF implementation**: mcp-kb computes document frequency (df) on-the-fly by scanning all articles for each query term. This is O(N*terms) per query. OKC's FTS5 uses a persistent inverted index — O(terms) lookup.
- **Ranking signals**: mcp-kb blends IR signals (TF-IDF) with business signals (helpfulness, freshness, feedback). OKC keeps ranking pure (BM25) and pushes business logic to post-filtering or application layer.
- **Field weighting**: mcp-kb hardcodes title/tags/body weights. OKC makes weights configurable via `Bm25Config` (title=10, description=5, headings=2, body=1).
- **Stop words**: mcp-kb uses a minimal hardcoded list. OKC leverages FTS5 tokenizers (porter, unicode61) with full stop-word support.

**Verdict**: OKC's BM25 is production-grade, scalable, and configurable. mcp-kb's TF-IDF is simpler, includes useful business-logic boosts, but doesn't scale beyond ~thousands of documents and loses index on restart.

---

## Knowledge Model Comparison

| Aspect | mcp-knowledge-base | OKC (OKF) |
|--------|-------------------|-----------|
| **Core Unit** | Article (KB-XXXX) | Document (Markdown + YAML front matter) |
| **Identifiers** | UUID-based `KB-<8char>` | File path (relative to root) |
| **Categories** | Free-text string | Directory hierarchy (path-based) |
| **Tags** | Array of strings | Array in front matter |
| **Audience** | Enum: Internal, EndUser, Admin, Engineering, Security | Free-text (or via custom field) |
| **Status** | Draft, Review, Published, Archived, Deprecated | Not modeled (file exists = published) |
| **Versioning** | Explicit version counter, new draft on update | Implicit via git/fs history |
| **Ownership** | Owner, created_by, updated_by, reviewer | Not in front matter (could be custom) |
| **Lifecycle** | Draft → Review → Published → Archived/Deprecated | File-based; no formal state machine |
| **Expiration** | `expires_at` field, penalized in search | Not modeled |
| **Replacement** | `replaced_by` link to newer version | Not modeled |
| **Feedback** | helpful_count, not_helpful_count, views | Not modeled |
| **Gaps** | Tracked per failed query with frequency | Not modeled |

**mcp-kb strengths**: Explicit lifecycle, versioning, governance (reviewer required), expiration, replacement tracking, feedback loop, gap detection — all modeled as first-class fields.

**OKC strengths**: Simplicity (files on disk), git-native versioning, directory hierarchy as category, extensible front matter, no schema lock-in.

**OKF concept types** (Metric, Dataset, Dimension, etc.) are domain-specific; mcp-kb uses generic "Article" with category. OKC is more opinionated about knowledge structure; mcp-kb is a generic article store.

---

## Architecture & Code Quality

### mcp-knowledge-base

**Structure**: 3 files (`main.rs`, `server.rs`, `store.rs`) — ~1,400 LOC total.

**Storage**: In-memory `HashMap<String, Article>` behind `Mutex`. No persistence — data lost on restart. Suitable for ephemeral/dev use only.

**Concurrency**: Coarse-grained `Mutex` on entire store. All operations serialize. Fine for low throughput; bottleneck at scale.

**Search**: O(N) scan per query. Computes df by scanning all articles for each term. No inverted index.

**Dependencies**: `rmcp`, `adk-mcp-sdk`, `serde`, `tokio`, `chrono`, `uuid`, `tracing`. Lightweight.

**MCP Integration**: Uses `adk-mcp-sdk` for health checks and `mcp-server.toml` manifest. Tools use `rmcp` macros (`#[tool_router]`, `#[tool]`). Risk classes defined (read_only, internal_write).

**Governance**: `publish_article` requires reviewer (`requires_approval: true`). Draft/publish workflow enforced.

**Code Quality**: Clean, idiomatic Rust. No `unsafe`. Good use of `schemars` for JSON Schema. Error handling with `anyhow`/`Result`. Some clone-heavy patterns (articles cloned on every search result).

**Production Readiness**: v1.2.0, but **no persistence** is a major gap for production. The CHANGELOG shows active development. In-memory only makes it unsuitable for durable knowledge bases without external persistence layer.

### OKC

**Structure**: ~50+ files, modular (scanner, parser, index, service, transport, config). ~10k LOC.

**Storage**: SQLite with FTS5 virtual table (`document_search`) + normalized tables (`documents`, `headings`, `links`, `tags`, `metadata`). Persistent, ACID.

**Indexing**: Incremental — Blake3 content hashing detects changes, only modified files re-parsed. FTS5 maintains inverted index automatically.

**Search**: FTS5 `MATCH` with BM25 ranking. Configurable field weights via `Bm25Config`. Sub-linear query time.

**Concurrency**: `r2d2` connection pool (SQLite WAL mode). Multiple readers, single writer.

**MCP Integration**: `rmcp` with custom tool router. 11 tools exposed. Both stdio and HTTP/SSE transports.

**Filesystem Watch**: `notify` crate with debouncing, temp-file filtering, gitignore-aware, periodic reconciliation.

**Validation**: 8-category repository validation (broken links, malformed YAML, circular refs, duplicate content, missing index files).

**Code Quality**: High. Clean architecture, separation of concerns, comprehensive error types, extensive tests, benchmarks, fuzzing.

**Production Readiness**: Published on crates.io, pre-built binaries for 5 platforms, CI/CD, used in production.

---

## Strengths vs OKC

| mcp-knowledge-base Strength | Why It Matters |
|----------------------------|----------------|
| **Feedback loop** (helpful/not helpful → ranking boost) | Closes the loop: usage improves retrieval |
| **Gap detection** (failed queries tracked) | Actionable: tells you what articles to write |
| **Draft/publish workflow** with reviewer gate | Governance for authoritative content |
| **Versioning** (new draft per update, original immutable) | Safe updates, audit trail |
| **Expiration/staleness modeling** | Prevents stale content from surfacing |
| **Audience enum** | Access control / targeting built-in |
| **Article replacement** (`replaced_by`) | Clear deprecation path |
| **9 focused MCP tools** | Complete CRUD + governance via MCP |
| **ADK-Rust Enterprise integration** | Registry, health checks, risk classes |

---

## Weaknesses vs OKC

| mcp-knowledge-base Weakness | Impact |
|----------------------------|--------|
| **No persistence** (in-memory HashMap) | Data lost on restart; not production-viable without external DB |
| **O(N) search** | Doesn't scale beyond ~5k articles |
| **No incremental indexing** | Full re-scan on any change (but N is small in-memory) |
| **No filesystem integration** | Can't point at existing Markdown repo; must use MCP tools to create |
| **No link graph** | No backlinks, traversal, or link validation |
| **No hierarchical browsing** | Flat category model vs OKC's directory tree |
| **No section extraction** | Must return full article body |
| **No structured metadata query** | Can't filter on arbitrary front-matter fields |
| **No repository validation** | No broken link detection, circular ref detection, etc. |
| **Coarse mutex** | Serializes all operations |
| **Hardcoded stop words, weights** | Not configurable without code change |
| **No CLI** | Only usable via MCP client |

---

## OKC Improvement Opportunities (from mcp-kb)

| Opportunity | Description | Effort |
|-------------|-------------|--------|
| **Feedback-driven ranking** | Add `helpful_count`/`not_helpful_count` to documents; boost BM25 score post-query | Medium (schema + search) |
| **Gap detection** | Track failed `search_documents` queries; expose via `get_knowledge_gaps` tool | Medium (store + tool) |
| **Draft/publish workflow** | Add `status` field to front matter; MCP tools for `create_draft`, `publish` (with reviewer) | High (schema, tools, governance) |
| **Article versioning** | Support `version` in front matter; `suggest_update` creates new draft file | High |
| **Audience/targeting** | Add `audience` field; filter in search | Low (front matter + filter) |
| **Expiration/staleness** | Add `expires_at`, `review_date`; penalize stale in ranking | Medium |
| **Article replacement** | Add `replaced_by` link; exclude deprecated from search | Low |
| **Related articles** | Tag/category-based `list_related` tool (complement to link-based `traverse`) | Low |
| **Views/usage tracking** | Increment view count on `get_document` | Low |
| **ADK-Rust SDK compliance** | Add `mcp-server.toml`, health check, risk classes for registry | Low |

---

## Verdict

**mcp-knowledge-base** is a **feature-rich MCP-native knowledge server** with excellent governance features (draft/publish, reviewer gates, feedback loops, gap detection, versioning, expiration). Its TF-IDF search includes practical business-logic boosts. **Critical flaw: no persistence** — in-memory only. Without a persistent backend (SQLite, PostgreSQL, Redis), it's limited to ephemeral/dev use.

**OKC** is a **production-grade local-first knowledge platform** with persistent SQLite+FTS5 storage, incremental indexing, filesystem watching, hierarchical browsing, link graph, validation, and scalable BM25 search. It lacks mcp-kb's governance workflow, feedback loop, and gap detection.

**Recommendation for OKC**: Adopt mcp-kb's governance and feedback features (draft/publish, feedback ranking, gap tracking) as optional MCP tools layered on top of OKC's persistent OKF repository. This combines OKC's durable, scalable foundation with mcp-kb's operational intelligence.

**Competitive Position**: mcp-kb targets ADK-Rust Enterprise users needing a managed KB with governance. OKC targets teams wanting a local-first, git-friendly, filesystem-native knowledge base with AI access. Different sweet spots; mcp-kb's lack of persistence is the blocker for serious comparison.