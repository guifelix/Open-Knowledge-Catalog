---
id: OKC-00102
title: Add safe agent-config generation and updates
status: To Do
assignee:
  - '@engineering'
created_date: '2026-07-28 01:23'
updated_date: '2026-08-06 19:56'
labels:
  - feature
  - mcp
  - cli
  - dx
  - automation
dependencies:
  - OKC-00097
references:
  - docs/ai-usage.md
priority: medium
type: feature
ordinal: 68000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Provide one supported CLI workflow for generating and optionally writing OKC MCP configuration for documented AI-agent clients. Configuration generation must be previewable, preserve unrelated user settings, and use the platform-appropriate global or project file for each target. This complements OKC-00043, which owns the explanatory onboarding documentation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A list command reports every supported target, its configuration scope, and its resolved destination path without modifying files
- [ ] #2 Generating configuration prints a valid target-specific snippet by default, with an explicit write option for filesystem changes
- [ ] #3 Writing configuration preserves unrelated keys and updates an existing OKC entry without duplication
- [ ] #4 Writes are atomic, create a recoverable backup when replacing an existing file, and report parse, permission, and unsupported-platform errors without partial changes
- [ ] #5 Tests cover every documented target format, first-time creation, idempotent update, malformed input, and preview behavior
- [ ] #6 CLI help and docs/ai-usage.md identify the command as the source of generated examples
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Supported agents list maintained in a single source of truth
- [ ] #2 Each agent format tested with integration test
- [ ] #3 CLI --help updated with examples
- [ ] #4 docs/ai-usage.md references the supported agent-config workflow and generated formats
<!-- DOD:END -->
