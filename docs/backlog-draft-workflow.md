---
type: guide
title: Backlog Draft Workflow
description: Workflow instructions for creating backlog drafts with complete metadata including acceptance criteria, definition of done, dependencies, and references
tags: [workflow, backlog, drafts, process]
owner: felix
status: draft
---

# How to Create Full Drafts in Backlog.md

> **For AI Agents**: This guide shows the **only way** to create drafts with complete metadata (ACs, DoD, dependencies, references, priority, type, etc.).

---

## The Constraint

**Drafts created via `backlog draft create` only support:**
- Title
- Description
- Assignee
- Status
- Labels

**They CANNOT have:**
- Acceptance Criteria (`--ac`)
- Definition of Done (`--dod`)
- Dependencies (`--dep`)
- References (`--ref`)
- Documentation links (`--doc`)
- Priority (`--priority`)
- Type (`--type`)
- Milestone, parent, modified files, etc.

---

## The Solution: Create Task → Demote to Draft

This is the **only workflow** that produces a draft with full metadata.

```bash
# 1. Create a FULL task with all metadata
backlog task create "Add JWT authentication" \
  -d $'Implement JWT-based auth with access/refresh tokens\n\n**Flow:**\n1. POST /auth/login → returns access (15m) + refresh (7d) tokens\n2. POST /auth/refresh → rotates refresh token, returns new pair\n3. POST /auth/logout → revokes refresh token\n\n**Security:**\n- RS256 signing\n- Refresh token rotation + reuse detection\n- HttpOnly Secure cookies for tokens' \
  -a "@backend-agent" \
  -s "To Do" \
  -l "backend,auth,security,feature" \
  --priority "High" \
  --type "feature" \
  --ac "User can login with email/password and receive token pair" \
  --ac "Access token expires in 15 minutes" \
  --ac "Refresh token rotates on each use; reuse revokes all sessions" \
  --ac "Logout invalidates refresh token immediately" \
  --ac "Invalid/expired access token returns 401 with WWW-Authenticate" \
  --dod "Unit tests cover token generation, validation, rotation" \
  --dod "Integration test covers full login→refresh→logout flow" \
  --dod "Security review: no tokens in logs, proper HttpOnly/Secure flags" \
  --ref "https://tools.ietf.org/html/rfc7519" \
  --ref "https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html" \
  --ref "https://auth0.com/blog/refresh-tokens-what-are-they-and-when-to-use-them/" \
  --dep "OKC-42" \
  --doc "docs/auth-architecture.md" \
  --plain

# Output: OKC-127 (task ID)

# 2. Immediately demote to draft — ALL METADATA PRESERVED
backlog task demote OKC-127
```

**Result:** A draft (`OKC-127`) in `drafts/` with:
- ✅ All ACs
- ✅ All DoD items
- ✅ All references
- ✅ All dependencies
- ✅ Priority, type, labels, assignee
- ✅ Documentation links

---

## Complete Workflow

```bash
#!/usr/bin/env bash
# create-full-draft.sh

TITLE="Add JWT authentication"
DESCRIPTION=$'Implement JWT-based auth with access/refresh tokens\n\n**Flow:**\n1. POST /auth/login → returns access (15m) + refresh (7d) tokens\n2. POST /auth/refresh → rotates refresh token, returns new pair\n3. POST /auth/logout → revokes refresh token\n\n**Security:**\n- RS256 signing\n- Refresh token rotation + reuse detection\n- HttpOnly Secure cookies for tokens'

# Create full task
# valid priority types: High, Medium, Low
# valid task types: bug, feature, enhancement, task, chore, docs, spike
TASK_ID=$(backlog task create "$TITLE" \
  -d "$DESCRIPTION" \
  -a "@backend-agent" \
  -s "To Do" \
  -l "backend,auth,security,feature" \
  --priority "High" \ 
  --type "feature" \
  --ac "User can login with email/password and receive token pair" \
  --ac "Access token expires in 15 minutes" \
  --ac "Refresh token rotates on each use; reuse revokes all sessions" \
  --ac "Logout invalidates refresh token immediately" \
  --ac "Invalid/expired access token returns 401 with WWW-Authenticate" \
  --dod "Unit tests cover token generation, validation, rotation" \
  --dod "Integration test covers full login→refresh→logout flow" \
  --dod "Security review: no tokens in logs, proper HttpOnly/Secure flags" \
  --ref "https://tools.ietf.org/html/rfc7519" \
  --ref "https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html" \
  --dep "OKC-42" \
  --doc "docs/auth-architecture.md" \
  --plain | grep -o 'OKC-[0-9]*')

echo "Created task: $TASK_ID"

# Demote to draft — THIS IS THE KEY STEP
backlog task demote "$TASK_ID"
echo "Demoted $TASK_ID to draft with full metadata preserved"

# Later, when ready to work:
# backlog draft promote "$TASK_ID"
# backlog task edit "$TASK_ID" -s "In Progress" -a "@backend-agent"
```

