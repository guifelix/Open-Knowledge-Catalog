---
id: OKC-00033
title: MAANG-level code quality remediation
status: To Do
assignee: []
created_date: '2026-07-24 18:20'
updated_date: '2026-07-24 18:20'
labels: []
dependencies: []
references:
  - docs/implementation-plan.md
  - docs/language-comparison.md
  - docs/library-analysis.md
priority: high
type: feature
ordinal: 25000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Comprehensive code quality remediation based on systematic MAANG-level assessment across 7 dimensions (architecture, correctness, performance, testing, code quality, documentation, security).

## Assessment Summary

| Dimension | Score | Critical Issues |
|-----------|-------|-----------------|
| Architecture & Design | 4/5 | RepositoryIndex god-struct, dead code, non-typed includes |
| Correctness & Safety | 4/5 | SQL injection (OKC-00001), thread safety (OKC-00002), stringly-typed errors |
| Performance | 4/5 | FTS5 ranking (OKC-00003), full re-scan (OKC-00005), no batch operations |
| Testing & CI | 4/5 | No CI pipeline, no MCP E2E tests, no mutation testing |
| Code Quality & Maintenance | 3.5/5 | Dead code, unwrap in prod, no linting config, sparse module docs |
| Documentation | 3/5 | Outdated README (OKC-00032), no rustdoc, no architecture diagram |
| Security | 3/5 | SQL injection, no path traversal validation, no dep scanning |

## Existing Tasks Covering Issues

Already tracked (HIGH priority):
- OKC-00001: SQL injection
- OKC-00002: Thread safety
- OKC-00003: FTS5 ranking
- OKC-00005: Incremental scan
- OKC-00006: Input validation
- OKC-00015: OpenTelemetry
- OKC-00022: JSON output
- OKC-00026: Structured errors

Already tracked (MEDIUM priority):
- OKC-00007: Link resolution edge cases
- OKC-00008: YAML aliases
- OKC-00025: Remove unused Tokio

Already tracked (LOW priority):
- OKC-00010: Config file support
- OKC-00011: Shell completions
- OKC-00013: Semantic embeddings
- OKC-00030: Organize docs
- OKC-00031: Split service
- OKC-00032: Update README

## Assessment Gaps (Not Yet Tracked)

The assessment identified these issues NOT covered by existing tasks:

1. CI pipeline (GitHub Actions): lint, test, bench, fuzz on every PR
2. RepositoryIndex god-struct refactor (storage trait extraction remains partial)
3. Rustfmt/clippy config committed to repo
4. Path traversal validation in link resolver
5. MCP server E2E tests
6. Architecture documentation / ADRs
7. Remove pervasive unwrap() from production code
8. Module-level doc comments
9. Dependency vulnerability scanning (cargo audit)
10. Mutation testing (cargo-mutants)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Assessment covers all 7 MAANG code review dimensions with traceable evidence
- [ ] #2 Every finding is either linked to an existing backlog task or flagged as a new gap
- [ ] #3 Prioritized execution plan sequences work by risk reduction value
- [ ] #4 Plan accounts for ordering dependencies between remediation tasks
- [ ] #5 New gap tasks are created for issues not yet tracked
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment report presented to user
- [ ] #2 Gap analysis cross-referenced against all 32 existing backlog tasks
- [ ] #3 Execution plan committed as task plan
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Prioritized Execution Plan

The plan is organized in 4 phases. Each phase must complete before the next begins. Within a phase, tasks can run in parallel unless noted.

### Phase 1: Critical Safety (risk-reduction, ship first)
*Ordering: any within phase, but all before Phase 2*

- OKC-00001 — Fix SQL injection in query_metadata [HIGH, bug, depends: nothing]
- OKC-00033.03 — Add path traversal validation to link resolver [HIGH, bug, depends: nothing]
- OKC-00033.08 — Add cargo-audit dependency scanning [MEDIUM, chore, depends: OKC-00033.01 for CI]

### Phase 2: Core Correctness & Infrastructure
*Ordering: CI first (needed for quality gates), then parallel*

1. OKC-00033.01 — Add CI pipeline (GitHub Actions) [HIGH, feature]
2. (parallel after CI is green):
   - OKC-00002 — Thread safety (RefCell → Mutex/RwLock) [HIGH, feature]
   - OKC-00006 — Input validation and size limits [HIGH, feature]
   - OKC-00003 — FTS5 BM25 relevance ranking [HIGH, feature]
   - OKC-00033.04 — MCP server E2E tests [MEDIUM, feature]

### Phase 3: Architecture & Quality
*Ordering: any within phase, but all before Phase 4*

- OKC-00026 — Structured error types (replace stringly-typed errors) [HIGH, enhancement]
- OKC-00033.02 — RepositoryIndex god-struct refactor [MEDIUM, enhancement]
- OKC-00033.06 — Remove pervasive unwrap() [MEDIUM, enhancement]
- OKC-00007 — Link resolution edge cases [MEDIUM, bug]
- OKC-00008 — YAML aliases/anchors [MEDIUM, feature]
- OKC-00005 — Incremental filesystem watcher [HIGH, feature]

### Phase 4: Polish & Documentation
*Ordering: any within phase*

- OKC-00033.05 — Architecture docs and ADRs [MEDIUM, docs]
- OKC-00033.07 — Module-level doc comments [LOW, docs]
- OKC-00033.09 — Mutation testing [LOW, feature]
- OKC-00015 — OpenTelemetry structured logging [HIGH, feature]
- OKC-00022 — JSON output mode [HIGH, feature]
- OKC-00025 — Remove unused Tokio [MEDIUM, chore]
- OKC-00010 — Config file support [LOW, feature]
- OKC-00011 — Shell completions [LOW, feature]
- OKC-00013 — Semantic embeddings [LOW, feature]
- OKC-00030 — Organize docs [LOW, chore]
- OKC-00031 — Split service modules (already partially done) [LOW, chore]
- OKC-00032 — Update README [chore]
<!-- SECTION:NOTES:END -->
