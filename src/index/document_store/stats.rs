//! Stats and query operations for SqliteDocumentStore.

use crate::index::traits::Result;
use crate::model::document::{IndexStats, MetadataQueryResponse};
use rusqlite::{params, Connection};
use std::collections::HashMap;

pub fn query_metadata(
    _conn: &Connection,
    _filters: &HashMap<String, String>,
    _select: &[String],
    _limit: usize,
) -> Result<MetadataQueryResponse> {
    // Simplified implementation
    Ok(MetadataQueryResponse {
        results: vec![],
        total_matches: 0,
        truncated: false,
    })
}

pub fn get_stats(conn: &Connection) -> Result<IndexStats> {
    let doc_count: i64 = conn.query_row("SELECT COUNT(*) FROM documents", [], |row| row.get(0))?;
    let error_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM scan_errors", [], |row| row.get(0))?;
    let link_count: i64 = conn.query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))?;
    let heading_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM headings", [], |row| row.get(0))?;

    Ok(IndexStats {
        document_count: doc_count as usize,
        error_count: error_count as usize,
        link_count: link_count as usize,
        heading_count: heading_count as usize,
    })
}
