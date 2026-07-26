---
id: OKC-00042
title: Ship v0.1 release (crates.io + GitHub Releases binaries)
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:00'
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
1. **Versioning**: Set `version = "0.1.0"` in `Cargo.toml`; add `version` field to binary via `clap` or `vergen`
2. **CI release workflow** (depends on DRAFT-00026):
   - Trigger: Git tag `v*` pushed
   - Matrix: `ubuntu-latest` (musl), `macos-latest` (universal), `windows-latest` (MSVC)
   - Steps: `cargo build --release --target <triple>`, strip binary, create checksums
   - Upload: `gh release create` with artifacts + `SHA256SUMS`
3. **crates.io publish**: 
   - `cargo login` with token (GitHub secret)
   - `cargo publish` in release workflow after successful builds
   - Verify `cargo install open-knowledge-catalog` works
4. **README install section**: Add badges (crates.io version, downloads); show `cargo install` + direct download URLs for each platform
5. **CHANGELOG**: Generate from conventional commits (`git cliff` or manual); include in release body
6. **Metadata**: Fill `Cargo.toml` `description`, `repository`, `homepage`, `license`, `keywords`, `categories`; verify on crates.io
7. **Smoke test**: Download linux binary from release; run `okc --version`, `okc scan --help`, `okc mcp --help`
<!-- SECTION:PLAN:END -->
