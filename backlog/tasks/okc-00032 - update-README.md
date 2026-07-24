---
id: OKC-00032
title: update README
status: Done
assignee: []
created_date: '2026-07-24 01:24'
updated_date: '2026-07-24 19:47'
labels: []
dependencies: []
ordinal: 24000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
- break README into smaller markdown documents in `docs/`
- update relevant sections with repo's context
- update AGENTS.md file with instruction to always run tests and format check and review docs (to see if they need updating)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 README.md references multiple scoped markdown documents in docs folder
- [ ] #2 docs folder follows the OKF format/convention
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Split README into 7 focused docs in docs/ (getting-started, architecture, configuration, ai-usage, development, roadmap, references). Updated README.md as concise overview with links. Updated AGENTS.md with quality gate instructions. Fixed clippy warnings in mcp.rs and watcher.rs. Aligned all docs with OKF v0.2 spec: frontmatter fields (type, title, description, resource, tags, generated, verified, status, stale_after, sources), actor convention (human:, process:, producer/version), trust tiers (unverified/machine-confirmed/human-reviewed), reserved filenames (index.md, log.md), bundle-relative links. All quality gates pass: cargo test (19 passed), cargo fmt --check, cargo clippy -- -D warnings.
<!-- SECTION:FINAL_SUMMARY:END -->
