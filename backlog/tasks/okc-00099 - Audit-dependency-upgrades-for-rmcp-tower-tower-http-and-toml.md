---
id: OKC-00099
title: 'Audit dependency upgrades for rmcp, tower, tower-http, and toml'
status: Done
assignee:
  - '@felix'
created_date: '2026-07-29 18:48'
updated_date: '2026-08-07 17:29'
labels:
  - deps
  - audit
  - rust
dependencies: []
references:
  - Cargo.toml
  - Cargo.lock
modified_files:
  - Cargo.toml
priority: medium
type: spike
ordinal: 73000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Review the current dependency pins and determine which upgrades are safe to take now versus which require a planned migration. Focus on rmcp, tower, tower-http, and toml, since they affect MCP transport, HTTP plumbing, and config parsing.

The audit should produce a concrete compatibility matrix and migration order so the follow-up implementation task can be scoped without guesswork.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Document the latest available compatible release for each target dependency and note whether it is a patch, minor, or major jump
- [x] #2 Call out the API or behavior changes that matter to OKC for each major upgrade candidate
- [x] #3 Recommend a safe upgrade order and identify any dependencies that should move together
- [x] #4 Record whether the current test suite is sufficient to validate each upgrade path or whether new coverage is needed
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inventory current pins (rmcp=2.2.0, tower=0.4.13, tower-http=0.6.11, toml=0.8.23), all major jumps to latest.
2. Capture upstream breaking changes via release notes/CHANGELOGs for each.
3. Map each change to OKC usage sites (src/transport/mcp/mod.rs, src/config.rs, tests).
4. Determine safe upgrade order and test-suite sufficiency; record matrix + order.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Compatibility matrix (current -> latest OKC-candidate)
- rmcp    =2.2.0 (exact pin) -> 3.1.1  = MAJOR
- tower   0.4.13 -> 0.5.3               = MAJOR (but OKC does NOT use tower directly)
- tower-http 0.6.11 -> 0.7.0            = MAJOR (but OKC does NOT use tower-http directly)
- toml    0.8.23 -> 1.1.4               = MAJOR (low risk; only high-level API used)

## Key finding: rmcp + tower/tower-http are coupled via server handler types
OKC only touches the `tower` layer through rmcp re-exports:
src/transport/mcp/mod.rs:35  transport::streamable_http_server::tower::{StreamableHttpServerConfig, StreamableHttpService}
grep over src/ confirms NO direct use of TraceLayer/CorsLayer/any tower_http import.
=> tower="0.4" and tower-http="0.6" in Cargo.toml (features cors,trace) are direct-but-unused deps.
   They are almost certainly leftover. The real constraint is rmcp's own tower/tower-http.

## rmcp 3.0 breaking changes (adopts MCP 2026-07-28 sessionless protocol) - AC2
1. SESSIONLESS HTTP: no Mcp-Session-Id, no standalone GET stream, no DELETE terminate, no Last-Event-ID.
   RMCP creates a fresh handler per request; persistent state must live OUTSIDE the handler.
   `with_stateful_mode(...)` is RENAMED to `with_legacy_session_mode()` and now only gates legacy protocol versions.
   => OKC src/transport/mcp/mod.rs:146  .with_stateful_mode(true)  MUST become .with_legacy_session_mode().
2. Lifecycle: initialize/notifications/initialized removed. Each request carries protocol version in _meta;
   server advertises server/discover. Clients opt in via serve_with_lifecycle / ClientLifecycleMode::Discover|Auto.
   serve() path remains for legacy.
3. Handler returns MRTR-aware enums: ServerHandler::call_tool/get_prompt/read_resource now return
   InputRequiredResult-aware response types. Manual handler impls + exhaustive ServerResult matches must handle it.
   OKC uses #[tool] macros -> codegen handles this; verify no manual ServerHandler impl in src/.
4. MSRV -> 1.88 (OKC rust-version 1.95, unaffected).
5. 3.1.0 added strict stateless protocol metadata validation + SEP-2260 stream request association.
   3.1.1: MRTR state exposed to tool handlers; async-trait made optional.

## tower / tower-http breaking changes (AC2) - largely NOT OKC's problem
tower-http 0.6->0.7: compression Accept-Encoding wildcard/identity now 406; SizeAbove threshold u16->u64;
removed implicit no-op tokio/async-compression features; GrpcCode/GrpcFailureClass now #[non_exhaustive],
GrpcStatus exported; follow-redirect extensions forwarding change; MSRV->1.65.
tower 0.4->0.5: MSRV->1.64. OKC paths (cors/trace) unaffected because OKC never calls them directly.
Impact is only via rmcp 3.x's pinned tower/tower-http (resolve those transitively; rmcp is source of truth).

## toml 0.8->1.x (AC2) - LOW RISK
0.9.0: new parser/writer; low-level Serializer::new/pretty now take &mut Buffer (preserve_order feature needed);
not the high-level from_str/to_string_pretty OKC uses. 1.0.0: Time::second/nanosecond -> Option;
borrowed &str deserialization. 1.1.0 MSRV->1.85. Latest 1.1.4 (datetime-preserving fix).
OKC usage sites: src/config.rs 203-206 (toml::de::Error / toml::ser::Error), ~237 toml::from_str, ~567 toml::to_string_pretty.
Only residual risk: exact to_string_pretty output formatting may shift; verify config round-trip snapshots/tests.

