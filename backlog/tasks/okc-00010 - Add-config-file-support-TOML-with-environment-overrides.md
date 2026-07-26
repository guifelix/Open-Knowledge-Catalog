---
id: OKC-00010
title: Add config file support (TOML) with environment overrides
status: Done
assignee: []
created_date: '2026-07-23 00:50'
updated_date: '2026-07-26 20:16'
labels:
  - config
dependencies: []
priority: high
type: feature
ordinal: 15400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace hardcoded defaults with config file: okc.toml in repo root or ~/.config/okc/config.toml. Environment variable overrides for all settings. Config validation at startup.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 okc.toml parsed and applied
- [ ] #2 Environment variables override config (OKC_ROOTS, OKC_DB_PATH, etc.)
- [ ] #3 Config validation with clear error messages
- [ ] #4 Example config file in repo
- [ ] #5 TOML config file at XDG_CONFIG_HOME/okc/config.toml is auto-created with defaults on first run
- [ ] #6 CLI flags override config file values (CLI wins)
- [ ] #7 Config includes: index_dir, bundle_roots[], concurrency_limit, max_file_size, max_graph_depth
- [ ] #8 --config /path/to/custom.toml flag supported
<!-- AC:END -->
