# OKF Ecosystem — Competitor Assessment Index

> **⚠️ IMPORTANT DISCLAIMER:** There is **no coordinated OKF ecosystem**. These are **6 independent crates by 6 different authors** on crates.io. No shared governance, no coordinated releases, no technical integration between them. The only "ecosystem" is the [okf.md/tools/](https://okf.md/tools/) human-curated directory (OWOX blog post) that lists them alongside other tools. Each crate stands alone.

---

## Individual Assessment Files

Each tool has its own detailed assessment (170-230 lines) with feature-comparison tables, architecture analysis, MCP capability inventories, strengths/weaknesses vs OKC, improvement opportunities, threat levels, and strategic verdicts.

| Tool | Author | Type | Threat | Assessment File |
|------|--------|------|--------|-----------------|
| **okf** | W4G1 | Rust library + CLI (spec reference) | **Low** (symbiotic) | [`okf-w4g1-assessment.md`](okf-w4g1-assessment.md) |
| **okf-http** | kathrinmotzkus | HTTP server + Web UI + REST API | **Medium** | [`okf-http-assessment.md`](okf-http-assessment.md) |
| **okf-open-knowledge-format** | kathrinmotzkus | Core document model library | Low (workspace sibling) | *See okf-http assessment* |
| **okq** | mikevalstar | Tantivy search CLI + library | **Medium** | [`okq-assessment.md`](okq-assessment.md) |
| **copperbox/okf-mcp** | copperbox | TypeScript MCP server (OKF v0.1) | **Medium** | [`copperbox-okf-mcp-assessment.md`](copperbox-okf-mcp-assessment.md) |
| **travisjakel/okf-ingest + okf-mcp** | travisjakel | Python ingestion (DuckDB) + MCP server | **Medium** | [`travisjakel-okf-ingest-assessment.md`](travisjakel-okf-ingest-assessment.md) |
| **hermes-okf** | EliaszDev | Hermes agent memory (hot/cold + Git) | **Low** (specialized) | [`hermes-okf-assessment.md`](hermes-okf-assessment.md) |
| **okf-lint** | rpmoore | OKF bundle linter | Low (complementary) | *Brief note below* |
| **okf-cli** | raimannma | General CLI (not OKF-specific) | **None** | *Not an OKF tool* |
| **GoogleCloudPlatform/knowledge-catalog** | Google | OKF spec reference + cloud platform | **None** (upstream) | *Not a competitor* |

---

## Independent Crates Comparison

These crates share **only** the OKF name and format. They do not depend on each other (except kathrinmotzkus's two crates which share a workspace).

| Crate | Author | Downloads | Stars | Commits | Rust Version | Reality |
|-------|--------|-----------|-------|---------|--------------|---------|
| `okf` | W4G1 | 112 | 7 | 25 | Stable | **Only zero-dep spec impl**; OKC depends on it |
| `okf-open-knowledge-format` | kathrinmotzkus | 77 | 0 | 36* | 1.88+ | Workspace lib for okf-http |
| `okf-http` | kathrinmotzkus | 105 | 0 | 36* | **1.88+ (bleeding)** | HTTP server, 85+ deps, prebuilt .deb |
| `okf-cli` | raimannma | 22 | 0 | — | Stable | **NOT OKF-specific** — naming collision |
| `okf-lint` | rpmoore | 62 | 0 | — | Stable | Linter only, custom license |
| `okq` | mikevalstar | 147 | 0† | — | Stable | Tantivy search, 9 versions, private org |

\* Shared workspace commits  
† Private org (trustpublish), stars not public

---

## What "Ecosystem" Actually Exists

1. **okf.md/tools/ directory** — Human-curated list (OWOX blog). Lists: copperbox/okf-mcp, travisjakel/okf-mcp, travisjakel/okf-ingest, hermes-okf, GCP knowledge-catalog, OKC. No technical integration.

2. **kathrinmotzkus workspace** — The only actual coordination: `okf-open-knowledge-format` (lib) + `okf-http` (server) in one Cargo workspace.

3. **OKC uses `okf` crate** — OKC depends on W4G1's `okf` for parsing/validation. This is a **dependency relationship**, not ecosystem coordination.

---

## Threat Summary (Per Independent Crate)

| Crate | Threat | Primary Concern |
|-------|--------|-----------------|
| **okf (W4G1)** | **Low** | Symbiotic — OKC *depends* on it for parsing/validation |
| **okf-http** | **Medium** | Human-facing UI + packaging; could add MCP |
| **okq** | **Medium** | Best pure search (Tantivy); library embeddable |
| **copperbox/okf-mcp** | **Medium** | MCP-native + remote bundle federation |
| **travisjakel stack** | **Medium** | Concept-first model + agent-oriented MCP tools |
| **hermes-okf** | **Low** | Specialized for Hermes; architectural lessons for OKC |
| **okf-lint** | **Low** | Complementary — validates OKC bundles |
| **okf-cli** | **None** | Not an OKF tool (naming collision) |
| **GCP knowledge-catalog** | **None** | Upstream spec owner |

---

## Strategic Recommendations for OKC (Consolidated)

### From okf (W4G1) — Upstream Dependency
1. **Adopt provenance model** — Add `Attestation`/`TrustRoot`/`VerificationResult` to front-matter; implement `verify` MCP tool
2. **Align link model with `Relation`** — Support typed links (`[[type:target]]`); map to okf's `Relation`
3. **Expose okf CLI via OKC** — Add `okc bundle parse|validate|graph|trust` subcommands
4. **Track okf versions tightly** — Pin in `Cargo.toml`; test against each release

### From okf-http — Human-Facing Gap
5. **Add optional web UI** — `okc serve --web` with embedded SPA for browse/search
6. **Publish prebuilt binaries** — GitHub Releases, Homebrew tap, Scoop bucket
7. **Optional TLS for MCP HTTP** — `okc serve --http --tls` with auto-certs
8. **Evaluate `okf-open-knowledge-format` dependency** — For spec compliance

### From okq — Search Excellence
9. **Faceted search in `search` tool** — Return facet counts inline (concept_type, tags, bundles)
10. **Public search library** — Expose `okc::search` as crate for embedding in linters/CI
11. **Optional Tantivy backend** — Feature flag for large catalogs (>50k docs)
12. **Search receipts/audit** — Provenance for agent trust (first-mover advantage)

### From copperbox/okf-mcp — Federation & Agent UX
13. **Remote bundle federation** — Config-driven background sync, cross-bundle validation
14. **Agent Operations Guide (`AGENTS.md`)** — Prompt templates, workflow examples, few-shot tool use
15. **MCP Resource Templates** — `okc://bundle/{id}`, `okc://graph/summary` for resource-based browsing
16. **TypeScript MCP client SDK** — `@okc/mcp-client` for TS/JS agent integration

### From travisjakel stack — Concept-First Architecture
17. **Wikilinks + concept extraction** — Parse `[[concept]]`; populate `concepts` table with type/def/aliases
18. **Context pack tool** — `search` + `traverse` + metadata with `max_tokens` budget
19. **Impact analysis tool** — Reverse dependency traversal from concept/file
20. **Bundle versioning + diff** — `version` front-matter; snapshot index; `diff` tool
21. **Health diagnostics** — Extend `validate` with orphan concepts, broken wikilinks, stale chunks
22. **SQL escape hatch** — Read-only `query_sql` against allowlisted tables

### From hermes-okf — Agent Memory Primitives
23. **Session memory tier + snapshots** — Hot (ephemeral) + cold (persistent) with explicit `snapshot`/`restore`
24. **Git-tag index snapshots** — Leverage Git for free versioning
25. **Agent checkpoint MCP tool** — `checkpoint`/`resume` for session continuity
26. **Hermes compatibility profile** — Tool subset for migration path

---

## Notes

- **okf-lint (rpmoore)** — CLI linter validating OKF bundles + Markdown hygiene. 62 downloads, 2 versions, 2.2k LoC, custom license. Complementary: validates OKC's own bundles. No MCP, no search, no competition.
- **okf-cli (raimannma)** — "Cross-platform command-line tool" with optional `toon-format` feature. 22 downloads, 1 version, 8.8k LoC, MIT. **Not OKF-specific** — naming collision only. No competitive relevance.
- **GoogleCloudPlatform/knowledge-catalog** — 7.8k stars, 650 forks. Contains the OKF spec in `okf/` directory. Cloud metadata platform, not an OKF tool competitor. Upstream dependency.

---

*This index links to individual assessment files. See each file for detailed analysis, tables, and strategic recommendations.*