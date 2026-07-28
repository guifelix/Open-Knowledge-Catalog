---
type: design
title: Implementation Plan
description: Design document outlining the practical implementation plan for the Open Knowledge Catalog
tags: [design, implementation, planning]
owner: felix
status: draft
---

# Open Knowledge Catalog

Here is a practical implementation plan.

Implementation Plan: OKF Repository Tool for AI Agents

1. Objective

Build a local-first tool that allows an AI agent to safely browse, parse, search, and reason over an Open Knowledge Format repository.

The tool will treat an OKF repository as a collection of Markdown documents containing:

- YAML front matter
- Markdown content
- directory hierarchy
- links between documents
- optional directory-level "index.md" files

The tool will convert this filesystem-based knowledge into a structured, searchable representation that an AI can query through a small set of deterministic operations.

The tool is not intended to generate answers itself. Its purpose is to retrieve accurate, bounded, source-backed context that an AI model can use to answer questions.

2. What the Tool Does

The tool will:

1. Walk one or more approved repository directories.
2. Discover Markdown files.
3. Extract and parse YAML front matter.
4. Parse Markdown headings and links.
5. Build a directory hierarchy.
6. Build a graph of relationships between documents.
7. Index metadata and textual content.
8. Detect changed, added, and deleted files.
9. Expose repository operations to AI agents.
10. Return compact results with source paths and relevant excerpts.

The resulting system should support both deterministic queries and exploratory retrieval.

Examples of deterministic queries:

- Find all concepts with "type: Metric".
- Find documents tagged "security".
- List documents with malformed front matter.
- Find broken Markdown links.
- Return all backlinks to a concept.
- List recently modified documents.

Examples of exploratory queries:

- Find documentation related to OAuth retries.
- Locate the definition of monthly recurring revenue.
- Find concepts connected to the customer orders dataset.
- Identify the most relevant section explaining deployment rollback.

3. Why This Helps in an OKF and AI Setup

OKF gives knowledge a portable, human-readable representation, but it does not by itself provide fast retrieval, structured querying, validation, or an AI tool interface.

Without a dedicated tool, an AI agent would need to:

- recursively inspect the filesystem;
- open many files individually;
- repeatedly parse front matter;
- search raw Markdown text;
- resolve relative links;
- infer directory structure;
- manage its own context limits.

That approach is slow, expensive, inconsistent, and difficult to secure.

The proposed tool moves those responsibilities into deterministic software.

Instead of asking the AI to understand the entire repository directly, the tool gives the AI a controlled view of it:

OKF files
    ↓
scanner and parser
    ↓
structured index
    ↓
bounded AI tool calls
    ↓
relevant source context
    ↓
AI answer

This improves the setup in several ways.

Performance

The repository is parsed once and updated incrementally. Unchanged files do not need to be reparsed for every AI request.

Accuracy

YAML metadata is queried as structured data instead of searched as plain text.

For example, a query for:

status: draft

will match the actual front-matter field, not prose or examples containing the same text.

Context efficiency

The AI receives only relevant metadata, headings, excerpts, or sections rather than complete files or the entire repository.

Navigability

The directory hierarchy supports progressive browsing, while the document graph supports link-following and relationship reasoning.

Safety

The tool can restrict accessible directories, file types, file sizes, traversal depth, output size, and symlink handling.

Source traceability

Every result includes its repository path and relevant source location, allowing the AI to cite or reopen the original document.

4. Core Architecture

The system should have five main layers.

1. Filesystem layer
2. Parsing layer
3. Repository model
4. Index and storage layer
5. AI tool interface

5. Filesystem Layer

The filesystem layer discovers repository files.

Responsibilities:

- recursively walk approved roots;
- include Markdown files;
- optionally respect ".gitignore";
- skip hidden or excluded directories;
- normalize paths;
- enforce symlink policy;
- collect file size and modification time;
- detect added, changed, and deleted files.

Recommended Rust libraries:

- "ignore" for repository traversal and ignore-file support;
- "notify" for filesystem watching in a long-running process.

The initial implementation does not need a watcher. A scan-on-start strategy with incremental comparison is sufficient.

Each discovered file should produce a record similar to:

{
  "path": "metrics/monthly-revenue.md",
  "absolute_path": "/repository/metrics/monthly-revenue.md",
  "size": 4821,
  "modified_at": 1784214000
}

The absolute path should remain internal. AI-facing responses should normally use repository-relative paths.

6. Parsing Layer

6.1 Front-Matter Extraction

Read only the beginning of each Markdown file until the closing front-matter delimiter is found.

Expected format:

---
type: Metric
title: Monthly Revenue
tags:
  - finance
  - executive
---

# Definition

The extractor should:

- support UTF-8 BOMs;
- recognize the opening delimiter only at the beginning;
- support "\n" and "\r\n";
- enforce a maximum front-matter size;
- report missing closing delimiters;
- preserve the raw YAML for diagnostics.

Do not parse the entire Markdown body merely to find front matter.

6.2 YAML Parsing

Parse the extracted front matter into structured values.

The parser should preserve custom OKF fields rather than requiring a fixed schema.

A normalized document record might contain:

{
  "type": "Metric",
  "title": "Monthly Revenue",
  "description": "Recognized recurring revenue for the month.",
  "tags": ["finance", "executive"],
  "custom": {
    "owner": "Finance Analytics",
    "status": "published"
  }
}

Standard fields may be promoted into dedicated columns. Unknown fields should remain available as a generic map.

6.3 Markdown Parsing

Parse the body only for information needed by the index.

Initially extract:

- headings and heading levels;
- links;
- plain searchable text;
- section boundaries;
- optional code-block exclusion.

Recommended library:

- "pulldown-cmark" for event-based Markdown parsing.

Avoid building a full Markdown AST unless document transformation becomes a requirement.

6.4 Link Resolution

Resolve relative links against the source document path.

For example:

[Customer orders](../datasets/customer-orders.md)

should become:

datasets/customer-orders.md

Store:

- the raw link;
- the normalized target;
- whether the target exists;
- optional anchor fragments;
- external versus internal status.

7. Repository Model

The repository model should contain both a hierarchy and a graph.

7.1 Directory Tree

The tree represents filesystem containment.

/
├── datasets/
│   ├── index.md
│   └── customer-orders.md
└── metrics/
    ├── index.md
    └── monthly-revenue.md

Use it for:

- browsing;
- progressive disclosure;
- reading directory summaries;
- limiting searches to a subtree;
- answering “what is under this area?” questions.

Each directory node may include:

{
  "path": "metrics",
  "index_document": "metrics/index.md",
  "child_directories": [],
  "documents": [
    "metrics/monthly-revenue.md"
  ]
}

7.2 Document Graph

The graph represents relationships between concepts.

Initial edge types:

- "contains"
- "parent"
- "links_to"
- "linked_from"

Later edge types may be derived from front matter:

- "depends_on"
- "owned_by"
- "implements"
- "uses"
- "related_to"

Do not infer semantic relationship types from prose in the first version. Start with explicit Markdown links and explicit metadata fields.

A graph edge should look like:

{
  "source": "metrics/monthly-revenue.md",
  "target": "datasets/customer-orders.md",
  "relation": "links_to"
}

The tree and graph should coexist. The directory hierarchy should remain a first-class API even if containment is also represented as graph edges.

8. Storage and Indexing

Use SQLite as the first storage engine.

SQLite is suitable because it provides:

- structured metadata storage;
- transactions;
- incremental updates;
- indexes;
- JSON support;
- full-text search through FTS5;
- simple deployment;
- no external service requirement.

Suggested tables:

documents
directories
document_tags
headings
links
metadata_fields
scan_errors

A simplified schema:

CREATE TABLE documents (
    id INTEGER PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    parent_path TEXT NOT NULL,
    title TEXT,
    type TEXT,
    description TEXT,
    body_text TEXT,
    file_size INTEGER NOT NULL,
    modified_at INTEGER NOT NULL,
    content_hash TEXT,
    parse_status TEXT NOT NULL
);

CREATE TABLE document_tags (
    document_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    FOREIGN KEY(document_id) REFERENCES documents(id)
);

CREATE TABLE headings (
    document_id INTEGER NOT NULL,
    level INTEGER NOT NULL,
    title TEXT NOT NULL,
    anchor TEXT,
    position INTEGER,
    FOREIGN KEY(document_id) REFERENCES documents(id)
);

