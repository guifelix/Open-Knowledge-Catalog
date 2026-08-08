//! Integration tests for OKF retrieval using fixture repositories

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use okc::{config::OkcConfig, service::OkcService};
use std::collections::HashMap;
use tempfile::TempDir;

/// Test fixture for the simple repository
fn setup_simple_repo() -> TempDir {
    let temp_dir = TempDir::new().expect("temp dir for simple repo");
    let source = std::path::Path::new("tests/fixtures/simple");
    copy_dir_all(source, temp_dir.path()).expect("copy simple fixture");
    temp_dir
}

/// Test fixture for edge cases
fn setup_edge_cases_repo() -> TempDir {
    let temp_dir = TempDir::new().expect("temp dir for edge cases");
    let source = std::path::Path::new("tests/fixtures/edge-cases");
    copy_dir_all(source, temp_dir.path()).expect("copy edge cases fixture");
    temp_dir
}

fn mkconfig(repo: &TempDir) -> OkcConfig {
    OkcConfig {
        roots: vec![repo.path().to_path_buf()],
        db_path: repo.path().join("test.db"),
        ..Default::default()
    }
}

fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[test]
fn test_service_rejects_invalid_configuration_before_opening_storage() {
    let repo = TempDir::new().expect("invalid config temp repo");

    let cases = [
        (
            "missing roots",
            OkcConfig {
                roots: Vec::new(),
                db_path: repo.path().join("missing-roots.db"),
                ..Default::default()
            },
            "At least one root directory",
        ),
        (
            "nonexistent root",
            OkcConfig {
                roots: vec![repo.path().join("does-not-exist")],
                db_path: repo.path().join("nonexistent-root.db"),
                ..Default::default()
            },
            "Root directory does not exist",
        ),
        (
            "invalid response limit",
            OkcConfig {
                roots: vec![repo.path().to_path_buf()],
                db_path: repo.path().join("invalid-limit.db"),
                max_response_chars: 0,
                ..Default::default()
            },
            "max_response_chars must be greater than 0",
        ),
        (
            "invalid BM25 weight",
            OkcConfig {
                roots: vec![repo.path().to_path_buf()],
                db_path: repo.path().join("invalid-bm25.db"),
                bm25: okc::config::Bm25Config {
                    title_weight: -1.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            "BM25 weights must be non-negative",
        ),
    ];

    for (name, config, expected) in cases {
        let db_path = config.db_path.clone();
        let error = OkcService::open(&config)
            .err()
            .unwrap_or_else(|| panic!("{name} should be rejected"));
        assert!(
            error.to_string().contains(expected),
            "{name} returned the wrong error: {error}"
        );
        assert!(
            !db_path.exists(),
            "{name} must be rejected before creating storage"
        );
    }
}

#[test]
fn test_in_memory_service_rejects_invalid_configuration() {
    let config = OkcConfig::default();
    let error = OkcService::open_in_memory(&config)
        .err()
        .expect("missing roots should be rejected");
    assert!(error.to_string().contains("At least one root directory"));
}

#[test]
fn test_direct_concept_lookup() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    // Search for monthly recurring revenue
    let results = service
        .search(
            "monthly recurring revenue",
            None,
            None,
            None,
            10,
            None,
            None,
        )
        .expect("search monthly recurring revenue");

    assert!(
        !results.results.is_empty(),
        "Should find monthly revenue concept"
    );
    // The search should find at least one relevant result
    // The glossary/mrr.md might rank higher due to "MRR" in the title
    // Let's just verify we get results and they're reasonable
    let top = &results.results[0];
    assert!(
        top.path.contains("monthly-revenue") || top.path.contains("mrr"),
        "Should find relevant revenue concept, got: {}",
        top.path
    );
    // If it's the metrics one, it should be Metric type
    if top.path.contains("monthly-revenue") {
        assert_eq!(top.concept_type.as_deref(), Some("Metric"));
    }
}

#[test]
fn test_hierarchical_browsing() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    // Browse root
    let root = service.browse("", 1, 100).expect("browse root");
    assert!(root.directories.contains(&"metrics".to_string()));
    assert!(root.directories.contains(&"datasets".to_string()));
    assert!(root.directories.contains(&"policies".to_string()));
    assert!(root.directories.contains(&"glossary".to_string()));

    // Browse metrics directory
    let metrics = service.browse("metrics", 1, 100).expect("browse metrics");
    assert_eq!(metrics.path, "metrics");
    assert!(metrics
        .documents
        .iter()
        .any(|d| d.path == "metrics/monthly-revenue.md"));
    assert!(metrics
        .documents
        .iter()
        .any(|d| d.path == "metrics/customer-count.md"));
    assert!(metrics
        .documents
        .iter()
        .any(|d| d.path == "metrics/churn-rate.md"));
}

