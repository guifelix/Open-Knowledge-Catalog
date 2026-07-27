//! Statistics query operations.

use crate::index::database::RepositoryIndex;
use crate::model::document::IndexStats;
use rusqlite::params;

/// Get index statistics (document count, link count, etc.).
pub fn get_stats(index: &RepositoryIndex) -> Result<IndexStats, anyhow::Error> {
    let conn = index.pool().get()?;

    let document_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents",
        [],
        |row| row.get(0),
    )?;

    let error_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE parse_status != 'Ok'",
        [],
        |row| row.get(0),
    )?;

    let link_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM links",
        [],
        |row| row.get(0),
    )?;

    let heading_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM headings",
        [],
        |row| row.get(0),
    )?;

    Ok(IndexStats {
        document_count: document_count as usize,
        error_count: error_count as usize,
        link_count: link_count as usize,
        heading_count: heading_count as usize,
    })
}

/// Get recently modified documents.
pub fn get_recently_modified(
    index: &RepositoryIndex,
    limit: usize,
) -> Result<Vec<crate::model::document::DocumentSummary>, anyhow::Error> {
    let conn = index.pool().get()?;
    let mut stmt = conn.prepare(
        "SELECT path, title, type, description FROM documents
         ORDER BY modified_at DESC LIMIT ?1",
    )?;

    let docs = stmt
        .query_map(params![limit as i64], |row| {
            Ok(crate::model::document::DocumentSummary {
                path: row.get(0)?,
                title: row.get(1)?,
                concept_type: row.get(2)?,
                description: row.get(3)?,
                tags: vec![],
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(docs)
}