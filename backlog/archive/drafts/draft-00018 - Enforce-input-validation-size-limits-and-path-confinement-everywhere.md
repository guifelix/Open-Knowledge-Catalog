---
id: DRAFT-00018
title: 'Enforce input validation, size limits, and path confinement everywhere'
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:00'
labels:
  - security
  - correctness
  - p0
dependencies: []
documentation:
  - docs/architecture/security-boundaries.md
priority: high
type: feature
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Make the safety claims in the README and architecture docs real: max file size, max front-matter size, max response size, denied path patterns, symlink policy, and no escape from configured roots. Apply consistently to CLI, MCP, and scanner.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Configurable limits are enforced on scan, get, search, and graph operations
- [ ] #2 Path traversal / symlink escape attempts are rejected
- [ ] #3 Oversized responses are truncated with a clear truncated: true flag
- [ ] #4 Default denylist (.git, node_modules, secrets, etc.) cannot be accidentally bypassed
- [ ] #5 Tests cover adversarial paths and oversized inputs
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Security-related tests pass
- [ ] #2 Configuration docs list every limit and its default
- [ ] #3 Architecture Security Boundaries section matches implementation
<!-- DOD:END -->

## Implementation Plan
<!-- SECTION:PLAN:BEGIN -->
1. **Centralize limits config**: Create `SecurityLimits` struct in `config.rs` with defaults:
   - `max_file_size` (default: 10MB), `max_frontmatter_size` (1MB), `max_response_size` (1MB)
   - `deny_patterns` (default: `.git`, `node_modules`, `target`, `*.secret`, `*.key`, `.env*`)
   - `follow_symlinks` (default: false), `allow_absolute_paths` (default: false)
2. **Path confinement utility**: Add `validate_path(root: &Path, requested: &Path) -> Result<PathBuf>` that:
   - Canonicalizes both paths
   - Rejects if requested escapes root (prefix check after canonicalization)
   - Rejects symlinks pointing outside root (unless `follow_symlinks=true`)
   - Applies deny patterns (glob matching)
3. **Enforce at entry points**:
   - **Scanner**: Check file size before reading; skip/deny oversized files; apply path confinement on walk
   - **CLI get/search/browse**: Validate requested paths against repo root; enforce response truncation with `truncated: true` in JSON
   - **MCP tools**: Reuse same validation; return structured error (DRAFT-00022) on violation
4. **Response truncation helper**: Generic `truncate_response<T>(data: T, limit: usize) -> TruncatedResponse<T>` that serializes, checks byte size, truncates at token/struct boundary if needed
5. **Configuration**: Expose all limits via `okc config` and config file; document in `docs/architecture/security-boundaries.md`
6. **Adversarial test suite**: 
   - Path traversal: `../../../etc/passwd`, symlink loops, absolute paths, Unicode normalization tricks
   - Oversized: 100MB file, 10MB frontmatter, deeply nested JSON response
   - Denylist bypass attempts: case variation, encoded slashes, alternate data streams
<!-- SECTION:PLAN:END -->
