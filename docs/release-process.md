---
type: Documentation
title: Release Process
description: How to create a new OKC release
tags:
  - release
  - operations
  - ci
---

# Release Process

## Overview

Releases are automated via [GitHub Actions](https://github.com/guifelix/Open-Knowledge-Catalog/actions). Pushing a
semantic version tag triggers CI validation, binary builds, GitHub Release creation, and crates.io publishing.

## Creating a Release

### 1. Prepare

```bash
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Review what's changed since last tag
git log --oneline $(git describe --tags --abbrev=0 2>/dev/null || echo 'HEAD~100')..HEAD

# Update version in Cargo.toml if needed (already at target version)
```

### 2. Generate CHANGELOG

Changelogs are auto-generated from [conventional commits](https://www.conventionalcommits.org/)
using [`git-cliff`](https://git-cliff.org):

```bash
# Dry-run to preview
git cliff --unreleased --strip header

# Generate/update CHANGELOG.md
git cliff --tag v0.1.0 -o CHANGELOG.md
```

Review and commit the result:

```bash
git add CHANGELOG.md
git commit -m "chore: prepare v0.1.0 release"
```

### 3. Tag and Push

```bash
git tag v0.1.0
git push origin v0.1.0
```

### 4. CI Does the Rest

The [release workflow](../.github/workflows/release.yml) will:

1. Run CI checks (fmt, clippy, test)
2. Build release binaries for:
   - `x86_64-unknown-linux-gnu` (Linux glibc)
   - `x86_64-unknown-linux-musl` (Linux musl — fully static)
   - `x86_64-apple-darwin` (Intel macOS)
   - `aarch64-apple-darwin` (Apple Silicon macOS)
   - `x86_64-pc-windows-msvc` (Windows)
3. Create a [GitHub Release](https://github.com/guifelix/Open-Knowledge-Catalog/releases) with binaries and checksums
4. Publish to [crates.io](https://crates.io/crates/okc)

### 5. Verify

```bash
# crates.io
cargo install okc
okc --version

# GitHub Release — download the binary for your platform
# e.g. for linux amd64:
curl -LO https://github.com/guifelix/Open-Knowledge-Catalog/releases/download/v0.1.0/okc-x86_64-unknown-linux-gnu
chmod +x okc-x86_64-unknown-linux-gnu
./okc-x86_64-unknown-linux-gnu --version
```

## Commit Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/) so `git-cliff` can
auto-generate changelogs. Prefix your commits accordingly:

| Prefix       | Changelog section |
|--------------|-------------------|
| `feat:`      | Added             |
| `fix:`       | Fixed             |
| `perf:`      | Performance       |
| `refactor:`  | Changed           |
| `docs:`      | Documentation     |
| `test:`      | Testing           |
| `chore:`     | Skipped           |
| `ci:`        | Skipped           |

Breaking changes should include `!` after the prefix: `feat!: ...` or a
`BREAKING CHANGE:` footer.

## Prerequisites

| Item | Status |
|------|--------|
| `CRATES_IO_TOKEN` GitHub secret | Set in repo settings |
| CI green on main | Verified automatically |
| CHANGELOG up-to-date | `git cliff --tag x.y.z -o CHANGELOG.md` before tagging |

## One-Time Setup

### crates.io Token

```bash
# Generate a token at https://crates.io/settings/tokens
# Then add it as a repository secret:
gh secret set CRATES_IO_TOKEN
```

Or go to GitHub → Settings → Secrets and variables → Actions → New repository secret.
