# Competitor Assessment: copperbox/okf-mcp

## Overview

**Project:** copperbox/okf-mcp  
**Repository:** https://github.com/copperbox/okf-mcp  
**License:** Not confirmed in public metadata (likely MIT or Apache-2.0 given ecosystem norms)  
**Language:** TypeScript (Node.js)  
**Architecture:** Native MCP server implementing OKF v0.1 specification with local and remote bundle support  
**Installation:** `npx @copperbox/okf-mcp` or `npm install -g @copperbox/okf-mcp` (Node.js required)  
**Stars:** 1 · **Commits:** 39 · **Maintainer:** copperbox  

**Description:** A native Model Context Protocol server that gives AI agents an OKF (Open Knowledge Format) backend. It treats Markdown files with YAML frontmatter as first-class knowledge units, indexes them into a link graph, and exposes search, traversal, validation, and authoring operations through MCP tools and resources. The server supports both colocated bundles (local filesystem folders) and remote bundles (URL-based), enabling cross-bundle awareness and linking. Its README includes a notable "teaching agents to maintain the brain" section — a novel approach to agent knowledge management that treats the MCP server as a living system agents actively curate.

---

## Knowledge Model Comparison

| Dimension | copperbox/okf-mcp | OKC | Notes |
|-----------|-------------------|-----|-------|
| **Knowledge format** | OKF v0.1: Markdown + YAML frontmatter, wikilinks/references for linking | OKF v0.2: Markdown + YAML frontmatter, typed relationships, cross-bundle refs | Both use OKF; copperbox implements v0.1, OKC targets v0.2 |
| **Concept model** | Concepts as files with frontmatter (id, title, tags, references, etc.) | Concepts as files with richer frontmatter (confidence, lineage, validity, etc.) | copperbox: simpler schema; OKC: extensible schema with evidence fields |
| **Link graph** | Explicit link graph indexing (wikilinks + references), cross-bundle awareness | SQLite+FTS5 index with graph edges, file watcher for live updates | Both maintain link graphs; OKC adds FTS5 + vector search |
| **Bundle model** | Colocated (local folder) + Remote (URL) bundles, cross-bundle linking | Single OKC catalog with multi-bundle support via CLI/MCP | copperbox: explicit multi-bundle architecture; OKC: unified catalog |
| **Validation** | `validate_bundle` tool checks structure, frontmatter, links | `okc validate` CLI + MCP `validate` tool with schema enforcement | Both have validation; OKC adds schema version checking |
| **Authoring** | `write_concept`, `read_document`, update tools via MCP | `okc concept add/update`, MCP `ingest`, `write` tools | Both support full CRUD via MCP; copperbox more document-centric |
| **Search** | `search` tool across bundles (implementation details sparse) | Hybrid: vector (embeddings) + FTS5 + graph traversal | OKC has richer search pipeline; copperbox search less documented |
| **Versioning** | Git-backed (files are Markdown) | Git-backed (OKF bundles) + `okc lineage` for concept history | Parity — both leverage Git for version control |
| **Human readability** | ✅ Native Markdown + YAML | ✅ Native Markdown + YAML | Both excel at human-auditable, git-friendly format |

---

## MCP Server Capability Inventory

| Capability | copperbox/okf-mcp | OKC MCP | Gap / Advantage |
|------------|-------------------|---------|-----------------|
| **Search across bundles** | ✅ `search` tool | ✅ `search` tool (hybrid vector+FTS+graph) | OKC: richer fusion pipeline |
| **Get concept by ID** | ✅ `get_concept` | ✅ `read` / `describe` tools | Parity |
| **Graph summary/overview** | ✅ `graph_summary` | ✅ `graph` tool + `status` | Parity |
| **Bundle validation** | ✅ `validate_bundle` | ✅ `validate` tool | Parity |
| **Write/create concept** | ✅ `write_concept` | ✅ `ingest` + `write` tools | Parity |
| **Read document content** | ✅ `read_document` | ✅ `read` tool | Parity |
| **Graph traversal** | ⚠️ Likely via link graph (undocumented) | ✅ `traverse` tool (BFS/DFS, typed edges) | OKC: explicit traversal API |
| **Lineage/history** | ❌ Not exposed via MCP | ✅ `lineage` tool (concept evolution) | OKC: native concept versioning |
| **File watching / live updates** | ❌ Not mentioned | ✅ `observe` tool + fs watcher | OKC: real-time sync |
| **Cross-bundle linking** | ✅ Explicit support | ✅ Via unified catalog | Parity (different architectures) |
| **Remote bundle support** | ✅ URL-based bundles | ❌ Local-only currently | copperbox: remote bundles |
| **Tool profiles (security)** | ❌ Not implemented | ❌ Not implemented | Gap for both |
| **Witnessed retrieval / receipts** | ❌ Not implemented | ❌ Not implemented | Gap for both |
| **Evidence scoring** | ❌ Not in OKF v0.1 | ⚠️ Planned for v0.2 | Both: future work |
| **Bitemporal queries** | ❌ Not supported | ❌ Not supported | Gap for both |
| **Contradiction detection** | ❌ Not implemented | ❌ Not implemented | Gap for both |
| **Document ingestion pipeline** | ❌ Manual concept creation | ❌ Not yet (planned) | Gap for both |
| **MCP Resources** | ✅ Bundle/concept resources | ✅ Concept/graph resources | Parity |
| **MCP Resource Templates** | ❌ Not documented | ❌ Not implemented | Gap for both |