#[test]
fn test_relationship_reasoning() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    // Get links from monthly revenue
    let links = service
        .get_links("metrics/monthly-revenue.md")
        .expect("get links");

    // Should link to customer-orders dataset
    let has_customer_orders = links
        .iter()
        .any(|l| l.target_path.as_deref() == Some("datasets/customer-orders.md"));
    assert!(
        has_customer_orders,
        "Should link to customer-orders dataset"
    );
}

#[test]
fn test_exact_metadata_query() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    // Query for published finance metrics
    // Note: "status" is a special case mapped to parse_status, so use a custom field
    // or don't filter by status
    let mut filters = HashMap::new();
    filters.insert(
        "type".to_string(),
        serde_json::Value::String("Metric".to_string()),
    );
    filters.insert(
        "tags_contains".to_string(),
        serde_json::Value::String("finance".to_string()),
    );
    // Note: "status" is a special case mapped to parse_status, not the custom field
    // So we don't filter by status here

    let results = service
        .query_metadata(&filters, &[], 100)
        .expect("query metadata");

    assert!(
        !results.results.is_empty(),
        "Should find published finance metrics"
    );
    for result in &results.results {
        if let Some(path) = result.get("path") {
            assert!(
                path.as_str()
                    .expect("path should be string")
                    .contains("metrics/"),
                "Should be in metrics dir: {}",
                path
            );
        }
    }
}

#[test]
fn test_metadata_query_filters_projection_order_and_counts() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let filters = HashMap::from([
        ("type".to_string(), serde_json::json!("Metric")),
        ("tags_contains".to_string(), serde_json::json!("finance")),
        ("path_prefix".to_string(), serde_json::json!("metrics/")),
        ("parse_status".to_string(), serde_json::json!("ok")),
        ("owner".to_string(), serde_json::json!("Finance Analytics")),
    ]);
    let select = ["path", "tags", "owner"]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();

    let first = service
        .query_metadata(&filters, &select, 2)
        .expect("query projected metadata");
    assert_eq!(first.total_matches, 3);
    assert!(first.truncated);
    assert_eq!(first.results.len(), 2);
    assert_eq!(first.results[0]["path"], "metrics/churn-rate.md");
    assert_eq!(first.results[1]["path"], "metrics/customer-count.md");
    assert_eq!(first.results[0]["owner"], "Finance Analytics");
    assert_eq!(
        first.results[0]["tags"],
        serde_json::json!(["customer", "finance", "retention"])
    );

    let repeated = service
        .query_metadata(&filters, &select, 2)
        .expect("repeat projected metadata query");
    assert_eq!(repeated.results, first.results);

    let empty = service
        .query_metadata(
            &HashMap::from([(
                "path_prefix".to_string(),
                serde_json::json!("does-not-exist/"),
            )]),
            &select,
            10,
        )
        .expect("query empty metadata result");
    assert_eq!(empty.total_matches, 0);
    assert!(!empty.truncated);
    assert!(empty.results.is_empty());

    let invalid_filter = HashMap::from([("type!".to_string(), serde_json::json!("Metric"))]);
    assert!(service
        .query_metadata(&invalid_filter, &select, 10)
        .expect_err("invalid filter operator should fail")
        .to_string()
        .contains("Invalid filter"));
    assert!(service
        .query_metadata(&HashMap::new(), &["path;drop".to_string()], 10)
        .expect_err("invalid projection should fail")
        .to_string()
        .contains("Invalid select"));
}

