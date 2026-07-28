# Open Knowledge Catalog (OKC) — Competitor Landscape Compilation

**Generated:** 2026-07-27  
**Source:** 35 individual assessment files in `docs/competitors/`  
**Scope:** Complete competitive analysis for OKC positioning, threat assessment, and strategic roadmap

---

## Executive Summary

OKC (Open Knowledge Catalog) is a **Rust-based, local-first knowledge catalog** with SQLite+FTS5 storage, file watching, and an MCP server (11 tools over stdio/HTTP/SSE). It targets AI agents as first-class consumers via MCP while maintaining human-readable Markdown+YAML (OKF v0.2) as the source of truth.

**Competitive Position:** OKC occupies a unique niche — **structured knowledge catalog + MCP server** — but faces pressure from three directions:
1. **Agent memory systems** (memcrate, ai-memory, hermes-okf) — session context, not KB
2. **OKF-native tools** (vaultdb, okq, okf-http, copperbox/okf-mcp, travisjakel stack) — direct OKF overlap
3. **General agent knowledge infra** (knowledge-mcp, nexi-lab/nexus, relay-knowledge, tribal) — broader scope, deeper MCP

**Strategic Verdict:** OKC is the **most complete OKF catalog + MCP server today** but must close critical gaps in **graph-relational queries, mutation safety, vector search, and agent onboarding** to defend its position.

---

## Competitor Taxonomy (36 Assessments)

### Tier 1: Direct OKF Competitors (Highest Relevance)

| Competitor | Type | Threat | Key Differentiator |
|------------|------|--------|-------------------|
| **vaultdb** | Rust lib+CLI+MCP | **High** | Graph-relational joins (`LinkPredicate::Where`), plan/execute mutations, typed ORM, virtual fields |
| **okq** | Rust Tantivy search | **Medium** | Best pure search (Tantivy BM25, facets, phrase/fuzzy), library-first, 147 downloads |
| **okf-http** (kathrinmotzkus) | Rust HTTP server + Web UI | **Medium** | Human-facing web UI, prebuilt .deb packages, built-in TLS/auth, canonical OKF model lib |
| **copperbox/okf-mcp** | TypeScript MCP server | **Medium** | Remote bundle federation, agent onboarding docs, MCP-native architecture |
| **travisjakel stack** (okf-ingest + okf-mcp) | Python+R ingestion → DuckDB → MCP | **Medium** | Concept-first model, wikilinks, RAG-ready chunks, context packs, impact analysis, SQL escape hatch |
| **okf-w4g1** | Rust reference parser (zero-dep) | **Low** (symbiotic) | Spec authority, provenance/trust model, cross-bundle resolution — OKC *depends* on this crate |
| **okf-tools** (npm) | TypeScript CLI | **Low** | v0.0.1, corporate-backed (okf-brain), zero deps, config-driven — extremely early |
| **okapi-okf** | TypeScript Web UI + CLI | **Low** | Human-facing graph viz, bundle editor, AI "Ask bundle", multi-format distro (npx/Homebrew/binary) |
| **galdor/memory-okf** | TypeScript lib for galdor agents | **Low** | Code-aware BM25, progressive disclosure, change logs, citations — galdor-locked |
| **hermes-okf** | Python Hermes agent memory | **Low** | Hot/cold memory tiers, Git snapshots, tracks latest OKF spec (v0.2.9) |
| **okf-generator** (tommypacker/UmairBaig8) | Python bundle generators | **Low** | Repo→bundle generation, dogfooding, v0.2 extensions — complementary producers |
| **okf-tool** (npm) | TypeScript lib+CLI | **Low** | Library-first, bundle write/modify, minimal deps — no MCP, no persistence |
| **okf-toolset** (3 tools) | Python/TS linters | **Low** | Semantic cohesion scoring (okflint), manifest validation, index generation — validation layer only |
| **spec-okf** | TypeScript scaffolding | **Low** | Project init + methodology + multi-agent config gen — scaffolding, not runtime |
| **okf-cran** (travisjakel) | R package | **Low** | R ecosystem, DuckDB, semantic search/RAG, validation+auto-fix, bundle diff, context windows |
| **vagus** | Rust CLI hybrid search | **Medium** | Tantivy BM25 + EmbeddingGemma + RRF + cross-encoder rerank + HyDE query expansion — PARA-focused |

### Tier 2: Agent Memory Systems (Adjacent Category)

