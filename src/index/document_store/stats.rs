//! Stats and query operations for SqliteDocumentStore.

use crate::index::traits::Result;
use crate::model::document::stats::RootStats;
use crate::model::document::{IndexStats, MetadataQueryResponse};
use rusqlite::{params, Connection, Row};
use std::collections::{BTreeMap, HashMap};

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

fn row_to_root_stats(row: &Row<'_>) -> rusqlite::Result<RootStats> {
    Ok(RootStats {
        root_id: row.get(0)?,
        document_count: row.get::<_, i64>(1)? as usize,
        error_count: row.get::<_, i64>(2)? as usize,
        link_count: row.get::<_, i64>(3)? as usize,
        heading_count: row.get::<_, i64>(4)? as usize,
    })
}

pub fn get_stats(conn: &Connection) -> Result<IndexStats> {
    let doc_count: i64 = conn.query_row("SELECT COUNT(*) FROM documents", [], |row| row.get(0))?;
    let error_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM scan_errors", [], |row| row.get(0))?;
    let link_count: i64 = conn.query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))?;
    let heading_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM headings", [], |row| row.get(0))?;

    // Per-root stats
    let mut stmt = conn.prepare(
        "SELECT r.root_id,
                COUNT(d.id) as doc_count,
                COUNT(se.id) as error_count,
                COUNT(l.id) as link_count,
                COUNT(h.document_id) as heading_count
         FROM roots r
         LEFT JOIN documents d ON d.root_id = r.id
         LEFT JOIN scan_errors se ON se.path = d.path
         LEFT JOIN links l ON l.source_document_id = d.id
         LEFT JOIN headings h ON h.document_id = d.id
         GROUP BY r.id, r.root_id
         ORDER BY r.root_id",
    )?;
    let roots: BTreeMap<String, RootStats> = stmt
        .query_map([], row_to_root_stats)?
        .filter_map(|r| r.ok())
        .map(|rs| (rs.root_id.clone(), rs))
        .collect();

    Ok(IndexStats {
        document_count: doc_count as usize,
        error_count: error_count as usize,
        link_count: link_count as usize,
        heading_count: heading_count as usize,
        roots,
    })
}
