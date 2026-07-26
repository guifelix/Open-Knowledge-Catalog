---
id: DRAFT-00020
title: Complete JSON output mode for every CLI subcommand
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 19:56'
labels:
  - agent-ux
  - cli
  - mcp
  - interoperability
dependencies:
  - OKC-00022
documentation:
  - docs/json-output.md
priority: high
type: feature
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Agents that cannot (or prefer not to) use MCP still need machine-readable output. Every subcommand must support --json with a stable schema (status, data, meta / error shape) as already sketched in docs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All core commands accept --json and emit the documented envelope
- [ ] #2 Preserve source traceability, scores, and metadata in JSON
- [ ] #3 Error responses use the structured error shape (code + message)
- [ ] #4 Consistent schema with proper error handling
- [ ] #5 No human-only formatting leaks into JSON mode
- [ ] #6 Schema is documented and versioned lightly (or at least stable)
- [ ] #7 Backward-compatible with existing text output
- [ ] #8 Tests verify both formats
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 CLI parsing and service layer updated
- [ ] #2 Snapshot / golden tests for JSON output of each command
- [ ] #3 Examples added to README and AGENTS.md
- [ ] #4 docs/ai-usage.md matches reality
- [ ] #5 CLI help text mentions --json
- [ ] #6 Documentation updated
<!-- DOD:END -->

## Implementation Plan
<!-- SECTION:PLAN:BEGIN -->
1. **Define JSON envelope schema** (in `docs/json-output.md`):
   - Success: `{ "status": "ok", "data": T, "meta": { "count": N, "took_ms": M, "source": "..." } }`
   - Error: `{ "status": "error", "error": { "code": "NOT_FOUND|VALIDATION|INTERNAL|...", "message": "...", "details": {} } }`
   - Version header: `x-okc-json-version: 1` (for future evolution)
2. **Add `--json` flag globally**: Extend CLI root command with `json: bool`; propagate to all subcommands via shared `OutputFormat` enum
3. **Implement serializers**: For each command, create `to_json(&self) -> serde_json::Value` using `serde`; reuse existing response types where possible
4. **Error mapping**: Convert all `anyhow::Error` / custom errors to structured error codes via DRAFT-00022 (dependency)
5. **Golden tests**: Add `tests/json_output.rs` using `insta` for snapshot testing; one test per subcommand with representative inputs
6. **CLI help**: Update `clap` long/about help to mention `--json` flag
7. **Documentation**: Write `docs/json-output.md` with schema, examples, versioning policy; update `docs/ai-usage.md` and `AGENTS.md`
8. **Backward compat**: Ensure default (no `--json`) produces identical text output as before
<!-- SECTION:PLAN:END -->