| Competitor | Type | Threat | Key Differentiator |
|------------|------|--------|-------------------|
| **memcrate** | Rust vault + skills | **Low** (direct) / **Medium** (architectural) | Three verbs (/save, /pin, /load), markdown vault, skill install for Claude Code |
| **ai-memory** | Rust MCP + HTTP + CLI | **Low** (direct) / **Medium** (architectural) | Tiered TTL memory (short/mid/long), auto-promotion, multi-user, replication, 89 CLI cmds |
| **basic-memory** | Python MCP server | **Medium-High** | Semantic search (FastEmbed), full CRUD via MCP, Claude Code plugin, cloud/teams, 3.5k★ |
| **hermes-okf** | Python agent memory | **Low** | Hot/cold memory tiers, Git snapshots, session checkpoints, OKF v0.2.9 |
| **semantic-memory-mcp** | Rust MCP (48+ tools) | **Medium** | Evidence-scored retrieval, contradiction detection, bitemporal, claim ledger, trust layer |
| **tf-idf-mcp** (category) | Various MCP servers | **Medium-High** | Zero-dep TF-IDF/BM25, write-capable MCP, self-improving loops, Obsidian compat (Knowledge Keeper) |
| **vagus** | Rust CLI hybrid search | **Medium** | Tantivy BM25 + EmbeddingGemma + RRF + cross-encoder rerank + HyDE query expansion — PARA-focused |

### Tier 3: General Agent Knowledge Infrastructure (Broader Scope)

| Competitor | Type | Threat | Key Differentiator |
|------------|------|--------|-------------------|
| **knowledge-mcp** (fulminate-io) | Go MCP daemon | **Medium/High** | 10-graph architecture, DeGroot reasoning, AST search, collectors (code/cloud/logs), workflow engine |
| **nexi-lab/nexus** | Rust VFS + Python SDK | **High** (indirect) | Distributed VFS, 35+ bricks, 15 drivers, ReBAC, Raft federation, sub-μs IPC |
| **relay-knowledge** | Rust GraphRAG + code intel | **High** | Hybrid GraphRAG (BM25+local sigs+hashed ANN), tree-sitter (25 langs), context packs, partitioned SQLite |
| **tribal** | Rust MCP + Postgres | **Medium** | Typed knowledge items (Fact/Heuristic/Procedure/DecisionRecord), 4 relation types, OAuth scopes, skills |
| **openwiki** | TypeScript LLM wiki gen | **Medium-High** | Generative wiki synthesis via LLM, multi-source connectors, team mode, 7.8k weekly downloads |
| **wicked-knowledge** (absorbed) | Rust MCP (now in wicked-estate) | **Low-Medium** | Engine isolation (separate SQLite per domain), cross-engine linking, 105 languages, Postgres option |

### Tier 4: Specialized / Non-Competitive

| Competitor | Type | Threat | Notes |
|------------|------|--------|-------|
| **a3s/coding-tools** | NestJS framework / elizaOS plugin | **None** | Backend framework + coding assistant — wrong problem space |
| **arkouda-memstead** | Chapel HPC / Memstead (schema-validated KG) | **None** / **Medium** | Arkouda unrelated; Memstead: schema-enforced entities, typed relations, git backend, registry |
| **atheneum/Athenaeum/YantrikDB** | Agentic memory (3 projects) | **Low** (direct) / **Medium** (architectural) | Session memory → consolidation → KB pipeline; HopGraph, librarian, 5-index cognitive engine |
| **gnosis/dkp** | Container runtime / Knowledge packs | **None** / **Medium-High** | gnosis unrelated; dkp: knowledge-as-artifact, 8-gate cert, eval sets, MCP bundling, registry |
| **gobline-gooseberry-nexis-okul** | 3 projects (nexus-memory, knowledge-mcp, nexi-lab) | **Low/Medium/High** | See Tier 3 entries |
| **markbase** | Rust DuckDB + templates | **Medium** | Template-driven schema enforcement, link target type checking, agent verify command |
| **mcp-knowledge-base** | Rust MCP (ADK-Rust) | **Medium** | TF-IDF + feedback loops, gap detection, draft/publish workflow, versioning — in-memory only |
| **mdvault** | Rust vault manager | **Low** | Note types, Lua scripting, TUI, task/project mgmt — productivity vault, not catalog |
| **obscure-tools** | 6 minor tools | **None** | Ghost, abandoned, wrong domain, or adjacent (snippets, secrets) |
| **copperbox-okf-mcp** | See Tier 1 | **Medium** | Already categorized |

---

## Comparative Capability Matrix

### Core Capabilities (OKC vs Top Competitors)