---

## Architecture & Code Quality

| Aspect | copperbox/okf-mcp | OKC | Assessment |
|--------|-------------------|-----|------------|
| **Language** | TypeScript (Node.js) | Rust | OKC: single binary, no runtime deps; copperbox: Node.js ecosystem |
| **Runtime dependency** | Node.js ≥18 required | None (static binary) | OKC: simpler deployment |
| **MCP SDK** | `@modelcontextprotocol/sdk` (TypeScript) | `rmcp` (Rust) | Both use official SDKs |
| **Commits / Activity** | 39 commits, 1 star (low traction) | Active development, higher engagement | OKC: more momentum |
| **Documentation** | Excellent README with agent teaching guide | Good CLI docs, MCP docs in progress | copperbox: better agent onboarding docs |
| **Testing** | Not visible in repo metadata | Unit + integration tests in CI | OKC: verified test coverage |
| **CI/CD** | Not confirmed | GitHub Actions (lint, test, build) | OKC: automated quality gates |
| **Packaging** | npm package | Cargo + binary releases | OKC: easier cross-platform install |
| **Platform support** | Node.js platforms (Linux, macOS, Windows) | Linux, macOS, Windows (native) | Parity |
| **Architecture style** | MCP-native (server IS the product) | Hybrid: CLI + MCP server | copperbox: purer MCP-first design |
| **Bundle indexing** | In-memory link graph at startup | SQLite+FTS5 persistent index + file watcher | OKC: persistent, incremental |
| **Remote bundles** | HTTP fetch + parse at runtime | Not supported | copperbox: unique capability |

---

## Search and Retrieval Capabilities

### copperbox/okf-mcp Search Architecture
- **Index**: In-memory link graph built at server startup from local and remote bundles
- **Query interface**: Single `search` tool accepting query string, returns matching concepts across bundles
- **Ranking**: Not documented in README; likely lexical/wikilink-based given OKF v0.1 scope
- **Cross-bundle**: Explicitly supported — search spans all configured bundles (local + remote)
- **Remote bundles**: Fetched via HTTP, parsed, and merged into local graph at startup
- **Refresh**: Not documented; likely requires server restart for index updates

### OKC Search Architecture
- **Index**: Persistent SQLite with FTS5 (full-text) + vector sidecar (embeddings) + graph edges
- **Query interface**: `search` tool with hybrid BM25 + vector (RRF fusion), graph traversal via `traverse`
- **Ranking**: Configurable weights, recency boosting, graph proximity
- **Live updates**: File system watcher triggers incremental re-index
- **Point-in-time**: Not yet supported (planned bitemporal)

### Key Differentiators
| Feature | copperbox/okf-mcp | OKC |
|---------|-------------------|-----|
| **Vector/semantic search** | ❌ Not in v0.1 | ✅ Planned/implemented |
| **Full-text (FTS5)** | ❌ | ✅ |
| **Graph traversal API** | ⚠️ Implicit via links | ✅ Explicit `traverse` tool |
| **Live index updates** | ❌ | ✅ File watcher |
| **Remote bundle search** | ✅ Native | ❌ |
| **Search receipts/audit** | ❌ | ❌ (planned) |

---

## Strengths vs OKC

1. **Native MCP-first architecture** — copperbox/okf-mcp is built exclusively as an MCP server with no CLI alternative. This purity means every design decision optimizes for agent interaction patterns (tools, resources, prompts) rather than human CLI ergonomics. OKC's dual CLI+MCP approach occasionally leaks CLI concepts into MCP tool design.

