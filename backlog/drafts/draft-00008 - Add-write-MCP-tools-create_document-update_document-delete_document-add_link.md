---
id: DRAFT-00008
title: >-
  Add write MCP tools: create_document, update_document, delete_document,
  add_link
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:49'
labels:
  - mcp
  - backend
  - feature
  - high-priority
dependencies: []
references:
  - src/transport/mcp.rs
  - src/index/document_store.rs
  - src/index/graph_store.rs
  - src/index/search_index.rs
  - src/parser/frontmatter.rs
documentation:
  - docs/ai-usage.md#mcp-tools
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MCP server is read-only. Agents cannot maintain the knowledge base.

**Missing tools:**
- create_document: Create new OKF document with frontmatter and body
- update_document: Update frontmatter fields, body, or both
- delete_document: Remove document and its index entries
- add_link: Add validated internal link between documents
- fix_broken_links: Auto-repair broken links from validate()

**Requirements:**
- Validate OKF frontmatter on create/update
- Update search index, graph, metadata atomically
- Return created/updated document with new content_hash
- Emit change events for watch() subscribers
- Respect max_file_size, max_front_matter_size limits
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 create_document creates valid OKF document, returns new document with content_hash
- [ ] #2 update_document modifies frontmatter/body, updates all indexes
- [ ] #3 delete_document removes document, cleans up links, backlinks, graph edges
- [ ] #4 add_link validates target exists, adds link to source, updates backlinks
- [ ] #5 fix_broken_links suggests repairs, applies with confirmation
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit tests cover each write operation with validation
- [ ] #2 Integration test: create → update → add_link → delete roundtrip
- [ ] #3 Concurrent write safety (no index corruption)
- [ ] #4 Rollback on validation failure
<!-- DOD:END -->
