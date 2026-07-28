# Competitor Assessment: Google Cloud Knowledge Catalog (formerly Dataplex) + OKF

## Overview

**Google Cloud Knowledge Catalog** (rebranded from Dataplex Universal Catalog, April 2026) is Google's **managed, enterprise-grade metadata and AI context platform**. It is the **direct commercial counterpart** to what OKC aims to be: a universal knowledge catalog that grounds AI agents in organizational truth.

**Open Knowledge Format (OKF)** is the **open specification** (v0.2, June 2026) that both systems use as their canonical interchange format. Google authored the spec; OKC implements it.

| Dimension | Google Knowledge Catalog | OKC (Open Knowledge Catalog) |
|-----------|-------------------------|------------------------------|
| **Category** | Managed GCP service (Dataplex evolution) | Local-first open-source Rust tool |
| **Deployment** | Cloud-only, pay-as-you-go | Self-hosted, single binary (`cargo install okc`) |
| **Knowledge Format** | OKF v0.2 (author of spec) | OKF v0.2 (consumer via `okf` crate) |
| **Target User** | Enterprise data platforms, AI teams | AI agents, knowledge workers, developers |
| **License** | Proprietary (GCP Terms of Service) | Apache-2.0 / MIT dual-license |
| **Cost Model** | DCU-hours ($0.06–0.089/hr) + storage ($0.04/GB-mo) | Free (open source) |

---

## Google Knowledge Catalog — Capability Inventory

### Core Pillars (Per Google Cloud Next '26)

| Pillar | Capabilities |
|--------|--------------|
| **Aggregation** | Auto-harvest metadata from BigQuery, AlloyDB, Spanner, Cloud SQL, Firestore, Looker, Vertex AI, Dataform, Dataproc Metastore, Iceberg REST Catalog (Databricks, Glue, Snowflake), 3rd-party catalogs (Collibra, Atlan, Datahub, etc.) |
| **Enrichment** | Schema mining, log analysis, BI model extraction, unstructured entity extraction, BigQuery measures (programmatic business logic in SQL), continuous learning loops |
| **Data Products** | Self-contained units: data assets + semantic wrapper (intent, SLAs, governance), discoverable, governed, distributable |
| **Governance** | Centralized policies, IAM, lineage, quality checks, profiling, business glossaries, column-level security via IAM tags |
| **Agent Interface** | **OneMCP** — MCP server exposing catalog tools for agent grounding; Deep Research Agent in Gemini Enterprise powered by Knowledge Catalog |

### Technical Architecture

- **Backend:** Managed Dataplex infrastructure (Apache Spark, Dataflow, BigQuery)
- **Storage:** Proprietary metadata graph + BigQuery for analytics
- **API:** gRPC/REST + **OneMCP** (Streamable HTTP) for agent access
- **Auth:** Google Cloud IAM, OAuth 2.1, VPC-SC, CMEK
- **Multi-tenancy:** Native (projects, folders, organizations)
- **Regions:** Global GCP regions, data residency controls

### What It Indexes (Data Plane)

| Category | Asset Types |
|----------|-------------|
| Analytics | BigQuery datasets/tables/views/models/routines, Dataform, Dataproc Metastore, Iceberg tables |
| AI/ML | Vertex AI models/datasets/feature groups/online stores |
| BI | Looker instances/dashboards/LookML projects (Preview) |
| Databases | Bigtable, Cloud SQL, Spanner, AlloyDB |
| Unstructured | Cloud Storage filesets, Document AI processors |
| 3rd Party | Collibra, Atlan, Datahub, Ab Initio, Anomalo, Confluent, etc. |

### What It Does NOT Index (Scope Boundary)

- Workplace knowledge: Slack threads, meeting transcripts, email, Confluence, Notion, Jira tickets
- Tribal knowledge: undocumented heuristics, architectural decisions, runbooks not in data systems
- Code intelligence: symbols, call graphs, dependencies (unless in Dataform/BigQuery routines)
- Personal/team wikis: markdown bundles, Zettelkasten, Obsidian vaults

---

## OKC — Capability Inventory (Recap)