| Capability | OKC | vaultdb | okq | okf-http | copperbox | travisjakel | basic-mem | knowledge-mcp | relay-knowledge | tribal | vagus |
|------------|-----|---------|-----|----------|-----------|-------------|-----------|---------------|-----------------|--------|-------|
| **OKF Spec** | v0.2 | v0.1 | v0.1 | v0.2 | v0.1 | v0.1 | Proprietary | Proprietary | Proprietary | Proprietary | Any markdown |
| **Storage** | SQLite+FTS5 | SQLite+FTS5 (opt) | Tantivy | SQLite | In-memory | DuckDB | SQLite+FastEmbed | In-memory | SQLite (partitioned) | Postgres+pgvector | Tantivy + usearch |
| **Full-text Search** | BM25/FTS5 | ❌ (separate crate) | Tantivy BM25 | Basic LIKE | ❌ | DuckDB FTS | Hybrid (FastEmbed) | TF-IDF | BM25+local sigs+ANN | pgvector | Tantivy BM25 |
| **Vector Search** | ❌ Planned | ❌ | ❌ | ❌ | ❌ | Embedding col (unused) | ✅ FastEmbed | ❌ | ✅ Local hashed ANN | ✅ pgvector | ✅ EmbeddingGemma ONNX |
| **Reranking** | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ Cross-encoder |
| **Query Expansion** | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ HyDE via Qwen3 GGUF |
| **Graph Traversal** | ✅ BFS (depth/node limits) | ✅ LinkPredicate::Where | ❌ | ✅ Relations API | ⚠️ Implicit | ✅ Context/impact | ❌ | ✅ 10 graphs | ✅ Hybrid + code graph | ✅ Typed relations | ❌ |
| **MCP Server** | ✅ 11 tools, stdio+HTTP/SSE | ✅ 12 tools, stdio only | ❌ | ❌ | ✅ 11 tools | ✅ 8 tools, stdio only | ✅ 15+ tools | ❌ (hooks only) | ✅ Streamable HTTP | ✅ stdio/HTTP/SSE | ❌ CLI only |
| **File Watching** | ✅ notify + debounce | ❌ Philosophy | ❌ | ❌ | ❌ | ❌ | Manual sync | ❌ | ✅ Incremental | ❌ | ❌ |
| **Mutation Safety** | ❌ Read-only MCP | ✅ Plan/execute, lock, journal | ❌ | ❌ | ❌ | ❌ | ✅ Full CRUD via MCP | ✅ Draft/publish | ✅ Worker proposals | ❌ | ❌ |
| **Export Formats** | ❌ JSON only | ✅ CSV/TSV/JSON/YAML/XLSX | ❌ | ❌ | ❌ | ❌ | Markdown files | ❌ | ❌ | ❌ | ❌ |
| **Virtual Fields** | ⚠️ Implicit | ✅ 12 computed fields | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Schema Validation** | ✅ 8-category | ✅ Inference+validation | ❌ | ❌ | ❌ | ✅ Pydantic strict | ⚠️ Schema infer | ❌ | ❌ | ❌ | ❌ |
| **Lineage/History** | ✅ `lineage` tool | ❌ | ❌ | ❌ | ❌ | ❌ | Git history | ❌ | ❌ | Graph versioning | ❌ |
| **Section Extraction** | ✅ `get_section` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Metadata Query** | ✅ `query_metadata` | ❌ | ✅ Facets | ❌ | ❌ | ❌ | Tags only | ❌ | ❌ | ❌ | ❌ |
| **Agent Onboarding** | ❌ | ❌ | ❌ | ❌ | ✅ Excellent docs | ❌ | ✅ Claude Code plugin | ❌ | ❌ | ✅ Skills repo | ❌ |
| **Distribution** | `cargo install` | `cargo install` | `cargo install` | Prebuilt .deb, nightly CI | `npx` / npm | Python + R deps | `uvx` / Homebrew | curl/sh / brew | Single binary | Docker Compose | `cargo install` |
| **Auth/Scopes** | ❌ | ❌ | ❌ | ✅ Argon2 + sessions | ❌ | ❌ | ❌ | ❌ | Scope policy | ✅ OAuth 2.1 + scopes | ❌ |

---

## Threat Assessment Summary

### 🔴 HIGH Threat (Direct Competitors with Overlapping Value Prop)

