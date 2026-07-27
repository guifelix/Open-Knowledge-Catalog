//! MCP tool handler for scanning and indexing a knowledge base directory.
//!
//! Constructs an [`OkcConfig`] from [`ScanParams`], opens the service, and
//! runs a full scan (add, update, remove files). Returns a JSON-serialized
//! scan result with file counts and timing information.

use std::sync::{Arc, Mutex};
use crate::config::OkcConfig;
use crate::service::OkcService;
use crate::transport::mcp::types::{ScanParams, ScanResultOutput};

pub fn handle_scan(service: &Arc<Mutex<OkcService>>, params: ScanParams) -> String {
    let config = OkcConfig {
        roots: params.roots.into_iter().map(std::path::PathBuf::from).collect(),
        db_path: params.db_path.map_or_else(
            || std::path::PathBuf::from("okc_index.db"),
            std::path::PathBuf::from,
        ),
        ..Default::default()
    };

    match OkcService::open(&config).and_then(|mut svc| svc.scan()) {
        Ok(r) => serde_json::to_string(&ScanResultOutput {
            total_files: r.total_files,
            added: r.added,
            modified: r.modified,
            deleted: r.deleted,
            parse_failures: r.parse_failures,
            broken_links: r.broken_links,
            total_links: r.total_links,
            duration_secs: r.duration_secs,
        })
        .unwrap_or_default(),
        Err(e) => format!("Error: {}", e),
    }
}