| Layer | OKC Capabilities |
|-------|------------------|
| **Format** | OKF v0.2 bundles (markdown + YAML frontmatter) |
| **Storage** | SQLite + FTS5 (single file, WAL mode, embedded migrations) |
| **Search** | FTS5/BM25 with configurable weights, path/type/tag filters |
| **Graph** | BFS traversal (`traverse`), `get_links`, `get_backlinks`, `get_neighbors` |
| **Metadata** | `query_metadata` — structured KV filtering on frontmatter |
| **Document** | `get_document`, `get_section` (by heading/anchor) |
| **Validation** | 8-category: broken links, malformed YAML, circular refs, duplicates, missing index/log, heading hierarchy, frontmatter schema |
| **Lineage** | `lineage` tool — concept evolution (splits, merges, renames) |
| **Live Sync** | `watch` — `notify` crate, debounced incremental re-index |
| **MCP Server** | 11 tools over stdio + HTTP/SSE (rmcp + axum) |
| **CLI** | `scan`, `browse`, `search`, `metadata`, `traverse`, `validate`, `stats`, `serve`, `watch` |
| **Distribution** | Single static binary, `cargo install`, no runtime deps |

---

## Direct Comparison: Where They Overlap

### Shared Vision
Both systems exist to **ground AI agents in organizational knowledge** so agents stop hallucinating and start knowing.

| Shared Goal | Google Knowledge Catalog | OKC |
|-------------|-------------------------|-----|
| **Agent-ready context** | OneMCP + Deep Research Agent | MCP stdio/HTTP/SSE + 11 tools |
| **Structured knowledge format** | OKF v0.2 (author) | OKF v0.2 (consumer via `okf` crate) |
| **Semantic search** | Vertex AI embeddings + hybrid retrieval | FTS5 BM25 (vector planned via `sqlite-vec`) |
| **Governance/trust** | IAM, lineage, data quality, verification_status | OKF `sources[]`, `trust_tier`, `verification_status`, `lineage` |
| **Data products / bundles** | Data Products (managed) | OKF Bundles (git-native) |
| **Cross-system aggregation** | BigQuery, Vertex AI, 3rd-party catalogs | Multi-root `okc.toml` + future federation |

### Shared OKF Fields (v0.2)
Both systems speak the same language:
```yaml
type: Metric
title: Weekly Active Users
description: ...
resource: https://console.cloud.google.com/bigquery?...
tags: [product, engagement]
sources:
  - id: bigquery-wau
    author: data-team
    verification_status: verified
    trust_tier: official
status: stable
stale_after: 2026-09-23
```

---

## Critical Differences: The "Google-Free" Positioning

| Dimension | Google Knowledge Catalog | OKC (Google-Free Alternative) |
|-----------|-------------------------|------------------------------|
| **Cloud Dependency** | **Hard requirement** — GCP project, billing, IAM, VPC | **Zero** — runs anywhere (laptop, CI, edge, air-gapped) |
| **Data Ingestion** | Auto-harvest from GCP services + connectors | `okc scan` on local markdown files (git repo, Obsidian vault, any FS) |
| **Knowledge Scope** | Data plane only (tables, models, dashboards) | **Any knowledge** — runbooks, decisions, tribal knowledge, API docs, meeting notes |
| **Format Lock-in** | Proprietary metadata graph + BigQuery | **Plain markdown + YAML** — human-readable, git-diffable, editor-agnostic |
| **Cost Predictability** | Pay-as-you-go DCU-hours (unpredictable at scale) | **Free** — Apache-2.0, no metering, no billing surprises |
| **Data Residency** | GCP regions only | **Your infrastructure** — on-prem, any cloud, laptop, USB stick |
| **Offline/Air-gapped** | ❌ Impossible | ✅ Native — fully local, no network calls |
| **Customization** | Config within GCP guardrails | **Source access** — fork, extend, embed `okc-lib` crate |
| **Vendor Risk** | Google deprecation history (Dataplex→Knowledge Catalog) | **Community-owned** — no single vendor can sunset it |
| **Agent Protocol** | OneMCP (GCP-managed, Streamable HTTP) | **Open MCP** — stdio (Claude Code) + HTTP/SSE (any client) |
| **Developer Experience** | GCP Console, `gcloud`, Terraform | `cargo install okc` → `okc scan` → `okc serve` in 30 seconds |

---

