---
id: OKC-00042
title: Ship v0.1 release (crates.io + GitHub Releases binaries)
status: In Progress
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:00'
updated_date: '2026-08-07 20:02'
labels:
  - release
  - distribution
  - p0
dependencies: []
documentation:
  - docs/release-process.md
priority: high
type: task
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Make the tool installable without cloning and building from source. Publish to crates.io and attach pre-built binaries for the major platforms on GitHub Releases. This is required for any non-trivial adoption.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 cargo install open-knowledge-catalog works
- [ ] #2 GitHub Release for v0.1.0 (or 0.1.x) contains linux/macOS/windows binaries (or at least linux + macOS)
- [ ] #3 README install section shows both cargo and binary download paths
- [ ] #4 Version is consistent across Cargo.toml, binary, and docs
- [ ] #5 LICENSE and basic metadata are correct on crates.io
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 CI builds and uploads release artifacts
- [ ] #2 Manual smoke test of downloaded binary on at least one platform
- [ ] #3 CHANGELOG or release notes exist
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Preflight verified: CRATES_IO_TOKEN set (gh secret), Cargo.toml 0.1.0, release.yml present, CHANGELOG.md generated, README install section, all tests green, HEAD == origin/main, clean tree.
2. Version: clap auto-version reports 'okc 0.1.0' (no code change needed for AC#1).
3. Smoke test: okc --version (okc 0.1.0), okc scan --help, okc serve --help (MCP server; subcommand is 'serve', not 'mcp').
4. Tag v0.1.0 + push to trigger release.yml (build matrix + GitHub Release + crates.io publish).
5. Verify CI runs, GitHub Release assets, and crates.io page.
<!-- SECTION:PLAN:END -->