CREATE TABLE links (
    source_document_id INTEGER NOT NULL,
    target_path TEXT,
    target_anchor TEXT,
    external_url TEXT,
    exists_in_repository INTEGER NOT NULL,
    FOREIGN KEY(source_document_id) REFERENCES documents(id)
);

Use an FTS5 table for searchable text:

CREATE VIRTUAL TABLE document_search USING fts5(
    path,
    title,
    description,
    headings,
    body
);

Search ranking should prioritize fields roughly as follows:

title        highest weight
description
headings
body         lowest weight

SIMDJSON is not needed. It parses JSON quickly but does not provide search or indexing.

9. Incremental Indexing

The first scan processes every Markdown file.

Subsequent scans should compare:

- repository-relative path;
- modification time;
- file size.

If these values are unchanged, skip parsing.

For stronger correctness, calculate a content hash only when modification time or size changes.

Incremental update process:

discover current files
    ↓
compare with stored file records
    ↓
parse new and modified files
    ↓
delete records for removed files
    ↓
rebuild affected links and search entries

Do not rebuild the complete database after every change.

10. AI Tool Interface

Expose a small number of high-level operations.

The AI should not receive unrestricted shell or filesystem access through this tool.

10.1 "browse_directory"

Purpose: Inspect one area of the OKF hierarchy.

Input:

{
  "path": "metrics",
  "depth": 1,
  "limit": 50
}

Output:

{
  "path": "metrics",
  "summary_document": "metrics/index.md",
  "directories": [],
  "documents": [
    {
      "path": "metrics/monthly-revenue.md",
      "title": "Monthly Revenue",
      "type": "Metric",
      "description": "Recognized recurring revenue for the month."
    }
  ],
  "truncated": false
}

10.2 "get_document"

Purpose: Retrieve one known concept.

Input:

{
  "path": "metrics/monthly-revenue.md",
  "include": [
    "metadata",
    "headings",
    "body"
  ],
  "max_body_chars": 12000
}

The response should clearly report truncation.

10.3 "search_documents"

Purpose: Search repository content.

Input:

{
  "query": "revenue recognition",
  "path_prefix": "metrics",
  "types": ["Metric", "Policy"],
  "tags": ["finance"],
  "limit": 20
}

Output:

{
  "results": [
    {
      "path": "metrics/monthly-revenue.md",
      "title": "Monthly Revenue",
      "type": "Metric",
      "score": 12.48,
      "matching_section": "Recognition rules",
      "excerpt": "Revenue is recognized when..."
    }
  ],
  "total_matches": 7,
  "truncated": false
}

10.4 "query_metadata"

Purpose: Perform exact structured filtering.

Input:

{
  "where": {
    "type": "Metric",
    "status": "published",
    "tags_contains": "finance"
  },
  "select": [
    "path",
    "title",
    "owner"
  ],
  "limit": 100
}

This should not use full-text search.

10.5 "get_links"

Purpose: Retrieve outgoing links from a document.

Input:

{
  "path": "metrics/monthly-revenue.md"
}

10.6 "get_backlinks"

Purpose: Find documents that reference a concept.

Input:

{
  "path": "datasets/customer-orders.md",
  "limit": 50
}

10.7 "traverse_graph"

Purpose: Explore related concepts.

Input:

{
  "start": "metrics/monthly-revenue.md",
  "relations": ["links_to"],
  "max_depth": 2,
  "max_nodes": 30
}

10.8 "get_section"

Purpose: Retrieve a specific Markdown section without returning the complete document.

Input:

{
  "path": "metrics/monthly-revenue.md",
  "heading": "Recognition rules",
  "max_chars": 8000
}

This operation is particularly useful for controlling AI context size.

10.9 "validate_repository"

Purpose: Report structural problems.

Checks should include:

- invalid YAML;
- missing required metadata;
- duplicate concept identifiers;
- broken internal links;
- missing directory index files, when required by local policy;
- unsupported encoding;
- oversized front matter;
- malformed Markdown links.

11. How an AI Would Use the Tool

The AI should combine deterministic retrieval with iterative navigation.

Scenario 1: Direct concept lookup

User asks:

«What is monthly recurring revenue?»

AI process:

1. search_documents("monthly recurring revenue")
2. inspect top results
3. get_document(best match, metadata + headings)
4. get_section("Definition")
5. answer with the source path

Scenario 2: Hierarchical browsing

User asks:

«What metrics are available for customer engagement?»

AI process:

1. browse_directory("/")
2. identify "metrics" or "engagement"
3. browse_directory("metrics/engagement")
4. inspect concept titles and descriptions
5. open only relevant documents
6. summarize the available metrics

This follows OKF's progressive-disclosure model.

Scenario 3: Relationship reasoning

User asks:

«Which datasets are used to calculate monthly revenue?»

AI process:

1. search_documents("monthly revenue")
2. select metrics/monthly-revenue.md
3. get_links(metrics/monthly-revenue.md)
4. inspect linked dataset concepts
5. optionally traverse_graph(depth = 2)
6. answer with linked sources

Scenario 4: Exact metadata query

User asks:

«List all published finance metrics owned by Analytics.»

AI process:

1. query_metadata({
     type: "Metric",
     status: "published",
     tags_contains: "finance",
     owner: "Analytics"
   })
2. return the matching concepts

No semantic search or LLM interpretation is required for this step.

Scenario 5: Repository validation

User asks:

«Are there broken references in this knowledge repository?»

AI process:

1. validate_repository()
2. group broken links by source document
3. explain the affected concepts

12. AI Usage Principles

The tool contract should encourage the model to:

1. Browse narrowly before reading broadly.
2. Use metadata filters for exact conditions.
3. Use text search for lexical discovery.
4. Follow graph links for related concepts.
5. Retrieve individual sections instead of entire documents.
6. Include source paths in final answers.
7. Stop traversal when evidence is sufficient.

The tool should enforce limits even when the AI requests excessive output.

13. Security and Resource Limits

Required protections:

- fixed allowed repository roots;
- no ".." path escape;
- no arbitrary absolute paths;
- configurable symlink policy;
- maximum file size;
- maximum front-matter size;
- maximum number of scan results;
- maximum graph depth;
- maximum graph nodes;
- maximum response characters;
- binary-file rejection;
- excluded secret directories;
- read-only operation by default.

Suggested excluded paths:

.git/
node_modules/
vendor/
target/
.env*
secrets/
credentials/

The exclusion policy should be configurable because some repositories may intentionally document similarly named concepts.

14. Error Handling

Parsing failures should not stop the complete scan.

Store errors as structured records:

{
  "path": "metrics/broken.md",
  "stage": "yaml",
  "message": "Unexpected scalar at line 4",
  "line": 4
}

Documents with invalid metadata may still be indexed for path and body search, but their parse status must be visible.

The tool should distinguish:

- unreadable file;
- invalid UTF-8;
- malformed front matter;
- invalid YAML;
- malformed Markdown;
- unresolved link;
- truncated content.

15. Observability

Track at least:

- number of discovered files;
- number of parsed files;
- number of unchanged files skipped;
- number of parse failures;
- number of broken links;
- total scan duration;
- parsing time;
- database update time;
- average tool-response size;
- search latency.

This will reveal whether the actual bottleneck is filesystem access, parsing, indexing, or AI consumption.

16. Development Phases

Phase 1: Minimal repository reader

Implement:

- approved-root configuration;
- Markdown file traversal;
- front-matter extraction;
- YAML parsing;
- normalized document records;
- basic CLI output.

Deliverable:

scan repository → print parsed concepts and errors

Phase 2: Markdown structure

Add:

- heading extraction;
- internal link extraction;
- relative-path resolution;
- broken-link detection;
- directory tree construction.

Deliverable:

repository tree + document graph

Phase 3: Persistent index

Add:

- SQLite schema;
- incremental file updates;
- metadata indexes;
- FTS5 search;
- deleted-file handling.

Deliverable:

fast repeated metadata and text queries

Phase 4: AI-facing operations

Expose:

- "browse_directory";
- "get_document";
- "get_section";
- "search_documents";
- "query_metadata";
- "get_links";
- "get_backlinks";
- "traverse_graph";
- "validate_repository".

Transport options:

- MCP server;
- local HTTP API;
- command-line JSON interface;
- native function calls inside an existing agent runtime.

