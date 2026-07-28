# Competitor Assessment: okf-http (kathrinmotzkus)

## Overview

**okf-http** (crates.io: `okf-http`, v0.4.4, 105 downloads across 8 versions, https://github.com/kathrinmotzkus/open-knowledge-format) — HTTP server and web UI for browsing and managing OKF repositories. Built as a Rust CLI binary (`okf-http`) with embedded Axum web server, SQLite persistence (rusqlite), Argon2 password authentication, and built-in TLS via rustls. Provides REST API for programmatic access and a browser-based UI for document browsing, search, and repository management. Packaged as prebuilt `.deb` packages with nightly CI builds for Linux x86_64/aarch64 and macOS x86_64/aarch64 (Windows experimental). Requires Rust 1.88+ (bleeding edge).

**okf-open-knowledge-format** (crates.io: `okf-open-knowledge-format`, v0.4.0, 77 downloads, 4 versions) — Core document model library (~7,997 LoC Rust) defining the OKF v0.2 specification: `Document`, `Concept`, `Relation`, `Tag`, `Metadata` types with JSON/TOML/YAML serialization. Library-only crate, no CLI or server. Apache-2.0 license. Shared workspace with okf-http (36 total commits across both crates).

Both are **Rust-based OKF implementations** — okf-http provides the HTTP/REST interface and web UI that OKC lacks, while okf-open-knowledge-format provides the canonical document model. Neither exposes MCP; both use REST APIs, making them fundamentally different integration targets for AI agents compared to OKC's MCP-native architecture.

---

## Feature Comparison with OKC

| Capability | okf-http | okf-open-knowledge-format | OKC | Notes |
|------------|----------|---------------------------|-----|-------|
| **Protocol** | REST API + Web UI | Library only (no protocol) | MCP (stdio/HTTP/SSE) | okf-http: REST only; OKC: agent-native MCP |
| **Document model** | Uses okf-open-knowledge-format crate | Defines OKF v0.2 spec (Document, Concept, Relation, Tag) | OKF v0.2 compatible (markdown + front-matter) | Shared model via library |
| **Storage** | SQLite (rusqlite) persistent | N/A (library) | SQLite + FTS5 (r2d2 + rusqlite) | Both SQLite; OKC adds FTS5 |
| **Search** | REST endpoint (SQLite LIKE/basic) | N/A | FTS5/BM25 full-text + metadata filters | okf-http: basic; OKC: ranked FTS5 |
| **Graph traversal** | REST endpoint (relations) | N/A | MCP `traverse` (BFS, depth/node limits) | okf-http: relation API; OKC: link-graph BFS |
| **Authentication** | Password login (Argon2), session cookies | N/A | None (local-first) | okf-http: web auth; OKC: no auth |
| **Transport** | HTTP/HTTPS (Axum + rustls) | N/A | stdio, HTTP/SSE (MCP) | okf-http: REST; OKC: MCP |
| **Web UI** | ✅ Browser-based browse/search/manage | ❌ | ❌ | okf-http unique advantage |
| **File watching** | ❌ (polling via REST) | N/A | ✅ notify crate (live index updates) | OKC: live; okf-http: manual refresh |
| **Packaging** | Prebuilt .deb, nightly CI | crates.io library | Single binary (cargo install) | okf-http: distro packages |
| **Platforms** | Linux x86_64/aarch64, macOS x86_64/aarch64, Win exp. | All Rust targets | All Rust targets (single binary) | okf-http: prebuilt focus |
| **Rust version** | 1.88+ (bleeding edge) | 1.88+ | Stable (MSRV policy) | okf-http: risky for prod |
| **MCP tools** | ❌ | ❌ | 11 tools (search, traverse, scan, validate, etc.) | OKC only MCP-native |
| **Agent integration** | REST client required | Library consumer | Native MCP (Claude, Cursor, etc.) | Fundamental architecture diff |
| **Code indexing** | ❌ | ❌ | ❌ (markdown only) | Neither does code graph |
| **Vector/semantic search** | ❌ | ❌ | ❌ (BM25 only) | Both lack embeddings |
| **Multi-repo** | Single repo per server | N/A | Single roots config | Similar limitation |
| **Observability** | tracing logs | N/A | tracing logs only | Both minimal |
| **License** | Apache-2.0 | Apache-2.0 | Apache-2.0 | Aligned |

---

## Architecture & Code Quality

### okf-http
- **Structure**: Single binary crate (`okf-http`) depending on workspace sibling `okf-open-knowledge-format`. ~16,500 LoC Rust + ~3,000 LoC JS/CSS/HTML for embedded web UI.
- **Dependencies**: 85+ crates (axum, axum-server, tokio, rusqlite, argon2, rustls, serde, serde_json, sha2, tower-http, tracing, and transitive). Heavy dependency tree vs OKC's lean approach.
- **Database**: SQLite via `rusqlite` (blocking). No connection pooling visible (r2d2 not used). Schema migrations manual.
- **Async**: Tokio + Axum. Web server runs on dedicated thread pool. Blocking DB calls on `tokio::task::spawn_blocking`.
- **Auth**: Argon2 password hashing, session cookies, HTTPS via rustls (auto-generated certs or user-provided).
- **Web UI**: Embedded static assets (HTML/CSS/JS) served by Axum. SPA-style browsing with REST calls.
- **Testing**: No visible test suite in repository (no `tests/`, no `#[cfg(test)]` modules found in 36 commits).
- **Quality gates**: GitHub Actions CI (build, test?, package .deb). No `deny.toml`, `clippy.toml`, or `rustfmt.toml` visible.
- **Observability**: `tracing` structured logging only. No metrics, no OTLP, no Prometheus.
- **Maturity**: v0.4.4, 8 versions since Jul 2026, 105 total downloads, 0 stars, 0 forks, single maintainer. Prebuilt .deb packages indicate deployment maturity but zero community traction.

### okf-open-knowledge-format
- **Structure**: Single library crate, ~7,997 LoC. Clean domain model: `Document`, `Concept`, `Relation`, `Tag`, `Metadata`, `ConceptType`, `RelationType` enums. Serialization via `serde` (JSON, TOML, YAML).
- **Dependencies**: Minimal — `serde`, `serde_json`, `toml`, `yaml-rust`, `thiserror`, `uuid`, `chrono`. Lean compared to okf-http.
- **Testing**: No visible tests.
- **Quality gates**: Same CI as okf-http (shared workspace).
- **Maturity**: v0.4.0, 4 versions, 77 downloads. Library-only, no binary.

### OKC
- **Structure**: Single binary crate with modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport` (CLI + MCP). ~8k LoC est.
- **Dependencies**: Lean — `rusqlite`, `r2d2`, `tantivy`/`sqlite-fts5` for FTS, `notify` for file watching, `rmcp` for MCP, `clap`, `serde`, `tracing`. ~20 direct deps.
- **Database**: SQLite + FTS5 via `r2d2` pool. WAL mode. Schema migrations embedded.
- **Async**: Minimal Tokio (MCP transport only). Core indexing synchronous.
- **File watching**: `notify` crate for live index updates (key differentiator).
- **Testing**: Basic `cargo test` (unit + integration).
- **Quality gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- **Observability**: `tracing` logs only.
- **Maturity**: Pre-1.0, active development, single binary distribution.

---

## MCP Capability Inventory

**Not applicable — okf-http exposes a REST API, not MCP.** This is a fundamental architectural difference:

| Aspect | okf-http | OKC |
|--------|----------|-----|
| **Protocol** | REST (OpenAPI-ish, undocumented) | MCP (stdio/HTTP/SSE) |
| **Agent integration** | Custom HTTP client required | Native (Claude, Cursor, VS Code, any MCP client) |
| **Tool discovery** | Manual (read API docs/source) | `tools/list` automatic |
| **Authentication** | Session cookies (browser) / Bearer (API) | None (local-first) |
| **Streaming** | ❌ | ✅ (SSE for long-running tools) |
| **Resources** | ❌ | ❌ (planned) |
| **Prompts** | ❌ | ❌ (planned) |

**Implication**: okf-http cannot be used directly by MCP-capable agents. An adapter layer (REST→MCP bridge) would be required, adding latency and complexity. OKC's MCP-native design is a strategic advantage for agent ecosystems.

---

## Strengths vs OKC

1. **Web UI for human users** — okf-http provides a complete browser-based interface for browsing, searching, and managing OKF repositories. OKC has no UI (CLI + MCP only). This makes okf-http immediately usable by non-technical stakeholders.

2. **Prebuilt distribution packages** — `.deb` packages with nightly CI builds for Linux/macOS (x86_64/aarch64). OKC distributes only via `cargo install` or manual binary download. okf-http is easier to deploy on servers without Rust toolchain.

3. **Built-in HTTPS/TLS** — `rustls` integration with auto-generated certificates enables secure-by-default deployment. OKC's MCP HTTP transport requires external TLS termination (nginx, Cloudflare, etc.).

4. **Authentication system** — Argon2 password hashing + session cookies provides basic access control for multi-user scenarios. OKC is single-user/local-first with no auth.

5. **Canonical OKF model library** — `okf-open-knowledge-format` crate is the reference implementation of the OKF v0.2 spec. OKC implements a compatible but independent parser. Using the canonical library ensures spec compliance.

6. **REST API for integration** — While not MCP, a documented REST API enables integration with traditional HTTP clients, webhooks, and non-MCP tooling. OKC requires MCP client.

7. **Multi-platform prebuilt binaries** — Nightly builds for 4 target triples (Linux x86_64/aarch64, macOS x86_64/aarch64). OKC relies on `cargo install` compiling locally.

---

## Weaknesses vs OKC

1. **No MCP support** — Fundamental architectural gap. Cannot be used directly by AI agents (Claude, Cursor, etc.) without a bridge. OKC is agent-native.

2. **Bleeding-edge Rust requirement (1.88+)** — okf-http requires Rust 1.88 (released Jul 2026), which is not yet in stable distros (Debian stable, Ubuntu LTS, RHEL). OKC targets stable Rust (MSRV policy), making it deployable everywhere today.

3. **Heavy dependency footprint (85+ crates)** — Large attack surface, longer compile times, more supply chain risk. OKC's ~20 deps are auditable and minimal.

4. **No live file watching** — Index updates require manual REST call or polling. OKC's `notify`-based watcher provides instant index freshness on file save.

5. **No visible test suite** — 36 commits, 0 tests found. OKC has basic unit/integration tests. High risk for regressions.

6. **Zero community traction** — 0 stars, 0 forks, 105 downloads, single maintainer. OKC has active development (this repo). Bus factor = 1.

7. **Basic search only** — SQLite `LIKE`/simple queries vs OKC's FTS5/BM25 ranked full-text with filters (path prefix, concept type, tags).

8. **No MCP resources/prompts** — Cannot expose `okc://index/status` style resources or retrieval prompt templates for agents.

9. **Blocking SQLite calls** — Uses `spawn_blocking` for all DB ops. OKC uses `r2d2` connection pool with synchronous calls on dedicated threads (similar but more controlled).

10. **Single-repo architecture** — One server = one repository. OKC's `roots` config supports multiple root directories (though single logical index).

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Web UI** | No human-facing browser interface | okf-http: embedded SPA with browse/search/manage | Add optional `okc serve --web` with embedded UI (Axum + static assets) for human operators |
| **Prebuilt packages** | `cargo install` only | okf-http: .deb nightly CI builds for 4 targets | Add GitHub Actions workflow for `cargo deb` / `cargo bundle` / binary tarballs per release |
| **HTTPS/TLS** | External termination required | okf-http: built-in rustls auto-certs | Add `--tls` flag to `okc serve --http` with `rustls` + `rcgen` for dev certs |
| **Authentication** | None (local-first) | okf-http: Argon2 + session cookies | Optional `--auth` flag for multi-user deployments (basic auth or OIDC) |
| **REST API** | MCP only | okf-http: REST endpoints for CRUD/search | Add optional REST gateway mode (`okc serve --rest`) for non-MCP integrations |
| **Canonical OKF model** | Independent parser | okf-open-knowledge-format: reference crate | Evaluate depending on `okf-open-knowledge-format` crate for spec compliance |
| **Distribution simplicity** | Requires Rust toolchain | okf-http: prebuilt .deb, no Rust needed | Publish static binaries to GitHub Releases; add Homebrew tap, Scoop bucket |
| **Multi-platform CI** | Linux only (assumed) | okf-http: Linux x86_64/aarch64, macOS x86_64/aarch64 | Expand CI matrix to match; test aarch64 Linux + macOS |
| **Spec compliance** | Self-validated | okf-open-knowledge-format: canonical types | Add conformance tests against OKF v0.2 test suite (if exists) |

---

## Threat Level

**Medium**

**Rationale**: okf-http is not a direct competitor for *agent-facing* knowledge catalogs (no MCP), but it **is** a competitor for *human-facing* OKF repository management. Its web UI, prebuilt packages, built-in TLS, and auth make it a viable "OKF server" for teams wanting a traditional web app. If okf-http adds MCP support (trivial architecturally — Axum + rmcp), it becomes a direct threat. The zero-community-traction and bleeding-edge Rust requirement currently limit adoption, but the packaging maturity suggests serious intent. OKC's MCP-native architecture is a defensible moat *only while* MCP remains the dominant agent protocol.

---

## Verdict

**okf-http** is the **human-facing counterpart** to OKC's **agent-facing** design. They share the same OKF v0.2 document model (via `okf-open-knowledge-format`) but diverge on protocol: REST + Web UI vs MCP + CLI. This is a **complementary niche**, not a zero-sum conflict — *unless* okf-http adds MCP support.

**OKC's competitive advantages** (agent-native MCP, live file watching, FTS5 search, lean deps, stable Rust, single binary) are strong for AI-assisted workflows. **okf-http's advantages** (web UI, .deb packages, built-in TLS, auth, prebuilt binaries) are strong for traditional server deployment and human operators.

**Recommended priority actions for OKC**:
1. **Add optional web UI** (`okc serve --web`) — closes the human-operator gap. Embed a minimal SPA (Axum + static files) for browse/search. Leverage existing MCP tools via internal calls.
2. **Publish prebuilt binaries** — GitHub Releases with `cargo dist` or manual `cargo build --release` artifacts for Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64. Add Homebrew tap.
3. **Optional TLS for MCP HTTP** — `okc serve --http --tls` with auto-generated certs (rcgen) for dev, user certs for prod.
4. **Evaluate `okf-open-knowledge-format` dependency** — If the crate stabilizes, depending on it ensures spec compliance and reduces parser maintenance burden.
5. **Monitor okf-http for MCP adoption** — If they add `rmcp` support, OKC's protocol moat evaporates. Accelerate MCP resources/prompts (planned) to deepen agent integration.

**Strategic position**: OKC wins for **AI agent workflows** (MCP-native, live indexing, ranked search). okf-http wins for **human server deployments** (web UI, packages, TLS, auth). The markets overlap at "team knowledge base" — OKC should capture the AI-first segment; okf-http captures the traditional-web-app segment. Coexistence is likely unless one crosses the protocol boundary.