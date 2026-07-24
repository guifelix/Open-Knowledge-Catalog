Rust is a strong choice, but not automatically the best choice.

For the tool you described, I would choose Rust when you want to ship a durable, local-first executable that scans large repositories efficiently and runs safely as an MCP server.

Why Rust fits

Your workload has several Rust-friendly characteristics:

filesystem traversal;

parsing untrusted YAML and Markdown;

bounded parallelism;

incremental indexing;

SQLite;

long-running file watching;

low memory overhead;

distribution as a single binary;

strict control over paths, limits and failures.


Rust also has an official MCP SDK, rmcp, built around Tokio, so MCP support is no longer a reason to avoid Rust. 

A Rust implementation could realistically remain:

one binary
+ one SQLite file
+ one configuration file

That is attractive for a local AI tool.

Where Rust will cost you

Rust will slow down initial development compared with TypeScript, Python or Go.

The difficult areas will likely be:

YAML parser ecosystem decisions;

async server code mixed with blocking scanning;

SQLite ownership and transaction architecture;

path handling across platforms;

generic metadata representations;

compiler friction during schema evolution.


The parser and indexing core will benefit from Rust’s type system. The MCP wrapper itself probably will not.

For hundreds of files, Rust’s raw performance is irrelevant. Python, TypeScript and Go can all scan that volume adequately. Rust becomes valuable because of deployment, safety, predictability and scale, not because 500 Markdown files require SIMD-level performance.

The practical alternatives

Go

Go is probably the strongest alternative.

Choose Go when you want:

a single static-ish binary;

easy concurrency;

simpler development;

good filesystem and SQLite support;

straightforward operations;

lower implementation complexity.


There is now an official Go MCP SDK as well. 

For this product, Go may deliver 80 to 90 percent of Rust’s operational benefits with less engineering friction.

Its weaknesses relative to Rust:

less expressive parsing models;

weaker compile-time guarantees;

more allocation and garbage collection;

error handling can become repetitive;

fewer high-performance embedded search libraries comparable to Tantivy.


A Go stack might be:

filepath.WalkDir or godirwalk
goldmark
go-yaml
modernc.org/sqlite or mattn/go-sqlite3
official MCP Go SDK
fsnotify

Honestly, Go may be the best business decision for an MVP.

TypeScript

Choose TypeScript when:

MCP integration speed is the top priority;

your team already works in Node;

this will primarily run as a developer tool;

rapid schema iteration matters;

the corpus is moderate;

you want the broadest AI tooling ecosystem.


The official TypeScript SDK implements MCP clients and servers and supports standard transports. 

A TypeScript stack has excellent convenience libraries:

fast-glob
gray-matter
yaml
unified / remark
better-sqlite3
chokidar
official MCP TypeScript SDK

This would probably produce the fastest prototype.

Weaknesses:

higher idle memory;

Node runtime dependency unless bundled;

filesystem concurrency is easier to misuse;

native SQLite packaging can be irritating;

less attractive as a small, trusted system utility.


TypeScript is good for validating the tool API. It is less compelling for the final high-performance local indexer.

Python

Choose Python for experimentation, research or very rapid delivery.

There is an official Python MCP SDK. 

A Python stack could use:

os.scandir
pathspec
python-frontmatter
ruamel.yaml
markdown-it-py
sqlite3
watchfiles
official MCP Python SDK

Python is particularly good when you expect to experiment with:

embeddings;

rerankers;

LLM-based summarization;

NLP pipelines;

retrieval evaluation.


Weaknesses:

packaging a reliable local executable;

startup and memory use;

concurrency;

type and schema discipline;

dependency management;

long-term maintenance of a system utility.


The scanner itself does not need Python’s ML ecosystem, so I would not use Python unless advanced retrieval experimentation is central.

My recommendation by project stage

Prototype

Use TypeScript or Go.

Build:

scan
parse
SQLite index
MCP tools

Validate:

whether AI agents understand the tool interface;

which operations they actually call;

whether tree and graph navigation help;

whether FTS5 is sufficient;

what metadata queries users need.


The largest product risk is not parser performance. It is designing a useful AI-facing API.

Production local tool

Use Rust or Go.

Choose Rust when:

you expect large repositories;

low memory matters;

security boundaries matter strongly;

you want sophisticated embedded search;

you plan to distribute one robust binary;

this is intended to become infrastructure.


Choose Go when:

development speed matters more;

the team is not already productive in Rust;

SQLite FTS5 is likely enough;

maintainability by a wider developer pool matters;

the performance requirements are moderate.


Decision matrix

Requirement	Rust	Go	TypeScript	Python

Development speed	Medium-low	High	Very high	Very high
Single-binary distribution	Excellent	Excellent	Weak-medium	Weak
Memory efficiency	Excellent	Good	Medium-low	Low
Filesystem performance	Excellent	Excellent	Good	Adequate
Parser safety	Excellent	Good	Medium	Medium
MCP ecosystem	Good	Good	Excellent	Excellent
SQLite integration	Excellent	Good	Good	Excellent
Advanced embedded search	Excellent	Medium	Medium	Medium
AI/ML experimentation	Low	Low	Medium	Excellent
Long-term system utility	Excellent	Excellent	Medium	Medium


Direct answer

For your exact plan, my ranking would be:

1. Rust, if you are comfortable investing in it
2. Go, if you want the best speed-to-maintainability ratio
3. TypeScript, for a fast MCP prototype
4. Python, for retrieval experiments

Given that your main goal is an AI skill/tool rather than building a search engine company, I would seriously consider Go for version one.

Rust becomes the right choice when at least two of these are true:

you already know Rust;

you need excellent cross-platform binary distribution;

repositories may reach hundreds of thousands of documents;

memory and latency are product requirements;

you expect to add Tantivy or deeper graph processing;

the tool will process untrusted repositories and needs strong defensive guarantees.


Otherwise, Rust may be technically elegant but economically excessive.