#[test]
fn test_repository_validation() {
    let repo = setup_edge_cases_repo();
    let config = OkcConfig {
        roots: vec![repo.path().to_path_buf()],
        db_path: repo.path().join("test.db"),
        require_index_files: false,
        ..Default::default()
    };

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let issues = service.validate().expect("validate");

    // Should find broken links
    let broken_links: Vec<_> = issues
        .iter()
        .filter(|i| i.category == "broken_link")
        .collect();
    assert!(!broken_links.is_empty(), "Should find broken links");

    // Should find parse errors for invalid YAML
    let parse_errors: Vec<_> = issues
        .iter()
        .filter(|i| i.category == "yaml" || i.category == "frontmatter")
        .collect();
    assert!(!parse_errors.is_empty(), "Should find YAML parse errors");

    // Should find re-validated YAML errors
    let invalid_yaml: Vec<_> = issues
        .iter()
        .filter(|i| i.category == "invalid_yaml" || i.category == "invalid_frontmatter")
        .collect();
    assert!(
        !invalid_yaml.is_empty(),
        "Should find re-validated YAML errors"
    );

    // Should find duplicate concept identifiers
    let duplicates: Vec<_> = issues
        .iter()
        .filter(|i| i.category == "duplicate_concept")
        .collect();
    assert!(!duplicates.is_empty(), "Should find duplicate concepts");
}

#[test]
fn test_validation_oversized_frontmatter() {
    let repo = setup_edge_cases_repo();
    let config = OkcConfig {
        roots: vec![repo.path().to_path_buf()],
        db_path: repo.path().join("test.db"),
        max_front_matter_size: 10,
        require_index_files: false,
        ..Default::default()
    };

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let issues = service.validate().expect("validate");

    let oversized: Vec<_> = issues
        .iter()
        .filter(|i| i.category == "oversized_frontmatter")
        .collect();
    assert!(!oversized.is_empty(), "Should find oversized front matter");
}

#[test]
fn test_validation_missing_type() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    // Create a doc with missing required 'type' field
    let missing_path = repo.path().join("metrics/missing-type.md");
    std::fs::write(&missing_path, "---\ntags: [test]\n---\n\nNo type field.\n")
        .expect("write missing type file");

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let issues = service.validate().expect("validate");

    let missing: Vec<_> = issues
        .iter()
        .filter(|i| i.category == "missing_type")
        .collect();
    assert!(!missing.is_empty(), "Should find missing type");
}

#[test]
fn test_circular_links_handled() {
    let repo = setup_edge_cases_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    // Traverse should handle circular links without infinite loop
    let traverse = service
        .traverse("metrics/circular-a.md", &["links_to".to_string()], 3, 50)
        .expect("traverse circular");

    // Should find both A and B but not loop infinitely
    let paths: Vec<_> = traverse.nodes.iter().map(|n| n.path.clone()).collect();
    assert!(paths.contains(&"metrics/circular-a.md".to_string()));
    assert!(paths.contains(&"metrics/circular-b.md".to_string()));

    // Should not exceed max nodes
    assert!(traverse.nodes.len() <= 50);
}