## Test-suite sufficiency (AC4)
tests/mcp_e2e_tests.rs (8 tests) launches the packaged okc binary over STDIO transport and drives it
with an rmcp CLIENT (ClientHandler / RoleClient / .serve() / client.call_tool(params)).
=> (a) The test code ITSELF uses rmcp client API and must be rewritten for rmcp 3.x
     (client lifecycle now requires serve_with_lifecycle / lifecycle mode; call_tool returns MRTR-aware Response).
=> (b) NO HTTP transport test exists: serve_http / StreamableHttpService / the with_stateful_mode->legacy rename
     have ZERO coverage. NEW coverage needed to validate the sessionless HTTP path + handler state boundary.
tests/integration_tests.rs (20 tests) hit the service layer directly (not MCP) -> good toml/config coverage
(e.g. test_service_rejects_invalid_configuration_before_opening_storage), helps validate toml round-trip.

## Upgrade order (AC3)
1. toml 0.8->1.x FIRST (independent, low risk): update toml dep; run cargo test incl. integration_tests config
   round-trip; grep to_string_pretty snapshot diffs.
2. rmcp 2.2->3.x TOGETHER with tower/tower-http since OKC's tower surface is rmcp's re-export:
   - fix src/transport/mcp/mod.rs .with_stateful_mode->.with_legacy_session_mode
   - remove or bump the unused direct tower/tower-http deps (drop them in same change after confirming
     nothing else uses them; rely on rmcp's transitive ones)
   - rewrite tests/mcp_e2e_tests.rs client side; add an HTTP-transport test to cover stateful->legacy rename.
   Run cargo test, fmt, clippy.

## Next actions for OKC-00100 (migration impl)
- Confirm no manual ServerHandler impl in src/ (macros only).
- Add HTTP e2e/session test coverage (missing today).
- Decide state relocation when exposing new sessionless protocol.

Correction to previous note: upgrade order item 2 should read 'rmcp 3.x TOGETHER with tower/tower-http' (not 'towercase'). The principle is unchanged: rmcp and the tower layer move as one unit since OKC uses rmcp's tower re-export.

Verified: src/transport/mcp/mod.rs:542 is an EMPTY , with enum at line 174 annotated #[tool_router]. So MRTR-aware return handling is fully macro-generated (tool_router / tool_handler) -> no manual handler exhaustiveness work needed in src/. Only the transport wiring + tests require edits.

Correction: the previous 'Verified' note had text dropped by a shell quoting issue. Corrected text: verified at src/transport/mcp/mod.rs:542 that the ServerHandler impl block is an EMPTY impl (no manual handler methods, e.g. 'impl ServerHandler for McpServer {}'), with the enum at line 174 annotated with the #[tool_router] macro. MRTR-aware return handling is fully macro-generated via tool_router/tool_handler; no manual handler exhaustiveness work in src/. Only transport wiring + tests need edits.
<!-- SECTION:NOTES:END -->

## Comments

<!-- COMMENTS:BEGIN -->
author: @felix-agent
created: 2026-08-07 17:21
---
Starting dependency audit for rmcp, tower, tower-http, toml
---
<!-- COMMENTS:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Audit complete for the 4 target deps. All are major jumps to latest (verified against crates.io): rmcp 2.2.0->3.1.1, tower 0.4.13->0.5.3, tower-http 0.6.11->0.7.0, toml 0.8.23->1.1.4. KEY finding: OKC never imports tower_http/CorsLayer/TraceLayer directly (grep over src, tests, build returned nothing) and tower is touched only through rmcp's re-export streamable_http_server::tower (src/transport/mcp/mod.rs:35) — so tower='0.4' and tower-http='0.6' are direct-but-unused deps; rmcp's transitive versions are the real constraint. rmcp 3.x adopts MCP 2026-07-28 sessionless protocol: with_stateful_mode->with_legacy_session_mode rename (breaks src/transport/mcp/mod.rs:146), handler returns become MRTR-aware enums (mitigated: src line 542 is an empty macro-based impl), MSRV 1.88. toml 0.8->1.x low risk (OKC only uses high-level from_str/to_string_pretty at src/config.rs 237/567; low-level Serializer/Time changes don't apply) — only risk is to_string_pretty formatting shifts. Upgrade order: (1) toml first, then (2) rmcp+tower+tower-http moved together (drop the unused direct tower/tower-http deps, fix the stateful_mode rename, rewrite the rmcp client in tests/mcp_e2e_tests.rs which today uses the old client lifecycle API, and ADD an HTTP-transport test — none exists, only stdio; integration_tests.rs already gives good toml/config roundtrip coverage). Verified via: crates.io max_version lookup, grep over src/tests/build, code reads of src/transport/mcp/mod.rs and src/config.rs, rmcp release notes + toml CHANGELOG. Recorded full matrix + migration order in task. Implementation to be scoped in OKC-00100.
<!-- SECTION:FINAL_SUMMARY:END -->
