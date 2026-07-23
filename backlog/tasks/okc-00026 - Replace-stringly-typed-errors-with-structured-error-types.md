---
id: OKC-00026
title: Replace stringly-typed errors with structured error types
status: To Do
assignee: []
created_date: '2026-07-23 19:03'
updated_date: '2026-07-23 19:04'
labels:
  - quality
dependencies: []
priority: high
type: enhancement
ordinal: 9400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace ad-hoc String error propagation with a proper error enum using thiserror.\n\nCurrent state: most fallible functions return Result<_, String> or use anyhow. Error messages are inconsistent, lack machine-readable codes, and make it impossible for MCP clients to handle errors programmatically.\n\nTarget: a OkfError enum with variants per error category (Io, Parse, Validation, NotFound, Config, Sql, Serde, Internal) implementing std::error::Error via thiserror.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 OkfError enum exists with variants: Io, Parse, Validation, NotFound, Config, Sql, Serde, Internal, InvalidInput
- [ ] #2 All public functions return OkfError or a specific Result alias (not String)
- [ ] #3 Each variant carries structured context: file path, limit name, expected vs actual where relevant
- [ ] #4 OkfError implements Display with human-readable messages AND Debug with full context
- [ ] #5 MCP error codes map 1:1 from error variant (e.g., NotFound -> -32602)
- [ ] #6 cargo check passes with no regressions and no dead code warnings
<!-- AC:END -->
