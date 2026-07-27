//! Directory browsing operations.

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::index::database::RepositoryIndex;
use crate::model::directory::{BrowseResponse, DirectoryDocument};

/// Browse a directory in the knowledge base.
pub fn browse_directory(
    index: &RepositoryIndex,
    path: &str,
    depth: usize,
    limit: usize,
) -> Result<BrowseResponse, anyhow::Error> {
    let prefix = if path.is_empty() || path == "/" {
        String::new()
    } else {
        format!("{}/", path.trim_start_matches('/'))
    };

    let subdirs = if depth > 0 {
        let conn = index.pool().get()?;
        let mut stmt = conn.prepare(
            "SELECT DISTINCT parent_path FROM documents
             WHERE parent_path LIKE ?1 AND parent_path != ?2
             ORDER BY parent_path LIMIT ?3",
        )?;
        let pattern = format!("{}%", prefix);
        let dirs: Vec<String> = stmt
            .query_map(params![pattern, prefix, limit as i64], |row| {
                row.get::<_, String>(0)
            })?
            .filter_map(|r| r.ok())
            .filter(|p| {
                let remaining = p.strip_prefix(&prefix).unwrap_or(p);
                remaining.split('/').count() <= depth + 1 && !remaining.is_empty()
            })
            .collect();
        dirs
    } else {
        vec![]
    };

    let conn = index.pool().get()?;
    let mut stmt = conn.prepare(
        "SELECT path, title, type, description FROM documents
         WHERE parent_path = ?1 ORDER BY path LIMIT ?2",
    )?;
    let docs: Vec<DirectoryDocument> = stmt
        .query_map(params![prefix.trim_end_matches('/'), limit as i64], |row| {
            Ok(DirectoryDocument {
                path: row.get(0)?,
                title: row.get(1)?,
                concept_type: row.get(2)?,
                description: row.get(3)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    let truncated = subdirs.len() + docs.len() > limit;

    let index_path = if prefix.is_empty() {
        None
    } else {
        Some(format!("{}index.md", prefix))
    };

    Ok(BrowseResponse {
        path: path.to_string(),
        summary_document: index_path,
        directories: subdirs,
        documents: docs,
        truncated,
    })
}
