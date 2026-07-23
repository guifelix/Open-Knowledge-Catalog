---
title: "Complex OKF Repository"
description: "Complex test fixture with nested dirs, circular links, broken links, custom metadata"
type: "Index"
tags: ["test", "fixture", "complex", "validation"]
owner: "test-team"
status: "active"
---

# Complex OKF Repository

A comprehensive test fixture repository for the Open Knowledge Catalog.

## Structure

- **analytics/** - Analytics metrics, datasets, glossary
- **financial/** - Financial metrics
- **circular-a.md** - Circular link test (links to circular-b.md)
- **circular-b.md** - Circular link test (links to circular-a.md)
- **broken-links.md** - Broken link validation test
- **invalid-yaml.md** - Invalid YAML front-matter test

## Quick Links

- [Analytics Knowledge Base](analytics/index.md)
- [Financial Metrics](financial/index.md)
- [Circular Link A](circular-a.md)
- [Circular Link B](circular-b.md)
- [Broken Links Test](broken-links.md)