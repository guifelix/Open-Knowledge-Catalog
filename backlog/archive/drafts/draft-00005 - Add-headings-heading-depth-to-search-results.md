---
id: DRAFT-00005
title: Add headings + heading depth to search results
status: Draft
assignee:
  - '@backend-agent'
created_date: '2026-07-28 02:27'
labels:
  - backend
  - search
  - feature
  - headings
  - config
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Search results currently have `excerpt` but no structured headings list.

Add `extract_headings()` that parses markdown headings from body text.

## Default chain (both heading_depth and max_headings):
1. Per-request MCP param (if passed)
2. TOML config file `[search]` section (if configured)
3. Hard default = **1** (h1 only / one heading)

## heading_depth
- Controls which heading levels to extract. If heading_depth=2, only h1 and h2 are returned.
- Also limits which headings max_headings counts against. So depth=2 means "only count h1+h2 against the max_headings budget".

## max_headings
- Controls how many headings (at the allowed depths) to return.
- max_headings=3 means at most 3 headings at the configured depth.

## Combined example
heading_depth=2, max_headings=3 → return up to 3 headings from h1 and h2 levels.
<!-- SECTION:DESCRIPTION:END -->
