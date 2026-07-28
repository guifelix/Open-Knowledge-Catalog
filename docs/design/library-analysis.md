---
type: analysis
title: Library Analysis
description: Evaluation of Rust libraries for OKF scanning, indexing, AI retrieval, and CLI tooling
tags: [analysis, libraries, dependencies, rust]
owner: felix
status: stable
---

Yes. Assuming Rust, these are the libraries I would seriously evaluate for an OKF scanner and AI retrieval tool.

Recommended baseline stack

[dependencies]
ignore = "..."
memchr = "..."
serde = { version = "...", features = ["derive"] }
serde_json = "..."
serde-saphyr = "..."
pulldown-cmark = "..."
rusqlite = { version = "...", features = ["bundled"] }
petgraph = "..."
rmcp = "..."
blake3 = "..."
thiserror = "..."
tracing = "..."
tracing-subscriber = "..."

Later:

notify = "..."
tantivy = "..."
rayon = "..."

My default selection would be:

Layer	Default choice	Why

Filesystem traversal	ignore	Fast parallel walking and repository ignore rules
Front-matter boundaries	memchr plus custom code	Tiny, fast and fully controlled
YAML	serde-saphyr	Serde integration, source spans and defensive parsing
Markdown	pulldown-cmark	Streaming event parser without a heavy AST
Storage	rusqlite	Structured metadata, FTS5 and graph edges in one database
Graph algorithms	SQL first, petgraph optionally	Avoid unnecessary in-memory duplication
AI interface	rmcp	Official Rust MCP SDK
Change detection	notify	Incremental updates after the first release
Full-text search	SQLite FTS5 first	Simplest embedded solution
Advanced search	tantivy later	More control over fields, tokenization and ranking


1. Filesystem traversal

ignore

Recommended.

ignore is the library used for ripgrep-style directory traversal. WalkBuilder supports ignore files, glob overrides, hidden-file handling and file-type filtering. WalkParallel performs recursive traversal across multiple threads. 

Use it for:

.gitignore handling

custom exclusions

parallel discovery

multiple repository roots

directory-depth limits

hidden-file policies


use ignore::{WalkBuilder, WalkState};

WalkBuilder::new(root)
    .standard_filters(true)
    .hidden(false)
    .follow_links(false)
    .threads(thread_count)
    .build_parallel()
    .run(|| {
        Box::new(|entry| {
            // Filter and process paths.
            WalkState::Continue
        })
    });

It is the best fit for an OKF repository because an OKF bundle will often live inside a software or documentation repository where ignore semantics already matter.

walkdir

Use walkdir when you want a simple, synchronous iterator with fewer behavioural assumptions. It supports efficient recursive traversal, skipping subtrees, symlink controls and limits on simultaneously open file descriptors. It can also avoid crossing filesystem boundaries. 

Choose it over ignore when:

you do not want .gitignore semantics;

deterministic serial ordering matters;

you want to parallelize processing separately;

the scanner is intentionally minimal.


jwalk

Worth benchmarking for raw parallel directory traversal, but it provides less repository-aware policy than ignore. I would not choose it merely because it advertises parallel traversal. The overall workload will probably be dominated by opening, parsing and indexing files rather than enumerating several hundred paths.

async-walkdir

This wraps blocking filesystem work for async runtimes. It is useful when traversal must fit into an existing async service, but async filesystem APIs do not magically make local disk traversal faster. 

My recommendation remains to perform scanning on a bounded blocking worker pool and keep the MCP or HTTP server async.

2. File filtering and path handling

globset

globset is useful when users can configure patterns such as:

**/*.md
!archive/**
!**/generated/**

However, ignore already incorporates relevant glob and override functionality. Add globset directly only when your tool needs a separately compiled filter API.

camino

camino provides UTF-8 path types. It can make JSON responses and database keys easier because AI-facing paths must ultimately be strings.

The tradeoff is that real filesystem paths are not guaranteed to be UTF-8 on every platform. A robust design is:

use std::path::PathBuf internally for filesystem operations;

convert repository-relative paths into a defined escaped or UTF-8 representation at the API boundary;

reject or specially report paths that cannot be represented safely.


Do not casually call to_string_lossy() for identity keys. Two distinct paths could theoretically collapse to the same lossy representation.

dunce

Useful on Windows for normalizing certain canonicalized path forms. Do not use canonicalization as the only path-containment defence, particularly when files can disappear or symlinks can change during scanning.

path-clean

Useful for lexical normalization of paths containing . and ... It does not resolve symlinks and therefore should not be treated as a complete sandboxing mechanism.

3. Reading files and finding front matter

memchr

Recommended.

memchr provides optimized byte searching. Use it to find line endings or candidate delimiter bytes while locating the closing front-matter fence.

You do not need a large front-matter framework for this:

optional UTF-8 BOM
---
YAML bytes
---
Markdown body

A custom extractor gives you direct control over:

front-matter size limits;

\n versus \r\n;

accepted closing delimiters;

malformed input;

exact byte ranges;

diagnostic line numbers.


For this particular problem, custom boundary extraction is more reliable than adopting another crate’s opinionated document convention.

bstr

Useful when you want byte-oriented string operations without requiring UTF-8 at every intermediate step.

A possible flow is:

read bytes
→ detect BOM/encoding
→ identify front-matter byte range
→ validate/decode that range
→ parse YAML

This allows the scanner to return a controlled “invalid UTF-8” diagnostic rather than failing unpredictably.

encoding_rs and encoding_rs_io

Only add these when non-UTF-8 Markdown must be supported.

OKF should ideally mandate UTF-8 for portability. Supporting arbitrary encodings complicates:

byte offsets;

line and column reporting;

content hashing;

Markdown source ranges;

source citations.


My recommendation is UTF-8 only in version one.

memmap2

Do not use memory mapping by default.

It may help with very large, repeatedly accessed documents, but for hundreds or thousands of small Markdown files it adds lifecycle and file-mutation complexity. Ordinary buffered reads are likely simpler and sufficient.

4. YAML front-matter parsing

This is the least settled part of the Rust stack.

Avoid serde_yaml

The original serde_yaml project explicitly states that it is no longer maintained. Do not start a new project on it. 

serde-saphyr

My current first candidate.

serde-saphyr provides Serde-based YAML deserialization and emphasizes panic-free malformed-input handling, no unsafe code in the library, and direct deserialization without requiring an intermediate generic YAML tree. It also exposes source spans, including byte-position information for string input. 

Why it fits your tool:

you process untrusted or imperfect repository content;

good diagnostics matter;

exact source positions are valuable;

typed standard fields can coexist with generic custom metadata;

direct Serde integration keeps the model simple.


Potential concern:

it is a newer ecosystem than the retired serde_yaml;

you should build parser conformance fixtures before committing.


A useful data model could be:

#[derive(Debug, serde::Deserialize)]
struct FrontMatter {
    #[serde(rename = "type")]
    concept_type: String,

    title: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,

    #[serde(flatten)]
    custom: std::collections::BTreeMap<String, serde_json::Value>,
}

The exact flatten representation may require a YAML value type before conversion to JSON, particularly because YAML supports key and scalar types JSON does not.

noyalib

noyalib is a pure safe Rust YAML 1.2 implementation with Serde integration, typed deserialization, a generic Value, streaming support and configurable YAML-version behaviour. It does not preserve comments or original formatting when round-tripping through values or structs. 

This is a credible candidate when:

pure Rust is important;

YAML 1.2 correctness matters;

you want explicit version controls;

you may need streaming deserialization.


I would benchmark and fuzz-test noyalib against serde-saphyr with your actual front matter.

fyaml

fyaml wraps libfyaml and offers a Serde-compatible value representation, DOM navigation and YAML path queries. Its documentation distinguishes it from pure-Rust alternatives such as Saphyr or yaml-rust2. 

Choose it when:

full YAML behaviour is important;

DOM navigation is useful;

a C dependency is acceptable;

YAML itself is becoming a substantial subsystem.


I would avoid it for a portable single-binary MVP unless its conformance or performance clearly beats the pure-Rust choices.

yaml-rust2

This is a lower-level pure-Rust YAML parser. It exposes parsing events and its own Yaml representation rather than being primarily a direct typed-Serde solution. 

Use it when:

you want control over parser events;

you need a YAML syntax tree;

you plan to build normalization yourself;

Serde is not central.


For your tool, it likely creates more integration work than serde-saphyr or noyalib.

Important YAML policy decisions

Regardless of library, define these explicitly:

YAML 1.1 or 1.2 scalar semantics

duplicate-key behaviour

anchors and aliases

merge-key support

custom tags

maximum alias expansion

maximum nesting depth

maximum scalar size

whether non-string map keys are accepted

whether timestamps become strings or typed values


YAML 1.2 avoids surprising YAML 1.1 boolean interpretations such as treating certain ordinary words as booleans. The ecosystem is actively moving toward stricter YAML 1.2 behaviour. 

For OKF, I would restrict accepted front matter to a JSON-compatible YAML subset:

maps with string keys
arrays
strings
numbers
booleans
null

Reject or normalize aliases, custom tags and non-string keys. That creates a stable AI-facing data model.

5. Front-matter convenience crates

pulldown-cmark-frontmatter

This integrates front matter with pulldown-cmark, but its documented format is opinionated and not necessarily identical to OKF’s expected --- YAML block convention. The crate also states that it is unaffiliated with pulldown-cmark. 

I would not use it as a foundational dependency.

Your boundary parser is a small security-sensitive component. Owning those roughly 50 to 100 lines is justified.

gray_matter and similar crates

These may be useful for prototypes, but they bundle boundary recognition and multiple front-matter formats into an abstraction you probably do not need.

For an OKF-specific tool, explicit format handling is better than generic convenience.

6. Markdown parsing

pulldown-cmark

Recommended.

pulldown-cmark is a pull-based CommonMark parser that yields an iterator of events. It supports optional extensions such as tables, footnotes, task lists, math and selected GFM features. 

It fits because you need to extract:

headings;

links;

plain text;

section ranges;

possibly code blocks;

perhaps tables.


You do not initially need a mutable AST.

A single pass can build:

struct ParsedMarkdown {
    headings: Vec<Heading>,
    links: Vec<Link>,
    sections: Vec<Section>,
    searchable_text: String,
}

Its parser can be used as an event iterator, which allows you to avoid constructing a large object tree for every document. 

comrak

Choose comrak when:

exact CommonMark plus GFM behaviour is critical;

you need a navigable AST;

you will modify or regenerate Markdown;

extension compatibility outweighs lower allocation.


It is more appropriate for a future “edit or transform OKF documents” feature than for the read-only indexing MVP.

markdown-it

Choose a Markdown-it style parser when extensibility and custom syntax rules are core requirements. The Markdown-it architecture is designed around configurable rules and syntax extensions. 

That could matter if OKF eventually defines custom link syntax or semantic blocks. Today, it is likely unnecessary.

pulldown-cmark-to-cmark

Useful only when you need to transform events and write Markdown back. It converts pulldown-cmark events into Markdown. 

Do not add it to the read-only scanner.

7. Link and URL handling

url

Use url for external URLs and URI parsing.

Do not use it directly for every Markdown link, because relative Markdown links are filesystem-like references, not necessarily web URLs.

Recommended distinction:

https://...       → url::Url
mailto:...        → url::Url
../metrics/a.md   → repository path resolver
#definition       → same-document anchor

percent-encoding

Useful if links or anchors may include encoded components.

Slug generation library or custom implementation

Heading-anchor generation is not universal. GitHub, CommonMark renderers and static-site generators can generate different anchors.

Do not pretend there is one canonical anchor algorithm. Store:

raw heading text;

source position;

explicitly written anchor, when available;

a configured generated slug;

parser profile used to generate it.


For broken-anchor validation, your tool may need a configurable profile such as github, mdbook, or literal.

8. Concurrency

rayon

Use Rayon when parsing and normalization are CPU-bound and you already have a collection of files.

However, do not combine every available source of parallelism:

ignore WalkParallel
+ Rayon parallel iterator
+ async spawn_blocking per file

That can oversubscribe the machine.

Pick one bounded concurrency owner.

A reasonable design:

serial or parallel discovery
→ bounded channel
→ fixed parsing worker pool
→ single database writer

SQLite writes should normally be serialized or batched through one writer connection.

crossbeam-channel

Useful for a scanner pipeline with bounded backpressure:

walker → file queue → parser workers → database queue

A bounded queue prevents the scanner from loading thousands of file paths or parsed bodies into memory while the database falls behind.

tokio

Use Tokio for:

MCP transport;

HTTP transport;

cancellation;

watcher events;

server lifecycle.


Do not use Tokio as a reason to make the core filesystem parser async. Keep the indexing engine mostly synchronous and invoke it from a bounded blocking task.

tokio-util

Useful for CancellationToken, allowing a scan to stop cleanly when:

the server shuts down;

the client cancels;

a repository is removed;

a resource limit is exceeded.


9. Persistent storage

rusqlite

Recommended.

rusqlite is an ergonomic Rust wrapper around SQLite and exposes transactions, cached statements, hooks, tracing, custom functions and virtual-table support through features. 

It fits your application because you need:

exact metadata filters;

tags and custom fields;

path uniqueness;

link edges;

scan diagnostics;

incremental state;

FTS5;

one deployable local database.


Useful feature considerations:

rusqlite = {
    version = "...",
    features = [
        "bundled",
        "functions",
        "trace"
    ]
}

Use bundled when you want a predictable SQLite build. Confirm that your selected build includes FTS5.

sqlx

Consider SQLx when:

your whole service is async;

compile-time checked SQL is valuable;

you may later support PostgreSQL;

connection pooling is required.


For an embedded single-writer indexer, rusqlite is simpler.

r2d2_sqlite or deadpool-sqlite

Probably unnecessary for version one.

Searches can use separate read connections while indexing uses a single writer, but you do not need a large connection pool for a local AI tool.

refinery, rusqlite_migration or sqlx migrate

Use one migration library rather than embedding scattered CREATE TABLE IF NOT EXISTS statements.

Your index format will evolve. Schema migrations should be explicit, numbered and tested.

10. Full-text search

SQLite FTS5

Start here.

FTS5 gives you an embedded inverted index without deploying another service. It is enough for:

phrase search;

prefix search;

BM25 ranking;

snippets;

title, description, heading and body fields;

metadata prefilters through SQL joins.


Recommended fields:

path
title
description
headings
body

Keep exact metadata outside the FTS virtual table.

Do not store only one concatenated document string. Separate fields let you control weighting and debugging.

tantivy

Use Tantivy when SQLite FTS5 becomes limiting.

Tantivy supports explicitly defined schemas, text fields, tokenizers, index writers and query types. Its tokenizer layer can treat ordinary language, identifiers, URLs or other fields differently. 

Tantivy becomes attractive for:

custom tokenizers;

better multilingual strategies;

larger corpora;

high query throughput;

advanced field scoring;

fuzzy terms;

more complex ranking;

independent search-index lifecycle.


The downside is that you now maintain two stores:

SQLite: metadata, state and graph
Tantivy: text index

You must coordinate updates and recovery between them.

For hundreds or tens of thousands of OKF files, I would not accept that complexity until benchmarks or relevance tests justify it.

grep-searcher

This is a useful non-indexed fallback. It provides line-oriented search and optional multiline search. 

Potential use:

index unavailable or stale
→ restricted live search over candidate files

It is not a replacement for FTS because every query still scans file content.

Fuzzy matching crates

For path and title fuzziness, consider:

nucleo-matcher

fuzzy-matcher

skim

trigram similarity implemented in SQL


Do not apply fuzzy matching to the entire body corpus by default. Use it for:

filenames;

titles;

concept IDs;

tags;

command-palette-like discovery.


11. Graph support

Store graph edges in SQLite first

Your initial graph is straightforward:

links (
    source_document_id,
    target_document_id,
    target_path,
    target_anchor,
    relation
)

Most required operations can be done with SQL:

outgoing links;

backlinks;

unresolved links;

direct neighbours;

bounded recursive traversal using recursive CTEs.


Do not add an in-memory graph library merely because the repository contains links.

petgraph

Add petgraph when you need actual graph algorithms or want to materialize a working subgraph. It supports directed and undirected graph structures with arbitrary node and edge data. 

Useful future operations:

cycle detection;

strongly connected components;

shortest paths;

topological sorting;

connected-component analysis;

centrality-like custom computations.


Recommended design:

SQLite remains source of truth
→ load bounded relevant subgraph
→ run petgraph algorithm
→ return results

Do not keep the entire graph duplicated in memory unless measurements support it.

Graph database

Do not use Neo4j or another graph database for version one.

The operational cost is disproportionate to a filesystem knowledge repository unless graph queries become the product’s primary workload.

12. Content hashing and incremental scans

blake3

Recommended.

Use BLAKE3 for content fingerprints when mtime + size indicates a possible change.

Store:

mtime
size
content_hash
parser_version
index_schema_version

The parser version matters. A file may be unchanged while your parser or normalization rules change.

Do not hash every file on every scan unless stronger correctness is needed. That turns an incremental metadata scan back into a complete content read.

xxhash-rust

Use a non-cryptographic hash when raw speed matters and collision resistance is not important.

For index correctness, BLAKE3 is fast enough and gives safer identity semantics.

filetime

Useful in tests for controlling modification timestamps and validating incremental behaviour.

13. Filesystem watching

notify

Add after the first reliable full and incremental scan.

The watcher should be treated as a hint source, not the source of truth. Filesystem event streams can coalesce, duplicate or omit useful detail depending on platform and editor behaviour.

Recommended pattern:

watch event
→ debounce affected paths
→ stat/rescan those paths
→ periodically reconcile the full repository

notify-debouncer-mini or notify-debouncer-full

Useful because editors often save through a sequence such as:

create temporary file
write
rename
remove old file

Debouncing prevents reparsing the same logical document several times.

14. MCP and AI integration

rmcp

Recommended.

rmcp is the official Rust SDK for the Model Context Protocol. It supports building MCP servers and clients and exposes tools, resources and prompts. 

Use MCP tools for operations:

search_documents
query_metadata
browse_directory
get_document
get_section
get_links
get_backlinks
validate_repository

Use MCP resources for addressable documents or repository indexes when appropriate:

okc://repository/metrics/revenue
okc://repository/datasets/orders

Keep the tool surface small. Do not expose low-level calls like:

execute_sql
read_arbitrary_path
walk_any_directory

schemars

Very useful for generating JSON Schema from Rust input structs.

Example:

#[derive(
    serde::Deserialize,
    schemars::JsonSchema
)]
struct SearchDocumentsRequest {
    query: String,
    path_prefix: Option<String>,
    limit: Option<u32>,
}

This reduces drift between:

implementation structs;

MCP tool schemas;

documentation;

tests.


serde_json

Use ordinary serde_json for MCP requests and responses.

simdjson is not useful here. MCP tool payloads will be tiny compared with the Markdown corpus. JSON parsing will not be your bottleneck.

rmcp-server-kit

This is an optional layer around rmcp providing server infrastructure such as transport, security and observability. 

I would begin directly with rmcp. Add a framework only when deployment requirements justify it.

15. API and CLI libraries

clap

Recommended for:

okc-tool scan
okc-tool validate
okc-tool search
okc-tool serve

A CLI is extremely useful even when MCP is the primary interface. It lets you test the deterministic core without involving an AI client.

axum

Use it when you also want an HTTP API, health endpoints or metrics.

tower

Useful for:

timeouts;

concurrency limits;

tracing;

request IDs;

authentication layers.


utoipa or aide

Only needed when exposing and documenting a conventional HTTP API. MCP does not require OpenAPI.

16. Error handling and diagnostics

thiserror

Recommended for domain errors:

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("front matter exceeds {limit} bytes")]
    FrontMatterTooLarge { limit: usize },

    #[error("invalid YAML at {line}:{column}: {message}")]
    InvalidYaml {
        line: usize,
        column: usize,
        message: String,
    },
}

anyhow

Use in binaries and top-level command handlers where adding context matters.

Do not expose raw anyhow debug chains directly to AI clients. Convert internal errors into stable error codes and safe messages.

miette

Strong candidate for human-readable CLI diagnostics with source spans.

It can make malformed front matter look like a compiler error:

error[OKF002]: invalid YAML
  --> metrics/revenue.md:4:7

This is valuable for validate_repository.

17. Logging and observability

tracing

Recommended.

Use spans for:

repository.scan
file.parse
yaml.parse
markdown.parse
database.commit
search.execute
mcp.tool

Record:

repository ID;

relative path;

scan generation;

elapsed time;

bytes processed;

result count;

truncation state.


tracing-subscriber

Use structured JSON logs in server mode and readable text logs in CLI mode.

metrics

Use when you need counters and histograms independent of a particular exporter.

Potential metrics:

okc_files_discovered
okc_files_parsed
okc_files_skipped
okc_parse_failures
okc_broken_links
okc_scan_duration_seconds
okc_search_duration_seconds
okc_tool_response_bytes

OpenTelemetry libraries

Add only when the tool runs as a shared service. For a local MCP server, tracing logs are usually enough.

18. Configuration

figment

Useful for merging:

defaults;

configuration file;

environment variables;

CLI overrides.


config

A common alternative.

humantime-serde

Useful for configuration such as:

watch_debounce: 500ms
full_reconciliation_interval: 10m

serde_with

Useful for stricter or customized serialization rules.

19. Security and policy libraries

cap-std

Potentially useful for capability-oriented filesystem access. Instead of repeatedly checking absolute paths, you operate relative to an already-open directory capability.

This is conceptually strong for an AI-facing filesystem tool because it reduces accidental path escape.

Evaluate it carefully against:

Windows support;

symlink semantics;

integration with your walker;

canonicalization behaviour.


secrecy

Useful for credentials, but your OKF scanner should ideally not require secrets for local operation.

zeroize

Probably unnecessary unless credentials or sensitive decrypted content enter memory.

cargo-deny

Use in CI to enforce:

licence policies;

advisories;

duplicate dependency awareness;

banned crates.


cargo-audit

Use in CI for RustSec advisories.

20. Testing

tempfile

Recommended for creating fixture repositories safely.

assert_fs

Useful for expressive filesystem tests.

insta

Excellent for snapshot-testing:

parsed document records;

validation diagnostics;

directory browsing responses;

MCP JSON payloads;

search excerpts.


pretty_assertions

Improves failed structural comparisons.

proptest

Very useful for front-matter extraction and path normalization.

Properties worth testing:

never panic for arbitrary bytes
never return a slice outside input bounds
never escape the configured root
closing fence detection is deterministic
line offsets remain valid

cargo-fuzz

I strongly recommend fuzzing:

front-matter delimiter extraction;

YAML conversion;

Markdown link resolution;

path normalization;

anchor parsing.


Your tool processes files selected by an AI and potentially written by many humans. Parser robustness matters.

criterion

Use for microbenchmarks:

boundary extraction;

YAML parsing;

Markdown parsing;

link normalization;

hashing.


Do not use microbenchmarks as proof that the complete scanner is fast. Also benchmark complete repositories.

divan

A lighter benchmark alternative.

21. Test corpus and conformance tools

CommonMark specification examples

Use official CommonMark examples to validate the Markdown profile you claim to support.

YAML test suite

Run your selected YAML parser against the YAML test suite or at least a curated subset covering:

duplicate keys;

aliases;

multiline scalars;

tags;

timestamps;

YAML 1.1 versus 1.2 booleans;

deep nesting;

malformed mappings.


For OKF, add your own restricted-profile tests. Passing all YAML features is less important than consistently enforcing your chosen subset.

22. Optional semantic retrieval libraries

Do not add these initially.

Embedding client libraries

Use an API client appropriate to the embedding provider only after lexical retrieval failures are measured.

Store embeddings independently from source-of-truth metadata.

hnsw_rs or usearch

Possible embedded vector-index options.

However, vector search adds:

model versioning;

chunking policies;

embedding costs;

stale-vector handling;

semantic retrieval evaluation;

more index storage.


OKF already provides hierarchy, metadata and links. Exploit those first.

Reranking

A reranker can improve results after deterministic metadata filtering and lexical candidate generation.

Pipeline:

metadata constraints
→ FTS candidates
→ optional graph expansion
→ optional reranking
→ sections returned to AI

Do not let semantic retrieval bypass access controls or exact metadata filters.

23. Libraries I would not use initially

simd-json

Not useful for scanning Markdown, parsing YAML or searching documents. It might save microseconds on MCP payload parsing while adding complexity where no meaningful bottleneck exists.

A graph database client

SQLite edges are enough initially.

A vector database

Premature until you have retrieval-evaluation evidence.

A generic front-matter framework

The format boundary is small and security-sensitive enough to implement directly.

A full Markdown AST parser

Unnecessary unless you are modifying documents.

A separate search service

SQLite FTS5 keeps deployment simple and transactional.

24. My concrete Cargo shortlist

Version-one dependencies

[dependencies]
ignore = "..."
memchr = "..."

serde = { version = "...", features = ["derive"] }
serde_json = "..."
serde-saphyr = "..."
schemars = "..."

pulldown-cmark = "..."

rusqlite = {
    version = "...",
    features = ["bundled", "functions", "trace"]
}

blake3 = "..."

rmcp = "..."
tokio = {
    version = "...",
    features = ["macros", "rt-multi-thread", "signal"]
}
tokio-util = "..."

clap = { version = "...", features = ["derive"] }

thiserror = "..."
anyhow = "..."
miette = { version = "...", features = ["fancy"] }

tracing = "..."
tracing-subscriber = {
    version = "...",
    features = ["env-filter", "json"]
}

Development dependencies

[dev-dependencies]
tempfile = "..."
assert_fs = "..."
insta = { version = "...", features = ["json"] }
pretty_assertions = "..."
proptest = "..."
criterion = "..."

Add after MVP

[dependencies]
notify = "..."
notify-debouncer-mini = "..."
petgraph = "..."
rayon = "..."
crossbeam-channel = "..."

Add only after search evaluation

[dependencies]
tantivy = "..."
nucleo-matcher = "..."

Final recommendation

Build the first release around:

ignore
+ custom memchr-based front-matter extraction
+ serde-saphyr
+ pulldown-cmark
+ rusqlite with FTS5
+ rmcp

Treat these as replaceable boundaries:

trait FrontMatterParser { /* ... */ }
trait MarkdownParser { /* ... */ }
trait SearchIndex { /* ... */ }
trait RepositoryStore { /* ... */ }

The most important abstraction boundary is the YAML parser. That ecosystem has changed considerably since serde_yaml was deprecated, so your OKF domain model should not leak a particular crate’s Value type throughout the application. Normalize front matter into your own representation immediately after parsing.