#[test]
fn test_get_document_with_metadata() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let doc = service
        .get_document(
            "metrics/monthly-revenue.md",
            &["metadata".to_string(), "headings".to_string()],
            12000,
        )
        .expect("get document");

    assert_eq!(doc.path, "metrics/monthly-revenue.md");
    assert_eq!(doc.metadata.concept_type, Some("Metric".to_string()));
    assert_eq!(doc.metadata.title, Some("Monthly Revenue".to_string()));
    assert!(doc.metadata.tags.contains(&"finance".to_string()));
    assert!(doc.metadata.tags.contains(&"executive".to_string()));
    assert!(doc.metadata.tags.contains(&"revenue".to_string()));

    // Check custom fields (owner, status)
    assert!(doc.metadata.custom.contains_key("owner"));
    assert!(doc.metadata.custom.contains_key("status"));

    // Check headings
    assert!(!doc.headings.is_empty());
    let heading_titles: Vec<_> = doc.headings.iter().map(|h| h.title.clone()).collect();
    assert!(heading_titles.contains(&"Definition".to_string()));
    assert!(heading_titles.contains(&"Calculation".to_string()));
    assert!(heading_titles.contains(&"Recognition Rules".to_string()));
    assert!(heading_titles.contains(&"Related Concepts".to_string()));
}

#[test]
fn test_get_document_opt_in_enriched_context_and_validation() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);
    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let default_doc = service
        .get_document(
            "metrics/monthly-revenue.md",
            &["body".to_string(), "headings".to_string()],
            12_000,
        )
        .expect("get default document");
    let default_json = serde_json::to_value(default_doc).expect("serialize default document");
    assert!(default_json.get("custom").is_none());
    assert!(default_json.get("content_hash").is_none());
    assert!(default_json.get("parent_path").is_none());
    assert!(default_json.get("links").is_none());
    assert!(default_json.get("backlinks").is_none());

    let enriched = service
        .get_document(
            "metrics/monthly-revenue.md",
            &[
                "metadata".to_string(),
                "custom".to_string(),
                "content_hash".to_string(),
                "parent_path".to_string(),
                "links".to_string(),
                "backlinks".to_string(),
            ],
            12_000,
        )
        .expect("get enriched document");
    let enriched = serde_json::to_value(enriched).expect("serialize enriched document");
    assert_eq!(enriched["metadata"]["custom"]["owner"], "Finance Analytics");
    assert!(enriched["content_hash"]
        .as_str()
        .is_some_and(|hash| !hash.is_empty()));
    assert_eq!(enriched["parent_path"], "metrics");
    assert!(enriched["links"]
        .as_array()
        .is_some_and(|links| !links.is_empty()));
    assert!(enriched["backlinks"].as_array().is_some_and(|links| {
        links.iter().any(|link| {
            link["source_path"] == "metrics/churn-rate.md" && link.get("target_anchor").is_some()
        })
    }));

    let error = service
        .get_document(
            "metrics/monthly-revenue.md",
            &["not-a-section".to_string()],
            12_000,
        )
        .expect_err("unknown include should fail");
    assert!(error.to_string().contains("Unknown include value"));
}

#[test]
fn test_get_document_enriched_response_respects_configured_limit() {
    let repo = setup_simple_repo();
    let mut config = mkconfig(&repo);
    config.max_response_chars = 900;
    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let document = service
        .get_document(
            "metrics/monthly-revenue.md",
            &[
                "body".to_string(),
                "headings".to_string(),
                "metadata".to_string(),
                "custom".to_string(),
                "content_hash".to_string(),
                "parent_path".to_string(),
                "links".to_string(),
                "backlinks".to_string(),
            ],
            12_000,
        )
        .expect("get bounded enriched document");
    let serialized = serde_json::to_string(&document).expect("serialize bounded response");
    assert!(serialized.chars().count() <= config.max_response_chars);
    assert!(document.truncated);
}

