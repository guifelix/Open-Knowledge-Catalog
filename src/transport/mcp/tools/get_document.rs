//! MCP tool handler for retrieving a single document by path.
//!
//! Wraps [`OkcService::get_document`] into an MCP-compatible handler that
//! accepts [`GetDocumentParams`] and returns a JSON-serialized document with
//! full content, headings, links, backlinks, and metadata.

use std::sync::{Arc, Mutex};
use crate::service::OkcService;
use crate::transport::mcp::types::{GetDocumentParams, DocumentDetailOutput};

pub fn handle_get_document(service: &Arc<Mutex<OkcService>>, params: GetDocumentParams) -> String {
    let svc = service.lock().unwrap_or_else(|e| e.into_inner());
    match svc.get_document(&params.path) {
        Ok(r) => serde_json::to_string(&DocumentDetailOutput {
            path: r.path,
            title: r.title,
            concept_type: r.concept_type,
            description: r.description,
            content: r.content,
            headings: r.headings,
            links: r.links,
            backlinks: r.backlinks,
            metadata: r.metadata,
        })
        .unwrap_or_default(),
        Err(e) => format!("Error: {}", e),
    }
}