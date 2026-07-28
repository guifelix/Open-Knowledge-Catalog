---
type: reference
title: Incremental Scan and Safety Limits
description: Technical documentation for the incremental file scanning system, covering design decisions, safety limits, and behavioral guarantees
tags: [scanning, filesystem, performance, safety]
owner: felix
status: draft
---

# Incremental Scan & Safety Limits

## Overview

The incremental scanner efficiently detects file changes using content hashing and processes updates transactionally. This document covers design decisions, safety limits, and behavioral guarantees.

## Content Hashing

Files are hashed using Blake3 with a sampling strategy for large files.

| Setting | Default | Description |
|---------|---------|-------------|
| `full_hash_threshold` | 1 MiB | Files under this size get a full Blake3 hash |
| `sample_count` | 64 | Number of samples for large files |
| `sample_size` | 4 KiB | Size of each sample |

For files exceeding `full_hash_threshold`:
- The first `sample_size` bytes (header) are always hashed
- `sample_count - 2` samples are taken at regular intervals through the body
- The last `sample_size` bytes (footer) are always hashed
- Total file length is mixed into the final hash to distinguish files of different sizes with identical samples

This ensures deterministic behavior: identical files always produce the same hash, while different files (even with similar content) produce different hashes due to Blake3's avalanche property.

## Document Truncation

Large documents are truncated to fit within output size limits while preserving structural integrity.

| Setting | Default | Description |
|---------|---------|-------------|
| `max_chars` | configurable | Maximum character count for document body |

Truncation strategy (in order):
1. Front-matter (metadata) is always preserved in full
2. Content before the truncation point is preserved
3. Content after the truncation point is preserved (typically 50/50 split of available budget)
4. A `[... truncated ...]` marker is inserted at the truncation point
5. Document body never exceeds `max_chars`

## Transactional Updates

All change processing (add, modify, delete) runs within a single SQLite transaction:
- Failures during processing roll back all changes atomically
- Graph, search index, and document store all update within the same transaction
- Partial updates cannot leave the index in an inconsistent state

## Change Detection Guarantees

- New files: detected and indexed on the next scan
- Modified files: detected via content hash change; re-parsed and re-indexed
- Deleted files: removed from index, graph, and search on the next scan
- Unchanged files: skipped entirely (no I/O, no re-parsing) based on content hash
- Mtime-only changes: ignored; only content changes trigger re-indexing
- Empty states handled: empty repository, empty previous scan result, empty current scan result all produce correct (empty) output
- Deterministic: identical file states produce identical scan results

## Performance Bounds

| Operation | Bound | Notes |
|-----------|-------|-------|
| Content hash (small file) | O(n) | Full Blake3 over the entire file |
| Content hash (large file) | O(sample_count × sample_size) | Constant-time sampling for files > 1 MiB |
| Document truncation | O(n) | Scans character content for structure preservation |
| Incremental scan | O(changed files) | Only processes files whose content hash changed |
| Full scan | O(total files) | Initial scan or forced re-scan of all files |

## Safety Limits

- Content hash output: 64 hex characters (Blake3)
- Minimum sample count: 2 (header + footer)
- Minimum sample size: 1 byte
- Truncation marker: `[... truncated ...]` — preserved even at very small `max_chars`
- Transaction timeout: configured via SQLite default (typically 5s)