| Competitor | Why High Threat | OKC Response Priority |
|------------|-----------------|----------------------|
| **vaultdb** | Same trinity (lib+CLI+MCP), same Rust stack, same use case. Beats OKC on graph joins, mutation safety, typed ORM, virtual fields, export, concurrency. | **P0**: Graph-join predicates, plan/execute mutations, virtual fields, export, advisory lock |
| **relay-knowledge** | Production GraphRAG + code intel in single binary. Hybrid retrieval (BM25+local sigs+ANN), tree-sitter (25 langs), context packs with budgets, partitioned SQLite, OTLP/Prometheus, self-iteration. | **P0**: Optional vector index, tree-sitter MVP, context packs, observability, partitioned storage |
| **knowledge-mcp** (fulminate) | Broadest scope: 10 graphs, DeGroot reasoning, AST search, collectors, workflow engine, team sync. 22 MCP tools. | **P1**: Connector framework, optional generative synthesis, multi-repo support |
| **basic-memory** | 3.5k★, 57K downloads, semantic search, full MCP CRUD, Claude Code plugin, cloud/teams, Obsidian-native. AGPL but strong traction. | **P1**: Vector search, MCP write tools, agent plugin, cloud sync strategy |

### 🟡 MEDIUM Threat (Specialized Overlap or Architectural Inspiration)

| Competitor | Threat Vector | OKC Response Priority |
|------------|---------------|----------------------|
| **okq** | Best pure search (Tantivy), library embeddable, 147 downloads, faceted search, rich syntax | **P1**: Faceted search in `search`, optional Tantivy backend, public search library |
| **copperbox/okf-mcp** | Remote bundle federation, agent onboarding docs, MCP-native | **P1**: Remote bundle sync, AGENTS.md, MCP resource templates |
| **travisjakel stack** | Concept-first model, wikilinks, RAG chunks, context packs, impact analysis, SQL escape hatch | **P0**: Wikilinks + concept extraction, context packs, impact tool, bundle diff, SQL escape hatch |
| **okf-http** | Web UI, prebuilt packages, TLS, auth, canonical OKF model | **P1**: Optional web UI, prebuilt binaries, optional TLS, evaluate canonical model dep |
| **tf-idf-mcp category** | Zero-dep search, write-capable MCP, self-improving loops, Obsidian compat | **P1**: MCP write tools, feedback-driven ranking, gap detection, Obsidian compat |
| **semantic-memory-mcp** | Evidence scoring, contradiction detection, bitemporal, claim ledger, 48+ tools | **P2**: Evidence model, contradiction detection, bitemporal queries |
| **tribal** | Typed knowledge items, 4 relation types, OAuth scopes, skills, session resource | **P1**: Typed relations, fact lifecycle, agent auth/scopes, session resource |
| **arkouda-memstead** | Schema-enforced entities, typed relations, git backend, registry, multi-mem workspaces | **P1**: Lightweight schema layer, typed relations, registry model |
| **vagus** | Production hybrid search (BM25 + EmbeddingGemma + RRF + rerank + HyDE), PARA-focused, 2K LoC | **P1**: Optional vector search + hybrid fusion, keep structured metadata as differentiator |

### 🟢 LOW Threat (Complementary, Different Layer, or Early Stage)

| Competitor | Relationship | Notes |
|------------|--------------|-------|
| **okf-w4g1** | **Symbiotic** | OKC *depends* on this crate for parsing/validation. Upstream. |
| **okf-tools** | Complementary | v0.0.1, corporate-internal, no MCP/search/index. Monitor. |
| **okapi-okf** | Complementary | Human-facing IDE. OKC could add web UI. |
| **galdor/memory-okf** | Complementary | galdor-locked. Patterns: code-aware BM25, progressive disclosure, citations. |
| **hermes-okf** | Complementary | Hot/cold memory + snapshots. OKC should adopt session tiering. |
| **okf-generator** (both) | Complementary | Bundle producers. OKC should own generate→index→serve pipeline. |
| **okf-tool** | Complementary | Library-first. OKC should expose `okc-lib` crate. |
| **okf-toolset** | Complementary | Validation layer. OKC should adopt semantic cohesion, manifest validation. |
| **spec-okf** | Complementary | Scaffolding. OKC should add `okc init` + methodology docs. |
| **okf-cran** | Complementary | R ecosystem. Feature reference for RAG, validation+fix, diff, context. |
| **memcrate/ai-memory** | Adjacent category | Session memory vs KB. OKC should add session awareness + promotion pipeline. |
| **mdvault** | Different philosophy | Productivity vault (tasks, Lua, TUI). OKC = catalog + query engine. |
| **wicked-knowledge** | Different philosophy | Code intelligence graph. Format lock-in (SQLite), no git. OKC = portable markdown. |
| **openwiki** | Adjacent | Generative wiki. OKC = structured index. Add connector framework + optional synthesis. |
| **a3s/coding-tools** | Non-competitor | Wrong problem space. |
| **gnosis** | Non-competitor | Container runtime. |
| **obscure-tools** | Non-competitor | Ghost/abandoned/wrong domain. |

