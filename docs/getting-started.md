# Getting Started

## Installation

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- SQLite3 development headers (usually `libsqlite3-dev` on Linux)

### Build from Source

```bash
git clone https://github.com/your-org/open-knowledge-catalog
cd open-knowledge-catalog
cargo build --release
```

The binary will be at `target/release/okc`.

### Install

```bash
cargo install --path .
```

Or copy the binary to your PATH:

```bash
cp target/release/okc ~/.local/bin/
```

## Quick Start

### 1. Create an OKF Repository

```bash
mkdir -p my-knowledge/{metrics,datasets}
```

Create `my-knowledge/metrics/monthly-revenue.md`:

```markdown
---
type: Metric
title: Monthly Revenue
description: Recognized recurring revenue for the month
tags: [finance, executive]
owner: Finance Analytics
status: published
---

# Definition

Monthly Revenue represents the total recognized revenue for a calendar month.

# Recognition Rules

Revenue is recognized when:
1. Service is delivered
2. Payment is reasonably assured
3. Amount is measurable
```

Create `my-knowledge/datasets/customer-orders.md`:

```markdown
---
type: Dataset
title: Customer Orders
description: Raw order data from the e-commerce platform
tags: [sales, raw-data]
owner: Data Engineering
status: published
---

# Schema

| Column | Type | Description |
|--------|------|-------------|
| order_id | string | Unique order identifier |
| customer_id | string | Customer identifier |
| order_date | date | Date of order |
| amount | decimal | Order total in USD |
```

### 2. Scan the Repository

```bash
okc scan --root my-knowledge
```

Output:
```
Scan complete:
  Total files: 2
  Added: 2
  Modified: 0
  Deleted: 0
  Parse failures: 0
  Broken links: 0
  Total links: 0
  Duration: 0.01s
```

This creates `okc_index.db` in the current directory (configurable via `--db-path`).

### 3. Query the Knowledge Base

```bash
# Browse the hierarchy
okc browse

# Browse a specific directory
okc browse metrics --depth 1

# Search for concepts
okc search "revenue recognition"

# Get a document
okc get metrics/monthly-revenue.md --include metadata,headings,body

# Extract a specific section
okc section metrics/monthly-revenue.md "Recognition Rules"

# Exact metadata query
okc metadata --filter type=Metric --filter tags_contains=finance --select path,title,owner

# View links
okc links metrics/monthly-revenue.md
okc backlinks metrics/monthly-revenue.md

# Traverse the graph
okc traverse metrics/monthly-revenue.md --max-depth 2

# Validate
okc validate

# Statistics
okc stats
```

## Command-Line Options

```bash
okc --help
```

Global options:
- `--root <PATH>` - Root directory to scan (can be specified multiple times)
- `--db-path <PATH>` - SQLite database path (default: `okc_index.db`)
- `--config <PATH>` - Configuration file (not yet implemented)

## Configuration File (Planned)

Future versions will support a TOML config file:

```toml
[scanner]
roots = ["./knowledge"]
exclude_patterns = [".git/", "node_modules/", "target/", ".env*"]
max_file_size = 2097152          # 2 MB
max_front_matter_size = 65536    # 64 KB
follow_symlinks = false

[indexer]
max_scan_results = 1000
max_graph_depth = 5
max_graph_nodes = 100
max_response_chars = 500000

[validation]
require_index_files = false
```