## Threat Assessment: HIGH (Direct Competitor)

**Google Knowledge Catalog is the most direct commercial competitor to OKC's vision.**

### Why It's a Threat
1. **Same north star** — "universal context engine for AI agents"
2. **Same format** — OKF v0.2 is the lingua franca for both
3. **Same agent interface** — MCP (OneMCP vs open MCP)
4. **Google's distribution** — GCP sales channel, enterprise contracts, Gemini integration
5. **Managed convenience** — no ops burden for teams already on GCP

### Why OKC Can Win the "Google-Free" Segment
| OKC Advantage | Customer Profile |
|---------------|------------------|
| **Zero cloud dependency** | Air-gapped, regulated, multi-cloud, cost-sensitive |
| **Any knowledge, not just data** | Teams with tribal knowledge, runbooks, decisions, wikis |
| **Git-native workflow** | Engineers who want PR-based review, diffs, history |
| **Plain text portability** | Orgs avoiding proprietary metadata lock-in |
| **Predictable cost (free)** | Startups, open-source projects, individual developers |
| **Offline/edge capable** | Disconnected environments, local-first ideologies |
| **Extensible source** | Teams needing custom ingestion, plugins, `okc-lib` embedding |

---

## Strategic Recommendations for OKC

### P0 — Match Core Capabilities (Close the "Managed" Gap)
| Gap | Action | Effort |
|-----|--------|--------|
| **Vector/semantic search** | Add `sqlite-vec` feature flag for hybrid BM25+vector | Medium |
| **Auto-enrichment pipeline** | `okc ingest` with LLM extraction (BYO provider, feature-gated) | High |
| **Data quality/profiling** | Extend `validate` with freshness, completeness, anomaly checks | Medium |
| **Lineage visualization** | `okc lineage --graph` → Mermaid/DOT export | Low |
| **Multi-tenancy / scopes** | MCP tool authorization + `okc://index/status` resource | Medium |

### P1 — Differentiate on "Google-Free" Strengths
| Strength | Action | Effort |
|----------|--------|--------|
| **Git-native bundles** | `okc init` + `okc bundle create` + PR-based workflow docs | Low |
| **Any-knowledge ingestion** | Connector framework: git, GitHub, Confluence, Notion, Slack export | High |
| **Air-gapped deployment** | Document offline install, model bundling (for vector search) | Low |
| **Cost transparency** | Publish "OKC vs Knowledge Catalog TCO calculator" | Low |
| **Community governance** | RFC process, plugin ecosystem, `okc-lib` crate for embedding | Medium |

### P2 — Interoperability (Bridge, Don't Just Compete)
| Opportunity | Action |
|-------------|--------|
| **Export from Knowledge Catalog → OKF** | Build `gcloud knowledge-catalog export --format=okf` or partner with Google |
| **OKC as local cache** | `okc sync --from-gcp` for hybrid cloud/local workflows |
| **OneMCP compatibility** | Ensure OKC's MCP tools are compatible with OneMCP clients |

---

## Verdict

**Google Knowledge Catalog is the "AWS RDS" to OKC's "SQLite" — same conceptual role (knowledge catalog for agents), same format (OKF), but fundamentally different deployment models.**

| If Customer Needs... | They Choose |
|---------------------|-------------|
| Managed, integrated with BigQuery/Vertex AI, enterprise support, auto-harvest from GCP | **Google Knowledge Catalog** |
| Self-hosted, air-gapped, multi-cloud, git-native, any knowledge type, zero cost, no vendor lock-in | **OKC** |

**OKC's winning strategy:** Own the **"Google-free, git-native, any-knowledge"** segment completely. Don't try to out-managed-service Google. Make OKC the default choice for every team that *can't* or *won't* use GCP — which is most of the world.

---

## Appendix: Key References

- **OKF Spec:** https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
- **Knowledge Catalog Product:** https://cloud.google.com/products/knowledge-catalog
- **Announcement Blog:** https://cloud.google.com/blog/products/data-analytics/introducing-the-google-cloud-knowledge-catalog
- **OneMCP Docs:** https://docs.cloud.google.com/dataplex/docs/reference/mcp
- **OKC Repo:** https://github.com/felix/Open-Knowledge-Catalog (this project)