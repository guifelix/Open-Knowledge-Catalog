---
id: DRAFT-00021
title: Implement reliable filesystem watcher + incremental updates
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:01'
labels:
  - reliability
  - index
dependencies: []
documentation:
  - docs/filesystem-watcher.md
priority: high
type: feature
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Phase 5 is still open. Agents and long-running MCP servers need the index to stay current without full rescans. Debounced notify-based watching + periodic reconciliation is required for the always-on knowledge layer claim.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 File create/modify/delete events update the index within a configurable debounce window
- [ ] #2 Periodic full reconcile catches missed events / drift
- [ ] #3 Watcher can be started/stopped cleanly from CLI and MCP
- [ ] #4 No unbounded memory growth under high churn
- [ ] #5 Documented behavior for network filesystems / edge cases
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Integration tests with tempfile + simulated events
- [ ] #2 docs/architecture and roadmap updated
- [ ] #3 Default config values are sensible
<!-- DOD:END -->

## Implementation Plan
<!-- SECTION:PLAN:BEGIN -->
1. **Choose watcher crate**: Evaluate `notify` (cross-platform, debounced) vs `watchexec` (simpler); recommend `notify` with `PollWatcher` fallback for network FS
2. **Debounce layer**: Implement `DebouncedWatcher` wrapping `notify::RecommendedWatcher`; batch events within configurable window (default 500ms); deduplicate by path
3. **Event handler**: On batched events, call `Indexer::update_incremental(paths)` which:
   - Deletes: remove document + graph edges from DB
   - Creates/Modifies: re-parse, re-index, update FTS + graph
   - Uses transaction per batch for atomicity
4. **Periodic reconciliation**: Background task (default 5min interval) runs full scan of watched roots; compares file hashes (Blake3) vs DB; repairs drift
5. **Lifecycle management**: 
   - CLI: `okc watch start --root <path> --debounce-ms 500 --reconcile-interval 300`
   - MCP: `start_watcher` / `stop_watcher` tools with same params
   - Graceful shutdown: flush pending events, stop reconcile task, close watcher
6. **Memory bounds**: 
   - Event channel bounded (e.g., 10k events); drop oldest on overflow with metric
   - Reconcile uses streaming hash comparison, not loading all paths at once
7. **Config schema**: Add `watcher` section to config.toml with `debounce_ms`, `reconcile_interval_sec`, `enabled`, `roots[]`
8. **Tests**: 
   - Unit: debounce logic, event deduplication
   - Integration: temp dir + `notify` events → verify index updates within debounce window
   - Stress: rapid create/modify/delete → no memory leak, no missed events after reconcile
9. **Documentation**: `docs/filesystem-watcher.md` with architecture, config, edge cases (network FS, symlinks, permissions)
<!-- SECTION:PLAN:END -->
