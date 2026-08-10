//! Criterion benchmarks for Open Knowledge Catalog core operations.
//!
//! Benchmark groups (each parameterized by corpus size):
//! - scan/fresh: initial full scan
//! - browse/root: browse the root directory
//! - search/*: full-text and filtered search
//! - document/*: get_document and get_section
//! - graph/*: get_links, get_backlinks, traverse
//! - validate: validation pass
//! - stats: get_stats
//! - export: export_to_json
//!
//! Corpora: small (10), medium (50), large (200) documents.

#![allow(
    clippy::default_constructed_unit_structs,
    clippy::expect_used,
    clippy::panic
)]

use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use tempfile::TempDir;

use okc::config::OkcConfig;
use okc::service::OkcService;

// ---------------------------------------------------------------------------
// Corpus sizes
// ---------------------------------------------------------------------------

const SIZES: &[usize] = &[10, 50, 200, 1000, 10000];

// ---------------------------------------------------------------------------
// Document generation
// ---------------------------------------------------------------------------

fn generate_docs(root: &Path, count: usize) {
    let types = ["Metric", "Policy", "Dataset", "Glossary", "Guide"];
    let tag_sets: &[&[&str]] = &[
        &["finance", "executive"],
        &["security", "compliance"],
        &["engineering", "performance"],
        &["sales", "revenue"],
        &["product", "design"],
    ];
    let words = [
        "metric",
        "revenue",
        "policy",
        "compliance",
        "dataset",
        "glossary",
        "architecture",
        "design",
        "performance",
        "security",
        "optimization",
        "analysis",
        "report",
        "dashboard",
        "workflow",
    ];

    for i in 0..count {
        let ct = types[i % types.len()];
        let tags = tag_sets[i % tag_sets.len()];
        let title = format!("{} {}", ct, i);

        let link_indices: Vec<usize> = (0..3).map(|j| (i + j + 1) % count).collect();

        let mut content = String::new();
        content.push_str("---\n");
        content.push_str(&format!("type: {}\n", ct));
        content.push_str(&format!("title: \"{}\"\n", title));
        content.push_str(&format!(
            "description: \"Auto-generated {} for benchmarking\"\n",
            ct
        ));
        content.push_str("tags:\n");
        for t in tags {
            content.push_str(&format!("  - {}\n", t));
        }
        content.push_str(&format!("priority: \"P{}\"\n", i % 5 + 1));
        content.push_str("---\n\n");
        content.push_str(&format!("# {}\n\n", title));
        content.push_str(&format!("This is the definition of {}. ", ct));
        content.push_str(&format!(
            "Every {} has associated {} metrics and {} analysis. ",
            ct,
            words[i % words.len()],
            words[(i + 1) % words.len()]
        ));
        content.push_str("The values are tracked on the dashboard.\n\n");

        content.push_str("## Key Metrics\n\n");
        for j in 0..3 {
            let idx = (i + j) % words.len();
            content.push_str(&format!(
                "- **{}**: Target value for {} is {} units\n",
                words[idx],
                ct,
                (i * 100 + j * 10)
            ));
        }
        content.push_str("\n## Dependencies\n\n");
        content.push_str("This concept depends on the following:\n\n");

        for &li in &link_indices {
            let lt = types[li % types.len()];
            content.push_str(&format!("- [{} {}](doc_{}.md)\n", lt, li, li));
        }

        content.push_str("\n## Notes\n\n");
        content.push_str(&format!("Additional notes for {}. ", title));
        content.push_str("See the architecture document for more details.\n");

        let filename = format!("doc_{}.md", i);
        std::fs::write(root.join(&filename), content)
            .unwrap_or_else(|e| panic!("write {}: {}", filename, e));
    }
}