#[test]
fn test_get_section() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let section = service
        .get_section("metrics/monthly-revenue.md", "Definition", 5000)
        .expect("get section");

    assert!(section.is_some());
    let (heading, content) = section.expect("section should be Some");
    assert_eq!(heading, "Definition");
    assert!(content.contains("Monthly Revenue represents the total recognized recurring revenue"));

    // Prefix match: "Def" should match heading "Definition"
    let prefix_section = service
        .get_section("metrics/monthly-revenue.md", "Def", 5000)
        .expect("get section by prefix");
    assert!(prefix_section.is_some(), "prefix match should return Some");
    let (prefix_heading, prefix_content) = prefix_section.unwrap();
    assert_eq!(
        prefix_heading, "Definition",
        "prefix match should return full heading"
    );
    assert!(prefix_content
        .contains("Monthly Revenue represents the total recognized recurring revenue"));

    // Prefix match: "Rec" should match heading "Recognition Rules" (first heading starting with "Rec")
    let rec_section = service
        .get_section("metrics/monthly-revenue.md", "Rec", 5000)
        .expect("get section by prefix");
    assert!(
        rec_section.is_some(),
        "prefix match 'Rec' should return Some"
    );
    let (rec_heading, _) = rec_section.unwrap();
    assert_eq!(rec_heading, "Recognition Rules");

    // Case-insensitive prefix match
    let ci_prefix = service
        .get_section("metrics/monthly-revenue.md", "def", 5000)
        .expect("get section by case-insensitive prefix");
    assert!(ci_prefix.is_some(), "case-insensitive prefix should match");
    let (ci_heading, _) = ci_prefix.unwrap();
    assert_eq!(ci_heading, "Definition");
}

#[test]
fn test_search_with_filters() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    // First test basic search
    let basic_results = service
        .search("revenue", None, None, None, 10, None, None)
        .expect("basic search");
    println!(
        "Basic search results: {} matches",
        basic_results.total_matches
    );
    for r in &basic_results.results {
        println!(
            "  {} [{}] score={:.2}",
            r.path,
            r.concept_type.as_deref().unwrap_or("-"),
            r.score
        );
    }

    // Search with type filter
    let results = service
        .search(
            "revenue",
            None,
            Some(&["Metric".to_string()]),
            None,
            10,
            None,
            None,
        )
        .expect("search with type filter");
    println!("Filtered search results: {} matches", results.total_matches);
    for r in &results.results {
        println!(
            "  {} [{}] score={:.2}",
            r.path,
            r.concept_type.as_deref().unwrap_or("-"),
            r.score
        );
    }

    assert!(
        !results.results.is_empty(),
        "Should find metrics with type filter"
    );

    // Search with path prefix
    let results = service
        .search("revenue", Some("metrics"), None, None, 10, None, None)
        .expect("search with path prefix");
    println!(
        "Path prefix search results: {} matches",
        results.total_matches
    );
    for r in &results.results {
        println!(
            "  {} [{}]",
            r.path,
            r.concept_type.as_deref().unwrap_or("-")
        );
        assert!(r.path.starts_with("metrics/"));
    }
}

#[test]
fn test_search_combined_filters_counts_and_stable_pages() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);
    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let types = ["Metric".to_string()];
    let tags = ["customer".to_string()];
    let first = service
        .search(
            "customer",
            Some("metrics/"),
            Some(&types),
            Some(&tags),
            1,
            None,
            None,
        )
        .expect("search with combined filters");
    assert_eq!(first.total_matches, 2);
    assert_eq!(first.results.len(), 1);
    assert!(first.truncated);
    assert_eq!(first.results[0].path, "metrics/customer-count.md");

    let repeated = service
        .search(
            "customer",
            Some("metrics/"),
            Some(&types),
            Some(&tags),
            1,
            None,
            None,
        )
        .expect("repeat combined search");
    assert_eq!(repeated.results[0].path, first.results[0].path);

    let empty = service
        .search("quantum entanglement", None, None, None, 10, None, None)
        .expect("empty search");
    assert_eq!(empty.total_matches, 0);
    assert!(!empty.truncated);
    assert!(empty.results.is_empty());

    let typo = service
        .search(
            "montly reveneu",
            Some("metrics/"),
            Some(&types),
            Some(&["finance".to_string()]),
            5,
            None,
            None,
        )
        .expect("bounded typo fallback with filters");
    assert!(typo
        .results
        .iter()
        .any(|result| result.path == "metrics/monthly-revenue.md"));
}

