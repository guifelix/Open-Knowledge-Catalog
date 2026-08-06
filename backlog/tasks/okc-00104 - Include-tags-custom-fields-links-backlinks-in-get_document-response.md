---
id: OKC-00104
title: Add opt-in enriched context to get_document responses
status: Done
assignee:
  - '@codex'
created_date: '2026-08-06 00:49'
updated_date: '2026-08-06 22:56'
labels:
  - mcp
  - backend
  - enhancement
  - document-retrieval
dependencies:
  - OKC-00101
references:
  - src/index/queries/document.rs
  - src/transport/mcp/mod.rs
documentation:
  - docs/ai-usage.md
priority: high
type: enhancement
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Let agents request complete document context in one get_document call without expanding the default response or bypassing configured response limits. Tags already exist in the metadata response; this task adds explicit include options for custom metadata, document identity fields, outgoing links, and backlinks with stable typed shapes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The existing default get_document response remains backward compatible and bounded
- [x] #2 Explicit include values can request custom metadata, content_hash, parent_path, outgoing links, and backlinks
- [x] #3 Backlink entries identify their source document and link context using a documented stable shape
- [x] #4 Unknown include values return a structured validation error
- [x] #5 The aggregate enriched response enforces max_response_chars and reports truncation without returning partially malformed objects
- [x] #6 Service and packaged-MCP tests cover default, each optional section, combined enrichment, missing documents, and truncation
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Unit test verifies all fields present in response
- [x] #2 Integration test: single get_document call replaces 3 calls
- [x] #3 Response size still respects max_response_chars limit
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add red service and packaged-MCP tests for include validation, each enrichment field, combined retrieval, and bounded truncation.
2. Extend the document query model with optional identity, custom metadata, outgoing-link, and source-aware backlink data while preserving existing include semantics.
3. Add the configured aggregate response cap and trim only at valid body/item boundaries, reporting truncation.
4. Update MCP schemas and usage/configuration documentation, then run focused and full Rust quality gates before finalization.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented opt-in custom metadata, content_hash, parent_path, outgoing links, and source-aware backlinks while preserving the default serialized response. Added max_response_chars configuration and aggregate response trimming at valid body/item boundaries. Focused service and packaged-MCP tests pass for individual and combined includes, validation, and truncation.

Final validation: cargo fmt --check; cargo test (276 unit/integration/property tests plus 2 doc tests passed); cargo clippy -- -D warnings; git diff --check. Documentation now defines include values, backlink shape, default behavior, and max_response_chars configuration.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added backward-compatible opt-in document enrichment for custom metadata, identity fields, outgoing links, and source-aware backlinks. Enriched responses are bounded by max_response_chars and truncate only at valid boundaries. Verified through unit, service integration, packaged MCP, missing-document, validation-error, truncation, full test, formatting, lint, and diff checks.
<!-- SECTION:FINAL_SUMMARY:END -->
