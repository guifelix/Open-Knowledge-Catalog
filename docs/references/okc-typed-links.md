# OKC Typed Links Extension

**Version 1**

A versioned, additive extension over OKF v0.2 for expressing *typed*
relationships between knowledge documents. Canonical Markdown links stay
the portable graph (OKF §6.1); typed links enrich that graph with
relationship semantics for OKC-aware consumers.

OKF producers may add any front-matter keys (OKF §4.1 Extensions) and
consumers MUST NOT reject unrecognized fields. This extension follows
that rule: an otherwise valid OKF document remains readable even when
the `typed_links` block is unknown to a consumer. A consumer that does
not recognize the block simply retains it as generic metadata and keeps
working from the Markdown links.

---

## 1. Format

One optional, versioned front-matter block:

```yaml
---
typed_links:
  version: 1
  links:
    - target: /data/metrics.md
      relation: depends-on
      anchor: optional-anchor # optional
---
```

- `version` — required integer. The current scheme is `1`. Any other
  version is preserved verbatim in the document's `custom` field and
  surfaced as a warning; it is never a parse failure.
- `links` — required list of mappings, each with:
  - `target` — a bundle-relative path (leading `/` recommended) or an
    absolute URL (`http://`, `https://`, `mailto:`). Internal targets
    are resolved against the same repository that canonical Markdown
    links resolve against.
  - `relation` — **required** string identifying the relationship type.
    Typed links always declare one; there is no implicit default. Use
    `references` for a generic reference equivalent to a Markdown link.
  - `anchor` — optional string narrowing the target within the document.

A block that is not a mapping, has no `version`, no `links`, or contains
an entry missing `target` or `relation` is treated as malformed: it is
preserved verbatim in `custom`, reported as a warning, and the document
continues to parse normally (OKF tolerance, §4.1).

## 2. Relationship vocabulary

`relation` is a free-form string and is matched **literally** at query
time — a producer should use one spelling consistently. The documented
vocabulary (kebab-case is the recommended spelling):

| Relation       | Meaning                                                      |
|----------------|--------------------------------------------------------------|
| `references`   | Generic reference (canonical Markdown link equivalent)       |
| `depends-on`   | The target is required for this concept to be valid/useful   |
| `imports`      | The target is pulled in or incorporated here                 |
| `extends`      | This concept extends/refines the target                      |
| `implements`   | This concept implements the target (interface/contract/etc.) |
| `derived-from` | This concept is derived from the target                      |
| `supersedes`   | This concept replaces the target                             |
| `related-to`   | Related, without a stronger directional claim               |

Unknown values are preserved and returned without parse failure (AC #2);
only OKC-aware consumers can interpret them, which is exactly the
intended extension boundary.

## 3. Storage model

The document graph lives in a single `links` table. The migration to
schema version 3 adds one nullable column:

| Column     | Meaning                                              |
|------------|------------------------------------------------------|
| `relation` | `NULL` for canonical Markdown edges; the declared string for typed edges |

- Canonical Markdown links are stored as today, with `relation = NULL`
  — untouched by the migration.
- Each typed link is stored as its **own row** carrying its `relation`.
  Typed links are additive: they never mutate, merge, or overwrite the
  canonical edges produced from the same document.
- A rescan is idempotent: per-document writes DELETE then INSERT, so an
  unchanged document reproduces identical rows and an edited document is
  rewritten exactly. There are no duplicates or shadow tables.

Because rows are additive, one source may legitimately hold both an
untyped edge and a typed edge to the same target (see §5).

## 4. Reading the graph

- `get_links(path, relation)` / `get_backlinks(path, limit, relation)`:
  with no relation, all edges are returned (untyped and typed alike).
  With a relation, only edges carrying that exact relation are returned;
  untyped (`NULL`) edges are excluded.
- `traverse(start, relations, …)`: an empty relation list follows all
  edges (the unchanged default); a non-empty list follows only edges
  whose relation is in the list, excluding untyped edges.

## 5. Diagnostics and resolution

Two check categories extend repository validation:

- **`broken_link`** — a typed internal target that does not exist in the
  bundle is reported as a warning, mirroring canonical broken-link
  tolerance. The edge is kept with `exists_in_repository = 0` and the
  document is still indexed (AC #3).
- **`typed_link_conflict`** — when the same source targets the same path
  both as an untyped Markdown edge and as a typed edge, a warning is
  emitted (AC #6). Both edges are retained as separate rows. The
  deterministic resolution rule is: *filter by relation to isolate a
  specific edge; an unfiltered query returns both.* The warning is a
  portability diagnostic for consumers that only read Markdown links;
  it never drops or overrides either edge.

## 6. Backward compatibility

- Schema version 3 migration preserves all existing untyped links.
- Unfiltered `get_links`, `get_backlinks`, and `traverse` output is
  unchanged from the pre-extension defaults.
- MCP `get_links` / `get_backlinks` accept an optional `relation` input;
  when absent, responses are byte-identical to before (the `relation`
  field is omitted from legacy JSON when `None`).
- OKF-only consumers are unaffected: they read the Markdown links and
  ignore the `typed_links` block per §4.1.

See `docs/references/okf-spec.md` §4.1 (Extensions) and §6.1 (links
between concepts) for the portability foundation this extension builds
on.