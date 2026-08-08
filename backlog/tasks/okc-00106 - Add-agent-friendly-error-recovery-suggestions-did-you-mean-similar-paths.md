---
id: OKC-00106
title: Add bounded recovery hints to structured agent errors
status: Done
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:52'
updated_date: '2026-08-08 19:03'
labels:
  - mcp
  - backend
  - enhancement
  - agent-ux
  - errors
dependencies:
  - OKC-00026
references:
  - src/transport/mcp/mod.rs
  - src/index/queries.rs
documentation:
  - docs/ai-usage.md
priority: medium
type: enhancement
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Extend the structured error contract from OKC-00026 with deterministic recovery hints that help agents correct missing or invalid repository paths. Suggestions must be bounded, scoped to configured roots, and safe to expose; this task does not redefine the base error taxonomy.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Not-found errors can include a bounded ranked list of similar repository paths and one documented did-you-mean candidate
- [x] #2 Invalid-path errors include concise path rules and examples without exposing paths outside configured roots
- [x] #3 Parent-directory context is returned only when the parent is valid, in scope, and within configured result limits
- [x] #4 Suggestion ranking is deterministic, has a documented maximum candidate count, and remains responsive on large indexes
- [x] #5 Errors without useful recovery information retain the base structured error shape without empty or misleading hints
- [x] #6 Unit and packaged-MCP tests cover ranking, ties, no-match cases, traversal attempts, disclosure boundaries, and response limits
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Unit test covers error suggestion generation
- [x] #2 Integration test: agent recovers from NOT_FOUND using suggestions
- [x] #3 Consistent error schema across all 11 tools
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Error layer (src/error.rs): extend NotFound to optionally carry hints — add designated not_found_with_hints constructor. Display appends a single deterministic did-you-mean line ("Did you mean: ...?") only when hints exist; base (hint-free) shape unchanged. mcp_code stays -32602.

2. Hint/ranking helper (new src/index/queries/suggest.rs): pure, deterministic, bounded candidate generation given requested path + repository path list. Ranking: full-path slash-aware edit distance (Damerau-Levenshtein on last segment, then full path), tie-break lexicographic. Bounded: MAX_SUGGESTIONS const (4). Never returns the requested path itself; blocks traversal tokens (..), root-relative and URL forms; only emits candidates that exist in the index (already in roots by construction).

3. Query-layer conversions: get_document missing path -> OkfError::NotFound with hints (was Database from QueryReturnedNoRows). get_section missing doc: keep Ok(None) flow but MCP tool surfaces not-found message+ hint. browse_directory for missing dir returns empty (unchanged) unless target of a later task.

4. MCP tools (src/transport/mcp/mod.rs): all path-based tools already surface Err(format!("Error: {e}")) — hints propagate automatically via Display; no per-tool changes needed for get_document/browse. get_section tool converts its Ok(None) into a NOT_FOUND-style message including hints.

5. Docs (docs/ai-usage.md): document the error hints addendum: format, max candidate count, determinism, rules (no root/path disclosure, traversal rejected). Update error example.

6. Tests: unit (src/... suggest.rs) for ranking, ties, no-match -> no hints, bound enforcement, traversal/ path variants; error.rs tests for hint-annotated Display + hint-free passthrough. Integration (tests/mcp_e2e_tests.rs): get_document typo path -> error mentions a valid suggestion; clean-up none. Package-MCP tests exercise all 11 tools error schema consistency.

7. Verification: cargo fmt --check, cargo clippy -- -D warnings, cargo test; update docs/ai-usage.md if implementation drifts. Commit with conventional message (feat or fix), update task status to Done after DoD.
<!-- SECTION:PLAN:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-08-08 05:07
---
Plan recorded for OKC-00106
---
<!-- COMMENTS:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented bounded deterministic 'did you mean' recovery hints on structured not-found errors.

Code: added src/index/queries/suggest.rs (pure suggest_paths: bounded OSA edit distance, MAX_SUGGESTIONS=4, MAX_EDIT_DISTANCE=3, deterministic ranking, traversal/absolute/URL/drive-prefix rejected, never echoes query); extended OkfError::NotFound with hints + not_found_with_hints + fmt_not_found that appends 'Did you mean:' block only when hints exist (mcp_code stays -32602); get_document missing path now returns NotFound with hints; get_section MCP handler discriminates missing doc (NOT_FOUND + hints) from heading-not-found ({section:null}, no hints) via document_exists.

Docs: added 'Typo Recovery Hints' addendum to docs/ai-usage.md documenting format, bounds (<=4 candidates, edit distance <=3), determinism, no root/path disclosure, and the non-misleading {section:null} case.

Tests (all green): 14 suggest.rs unit tests (ranking, bounds, no-match, ties, traversal/dot/absolute/drive/URL/empty rejection, zero-max, edit-distance cap); error.rs hint display tests; 3 integration tests (document_exists discriminator, missing-doc surfaces hints, no-candidates no hints); 4 packaged-MCP e2e tests (get_document typo suggests path, far-away typo no suggestion, get_section missing-doc suggests path, get_section unknown heading is not missing-doc). cargo test (all suites: lib 149, integration 23, mcp_e2e 12, doc, property, proxy), cargo fmt --check, cargo clippy --all-targets -- -D warnings all pass.
<!-- SECTION:FINAL_SUMMARY:END -->
