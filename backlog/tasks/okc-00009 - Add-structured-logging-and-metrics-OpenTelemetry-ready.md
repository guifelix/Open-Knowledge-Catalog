---
id: OKC-00009
title: Add structured logging and metrics (OpenTelemetry ready)
status: Done
assignee: []
created_date: '2026-07-23 00:50'
updated_date: '2026-07-23 19:02'
labels:
  - superseded
dependencies: []
priority: medium
type: feature
ordinal: 9999
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add tracing spans for all operations (scan, search, get, traverse). Export Prometheus metrics: scan_duration, search_latency, index_size, error_count. OpenTelemetry trace export ready.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Tracing spans for all public operations
- [ ] #2 Prometheus metrics: scan_duration_seconds, search_latency_seconds, documents_indexed, errors_total
- [ ] #3 OpenTelemetry trace export configurable
- [ ] #4 Structured JSON logs in server mode
<!-- AC:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-07-23 06:50
---
Gap analysis finding: This task overlaps significantly with OKC-00015 (Add OpenTelemetry structured logging and Prometheus metrics). Consider whether to:

(1) Deprecate this task and fold scope into OKC-00015 (recommended — single observability initiative), or
(2) Keep separate with clear boundary: OKC-00009 focuses on structured JSON logging and tracing spans; OKC-00015 focuses on Prometheus RED metrics and health endpoints.

Current codebase has 0 structured log infrastructure — only 5 bare info!() calls. Recommendation: merge into OKC-00015 as it provides the full OpenTelemetry stack that makes structured logging trivial.
---
<!-- COMMENTS:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Scope absorbed into OKC-00015 (Observability/tracing). The structured logging concern is not useful without the full tracing + metrics story. Closing as superseded.
<!-- SECTION:FINAL_SUMMARY:END -->
