# TF-IDF MCP — Competitor Assessment

## Overview

"TF-IDF MCP" is not a single project but a category of MCP servers that use Term Frequency-Inverse Document Frequency (TF-IDF) for knowledge retrieval. The most mature implementations are:

- **Knowledge Keeper MCP** ([zsc-glitch/knowledge-keeper-mcp](https://github.com/zsc-glitch/knowledge-keeper-mcp)) — 32 MCP tools, BM25 + TF-IDF hybrid search, knowledge graph, versioning, Obsidian-compatible. The most feature-complete entry. 1.7k+ weekly npm downloads, MIT license.
- **knowledge-base-mcp** ([dyyz1993/knowledge-base-mcp](https://github.com/dyyz1993/knowledge-base-mcp)) — 18 MCP tools, 3-layer search (text/TF-IDF/semantic), Web UI. 108 weekly npm downloads, MIT license. Built with Bun.
- **mcp-knowledge-base** ([zavora-ai/mcp-knowledge-base](https://crates.io/crates/mcp-knowledge-base)) — Rust-based, 9 tools, TF-IDF with feedback loops and gap detection. Part of ADK-Rust Enterprise ecosystem.
- **Paddione/mcp** ([Paddione/mcp](https://github.com/Paddione/mcp)) — Lightweight Python/FastAPI TF-IDF vector store with MCP stdio server, ingests HTML/MD/PDF.
- **ClaudeHistoryMCP** ([jhammant/ClaudeHistoryMCP](https://github.com/jhammant/ClaudeHistoryMCP)) — BM25 + TF-IDF hybrid for Claude Code conversation history. TypeScript.
- **mcp_server_knowledge_engine** ([lhstorm/mcp_server_knowledge_engine](https://github.com/lhstorm/mcp_server_knowledge_engine)) — PDF-focused TF-IDF with proximity matching. Python.
- **knowflow** ([Bipin-24/knowflow](https://github.com/Bipin-24/knowflow)) — TF-IDF corpus search integrated with GA4 analytics and Jenkins CI.

The category emerged from a shared need: give AI agents searchable long-term memory without requiring embedding models, vector databases, or cloud APIs. TF-IDF and BM25 are zero-dependency, local-first alternatives to semantic search that work well for keyword-heavy technical content.

---

## Key Features

### Knowledge Keeper MCP (most feature-complete)
- **32 MCP tools** — Full CRUD, 4 search modes (basic, semantic/TF-IDF, BM25, hybrid/RRF), knowledge graph with Mermaid visualization, version history with diff/rollback, audit trail (SHA256), spaced repetition, duplicate detection, context explorer, import/export/batch/merge, cloud sync (Pro)
- **Hybrid search** — BM25 (R@5=95%) + TF-IDF semantic + Reciprocal Rank Fusion (R@5=97%+)
- **Zero AI dependency** — No API keys, no embedding model, no vector database required
- **Obsidian-compatible** — Read/write Markdown vaults directly on disk
- **Local-first** — All data in Markdown files on your machine
- **Optional upgrade path** — Drop-in @xenova/transformers for neural embeddings
- **Knowledge graph** — Entity detection, relationships, Mermaid export

### knowledge-base-mcp
- **18 MCP tools** — kb_write/read/search/list/delete/update/outline/recent, plus advanced: kb_ask/ingest_url/ingest_repo/stale_check/auto_link/suggest, plus file_read/grep/exists
- **3-layer search** — P0 text matching (0.2 weight), P1 TF-IDF (0.3), P2 multilingual semantic (0.5), weighted fusion
- **Multilingual** — paraphrase-multilingual-MiniLM-L12-v2 for 50+ languages (optional)
- **Self-improving loop** — kb_ask miss → Agent web search → kb_ingest_url → next hit
- **Miss log analysis** — kb_suggest recommends pre-fetch topics from failed queries
- **Dual transport** — Stdio (local) + HTTP (StreamableHTTP/SSE/REST)
- **Web UI** — Vite 6 + React 18 + Zustand + Tailwind + Ant Design

### mcp-knowledge-base (Rust/Enterprise)
- **9 MCP tools** — search_articles, get_article, list_related, create_draft, publish, suggest_update, record_feedback, list_articles, get_gaps
- **TF-IDF with business-logic boosts** — helpfulness, freshness, view count
- **Article lifecycle** — Draft → Review → Published → Archived/Deprecated
- **Gap detection** — Failed queries tracked and ranked by frequency
- **ADK-Rust Enterprise integration** — Registry, health checks, risk classes

---

## Architecture

### Knowledge Keeper MCP
- **Runtime**: Node.js / TypeScript, `@modelcontextprotocol/sdk`
- **Storage**: Markdown files on disk (local-first). Index is rebuilt from files on startup.
- **Search pipeline**: Tokenizer (lowercase, stop-word removal, stemming) → BM25 inverted index + TF-IDF sparse vectors → RRF fusion
- **Transport**: stdio (default). Supports npx instant-run.
- **Agent integration**: Designed for Claude Code, Cursor, Gemini CLI, Windsurf, hermes-agent
- **Upgrade path**: `npm install @xenova/transformers` swaps TF-IDF for neural embeddings

### knowledge-base-mcp
- **Runtime**: Bun / TypeScript
- **Storage**: Local JSON files in `.kb-mcp/` directory under project root
- **Search pipeline**: P0 (substring + field-weighted match) → P1 (TF-IDF + cosine similarity, Chinese bigram) → P2 (multilingual-MiniLM 384-dim vectors + cosine) → weighted fusion → TopK
- **Transport**: Stdio + HTTP (StreamableHTTP, SSE, REST) — dual mode
- **Web UI**: Separate Vite + React frontend served on HTTP transport
- **Self-improvement**: Miss logging → kb_suggest analysis → kb_ingest_url for gap filling

### mcp-knowledge-base
- **Runtime**: Rust (rmcp + adk-mcp-sdk)
- **Storage**: In-memory HashMap<String, Article> — **no persistence** (critical limitation)
- **Search**: O(N) scan computing TF-IDF scores on-the-fly with business-logic boosts (helpfulness, freshness, staleness penalties)
- **Transport**: stdio (rmcp protocol)
- **Governance**: Requires reviewer for publish, version counter on updates, expiration field

### Shared architectural patterns
All TF-IDF MCP servers share:

- **No external dependencies** for basic search — TF-IDF is pure math over local text
- **Stateless or rebuildable indexes** — No persistent index across restarts (except file-based in Knowledge Keeper)
- **MCP stdio transport** — Standard for all, some add HTTP/SSE
- **Zero API key requirement** — No OpenAI, no embeddings API needed
- **Lightweight** — Single binary (Rust) or zero-install npx (TypeScript), no Docker required

---

## Comparison with OKF

| Dimension | TF-IDF MCP (category) | OKF / OKC |
|-----------|----------------------|-----------|
| **Knowledge model** | Flat article/knowledge-item with tags and optional category. No typed relationships between items. Knowledge Keeper has a graph overlay but it's link-based, not type-constrained. | Structured concept types (Metric, Dataset, Dimension, Service, etc.) via YAML frontmatter `type` field. Enforced via OKF spec conventions. Links have direction and are typed. |
| **Storage** | Markdown files on disk (Knowledge Keeper) or JSON files (.kb-mcp/) or in-memory (mcp-knowledge-base) or SQLite (doc-search-mcp). No standard format — each project invents its own. | Markdown + YAML frontmatter per OKF spec. Filesystem-native, git-versionable. Standardized format across any OKF-compatible tool. |
| **MCP integration** | Native MCP servers with tools exposed. Knowledge Keeper: 32 tools. knowledge-base-mcp: 18 tools. All use stdio transport; some add HTTP/SSE. | Native MCP server (11 tools) + CLI (13 commands) + filesystem watcher. Dual transport (stdio + HTTP/SSE). |
| **Querying** | TF-IDF/BM25 keyword search with optional hybrid fusion. Some add metadata filtering (tags, category). No structured query on frontmatter fields. | BM25 full-text search (Tantivy/FTS5) + structured metadata queries (`query_metadata` with field filters) + graph traversal (`traverse_graph`, backlinks). Multiple query modes for different access patterns. |
| **Portability / format** | Proprietary internal formats (JSON files, in-memory structures). Knowledge Keeper uses Markdown files but with its own schema (knowledge-graph metadata, IDs). No interoperability between projects. | Open standard (OKF spec). Any OKF-compatible tool can read the same files. Standard YAML frontmatter fields (`title`, `type`, `description`, `tags`, `links`). |
| **Agent-readiness** | Good — tools are designed for agent consumption (search, CRUD, knowledge graph). Some have explicit agent workflows (kb_ask miss → gap fill). 32-18 tools per server. | Good — 11 focused tools with bounded responses. Section extraction, partial retrieval, directory browsing reduce token waste. Safety limits on all outputs. Source traceability built-in. |
| **Collaboration model** | Single-user or agent-alone. Knowledge Keeper has cloud sync (Pro) but no multi-user workflow. mcp-knowledge-base has draft/publish/review gates. No collaborative editing or shared repo model. | Git-native. Multi-user via standard Git workflows (branches, PRs, review). Multiple agents can index the same repo. Filesystem watcher supports concurrent edits. |

### Key differentiators

**OKF/OKC advantages:**
- **Structured knowledge model** — Concept types with domain-specific semantics (Metric, Dataset, Dimension) vs flat "articles"
- **Graph-native** — Links are first-class, traversable, typed. TF-IDF MCP has tags and optional graph overlays, but links are not part of the retrieval model
- **Portable format** — OKF spec means any tool in the ecosystem works. TF-IDF MCP projects are islands
- **Multiple query modes** — BM25 search + metadata query + graph traversal + directory browse. TF-IDF MCP primarily offers keyword search
- **Incremental filesystem watch** — `okc watch` detects changes in real-time. Most TF-IDF MCP projects require manual re-index
- **Validation** — 8-category structural validation (broken links, malformed YAML, etc.). No TF-IDF MCP server offers repository validation
- **Section extraction** — Retrieve specific Markdown sections without fetching full documents. TF-IDF MCP returns full items
- **Safety bounds** — Configurable limits on file sizes, traversal depth, output sizes. TF-IDF MCP has minimal or no safety bounds

**TF-IDF MCP advantages:**
- **Tool count** — Knowledge Keeper's 32 tools is the largest MCP tool surface in the knowledge management category. OKC has 11
- **Hybrid search** — BM25 + TF-IDF + RRF fusion achieves R@5=97%+. OKC uses BM25 alone
- **Write tools** — TF-IDF MCP servers allow agents to create, update, delete, publish knowledge via MCP. OKC is read-only via MCP (writes go through CLI or direct file edits)
- **Self-improving** — knowledge-base-mcp's kb_ask → miss → ingest_url loop means the knowledge base gets better with use. OKC has no equivalent
- **Gap detection** — mcp-knowledge-base and knowledge-base-mcp track failed queries and suggest articles to write
- **Zero-dependency search** — TF-IDF requires no external services, no embedding models, no vector DB. OKC needs SQLite + Tantivy (both embedded, but not zero-dependency)
- **Knowledge graph visualization** — Knowledge Keeper exports Mermaid diagrams. OKC has graph traversal but no visualization
- **Obsidian compatibility** — Knowledge Keeper reads/writes Obsidian vaults directly. OKC reads Obsidian-style wikilinks but has no vault integration
- **Spaced repetition** — Knowledge Keeper has review scheduling. Unique in the MCP knowledge space

---

## Strengths

1. **Zero infrastructure knowledge management** — No databases, no embedding models, no API keys. Just `npx` and a filesystem. The lowest-friction path to agent memory.
2. **Tool surface area** — Knowledge Keeper's 32 tools is the largest MCP tool surface of any knowledge management server. Covers every operation an agent might need.
3. **Hybrid search quality** — BM25 + TF-IDF + RRF achieves 97%+ recall@5, competitive with lightweight embeddings for keyword-heavy queries. Pure lexical search works well for technical domains.
4. **Self-improvement loops** — knowledge-base-mcp's miss → ingest cycle and mcp-knowledge-base's gap detection turn usage into content creation. The system gets smarter the more it's used.
5. **Obsidian ecosystem** — Knowledge Keeper's direct vault read/write is a powerful differentiator for the large Obsidian user base. Existing vaults become immediately searchable.
6. **Optional upgrade path** — The smooth progression from TF-IDF → BM25 → transformer embeddings means projects grow with user needs without breaking compatibility.
7. **Write-capable MCP tools** — Agents can create, edit, delete, publish, and manage knowledge entirely through MCP. No fallback to CLI or direct filesystem access needed.
8. **Low latency** — Sub-200ms search even with thousands of items. No network calls for inference.
9. **Large and growing category** — At least 9 distinct projects in this space, indicating strong market demand. Rapid iteration and feature addition.

---

## Weaknesses

1. **No standardized format** — Each project invents its own storage format (JSON with different schemas, custom Markdown frontmatter, in-memory). No interoperability. Switching tools means data migration.
2. **Flat knowledge model** — Knowledge is "articles" or "items" with tags. No typed relationships, no hierarchical classification, no domain-specific semantics (metric vs dataset vs service). Compare with OKF's rich type system.
3. **No structured query** — TF-IDF excels at keyword search but can't answer "show me all metrics with unit=eur and period=monthly." Metadata filtering is limited to tags/category, not arbitrary frontmatter fields.
4. **No graph as retrieval primitive** — Links exist in Knowledge Keeper's graph overlay, but search doesn't leverage them. You can't search "find concepts related to X through metrics that Y references." Links are for display, not retrieval.
5. **No validation** — No broken link detection, no schema enforcement, no circular reference detection, no duplicate content detection. Knowledge quality depends entirely on the agent.
6. **No incremental indexing** — Most TF-IDF MCP servers re-index on startup or on explicit command. Knowledge Keeper scans its Markdown directory at start. No filesystem watcher for real-time updates.
7. **Limited safety** — No bounds on file sizes, no traversal depth limits, no output size constraints. An agent can request a 100MB knowledge item.
8. **No section extraction** — Full item retrieval only. If a knowledge item is 200 lines, the agent gets all 200 lines. Compare with OKC's `get_section` for targeted retrieval.
9. **No structured metadata queries** — Can't filter by frontmatter fields, creation date ranges, or custom properties. Search is purely text-based.
10. **Persistence gaps** — mcp-knowledge-base is entirely in-memory. Others are file-based but may lose index state on crash.
11. **Scalability unknown** — BM25 + TF-IDF is O(N) per query without an inverted index (depending on implementation). R@5=97% is measured on small corpuses; performance with 100k+ items is unproven.

---

## Threat Level

**Medium-High** — as a category, not a single project.

**Why Medium-High:**
- The category is growing fast (9+ projects in 2025-2026). Knowledge Keeper MCP alone has 1.7k weekly npm downloads.
- The zero-dependency value proposition is compelling. Teams can add agent memory in 30 seconds with `npx`.
- Knowledge Keeper's Obsidian compatibility opens a large existing user base.
- The write-capable MCP tools (create, edit, publish) go beyond what OKC offers via MCP.
- Self-improvement loops (miss → ingest) are a genuinely innovative feature OKC lacks.

**Why not High:**
- No single project dominates. The fragmentation means no project has the network effects of a standardized format.
- The flat knowledge model limits utility for structured knowledge work. OKF's typed concepts with graph relationships serve different, more complex use cases.
- Lack of validation, safety, and structured query makes these tools suitable for "agent scratchpad" use but not "enterprise knowledge base" use.
- Projects are young (most <12 months old). Long-term maintenance unclear.

**Bottom line**: TF-IDF MCP servers are the strongest general-purpose alternative to OKC in the "agent memory" space. They solve a real problem (zero-infrastructure search) with pragmatic technology (TF-IDF/BM25). OKC's differentiation must come from its structured knowledge model (OKF), graph-native retrieval, validation, and multi-query-mode architecture. If OKC does not add write-capable MCP tools and a self-improvement loop, it will lose the "agent memory" use case to this category.

---

## Notes

### Implementation quality varies wildly

| Project | Language | Tools | Quality | Persistence |
|---------|----------|-------|---------|-------------|
| Knowledge Keeper MCP | TypeScript | 32 | High (70 tests, active development) | File-based Markdown |
| knowledge-base-mcp | Bun/TypeScript | 18 | Medium (149 files, 84 versions) | JSON files |
| mcp-knowledge-base | Rust | 9 | Medium (clean code, no persistence) | In-memory only |
| Paddione/mcp | Python/FastAPI | ~3 | Low (minimal docs, experimental) | JSON files |
| ClaudeHistoryMCP | TypeScript | ~6 | High (well-architected, ~20 modules) | SQLite |
| mcp_server_knowledge_engine | Python | ~2 | Low (single script, limited scope) | Pickle cache |

### The BM25 vs TF-IDF distinction matters

Several projects (Knowledge Keeper, ClaudeHistoryMCP) use BM25, which is an evolution of TF-IDF with document-length normalization and term-frequency saturation. The KUSH42/mcp_codebase_knowledege project explicitly explains why: "Raw TF-IDF has no document-length normalization. A 500-word document that mentions 'REST' once will outscore a 10-word document titled 'REST API design'." OKC uses BM25 via SQLite FTS5, which is the same algorithm these projects use. The AKC project's BM25 is equivalent to OKC's FTS5 BM25 — same algorithm, different backend.

### The write-capable MCP gap is real

OKC is read-only via MCP. An agent can query, browse, and search, but it cannot create or edit knowledge through MCP tools. Every TF-IDF MCP server in this category supports writes. This is a significant workflow difference: OKC treats the agent as a reader of human-authored knowledge; TF-IDF MCP treats the agent as a co-author of the knowledge base. Both models are valid, but the co-author model is more ambitious and better aligned with the "autonomous agent" vision.

### The self-improvement loop is an architectural insight

knowledge-base-mcp's `kb_ask` → miss → agent web search → `kb_ingest_url` → next-hit cycle is the most innovative feature in this category. It turns "knowledge not found" from a failure into a content creation event. The `kb_suggest` tool (recommends pre-fetch topics from miss log analysis) closes the loop further. No equivalent exists in OKC or any other competitor.

### OKC could adopt TF-IDF as a search signal

OKC currently uses BM25 (via Tantivy/FTS5). Adding TF-IDF as a complementary signal in a hybrid search (similar to Knowledge Keeper's BM25 + TF-IDF + RRF) could improve recall for edge cases. The RRF fusion pattern is well-documented and could be implemented as a post-query merge of Tantivy BM25 results with a lightweight in-memory TF-IDF index over frontmatter fields.

### Knowledge Keeper's ecosystem approach is worth watching

With 32 tools, cloud sync (Pro), Obsidian compatibility, and an npm distribution model (`npx` instant run), Knowledge Keeper is approaching platform territory. It's the closest thing this category has to a standard. If it develops an API for plugin tools or custom search backends, it could become the de facto agent knowledge management layer.