---

## Strategic Gap Analysis: OKC vs. Market Leaders

### Critical Gaps (P0 — Must Close to Compete)

| Gap | Market Leader | OKC Status | Effort | Impact |
|-----|---------------|------------|--------|--------|
| **Graph-join predicates** (`links_to_where`) | vaultdb | ❌ Missing | Medium | High — enables "notes linking to anything tagged X" in single query |
| **Plan/execute mutation safety** | vaultdb | ❌ Read-only MCP | High | High — unlocks safe agent-authored content |
| **Virtual fields** (`_backlink_count`, `_modified`, etc.) | vaultdb | ⚠️ Implicit only | Low | High — enables "most linked" queries without traversal |
| **Export on all MCP reads** | vaultdb | ❌ Missing | Medium | Medium — parity for data extraction workflows |
| **Wikilinks + concept extraction** | travisjakel, copperbox | ❌ Markdown links only | Medium | High — aligns with OKF ecosystem, enables typed relations |
| **Context packs** (token-bounded) | travisjakel, relay-knowledge | ❌ Raw traverse only | Medium | High — agent-friendly bounded retrieval |
| **Impact analysis** (reverse deps) | travisjakel, relay-knowledge | ❌ Generic BFS only | Medium | High — "what breaks if this changes" |
| **Bundle diff/versioning** | travisjakel, dkp | ❌ Git only | Medium | Medium — explicit version tracking |
| **SQL escape hatch** | travisjakel, relay-knowledge | ❌ Missing | Low | Medium — power-user analytical queries |
| **Optional vector search** | okq, relay-knowledge, basic-memory | ❌ Planned only | Medium | High — semantic retrieval parity |
| **Tree-sitter code indexing** | relay-knowledge, okf-generator (Umair) | ❌ Markdown only | High | Medium — code intelligence differentiator |
| **Agent onboarding (AGENTS.md, skills)** | copperbox, basic-memory, tribal | ❌ Missing | Low | High — reduces prompt engineering burden |
| **Prebuilt binaries + Homebrew** | okf-http, okapi, basic-memory | ❌ `cargo install` only | Low | High — distribution friction |
| **Observability (OTLP/Prometheus)** | relay-knowledge, tribal | ❌ Logs only | Medium | Medium — production readiness |

### Competitive Parity Gaps (P1 — Should Close)

| Gap | Reference | OKC Status | Effort |
|-----|-----------|------------|--------|
| Faceted search in `search` tool | okq | ❌ Separate `query_metadata` | Low |
| Remote bundle federation | copperbox | ❌ Local only | Medium |
| Web UI for humans | okf-http, okapi, knowledge-mcp | ❌ None | Medium |
| TLS for MCP HTTP | okf-http | ❌ External only | Low |
| Authentication/scopes | okf-http, tribal, relay | ❌ None | Medium |
| Semantic cohesion scoring | okflint | ❌ None | Low |
| Manifest-driven validation | okflint | ❌ Implicit conventions | Low |
| Index export (OKF §6) | okflint | ❌ None | Low |
| Reserved file validation | okflint | ❌ None | Low |
| Opinionated conventions flag | okf-lint | ❌ Basic validate only | Low |
| Health diagnostics resource | relay-knowledge | ❌ None | Low |
| Session memory tier + snapshots | hermes-okf, memcrate | ❌ Single catalog | Medium |
| Feedback-driven ranking | basic-memory, mcp-kb | ❌ None | Medium |
| Gap detection from failed queries | mcp-kb, knowledge-base-mcp | ❌ None | Medium |
| Draft/publish workflow | mcp-kb, dkp | ❌ File-based | Medium |
| Multi-repo / repo-set support | relay-knowledge, openwiki | ❌ Single roots | Medium |
| Connector framework (git, GitHub, fs) | openwiki, knowledge-mcp | ❌ Markdown only | Medium |
| Optional generative synthesis | openwiki, knowledge-mcp | ❌ Extractive only | Medium |

### Differentiation Opportunities (P2 — Unique Value)