---

## Verify the Draft Has Full Metadata

```bash
# Check draft — all fields should be present
backlog draft view OKC-127 --plain

# Compare with task view (should be identical minus status)
backlog task view OKC-127 --plain
```

---

## When to Use This

| Scenario | Use This Workflow? |
|----------|-------------------|
| Full spec ready (ACs, DoD, deps, refs) | **YES** |
| Sprint-ready stories | **YES** |
| Requirements still evolving | No — use `backlog draft create` then promote later |
| Quick capture for later | No — use `backlog draft create` |
| Spike/exploration | No — use `backlog draft create` |

---

## Shell Quoting Rules (Critical)

```bash
# ✅ Single quotes for backticks, $, special chars
backlog task create 'Investigate `backlog init` behavior' \
  -d 'Check `backlog init --defaults` output'

# ✅ $'...' for multi-line (ANSI-C quoting)
backlog task create "Title" -d $'Line 1\nLine 2\nLine 3'

# ✅ Literal newlines (most shells)
backlog task create "Title" -d "Line 1
Line 2
Line 3"
```

---

## Minimal Copy-Paste Template

```bash
backlog task create 'Add rate limiting to API' \
  -d $'Implement token bucket rate limiter on all /api/* routes\n\nConfig:\n- 100 req/min per IP\n- 1000 req/min per authenticated user\n- Return 429 with Retry-After header' \
  -a "@backend-agent" \
  -s "To Do" \
  -l "backend,api,security,rate-limiting" \
  --priority "High" \
  --type "feature" \
  --ac "Anonymous requests limited to 100/min per IP" \
  --ac "Authenticated requests limited to 1000/min per user" \
  --ac "Returns 429 with Retry-After header when limit exceeded" \
  --dod "Unit tests cover token bucket algorithm" \
  --dod "Integration test verifies 429 response with Retry-After" \
  --ref "https://github.com/throttled/throttled" \
  --dep "OKC-88" \
  --doc "docs/rate-limiting-design.md" \
  --plain

# THEN DEMOTE:
# backlog task demote OKC-XXX
```

---

## AI Agent Checklist

- [ ] Search existing tasks/drafts first
- [ ] Use single quotes for titles/descriptions with backticks or `$`
- [ ] Use `$'...'` or literal newlines for multi-line descriptions
- [ ] Set assignee to your agent identifier (`@backend-agent`)
- [ ] Use `To Do` status for initial creation
- [ ] Apply labels: component, type, priority, domain
- [ ] Include ALL: `--ac`, `--dod`, `--ref`, `--dep`, `--doc`
- [ ] Set `--priority` and `--type`
- [ ] **Immediately demote**: `backlog task demote <id>`
- [ ] Record task ID
- [ ] When ready: `backlog draft promote <id>` → `backlog task edit <id> -s "In Progress" -a "@agent"`

---

## Why Not `backlog draft create`?

```bash
# This ONLY creates a draft with: title, description, assignee, status, labels
backlog draft create "Title" -d "Desc" -a "@agent" -s "Draft" -l "labels"

# NO ACs, NO DoD, NO deps, NO refs, NO priority, NO type, NO docs
# You'd have to promote THEN edit to add them — two extra steps
```

**Task → Demote is atomic and complete.**