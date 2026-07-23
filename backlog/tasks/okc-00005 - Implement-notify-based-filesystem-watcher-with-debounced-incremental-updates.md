---
id: OKC-00005
title: Implement notify-based filesystem watcher with debounced incremental updates
status: To Do
assignee: []
created_date: '2026-07-23 00:49'
updated_date: '2026-07-23 19:02'
labels:
  - ux
dependencies: []
priority: high
type: feature
ordinal: 2400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add notify crate integration to watch repository roots. Debounce events (editor temp file patterns) and trigger incremental scan. Periodic full reconciliation for drift correction.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 notify watcher monitors all configured roots
- [ ] #2 Debounced events trigger incremental scan within 500ms
- [ ] #3 Periodic full reconciliation every 10min
- [ ] #4 Handles editor temp-file patterns (create+rename)
- [ ] #5 inotify/kqueue watcher monitors bundle directory for .md changes with <1s latency
- [ ] #6 Re-indexes only the changed bundle (incremental, not full rebuild)
- [ ] #7 Watches respect .gitignore — skips .git/ and vendor/ — avoids infinite re-index loops
- [ ] #8 Cross-platform: works on Linux (inotify) and macOS (kqueue/FSEvents)
<!-- AC:END -->
