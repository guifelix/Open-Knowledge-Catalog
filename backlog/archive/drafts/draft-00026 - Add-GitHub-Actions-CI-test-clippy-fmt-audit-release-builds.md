---
id: DRAFT-00026
title: 'Add GitHub Actions CI (test, clippy, fmt, audit, release builds)'
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:03'
labels:
  - ci
  - quality
dependencies: []
documentation:
  - docs/ci-setup.md
priority: medium
type: task
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MAANG-level code quality remediation includes basic CI. Without green CI, contributors and users cannot trust the project. Minimum viable: test + clippy + fmt on PR, plus release artifact builds.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PRs run cargo test, cargo clippy -D warnings, cargo fmt --check
- [ ] #2 Optional cargo audit / deny for known advisories
- [ ] #3 Release workflow produces binaries (or at least documents how)
- [ ] #4 Status badge on README
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 CI is green on main
- [ ] #2 Required checks configured (or documented as pending)
- [ ] #3 CONTRIBUTING.md points to the expected local checks
<!-- DOD:END -->
