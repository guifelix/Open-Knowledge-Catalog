---
id: OKC-00112
title: Enforce configuration validation at service boundaries
status: Done
assignee:
  - '@codex'
created_date: '2026-08-06 23:31'
updated_date: '2026-08-06 23:34'
labels:
  - config
  - reliability
  - backend
dependencies: []
priority: high
type: bug
ordinal: 79000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ensure invalid runtime configuration cannot reach repository initialization, scanning, MCP startup, or in-memory service construction. CLI root overrides must continue to work before validation. This is a product-integrity hardening task, not a compatibility restoration.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 OkcService::open rejects configurations with missing roots, nonexistent roots, invalid limits, or invalid BM25 values before opening storage
- [x] #2 OkcService::open_in_memory enforces the same configuration invariants
- [x] #3 CLI root overrides and the serve current-directory default are applied before validation and continue to work
- [x] #4 MCP construction and scan paths return structured errors for invalid configuration instead of opening an invalid service
- [x] #5 Regression tests cover invalid service configuration and valid CLI/MCP startup paths
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 README and referenced configuration documentation are reviewed and updated only if the public behavior changes
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add regression tests proving invalid configurations are rejected before database creation by disk and in-memory service entry points, and that MCP scan reports invalid configuration as a tool error.\n2. Enforce OkcConfig validation at OkcService construction boundaries while preserving the CLI order of load, overrides/default-root resolution, then validation.\n3. Review README and configuration documentation, run focused tests and the full Rust quality gate, then commit the completed task atomically.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added service-boundary validation before disk or in-memory repository initialization. Red tests proved missing roots and invalid limits/weights previously opened storage; they now fail before database creation. Packaged MCP scan now has coverage proving invalid roots return a tool error and create no database. README and docs/configuration.md already describe the same validation constraints and require no behavior update.

Final validation passed: cargo fmt --check; cargo test (285 non-doc tests plus 2 doc tests); cargo clippy -- -D warnings; git diff --check. Coverage includes disk and in-memory service rejection, no database side effects, MCP constructor rejection, packaged MCP scan errors, packaged stdio current-directory startup, and explicit CLI root rejection.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Enforced configuration validation at every OkcService construction boundary before storage initialization. Invalid disk, in-memory, and MCP configurations now fail deterministically without creating a database, while CLI overrides and zero-operator stdio startup remain green. Verified with 287 tests, formatting, strict Clippy, and diff checks.
<!-- SECTION:FINAL_SUMMARY:END -->
