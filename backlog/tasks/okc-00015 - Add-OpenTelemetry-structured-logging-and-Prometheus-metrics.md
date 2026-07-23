---
id: OKC-00015
title: Add OpenTelemetry structured logging and Prometheus metrics
status: To Do
assignee: []
created_date: '2026-07-23 00:51'
updated_date: '2026-07-23 19:02'
labels:
  - ops
dependencies: []
references:
  - 'absorbed: OKC-00009'
priority: high
type: feature
ordinal: 4400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Instrument all AI tool operations with spans. Export Prometheus metrics: request_latency, request_count, error_count, active_connections. Health endpoint with readiness/liveness.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 OpenTelemetry tracing initialized
- [ ] #2 Spans for all AI tools with query, latency, results
- [ ] #3 Prometheus /metrics endpoint with RED metrics
- [ ] #4 /healthz and /readyz endpoints
- [ ] #5 Correlation IDs propagated through request chain
- [ ] #6 scan_duration_seconds histogram tracks all scan operations
- [ ] #7 search_latency_seconds histogram tracks all search/query operations
- [ ] #8 response_size_bytes histogram on browse/get/section results
- [ ] #9 request_count_total and error_count_total counters by operation type
- [ ] #10 documents_indexed gauge reflects current index size
- [ ] #11 All entry/exit points logged via tracing crate with span nesting
- [ ] #12 tracing-subscriber configured for both human-readable (stderr) and JSON (file) output
- [ ] #13 Metrics: scan count, index size, query latency exported via metrics crate
- [ ] #14 CLI --verbose flag switches from INFO to DEBUG level
- [ ] #15 CLI --json-logs flag outputs structured JSON (absorbed from OKC-00009)
<!-- AC:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-07-23 06:50
---
Gap analysis finding: Expand scope to match doc1 specification. Current codebase has only 5 info!() calls across the entire project and zero warn!/error!/debug! calls. Doc1 requires:

Required metrics (all currently absent):
- scan_duration_seconds (histogram) — scanning a repo
- parsing_time_seconds (histogram) — parsing a single document
- db_update_time_seconds (histogram) — inserting/updating documents
- search_latency_seconds (histogram) — search query execution
- response_size_bytes (histogram) — response body sizes
- request_count_total (counter) — by operation type
- error_count_total (counter) — by error type
- documents_indexed (gauge) — current index size
- active_scans (gauge) — ongoing scan operations
- config_reloads_total (counter) — config changes

Doc1 also specifies timing instrumentation for all 9 AI operations (browse, get, section, search, query, links, backlinks, traverse, validate) — each should have a corresponding latency histogram.

Existing OKC-00009 (structured logging) overlaps with this task. Consider whether to merge OKC-00009 scope into OKC-00015 or keep separate.
---
<!-- COMMENTS:END -->