MCP is a strong default when multiple AI clients need to use the tool.

Phase 5: Continuous updates

Add:

- filesystem watcher;
- debounced updates;
- partial graph rebuilding;
- index health reporting.

Phase 6: Optional advanced retrieval

Only add these after measuring real retrieval failures:

- fuzzy filename matching;
- trigram search;
- semantic embeddings;
- reranking;
- generated directory summaries;
- PageIndex-style hierarchical reasoning;
- relationship extraction from custom metadata.

17. Suggested Rust Project Structure

src/
├── main.rs
├── config.rs
├── scanner/
│   ├── mod.rs
│   ├── walker.rs
│   └── changes.rs
├── parser/
│   ├── mod.rs
│   ├── frontmatter.rs
│   ├── yaml.rs
│   ├── markdown.rs
│   └── links.rs
├── model/
│   ├── mod.rs
│   ├── document.rs
│   ├── directory.rs
│   └── graph.rs
├── index/
│   ├── mod.rs
│   ├── database.rs
│   ├── metadata.rs
│   ├── fulltext.rs
│   └── migrations.rs
├── service/
│   ├── browse.rs
│   ├── search.rs
│   ├── documents.rs
│   ├── graph.rs
│   └── validation.rs
└── transport/
    ├── mod.rs
    ├── mcp.rs
    ├── http.rs
    └── cli.rs

18. Recommended Initial Technology Stack

Language:             Rust
Filesystem traversal: ignore
File watching:         notify, later
YAML parsing:          Serde-compatible maintained YAML parser
Markdown parsing:      pulldown-cmark
Storage:               SQLite
Full-text search:      SQLite FTS5
Serialization:         serde + serde_json
AI transport:          MCP or JSON-based local API

"serde_json" is sufficient for request and response serialization. SIMDJSON is not necessary unless benchmarks later show that parsing very large JSON payloads is a meaningful bottleneck.

19. Testing Strategy

Unit tests

Test:

- valid front matter;
- missing closing fence;
- BOM handling;
- Windows line endings;
- malformed YAML;
- nested YAML fields;
- heading extraction;
- relative link resolution;
- anchors;
- external links;
- path normalization.

Integration tests

Create fixture repositories containing:

- nested directories;
- "index.md" files;
- valid and invalid documents;
- circular links;
- broken links;
- duplicate titles;
- custom metadata;
- deleted and modified files.

Retrieval tests

Define representative AI questions and verify that the tool returns the required evidence.

Examples:

Question: What calculates monthly revenue?
Expected concepts:
- metrics/monthly-revenue.md
- datasets/customer-orders.md

These tests should evaluate retrieval results, not the final wording generated by an AI model.

Performance tests

Measure:

- cold full scan;
- warm incremental scan;
- metadata-filter latency;
- full-text search latency;
- graph traversal latency;
- memory usage;
- response size.

20. Definition of the First Useful Release

The first useful release should:

- scan an OKF repository;
- parse YAML front matter;
- extract headings and internal links;
- store documents in SQLite;
- support incremental rescans;
- provide metadata filtering;
- provide FTS5 text search;
- browse directories;
- retrieve one document or section;
- return outgoing and incoming links;
- validate malformed metadata and broken links;
- expose all operations through MCP or a JSON API;
- enforce path and output limits.

It should not initially include:

- embeddings;
- vector databases;
- LLM-generated summaries;
- semantic relationship extraction;
- document mutation;
- distributed indexing;
- custom search infrastructure.

21. Final Architecture

OKF repository
    ↓
parallel filesystem walker
    ↓
front-matter and Markdown parsers
    ↓
normalized documents
    ├── directory hierarchy
    ├── metadata
    ├── headings and sections
    └── document-link graph
    ↓
SQLite
    ├── metadata indexes
    ├── FTS5 text index
    └── graph edges
    ↓
bounded AI tools
    ├── browse
    ├── search
    ├── filter
    ├── read section
    ├── follow links
    └── validate
    ↓
AI-generated answer with source paths

The core design principle is simple:

«Use deterministic software to discover, parse, filter, and retrieve knowledge. Use the AI only to choose retrieval steps, combine evidence, and explain the result.»

This keeps the OKF repository human-readable while making it efficient, safe, and useful for AI agents.