| Opportunity | Inspiration | OKC Advantage |
|-------------|-------------|---------------|
| **Search receipts / audit trail** | Neither has it | First-mover for agent trust |
| **Provenance as first-class field** | Athenaeum (mandatory) | OKF citations + trust tiers |
| **Bitemporal queries** | semantic-memory-mcp | OKF v0.2 lineage fields |
| **Contradiction detection** | semantic-memory-mcp, YantrikDB | Graph + validation combo |
| **OKF conformance CI** | core-okf (golden fixtures) | Spec compliance guarantee |
| **Agent checkpoint/resume** | hermes-okf | Session continuity |
| **Cross-bundle reference resolution** | okf-w4g1 | Federation ready |
| **Typed relation syntax** (`[[supports:path]]`) | okf-w4g1, travisjakel | Richer graph semantics |

---

## OKC Improvement Roadmap (Consolidated from All Assessments)

### Phase 1: Close Critical Gaps (Q1 — Immediate)

| # | Action | Source Competitors | OKC Component |
|---|--------|-------------------|---------------|
| 1 | **Graph-join predicate** in `search`/`query_metadata` (`links_to_where: Expr`) | vaultdb, travisjakel | `SearchFilters`, FTS join or post-filter |
| 2 | **Plan/execute mutation MCP tools** + audit log (`.okc/audit.log`) | vaultdb, basic-memory | New MCP tools, mutation builders |
| 3 | **Virtual fields** as queryable columns (`_backlink_count`, `_modified`, `_created`) | vaultdb | Index-time compute or query-time |
| 4 | **Export parameter** on all MCP read tools (CSV/TSV/JSON/YAML) | vaultdb | MCP tool schemas, rendering |
| 5 | **Wikilinks parser** (`[[concept]]`) → concept IDs in front-matter | travisjakel, copperbox, okf-w4g1 | `parser`, `scanner`, concept table |
| 6 | **Context pack tool** (`search` + `traverse` + metadata, `max_tokens` budget) | travisjakel, relay-knowledge | New MCP tool |
| 7 | **Impact analysis tool** (reverse link traversal from concept/file) | travisjakel, relay-knowledge | New MCP tool |
| 8 | **Bundle diff tool** (compare index snapshots, `version` front-matter) | travisjakel, dkp | New CLI + MCP tool |
| 9 | **SQL escape hatch** (read-only `query_sql` on allowlisted tables) | travisjakel, relay-knowledge | New MCP tool, security review |
| 10 | **Optional vector index** (`sqlite-vec` feature flag) for hybrid BM25+vector | okq, relay, basic-mem | `index`, `search`, feature flag |
| 11 | **Tree-sitter MVP** (Rust/TS/Go/Python) → `symbol`, `definition`, `references` MCP tools | relay, okf-generator (Umair) | `scanner`, new concept types |
| 12 | **Agent onboarding**: `AGENTS.md` + `okc install <agent>` (Claude Code, OpenCode, Cursor) | copperbox, basic-mem, tribal | New CLI command, skill files |
| 13 | **Prebuilt binaries** (GitHub Releases, Homebrew tap, Scoop) | okf-http, okapi, basic-mem | CI/CD, `cargo dist` or manual |
| 14 | **Observability**: OTLP + Prometheus `/metrics` + `okc://index/status` resource | relay, tribal | `opentelemetry`, `prometheus`, MCP resources |

### Phase 2: Competitive Parity (Q2)

