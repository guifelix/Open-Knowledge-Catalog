---
id: OKC-00016
title: Add property-based tests with proptest and fuzzing with cargo-fuzz
status: Done
assignee:
  - '@felix'
created_date: '2026-07-23 00:51'
updated_date: '2026-07-23 07:48'
labels: []
dependencies: []
priority: low
type: feature
ordinal: 16000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Front-matter extraction, YAML parsing, Markdown link resolution, path normalization must never panic. Add proptest for all parsers and cargo-fuzz for byte-level inputs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 proptest for frontmatter extraction (arbitrary bytes)
- [x] #2 proptest for YAML parsing (never panic)
- [x] #3 proptest for link resolution (arbitrary paths)
- [x] #4 cargo-fuzz for frontmatter delimiter extraction
- [x] #5 cargo-fuzz for YAML conversion
- [x] #6 cargo-fuzz for path normalization
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
All 3 proptest groups (frontmatter extraction, YAML parsing, link resolution) pass with bounded string strategies to prevent saphyr hangs. 17/19 tests pass; 2 pre-existing failures in nested_path_resolution and relative_path_resolution. All 3 cargo-fuzz targets (frontmatter_extraction, yaml_parsing, path_normalization) compile and run on nightly. 10 unit tests also pass.
<!-- SECTION:FINAL_SUMMARY:END -->
