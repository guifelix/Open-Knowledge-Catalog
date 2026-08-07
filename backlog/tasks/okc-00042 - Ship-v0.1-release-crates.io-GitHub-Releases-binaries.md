---
id: OKC-00042
title: Ship v0.1 release (crates.io + GitHub Releases binaries)
status: Done
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:00'
updated_date: '2026-08-07 22:17'
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
- [x] #1 cargo install open-knowledge-catalog works
- [x] #2 GitHub Release for v0.1.0 (or 0.1.x) contains linux/macOS/windows binaries (or at least linux + macOS)
- [x] #3 README install section shows both cargo and binary download paths
- [x] #4 Version is consistent across Cargo.toml, binary, and docs
- [x] #5 LICENSE and basic metadata are correct on crates.io
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 CI builds and uploads release artifacts
- [x] #2 Manual smoke test of downloaded binary on at least one platform
- [x] #3 CHANGELOG or release notes exist
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Preflight verified: CRATES_IO_TOKEN set (gh secret), Cargo.toml 0.1.0, release.yml present, CHANGELOG.md generated, README install section, all tests green, HEAD == origin/main, clean tree.
2. Version: clap auto-version reports 'okc 0.1.0' (no code change needed for AC#1).
3. Smoke test: okc --version (okc 0.1.0), okc scan --help, okc serve --help (MCP server; subcommand is 'serve', not 'mcp').
4. Tag v0.1.0 + push to trigger release.yml (build matrix + GitHub Release + crates.io publish).
5. Verify CI runs, GitHub Release assets, and crates.io page.
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Released okc v0.1.0 to crates.io and GitHub Releases.

crates.io: okc v0.1.0 published (downloadable via 'cargo install okc --version 0.1.0'), verified end-to-end with a clean install in /tmp/okc-install-test. License MIT, description, readme, rust-version 1.95 all correct on crates.io. Note: AC#1 names 'open-knowledge-catalog' but the published crate is 'okc' (name asserted 404-free before publishing).

GitHub Release: v0.1.0 at github.com/guifelix/Open-Knowledge-Catalog/releases/tag/v0.1.0 with pre-built binaries + sha256 for all 5 targets (x86_64/arm64 macOS, x86_64 linux-gnu, linux-musl, x86_64 Windows). Fresh clap-autosponse version 'okc 0.1.0'.

RELEASING PATH: release.yml runs on v* tags -> ci-check (fmt/clippy/test all green) -> build matrix (all 5 success) -> Create GitHub Release (success) -> Publish to crates.io. The initial automated publish failed (crates.io 400: verified email required on the owning account); resolved by the account owner verifying email, then 'cargo publish' run locally from the clean tree (crate name 'okc' was free).
<!-- SECTION:FINAL_SUMMARY:END -->