/// Generate large technical documents with tables, code blocks, and complex structures.
fn generate_technical_docs(root: &Path, count: usize) {
    let types = [
        "API Reference",
        "Architecture Decision",
        "Technical Specification",
        "Data Model",
        "Integration Guide",
    ];
    let tag_sets: &[&[&str]] = &[
        &["api", "reference", "rest"],
        &["architecture", "decision", "adr"],
        &["specification", "technical", "design"],
        &["data", "model", "schema"],
        &["integration", "guide", "webhook"],
    ];

    let words = [
        "metric",
        "optimization",
        "pipeline",
        "schema",
        "query",
        "index",
        "throughput",
        "latency",
        "cache",
        "buffer",
    ];

    for i in 0..count {
        let ct = types[i % types.len()];
        let tags = tag_sets[i % tag_sets.len()];
        let title = format!("{} {}", ct, i);

        let mut content = String::new();
        content.push_str("---\n");
        content.push_str(&format!("type: {}\n", ct));
        content.push_str(&format!("title: \"{}\"\n", title));
        content.push_str("description: \"Large technical document for benchmarking\"\n");
        content.push_str("tags:\n");
        for t in tags {
            content.push_str(&format!("  - {}\n", t));
        }
        content.push_str("---\n\n");
        content.push_str(&format!("# {}\n\n", title));

        // Add multiple sections with tables and code blocks
        for section in 0..5 {
            content.push_str(&format!(
                "## Section {}: {}\n\n",
                section + 1,
                words[(i + section) % words.len()]
            ));
            content.push_str("This section contains detailed technical information.\n\n");

            // Add a table
            content.push_str("### Data Table\n\n");
            content.push_str("| Column A | Column B | Column C | Column D |\n");
            content.push_str("|----------|----------|----------|----------|\n");
            for row in 0..20 {
                content.push_str(&format!(
                    "| Value {} | Value {} | Value {} | Value {} |\n",
                    row,
                    row + 1,
                    row + 2,
                    row + 3
                ));
            }
            content.push('\n');

            // Add a code block
            content.push_str("### Code Example\n\n");
            content.push_str("```rust\n");
            content.push_str(&format!("// Example {} for {}\n", section, ct));
            content.push_str("fn example() -> Result<(), Error> {\n");
            content.push_str("    let data = fetch_data()?;\n");
            content.push_str("    process(data)?;\n");
            content.push_str("    Ok(())\n");
            content.push_str("}\n");
            content.push_str("```\n\n");

            // Add a list
            content.push_str("### Key Points\n\n");
            for point in 0..10 {
                content.push_str(&format!(
                    "- Point {}: Important detail about {}\n",
                    point,
                    words[(i + point) % words.len()]
                ));
            }
            content.push('\n');
        }

        // Add cross-references
        content.push_str("## Related Documents\n\n");
        for j in 0..3 {
            let idx = (i + j + 1) % count;
            let lt = types[idx % types.len()];
            content.push_str(&format!("- [{} {}](tech_{}.md)\n", lt, idx, idx));
        }

        let filename = format!("tech_{}.md", i);
        std::fs::write(root.join(&filename), content)
            .unwrap_or_else(|e| panic!("write {}: {}", filename, e));
    }
}

fn bench_config(root: &Path) -> OkcConfig {
    OkcConfig {
        roots: vec![okc::config::RootConfig {
            id: None,
            path: root.to_path_buf(),
        }],
        ..OkcConfig::default()
    }
}

// ---------------------------------------------------------------------------
// Build a single-sized corpus directory and return the config + scanned service
// ---------------------------------------------------------------------------

struct BenchSetup {
    #[allow(dead_code)]
    dir: TempDir,
    /// First document path for graph / document benchmarks
    first_doc: String,
    service: OkcService,
}

fn setup_scanned(count: usize) -> BenchSetup {
    let dir = TempDir::new().expect("temp dir");
    generate_docs(dir.path(), count);
    let config = bench_config(dir.path());
    let mut service = OkcService::open_in_memory(&config).expect("open");
    service.scan().expect("scan");
    let first_doc = if count > 0 {
        format!("doc_{}.md", 0)
    } else {
        String::new()
    };
    BenchSetup {
        dir,
        first_doc,
        service,
    }
}

// ---------------------------------------------------------------------------
// Benchmark functions
// ---------------------------------------------------------------------------

fn bench_scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan");
    group.sampling_mode(SamplingMode::Auto);

    for &size in SIZES {
        let dir = TempDir::new().expect("temp dir");
        generate_docs(dir.path(), size);
        let config = bench_config(dir.path());

        group.bench_with_input(BenchmarkId::new("fresh", size), &size, |b, &_| {
            b.iter(|| {
                let mut service = OkcService::open_in_memory(&config).expect("open");
                black_box(service.scan().expect("scan"));
            });
        });
    }
    group.finish();
}

fn bench_browse(c: &mut Criterion) {
    let mut group = c.benchmark_group("browse");
    group.sampling_mode(SamplingMode::Auto);

    for &size in SIZES {
        let setup = setup_scanned(size);

        group.bench_with_input(BenchmarkId::new("root", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .browse(black_box(""), black_box(0), black_box(1000))
                        .expect("browse"),
                );
            });
        });

        // Browse with depth=1 (shows subdirectories)
        group.bench_with_input(BenchmarkId::new("root_depth", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .browse(black_box(""), black_box(1), black_box(1000))
                        .expect("browse"),
                );
            });
        });
    }
    group.finish();
}

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("search");
    group.sampling_mode(SamplingMode::Auto);

    for &size in SIZES {
        let setup = setup_scanned(size);

        // Full-text term search -- "dashboard" appears in every doc
        group.bench_with_input(BenchmarkId::new("term_dashboard", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .search(
                            black_box("dashboard"),
                            None,
                            None,
                            None,
                            black_box(100),
                            None,
                            None,
                            None,
                        )
                        .expect("search"),
                );
            });
        });

        // Search with type filter
        let metric = "Metric".to_string();
        let types = vec![metric];
        group.bench_with_input(
            BenchmarkId::new("term_type_filter", size),
            &size,
            |b, &_| {
                b.iter(|| {
                    black_box(
                        setup
                            .service
                            .search(
                                black_box("dashboard"),
                                None,
                                Some(black_box(&types)),
                                None,
                                black_box(100),
                                None,
                                None,
                                None,
                            )
                            .expect("search"),
                    );
                });
            },
        );

        // Search with tag filter
        let tags = vec!["security".to_string(), "compliance".to_string()];
        group.bench_with_input(BenchmarkId::new("term_tag_filter", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .search(
                            black_box("dashboard"),
                            None,
                            None,
                            Some(black_box(&tags)),
                            black_box(100),
                            None,
                            None,
                            None,
                        )
                        .expect("search"),
                );
            });
        });
    }
    group.finish();
}