| # | Action | Source Competitors | OKC Component |
|---|--------|-------------------|---------------|
| 15 | **Faceted search** inline in `search` (return facet counts) | okq | `search` tool, `SearchResponse` |
| 16 | **Remote bundle federation** (config-driven background sync, cross-bundle validation) | copperbox | Config, background job, validator |
| 17 | **Optional web UI** (`okc serve --web`) with graph viz + editing | okf-http, okapi, knowledge-mcp | Axum + embedded SPA |
| 18 | **TLS for MCP HTTP** (`--tls` with auto-certs via `rcgen`) | okf-http | `serve --http --tls` |
| 19 | **Authentication/scopes** (scope config + per-tool auth middleware) | okf-http, tribal, relay | Config, MCP handler |
| 20 | **Semantic cohesion scoring** (`okc cohesion`) | okflint | New CLI/MCP tool |
| 21 | **Manifest-driven validation** (`okc validate --manifest`) | okflint | New validation mode |
| 22 | **Index export** (`okc index --export` per OKF §6) | okflint | New CLI command |
| 23 | **Reserved file check** (`okc validate --reserved`) | okflint | Validation check |
| 24 | **Opinionated conventions flag** (`okc validate --conventions`) | okf-lint | Validation flag |
| 25 | **Session memory tier** + `okc snapshot`/`okc restore` (Git tags) | hermes-okf, memcrate | New concepts, MCP tools |
| 26 | **Feedback-driven ranking** (`helpful_count`/`not_helpful_count` boost) | basic-mem, mcp-kb | Schema + search rerank |
| 27 | **Gap detection** (track failed searches → `get_knowledge_gaps`) | mcp-kb, knowledge-base-mcp | Search logging + tool |
| 28 | **Draft/publish workflow** (`status` front-matter, `create_draft`/`publish` MCP tools) | mcp-kb, dkp | Schema + MCP tools |
| 29 | **Multi-repo / repo-set support** (cross-repo traverse, workspace detection) | relay, openwiki | Config, `traverse` |
| 30 | **Connector framework** (git, GitHub, generic fs) for `okc ingest` | openwiki, knowledge-mcp | New `ingest` command |
| 31 | **Optional generative `context_pack`** (LLM summary of search+traverse, feature flag) | openwiki, knowledge-mcp | New MCP tool, BYO provider |

### Phase 3: Differentiation & Moats (Q3+)

| # | Action | Source Competitors | OKC Component |
|---|--------|-------------------|---------------|
| 32 | **Search receipts / audit trail** (provenance for agent trust) | Neither has it | New MCP tool + resource |
| 33 | **Provenance as first-class front-matter** (mandatory `source` field) | Athenaeum | Schema + validation |
| 34 | **Bitemporal queries** (`search_as_of` valid_time) | semantic-memory-mcp | `lineage` + query |
| 35 | **Contradiction detection** (content-based + belief propagation) | semantic-memory-mcp, YantrikDB | `validate` extension |
| 36 | **OKF conformance CI** (vendor core-okf golden fixtures) | core-okf | CI job |
| 37 | **Agent checkpoint/resume** (`checkpoint`/`resume` MCP tools) | hermes-okf | Session tools |
| 38 | **Cross-bundle reference resolution** (bundle_id in links) | okf-w4g1 | Link model + validator |
| 39 | **Typed relation syntax** (`[[supports:path]]` → `relation_type`) | okf-w4g1, travisjakel | Parser + link model |
| 40 | **Public search library** (`okc-search` crate) | okq | Crate split |
| 41 | **TypeScript bindings** (`napi-rs`/`wasm-bindgen`) | okf-tool, copperbox | FFI layer |
| 42 | **Permissive parse mode** (`--lenient` for messy bundles) | okf-tool, okq | Scanner flag |
| 43 | **OKC dogfooding** (own docs as OKF bundle, `okc scan` on self) | okapi, okf-generator | Docs restructure |

---

## Competitive Positioning Statement

> **OKC is the definitive local-first OKF catalog for AI agents.**
>
> - **vs. vaultdb**: OKC wins on search relevance (BM25 out of box), live sync, HTTP transport, section extraction, validation. Must close graph-join + mutation safety gap.
> - **vs. relay-knowledge/tribal/knowledge-mcp**: OKC wins on simplicity (single binary, no Postgres, no daemon), portability (Markdown+Git), OKF standardization. Must add optional vector search, code graph MVP, context packs.
> - **vs. basic-memory/tf-idf-mcp**: OKC wins on structured model (OKF), graph traversal, validation, safety bounds, incremental indexing. Must add MCP write tools, feedback loops, gap detection.
> - **vs. okq/okf-http/copperbox**: OKC wins on unified catalog runtime (index + search + traverse + MCP + watch). Must adopt their best patterns (facets, federation, web UI, distribution).

**Moat:** OKF v0.2 compliance + SQLite+FTS5 incremental indexing + MCP stdio/HTTP/SSE + graph traversal + validation + lineage — **no other tool combines all six.**

---

## Monitoring Watchlist