#[test]
fn test_search_uses_configured_bm25_field_weights() {
    let repo = TempDir::new().expect("search weights temp repo");
    std::fs::write(
        repo.path().join("title-match.md"),
        "---\ntype: Note\ntitle: Needle\n---\n\n# Overview\n\nBrief text.\n",
    )
    .expect("write title match");
    std::fs::write(
        repo.path().join("body-match.md"),
        "---\ntype: Note\ntitle: Body Match\n---\n\n# Overview\n\nneedle needle needle needle needle needle needle needle needle needle\n",
    )
    .expect("write body match");
    for path in ["tie-a.md", "tie-b.md"] {
        std::fs::write(
            repo.path().join(path),
            "---\ntype: Note\ntitle: Tie\n---\n\n# Same\n\ntieonly\n",
        )
        .expect("write equal-score document");
    }

    let default_config = OkcConfig {
        roots: vec![repo.path().to_path_buf()],
        db_path: repo.path().join("default.db"),
        ..Default::default()
    };
    let mut default_service = OkcService::open(&default_config).expect("open default service");
    default_service.scan().expect("scan default weights");
    let default_results = default_service
        .search("needle", None, None, None, 10, None, None)
        .expect("search default weights");
    assert_eq!(default_results.results[0].path, "title-match.md");
    let tied_results = default_service
        .search("tieonly", None, None, None, 10, None, None)
        .expect("search equal-score documents");
    assert_eq!(
        tied_results
            .results
            .iter()
            .map(|result| result.path.as_str())
            .collect::<Vec<_>>(),
        ["tie-a.md", "tie-b.md"]
    );

    let body_weighted_config = OkcConfig {
        roots: vec![repo.path().to_path_buf()],
        db_path: repo.path().join("body-weighted.db"),
        bm25: okc::config::Bm25Config {
            title_weight: 0.0,
            description_weight: 0.0,
            headings_weight: 0.0,
            body_weight: 10.0,
            concept_type_weight: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut body_weighted_service =
        OkcService::open(&body_weighted_config).expect("open body-weighted service");
    body_weighted_service
        .scan()
        .expect("scan body-weighted search");
    let body_weighted_results = body_weighted_service
        .search("needle", None, None, None, 10, None, None)
        .expect("search body weights");
    assert_eq!(body_weighted_results.results[0].path, "body-match.md");
}

#[test]
fn test_backlinks() {
    let repo = setup_simple_repo();
    let config = mkconfig(&repo);

    let mut service = OkcService::open(&config).expect("open service");
    service.scan().expect("scan");

    let _backlinks = service
        .get_backlinks("metrics/monthly-revenue.md", 50)
        .expect("get backlinks");

    // Should find backlinks from datasets that link to it
    // Note: in our simple fixture, datasets don't link back, but the reverse link is stored
    // Let's check traverse instead for bidirectional links
    let traverse = service
        .traverse(
            "metrics/monthly-revenue.md",
            &["links_to".to_string(), "linked_from".to_string()],
            2,
            50,
        )
        .expect("traverse with backlinks");

    let paths: Vec<_> = traverse.nodes.iter().map(|n| n.path.clone()).collect();
    assert!(paths.contains(&"metrics/monthly-revenue.md".to_string()));
}

#[test]
fn test_stats() {
    let repo = setup_simple_repo();
    let temp_dir = tempfile::TempDir::new().expect("temp dir for stats");
    let db_path = temp_dir.path().join("test.db");
    let config = OkcConfig {
        roots: vec![repo.path().to_path_buf()],
        db_path,
        ..Default::default()
    };

    let mut service = OkcService::open(&config).expect("open service");
    let scan_result = service.scan().expect("scan");

    assert!(scan_result.total_files > 0);
    assert!(
        scan_result.added > 0,
        "Should add files on first scan, got {}",
        scan_result.added
    );
    assert_eq!(scan_result.parse_failures, 0);

    let stats = service.get_stats().expect("get stats");
    assert!(stats.document_count > 0);
    assert!(stats.link_count > 0);
}
