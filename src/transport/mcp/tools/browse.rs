//! MCP tool handler for browsing the knowledge base directory tree.
//!
//! Wraps [`OkcService::browse`] into an MCP-compatible handler that accepts
//! [`BrowseParams`] and returns a JSON-serialized directory listing with
//! documents, subdirectories, and optional depth/limit controls.

use std::sync::{Arc, Mutex};
use crate::service::OkcService;
use crate::transport::mcp::types::{BrowseParams, BrowseResultOutput, DirectoryDocumentOutput};

pub fn handle_browse(service: &Arc<Mutex<OkcService>>, params: BrowseParams) -> String {
    let depth = params.depth.unwrap_or(1);
    let limit = params.limit.unwrap_or(100);
    let path = params.path.unwrap_or_default();

    let svc = service.lock().unwrap_or_else(|e| e.into_inner());
    match svc.browse(&path, depth, limit) {
        Ok(r) => serde_json::to_string(&BrowseResultOutput {
            path: r.path,
            summary_document: r.summary_document,
            directories: r.directories,
            documents: r
                .documents
                .into_iter()
                .map(|d| DirectoryDocumentOutput {
                    path: d.path,
                    title: d.title,
                    concept_type: d.concept_type,
                    description: d.description,
                })
                .collect(),
            truncated: r.truncated,
        })
        .unwrap_or_default(),
        Err(e) => format!("Error: {}", e),
    }
}