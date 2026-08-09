//! Statistics query operations.

use crate::error::Result;
use crate::index::database::RepositoryIndex;
use crate::index::traits::DocumentStore;
use crate::model::document::IndexStats;
use rusqlite::params;

/// Get index statistics (document count, link count, etc.).
pub fn get_stats(index: &RepositoryIndex) -> Result<IndexStats> {
    index.document_store.get_stats()
}

/// Get recently modified documents.
pub fn get_recently_modified(
    index: &RepositoryIndex,
    limit: usize,
) -> Result<Vec<crate::model::document::DocumentSummary>> {
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
