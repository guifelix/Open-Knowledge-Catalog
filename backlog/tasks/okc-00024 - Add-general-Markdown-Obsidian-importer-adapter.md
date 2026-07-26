---
id: OKC-00024
title: Add general Markdown/Obsidian importer adapter
status: To Do
assignee: []
created_date: '2026-07-23 19:03'
updated_date: '2026-07-25 23:57'
labels:
  - extensibility
dependencies: []
priority: medium
type: feature
ordinal: 18400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a general Markdown importer that works with any Markdown knowledge base (not just OKF-formatted bundles). This allows the tool to index Obsidian vaults, Docusaurus docs, or any structured Markdown directory.\n\nThe adapter should:\n- Detect document structure (headings, links, frontmatter) regardless of schema\n- Support a mapping config: 'my vault's 'topic' field -> our 'tags''\n- Fall back to content-only extraction when no frontmatter schema matches\n- Allow users to bring their own Markdown without reformatting
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Importer detects and indexes plain Markdown files with or without frontmatter
- [ ] #2 Configurable field mapping: map source YAML fields to internal concept fields
- [ ] #3 Fallback mode: no frontmatter → content-only concept with auto-generated id
- [ ] #4 Obsidian WikiLinks [[]] and regular Markdown links both captured
- [ ] #5 Importer works with: plain .md, Obsidian vault, Docusaurus docs, Hugo/Gatsby content directories
- [ ] #6 Existing OKF-based flow unchanged — this is additive
<!-- AC:END -->
