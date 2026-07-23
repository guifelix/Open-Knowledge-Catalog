---
id: OKC-00011
title: 'Add shell completions (bash, zsh, fish)'
status: To Do
assignee: []
created_date: '2026-07-23 00:50'
updated_date: '2026-07-23 19:02'
labels:
  - polish
dependencies: []
priority: low
type: feature
ordinal: 16400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Generate shell completions for okf CLI using clap's completion generation. Install script for bash/zsh/fish.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 okf completions bash/zsh/fish generate valid completions
- [ ] #2 Install script installs to correct locations
- [ ] #3 Subcommand and flag completions work
- [ ] #4 Bash completions generated via clap_complete for all subcommands and flags
- [ ] #5 Shell detection: auto-detects bash/zsh/fish/powershell from /bin/bash
- [ ] #6 --completions <shell> flag prints completion script
- [ ] #7 Fish completions include description strings for every argument
<!-- AC:END -->