2. **Remote bundle federation** — The ability to configure URL-based bundles that are fetched, parsed, and merged into the local link graph at startup is a genuine architectural differentiator. This enables distributed knowledge bases where agents can reference authoritative remote catalogs (e.g., a company's central OKF registry) while maintaining local extensions. OKC currently requires all bundles to be local.

3. **Agent onboarding documentation** — The README's "Teaching agents to maintain the brain" section is a thoughtful, practical guide for prompt-engineering agents to use the MCP tools effectively. It covers search strategies, concept creation workflows, and maintenance routines. OKC's documentation targets human operators first.

4. **OKF v0.1 reference implementation** — As one of the few (possibly only) complete OKF v0.1 MCP implementations, copperbox/okf-mcp serves as a valuable compatibility baseline. OKC targeting v0.2 can validate forward-compatibility against this implementation.

5. **TypeScript ecosystem integration** — For teams already invested in Node.js/TypeScript tooling, the server integrates naturally with existing build pipelines, testing frameworks (Vitest/Jest), and deployment targets (Vercel, Cloudflare Workers via MCP HTTP sidecar).

---

## Weaknesses vs OKC

1. **No persistent search index** — The in-memory link graph rebuilds on every server start. For large knowledge bases (10k+ concepts), startup latency and memory pressure become significant. OKC's SQLite+FTS5 index persists across restarts and supports incremental updates via file watching.

2. **No vector/semantic search** — OKF v0.1 doesn't mandate embeddings, and the implementation doesn't add them. OKC's hybrid BM25+vector pipeline enables semantic retrieval that wikilink graphs alone cannot provide (e.g., "concepts about authentication" without exact keyword matches).

3. **No graph traversal tool** — While the link graph exists internally, there's no exposed `traverse` or `graph_path` tool for agents to walk relationships programmatically. OKC's `traverse` tool supports BFS/DFS with depth limits, relationship filters, and path reconstruction.

4. **No lineage or concept history** — Concepts are files; history exists only in Git. OKC's `lineage` tool provides structured concept evolution tracking (splits, merges, renames) that Git alone cannot easily express.

5. **No live update mechanism** — File changes require server restart to re-index. OKC's `observe` tool + fs watcher pushes updates to connected MCP clients in real-time, critical for long-running agent sessions.

6. **Single-threaded Node.js event loop** — CPU-intensive operations (graph algorithms, large bundle parsing) block the event loop. OKC's Rust backend handles parallel indexing and search without blocking MCP request handling.

7. **No tool profiles for security** — Both projects lack runtime tool-profile selection (lean/agent/full), but OKC's architecture makes it easier to add via Rust feature flags and MCP tool annotations.

8. **Limited validation depth** — `validate_bundle` checks structure and links but lacks schema version enforcement, cross-bundle reference integrity, and semantic validation (e.g., circular reference detection). OKC's validator includes schema version gates and graph health checks.

9. **No document ingestion pipeline** — Agents must manually craft Markdown+YAML. OKC's planned `ingest` tool will accept raw documents, auto-chunk, embed, extract entities, and create linked concepts — a significant agent productivity multiplier.

10. **Low project traction** — 1 star, 39 commits, single maintainer. Bus factor risk is high. OKC has broader contributor base and more sustained development velocity.

---

## OKC Improvement Opportunities from copperbox/okf-mcp Patterns

### 1. **Remote Bundle Federation**
Adopt copperbox's remote bundle model for OKC:
- Add `remote_bundles` config section in `okc.toml` with URL, auth headers, refresh interval
- Implement background fetcher that pulls, parses, and merges remote bundles into local catalog
- Expose `bundle_sync` MCP tool for manual/triggered refresh
- Enable cross-bundle references (local concept → remote concept) with validation
- **Benefit**: Distributed knowledge graphs, centralized registries, multi-team catalogs

### 2. **Agent-Centric Documentation & Onboarding**
Create an "Agent Operations Guide" for OKC MCP:
- Document recommended search strategies for different query types (fact lookup, exploration, contradiction hunting)
- Provide prompt templates for concept creation, update, and maintenance workflows
- Include "teaching your agent" section with few-shot examples of effective tool use
- Publish as `AGENTS.md` in repo root for automatic discovery by coding agents
- **Benefit**: Reduces prompt engineering burden, improves agent success rates

### 3. **MCP Resource Templates for Bundle Discovery**
Implement MCP resource templates (currently missing in both):
- `okc://bundle/{bundle_id}` — bundle metadata, concept count, last sync
- `okc://bundle/{bundle_id}/concept/{concept_id}` — direct concept access
- `okc://graph/summary` — live graph statistics
- **Benefit**: Enables resource-based browsing without tool calls, better LLM context management

### 4. **Cross-Bundle Link Validation**
Extend OKC validator to check remote references:
- When `remote_bundles` configured, validate that cross-bundle wikilinks resolve
- Report dangling remote references as warnings (not errors — remote may be temporarily unavailable)
- Add `validate --check-remote` flag for CI/CD gates
- **Benefit**: Catches broken federation links before deployment

### 5. **Bundle Health & Sync Status Resources**
Expose bundle sync state via MCP resources:
- Last successful fetch timestamp, HTTP status, concept count delta
- `okc://bundle/{id}/health` resource for monitoring dashboards
- Alert on stale remote bundles (configurable threshold)
- **Benefit**: Operational visibility for federated deployments

### 6. **TypeScript SDK for Agent Integration**
Publish a lightweight TypeScript client (`@okc/mcp-client`) that:
- Wraps MCP tool calls with TypeScript types matching OKF frontmatter schema
- Provides helper methods: `searchConcepts()`, `traverseGraph()`, `createConcept()`
- Handles connection lifecycle, retries, and error mapping
- **Benefit**: First-class TypeScript agent development experience (matches copperbox's native TS advantage)

### 7. **OKF Version Negotiation**
Implement OKF version detection and negotiation:
- Read `okf_version` from bundle config or infer from frontmatter schema
- Support v0.1 (copperbox-compatible) and v0.2 (OKC-native) simultaneously
- Auto-migrate v0.1 → v0.2 on write with opt-in flag
- **Benefit**: Smooth ecosystem transition, backward compatibility

---

## Threat Level

**Medium**

**Rationale:**
- **Architectural overlap is high** — Both implement OKF + MCP, targeting identical use cases (agent knowledge bases). copperbox/okf-mcp is a direct alternative to OKC's MCP server.
- **Unique differentiator (remote bundles)** — The federation capability addresses a real need (distributed catalogs) that OKC doesn't yet serve. If OKC doesn't adopt this, copperbox captures the multi-team/enterprise segment.
- **Low current traction** — 1 star, 39 commits, single maintainer limits immediate competitive pressure. However, the codebase is functional and documented enough for adoption.
- **Ecosystem alignment** — As an OKF reference implementation, copperbox/okf-mcp benefits from OKF standardization efforts. If OKF gains traction, this implementation rides the wave.
- **Technology stack divergence** — TypeScript vs Rust means different contributor pools. copperbox may attract Node.js teams that find Rust intimidating.

---

## Verdict

**copperbox/okf-mcp** is a clean, MCP-native OKF v0.1 implementation with a compelling federation feature (remote bundles) and excellent agent-onboarding documentation. Its architecture prioritizes agent experience over human CLI workflows — a valid and differentiated stance. However, it lacks persistent indexing, semantic search, graph traversal APIs, live updates, and lineage tracking — all areas where OKC's Rust-based architecture excels.

**Strategic implication for OKC:** The remote bundle federation model is the highest-value adoption target. It solves a genuine architectural gap (distributed catalogs) with relatively low implementation cost. The agent onboarding documentation pattern should be replicated as `AGENTS.md`. Other gaps (vector search, traversal, lineage, live updates) are already on OKC's roadmap and represent areas where OKC can maintain technical leadership.

**Priority adoption order:**
1. **Remote bundle federation** — Config-driven, background sync, cross-bundle validation (Q1)
2. **Agent Operations Guide (`AGENTS.md`)** — Prompt templates, workflow examples, few-shot tool use (immediate)
3. **MCP Resource Templates** — Bundle/concept/graph resources for resource-based browsing (Q1)
4. **Cross-bundle link validation** — Extend validator for federation integrity (Q1)
5. **TypeScript MCP client SDK** — Reduce integration friction for TS/JS agents (Q2)
6. **OKF version negotiation** — v0.1 compatibility layer for ecosystem bridge (Q2)
7. **Bundle health resources** — Operational observability for federated deployments (Q2)

The competitor is not an existential threat but a useful reference implementation that validates OKF+MCP as a viable pattern and highlights federation as an underserved capability.