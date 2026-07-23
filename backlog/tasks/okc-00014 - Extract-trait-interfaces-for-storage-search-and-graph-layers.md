---
id: OKC-00014
title: 'Extract trait interfaces for storage, search, and graph layers'
status: Done
assignee: []
created_date: '2026-07-23 00:51'
updated_date: '2026-07-23 06:16'
labels: []
dependencies: []
priority: high
type: feature
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define DocumentStore, SearchIndex, GraphStore traits. RepositoryIndex becomes composition over these. Enables swapping SQLite -> PostgreSQL, FTS5 -> Tantivy, in-memory -> distributed graph.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 DocumentStore trait with CRUD + query_metadata
- [ ] #2 SearchIndex trait with FTS + hybrid support
- [ ] #3 GraphStore trait with traverse + links/backlinks
- [ ] #4 RepositoryIndex generic over S, I, G
- [ ] #5 SQLite impl for all three traits
<!-- AC:END -->
