//! Open Knowledge Catalog (OKC) - Command-line interface.
//!
//! This binary provides the `okc` command-line tool for indexing and querying
//! markdown-based knowledge bases with front-matter metadata, wiki-style links,
//! and graph-based navigation.
//!
//! See the library crate documentation for architecture details.

use crate::config::OkcConfig;
use crate::transport::cli::{Cli, Command, TransportType};
use clap::Parser;
use std::net::SocketAddr;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::service::OkcService;

mod config;
mod index;
mod model;
mod parser;
mod scanner;
mod service;
mod transport;

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // Load configuration: defaults -> config file -> env vars -> CLI flags
    let mut config = OkcConfig::load(cli.config.as_deref())?;

    // Apply CLI overrides (highest priority)
    apply_cli_overrides(&mut config, &cli.command);

    match cli.command {
        Command::Scan { root: _ } => {
            let mut service = OkcService::open(&config)?;
            let result = service.scan()?;
            println!("Scan complete:");
            println!("  Total files: {}", result.total_files);
            println!("  Added: {}", result.added);
            println!("  Modified: {}", result.modified);
            println!("  Deleted: {}", result.deleted);
            println!("  Parse failures: {}", result.parse_failures);
            println!("  Broken links: {}", result.broken_links);
            println!("  Total links: {}", result.total_links);
            println!("  Duration: {:.2}s", result.duration_secs);
        }
        Command::Browse { path, depth, limit } => {
            let service = OkcService::open(&config)?;
            let result = service.browse(&path.unwrap_or_default(), depth, limit)?;
            println!("Browsing: {}", result.path);
            if let Some(idx) = result.summary_document {
                println!("  Index: {}", idx);
            }
            println!("  Directories:");
            for d in &result.directories {
                println!("    {}", d);
            }
            println!("  Documents:");
            for d in &result.documents {
                println!(
                    "    {} [{}] - {}",
                    d.path,
                    d.concept_type.as_deref().unwrap_or("-"),
                    d.description.as_deref().unwrap_or("-")
                );
            }
            if result.truncated {
                println!("  ... (truncated)");
            }
        }
        Command::Get {
            path,
            include,
            max_chars,
        } => {
            let service = OkcService::open(&config)?;
            let result = service.get_document(&path, &include, max_chars)?;
            println!("Document: {}", result.path);
            println!("  Title: {:?}", result.metadata.title);
            println!("  Type: {:?}", result.metadata.concept_type);
            println!("  Description: {:?}", result.metadata.description);
            println!("  Tags: {:?}", result.metadata.tags);
            if !result.metadata.custom.is_empty() {
                println!("  Custom: {:?}", result.metadata.custom);
            }
            println!("  File size: {}", result.metadata.file_size);
            println!("  Modified: {}", result.metadata.modified_at);
            println!("  Parse status: {}", result.metadata.parse_status);
            if !result.headings.is_empty() {
                println!("  Headings:");
                for h in &result.headings {
                    println!(
                        "    {} {} (anchor: {:?})",
                        "#".repeat(h.level as usize),
                        h.title,
                        h.anchor
                    );
                }
            }
            if let Some(body) = result.body {
                println!("  Body: {}", body);
            }
            if result.truncated {
                println!("  ... (truncated)");
            }
            if !result.errors.is_empty() {
                println!("  Errors:");
                for e in &result.errors {
                    println!("    [{}] {} (line: {:?})", e.stage, e.message, e.line);
                }
            }
        }
        Command::Section {
            path,
            heading,
            max_chars,
        } => {
            let service = OkcService::open(&config)?;
            if let Some((h, content)) = service.get_section(&path, &heading, max_chars)? {
                println!("Section: {}", h);
                println!("{}", content);
            } else {
                println!("Section not found");
            }
        }
        Command::Search {
            query,
            path_prefix,
            types,
            tags,
            limit,
        } => {
            let service = OkcService::open(&config)?;
            let result = service.search(
                &query,
                path_prefix.as_deref(),
                if types.is_empty() { None } else { Some(&types) },
                if tags.is_empty() { None } else { Some(&tags) },
                limit,
            )?;
            println!("Search results for '{}':", query);
            println!("  Total matches: {}", result.total_matches);
            for r in &result.results {
                println!(
                    "  {} [{}] - score: {:.2}",
                    r.path,
                    r.concept_type.as_deref().unwrap_or("-"),
                    r.score
                );
                println!("    {}", r.excerpt);
            }
            if result.truncated {
                println!("  ... (truncated)");
            }
        }
        Command::Metadata {
            filter,
            select: _select,
            limit,
        } => {
            let service = OkcService::open(&config)?;
            let filters: std::collections::HashMap<String, serde_json::Value> = filter
                .into_iter()
                .filter_map(|(k, v)| Some((k, serde_json::Value::String(v))))
                .collect();
            let result = service.query_metadata(&filters, limit)?;
            println!("Metadata query results: {} matches", result.total_matches);
            for r in &result.results {
                println!("  {}", r);
            }
            if result.truncated {
                println!("  ... (truncated)");
            }
        }
        Command::Links { path } => {
            let service = OkcService::open(&config)?;
            let result = service.get_links(&path)?;
            println!("Links from {}:", path);
            for l in &result {
                if l.external_url.is_some() {
                    println!("  -> EXTERNAL: {:?}", l.external_url);
                } else {
                    println!(
                        "  -> {} (anchor: {:?}, exists: {})",
                        l.target_path.as_deref().unwrap_or("-"),
                        l.target_anchor,
                        l.exists_in_repository
                    );
                }
            }
        }
        Command::Backlinks { path, limit } => {
            let service = OkcService::open(&config)?;
            let result = service.get_backlinks(&path, limit)?;
            println!("Backlinks to {}:", path);
            for l in &result {
                if l.external_url.is_some() {
                    println!("  <- EXTERNAL: {:?}", l.external_url);
                } else {
                    println!(
                        "  <- {} (anchor: {:?})",
                        l.target_path.as_deref().unwrap_or("-"),
                        l.target_anchor
                    );
                }
            }
        }
        Command::Traverse {
            start,
            relations,
            max_depth,
            max_nodes,
        } => {
            let service = OkcService::open(&config)?;
            let result = service.traverse(&start, &relations, max_depth, max_nodes)?;
            println!("Graph traversal from {}:", start);
            println!("  Nodes: {}", result.nodes.len());
            println!("  Edges: {}", result.edges.len());
            for n in &result.nodes {
                println!(
                    "  [{}] {} [{}]",
                    n.depth,
                    n.path,
                    n.concept_type.as_deref().unwrap_or("-")
                );
            }
            for e in &result.edges {
                println!("  {} --[{}]-> {}", e.source, e.relation, e.target);
            }
            if result.truncated {
                println!("  ... (truncated)");
            }
        }
        Command::Validate { json } => {
            let service = OkcService::open(&config)?;
            let (has_errors, has_warnings) = if json {
                let report = service.validate_report()?;
                println!("{}", serde_json::to_string_pretty(&report)?);
                (report.summary.errors > 0, report.summary.warnings > 0)
            } else {
                let result = service.validate()?;
                if result.is_empty() {
                    println!("No validation issues found");
                } else {
                    println!("Validation issues ({}):", result.len());
                    for issue in &result {
                        println!(
                            "  [{}] {}: {} (line: {:?})",
                            issue.severity, issue.path, issue.message, issue.line
                        );
                    }
                }
                let has_errors = result.iter().any(|i| i.severity == "error");
                let has_warnings = result.iter().any(|i| i.severity == "warning");
                (has_errors, has_warnings)
            };
            if has_errors {
                std::process::exit(1);
            } else if has_warnings {
                std::process::exit(2);
            }
        }
        Command::Stats => {
            let service = OkcService::open(&config)?;
            let result = service.get_stats()?;
            println!("Index stats:");
            println!("  Documents: {}", result.document_count);
            println!("  Errors: {}", result.error_count);
            println!("  Links: {}", result.link_count);
            println!("  Headings: {}", result.heading_count);
        }

        Command::Watch { skip_initial, .. } => {
            let mut service = OkcService::open(&config)?;
            service.watch(!skip_initial)?;
        }

        Command::Serve {
            root,
            transport,
            host,
            port,
        } => {
            let roots = if root.is_empty() {
                vec![std::env::current_dir()?]
            } else {
                root
            };
            config.roots = roots;

            let server = crate::transport::mcp::McpServer::new(&config)?;

            match transport {
                TransportType::Stdio => {
                    let rt = tokio::runtime::Runtime::new()?;
                    rt.block_on(async {
                        let (stdin, stdout) = rmcp::transport::io::stdio();
                        rmcp::service::serve_server(server, (stdin, stdout)).await
                    })?;
                }
                TransportType::Http => {
                    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
                    tracing::info!("Starting MCP HTTP server on http://{}", addr);
                    let rt = tokio::runtime::Runtime::new()?;
                    rt.block_on(server.serve_http(addr))?;
                }
            }
        }
    }

    Ok(())
}

/// Apply CLI flag overrides to the configuration (highest priority).
fn apply_cli_overrides(config: &mut OkcConfig, command: &Command) {
    // Extract root directories from the command
    let roots = match command {
        Command::Scan { root } => root.clone(),
        Command::Serve { root, .. } => root.clone(),
        Command::Watch { root, .. } => root.clone(),
        _ => vec![],
    };
    if !roots.is_empty() {
        config.roots = roots;
    }

    // Apply watcher config overrides from CLI
    if let Command::Watch {
        debounce,
        reconcile,
        ..
    } = command
    {
        config.watcher_debounce_ms = *debounce;
        config.watcher_reconcile_secs = *reconcile;
    }
}