| Competitor | Trigger for Reassessment | Check Frequency |
|------------|-------------------------|-----------------|
| **vaultdb** | Adds HTTP transport, vector search, or MCP resources | Monthly |
| **relay-knowledge** | Adds document-centric workflow, simplifies deployment | Monthly |
| **knowledge-mcp** (fulminate) | OSS launch, adds MCP resources, simplifies architecture | Monthly |
| **basic-memory** | Adds graph traversal, validation, OKF import | Quarterly |
| **okq** | Adds MCP server, incremental indexing, graph traversal | Quarterly |
| **copperbox/okf-mcp** | Adds persistent index, HTTP transport, validation | Quarterly |
| **travisjakel stack** | Adds HTTP transport, v0.2 support, binary packaging | Quarterly |
| **okf-http** | Adds MCP server, graph traversal, incremental indexing | Quarterly |
| **semantic-memory-mcp** | Adds document model, OKF support, simpler deployment | Quarterly |
| **tribal** | Adds code intelligence, local file indexing, markdown retrieval | Quarterly |
| **openwiki** | Adds MCP server, incremental indexing, structured query | Quarterly |
| **dkp** | Gains ecosystem traction, registry adoption | Quarterly |
| **vagus** | Adds structured metadata query, graph traversal, MCP server, file watching | Quarterly |

---

## Appendix: Assessment File Index

All 35 source assessments in `docs/competitors/`:

1. `a3s-coding-tools-assessment.md` — Non-competitor (backend framework + coding plugin)
2. `arkouda-memstead-assessment.md` — Arkouda (HPC, unrelated) + Memstead (schema KG, Medium threat)
3. `atheneum-assessment.md` — 3 agentic memory systems (atheneum, Athenaeum, YantrikDB)
4. `basic-memory-assessment.md` — Basic Memory (Medium-High threat, 3.5k★)
5. `copperbox-okf-mcp-assessment.md` — copperbox/okf-mcp (Medium threat, federation)
6. `equationalapplications-core-okf-assessment.md` — core-okf (Low, symbiotic, spec authority)
7. `galdor-memory-okf-assessment.md` — galdor/memory-okf (Low, galdor-locked)
8. `gnosis-dkp-assessment.md` — gnosis (unrelated) + dkp (Medium-High, knowledge packs)
9. `gobline-gooseberry-nexis-okul-assessment.md` — nexus-memory, knowledge-mcp, nexi-lab/nexus
10. `hermes-okf-assessment.md` — hermes-okf (Low direct, Medium architectural)
11. `hyalo-assessment.md` — hyalo (Medium, best CLI for vault maintenance)
12. `markbase-assessment.md` — markbase (Medium, template-driven schema enforcement)
13. `mcp-knowledge-base-assessment.md` — mcp-knowledge-base (Medium, TF-IDF + governance, in-memory)
14. `mdvault-assessment.md` — mdvault (Low, productivity vault)
15. `memcrate-ai-memory-assessment.md` — memcrate + ai-memory (Low direct, Medium architectural)
16. `obscure-tools-assessment.md` — 6 minor tools (None/Low)
17. `okapi-okf-assessment.md` — okapi-okf (Low, human-facing IDE)
18. `okf-cran-travisjakel-assessment.md` — CRAN okf (Low direct, Medium feature ref)
19. `okf-ecosystem-assessment.md` — Ecosystem index (6 independent crates, no coordination)
20. `okf-generator-tommypacker-assessment.md` — okf-generator (Low, bundle producer)
21. `okf-generator-umairbaig8-assessment.md` — okf-generator (Medium, code intelligence + MCP)
22. `okf-http-assessment.md` — okf-http (Medium, web UI + packages + auth)
23. `okf-tool-assessment.md` — okf-tool (Low, library-first)
24. `okf-toolset-assessment.md` — okf-toolset (Low, validation layer collection)
25. `okf-tools-assessment.md` — okf-tools npm (Low, v0.0.1 corporate)
26. `okf-w4g1-assessment.md` — okf (W4G1) (Low, symbiotic, spec reference)
27. `okq-assessment.md` — okq (Medium, best pure search)
28. `openwiki-assessment.md` — openwiki (Medium-High, generative wiki, 7.8k dl/week)
29. `semantic-memory-mcp-assessment.md` — semantic-memory-mcp (Medium, 48 tools, evidence model)
30. `spec-okf-assessment.md` — spec-okf (Low, scaffolding + methodology)
31. `tf-idf-mcp-assessment.md` — TF-IDF MCP category (Medium-High, 9+ projects)
32. `travisjakel-okf-ingest-assessment.md` — travisjakel stack (Medium, closest architectural competitor)
33. `tribal-relay-knowledge-assessment.md` — tribal + relay-knowledge (Medium/High)
34. `vaultdb-assessment.md` — vaultdb (High, most direct competitor)
35. `vagus-assessment.md` — vagus (Medium, hybrid search with embeddings + rerank + HyDE)
36. `wicked-knowledge-assessment.md` — wicked-knowledge (Low-Medium, code intelligence graph)

---

*End of Compilation*