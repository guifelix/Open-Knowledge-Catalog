---
id: DRAFT-00003
title: >-
  Add okc install agent command to generate MCP config snippets for all
  supported agents
status: To Do
assignee:
  - '@engineering'
created_date: '2026-07-28 01:23'
labels:
  - feature
  - mcp
  - cli
  - dx
  - automation
dependencies: []
references:
  - >-
    https://github.com/guifelix/Open-Knowledge-Catalog/blob/main/docs/ai-usage.md
  - 'https://github.com/okfdeploy/okf'
priority: medium
type: feature
ordinal: 68000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The most common post-build action is configuring OKC as an MCP tool in various AI coding agents. Currently users must manually copy/paste JSON configs from docs/ai-usage.md into the right files. Competitors like `okf` already have `okf install` that writes config for 7 agents in one command.

Proposed CLI command:
```bash
okc install claude          # writes to ~/.claude/settings.json
okc install opencode        # writes to ./opencode.json  
okc install cursor          # writes to ./.cursor/mcp.json
okc install --all           # writes all supported agents
okc install --list          # list supported agents
```

This eliminates the friction of manual config while keeping the docs as a reference for the exact format produced.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 okc install --list shows all supported agents with their config file paths
- [ ] #2 okc install claude creates/updates Claude Desktop claude_desktop_config.json entry
- [ ] #3 okc install opencode creates/updates opencode.json MCP entry
- [ ] #4 okc install cursor creates/updates .cursor/mcp.json
- [ ] #5 okc install --all writes all known agent configs
- [ ] #6 okc install is idempotent (safe to re-run)
- [ ] #7 Existing config entry is updated, not duplicated
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Supported agents list maintained in a single source of truth
- [ ] #2 Each agent format tested with integration test
- [ ] #3 CLI --help updated with examples
- [ ] #4 docs/ai-usage.md updated to reference okc install command
<!-- DOD:END -->