fn bench_document(c: &mut Criterion) {
    let mut group = c.benchmark_group("document");
    group.sampling_mode(SamplingMode::Auto);

    for &size in SIZES {
        let setup = setup_scanned(size);
        let doc_path = &setup.first_doc;

        // get_document with metadata + headings only
        let includes: Vec<String> = vec!["metadata".to_string(), "headings".to_string()];
        group.bench_with_input(BenchmarkId::new("get_document", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .get_document(black_box(doc_path), black_box(&includes), black_box(5000))
                        .expect("get_document"),
                );
            });
        });

        // get_section for "Key Metrics"
        group.bench_with_input(BenchmarkId::new("get_section", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .get_section(
                            black_box(doc_path),
                            black_box("Key Metrics"),
                            black_box(2000),
                        )
                        .expect("get_section"),
                );
            });
        });
    }
    group.finish();
}

fn bench_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph");
    group.sampling_mode(SamplingMode::Auto);

    for &size in SIZES {
        let setup = setup_scanned(size);
        let doc_path = &setup.first_doc;

        // get_links
        group.bench_with_input(BenchmarkId::new("get_links", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .get_links(black_box(doc_path))
                        .expect("get_links"),
                );
            });
        });

        // get_backlinks
        group.bench_with_input(BenchmarkId::new("get_backlinks", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .get_backlinks(black_box(doc_path), black_box(50))
                        .expect("get_backlinks"),
                );
            });
        });

        // traverse_graph with default relations
        let relations: Vec<String> = vec!["*".to_string()];
        group.bench_with_input(BenchmarkId::new("traverse", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    setup
                        .service
                        .traverse(
                            black_box(doc_path),
                            black_box(&relations),
                            black_box(3),
                            black_box(50),
                        )
                        .expect("traverse"),
                );
            });
        });
    }
    group.finish();
}

fn bench_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("validate");
    group.sampling_mode(SamplingMode::Auto);

    for &size in SIZES {
        let setup = setup_scanned(size);

        group.bench_with_input(BenchmarkId::new("all", size), &size, |b, &_| {
            b.iter(|| {
                black_box(setup.service.validate().expect("validate"));
            });
        });
    }
    group.finish();
}

fn bench_stats(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats");
    group.sampling_mode(SamplingMode::Auto);

    for &size in SIZES {
        let setup = setup_scanned(size);

        group.bench_with_input(BenchmarkId::new("get_stats", size), &size, |b, &_| {
            b.iter(|| {
                black_box(setup.service.get_stats().expect("get_stats"));
            });
        });
    }
    group.finish();
}

fn bench_export(c: &mut Criterion) {
    let mut group = c.benchmark_group("export");
    group.sampling_mode(SamplingMode::Auto);

    for &size in SIZES {
        let setup = setup_scanned(size);

        group.bench_with_input(BenchmarkId::new("export_to_json", size), &size, |b, &_| {
            b.iter(|| {
                black_box(setup.service.export_to_json().expect("export"));
            });
        });
    }
    group.finish();
}

fn bench_technical_docs(c: &mut Criterion) {
    let mut group = c.benchmark_group("technical_docs");
    group.sampling_mode(SamplingMode::Auto);

    // Use smaller sizes for technical docs since they're larger
    let tech_sizes = [10, 50, 100];

    for size in tech_sizes {
        let dir = TempDir::new().expect("temp dir");
        generate_technical_docs(dir.path(), size);
        let config = bench_config(dir.path());

        // Benchmark full scan of technical documents
        group.bench_with_input(BenchmarkId::new("scan", size), &size, |b, &_| {
            b.iter(|| {
                let mut service = OkcService::open_in_memory(&config).expect("open");
                black_box(service.scan().expect("scan"));
            });
        });

        // Benchmark incremental scan (re-scan after initial)
        let mut service = OkcService::open_in_memory(&config).expect("open");
        service.scan().expect("initial scan");

        group.bench_with_input(
            BenchmarkId::new("incremental_scan", size),
            &size,
            |b, &_| {
                b.iter(|| {
                    black_box(service.scan().expect("incremental scan"));
                });
            },
        );

        // Benchmark search on technical content
        group.bench_with_input(BenchmarkId::new("search", size), &size, |b, &_| {
            b.iter(|| {
                black_box(
                    service
                        .search(
                            black_box("function"),
                            None,
                            None,
                            None,
                            black_box(100),
                            None,
                            None,
                            None,
                        )
                        .expect("search"),
                );
            });
        });
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// Criterion harness
// ---------------------------------------------------------------------------

criterion_group!(
    benches,
    bench_scan,
    bench_browse,
    bench_search,
    bench_document,
    bench_graph,
    bench_validate,
    bench_stats,
    bench_export,
    bench_technical_docs,
);
criterion_main!(benches);
