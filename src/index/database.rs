//! Main repository index implementation backed by SQLite.
//!
//! [`RepositoryIndex`] is the central storage component managing:
//! - Document metadata and content (titles, front-matter, body text)
//! - Heading hierarchy for each document
//! - Internal and external links with resolution status
//! - Custom metadata fields
//! - Full-text search index (via [`SqliteSearchIndex`])
//! - Graph edges for link traversal (via [`SqliteGraphStore`])
//! - Scan error tracking
//!
//! Uses SQLite with WAL mode for concurrent read access and supports
//! both file-based and in-memory storage for testing.

use std::collections::BTreeMap;
use std::path::Path;

use rusqlite::{params, Connection};
use tracing::info;

use crate::config::OkcConfig;
use crate::index::graph_store::SqliteGraphStore;

use crate::model::document::{
    FileRecord, FrontMatter, ParseError, ParseStatus, ProcessChangesResult, ScanResult,
};
use crate::parser::frontmatter::FrontMatterExtractor;
use crate::parser::links::LinkResolver;
use crate::parser::markdown::MarkdownParser;
use crate::parser::yaml::YamlParser;
use crate::scanner::changes::{ChangeDetector, FileChanges};
use crate::scanner::walker::Scanner;

/// Primary repository index backed by SQLite.
///
/// Manages document storage, full-text search, link graph, and metadata.
/// Coordinates with [`SqliteGraphStore`] for graph operations and
/// [`SqliteSearchIndex`] for full-text search.
pub struct RepositoryIndex {
    pub(crate) conn: Connection,
    pub(crate) graph_store: Option<SqliteGraphStore>,
    pub(crate) config: OkcConfig,
}

impl RepositoryIndex {
    /// Open a new repository index at the configured database path.
    ///
    /// Initializes the schema if needed (via migrations) and prepares
    /// the graph store connection.
    pub fn open(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        let conn = Connection::open(&config.db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let graph_conn = Connection::open(&config.db_path)?;
        let index = Self {
            conn,
            graph_store: Some(SqliteGraphStore::new(graph_conn)),
            config: config.clone(),
        };
        index.ensure_schema()?;
        Ok(index)
    }

    /// Open an in-memory repository index for testing.
    ///
    /// Uses an in-memory SQLite database with the same schema.
    /// The graph store is not available in this mode.
    #[allow(dead_code)]
    pub fn open_in_memory(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let index = Self {
            conn,
            graph_store: None,
            config: config.clone(),
        };
        index.ensure_schema()?;
        Ok(index)
    }

    fn ensure_schema(&self) -> Result<(), anyhow::Error> {
        crate::index::migrations::run(&self.conn)?;
        Ok(())
    }

    /// Perform a full repository scan, detecting and processing all changes.
    ///
    /// Discovers all markdown files under configured roots, compares against
    /// the previous index state, and incrementally processes additions,
    /// modifications, and deletions.
    pub fn scan(&mut self) -> Result<ScanResult, anyhow::Error> {
        let start = std::time::Instant::now();

        let current_files = Scanner::discover(&self.config);
        let previous_files = self.load_file_records()?;
        let changes = ChangeDetector::detect(&current_files, &previous_files);

        info!(
            "Scan: {} added, {} modified, {} deleted, {} unchanged",
            changes.added.len(),
            changes.modified.len(),
            changes.deleted.len(),
            changes.unchanged.len()
        );

        let known_paths: Vec<String> = current_files.iter().map(|f| f.path.clone()).collect();

        let result = self.process_changes(&changes, &known_paths)?;

        let duration = start.elapsed();

        Ok(ScanResult {
            total_files: current_files.len(),
            added: result.files_added,
            modified: result.files_modified,
            deleted: result.files_deleted,
            parse_failures: result.parse_failures,
            broken_links: result.broken_links,
            total_links: result.total_links,
            duration_secs: duration.as_secs_f64(),
        })
    }

    /// Process a set of file changes (added, modified, deleted) incrementally.
    /// Used by both full `scan()` and the incremental watcher.
    pub fn process_changes(
        &mut self,
        changes: &FileChanges,
        known_paths: &[String],
    ) -> Result<ProcessChangesResult, anyhow::Error> {
        let mut parse_failures = 0;
        let mut total_links = 0;
        let mut broken_links = 0;
        let mut collected_errors: Vec<(String, String, String, Option<usize>)> = Vec::new();

        let tx = self.conn.transaction()?;

        for path in &changes.deleted {
            info!("Removing deleted document: {path}");
            tx.execute("DELETE FROM documents WHERE path = ?1", params![path])?;
            tx.execute("DELETE FROM scan_errors WHERE path = ?1", params![path])?;
        }

        let extractor = FrontMatterExtractor::new(self.config.max_front_matter_size);

        for file in changes.added.iter().chain(changes.modified.iter()) {
            let path = Path::new(&file.absolute_path);
            let content = match std::fs::read(path) {
                Ok(c) => c,
                Err(e) => {
                    info!("Cannot read {}: {}", file.path, e);
                    parse_failures += 1;
                    collected_errors.push((
                        file.path.clone(),
                        "read".to_string(),
                        e.to_string(),
                        None,
                    ));
                    continue;
                }
            };

            let body = match String::from_utf8(content) {
                Ok(s) => s,
                Err(e) => {
                    info!("Invalid UTF-8 in {}: {}", file.path, e);
                    parse_failures += 1;
                    collected_errors.push((
                        file.path.clone(),
                        "utf8".to_string(),
                        e.to_string(),
                        None,
                    ));
                    continue;
                }
            };

            let (front_matter, parse_status, parse_errors, front_matter_end) =
                Self::parse_front_matter(&extractor, &file.path, &body);

            let markdown_body = if front_matter_end < body.len() {
                body[front_matter_end..].trim_start()
            } else {
                ""
            };

            let (headings, links, body_text, _sections) = MarkdownParser::parse(markdown_body);

            let resolved_links = LinkResolver::resolve_links(&file.path, &links, known_paths);
            total_links += resolved_links.len();
            broken_links += resolved_links
                .iter()
                .filter(|l| !l.is_external && !l.exists_in_repository)
                .count();

            let content_hash = blake3::hash(body.as_bytes()).to_hex().to_string();
            let parent_path = Path::new(&file.path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            let (title, concept_type, description, tags, custom_fields) = match &front_matter {
                Some(fm) => {
                    let tags_ref = &fm.tags;
                    let custom_ref = &fm.custom;
                    (
                        fm.title.as_deref(),
                        fm.concept_type.as_deref(),
                        fm.description.as_deref(),
                        tags_ref,
                        custom_ref,
                    )
                }
                None => (None, None, None, &Vec::new(), &BTreeMap::new()),
            };

            let headings_text: String = headings
                .iter()
                .map(|h| h.title.as_str())
                .collect::<Vec<_>>()
                .join(" ");

            tx.execute(
                "INSERT OR REPLACE INTO documents
                    (path, parent_path, title, type, description, body_text,
                     file_size, modified_at, content_hash, parse_status)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    file.path,
                    parent_path,
                    title,
                    concept_type,
                    description,
                    markdown_body,
                    file.size,
                    file.modified_at,
                    content_hash,
                    parse_status_to_str(&parse_status),
                ],
            )?;

            let doc_id: i64 = tx.last_insert_rowid();

            tx.execute(
                "DELETE FROM document_tags WHERE document_id = ?1",
                params![doc_id],
            )?;
            for tag in tags {
                tx.execute(
                    "INSERT INTO document_tags (document_id, tag) VALUES (?1, ?2)",
                    params![doc_id, tag],
                )?;
            }

            tx.execute(
                "DELETE FROM headings WHERE document_id = ?1",
                params![doc_id],
            )?;
            for heading in &headings {
                tx.execute(
                    "INSERT INTO headings (document_id, level, title, anchor, position)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        doc_id,
                        heading.level,
                        heading.title,
                        heading.anchor,
                        heading.position
                    ],
                )?;
            }

            tx.execute(
                "DELETE FROM links WHERE source_document_id = ?1",
                params![doc_id],
            )?;
            for link in &resolved_links {
                tx.execute(
                    "INSERT INTO links (source_document_id, target_path, target_anchor, external_url, exists_in_repository)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        doc_id,
                        if link.is_external { None } else { Some(&link.target) },
                        link.target_anchor,
                        if link.is_external { Some(&link.target) } else { None },
                        if link.is_external { 1 } else { link.exists_in_repository as i32 },
                    ],
                )?;
            }

            tx.execute(
                "DELETE FROM metadata_fields WHERE document_id = ?1",
                params![doc_id],
            )?;
            for (key, value) in custom_fields {
                let val_str = serde_json::to_string(value).unwrap_or_default();
                tx.execute(
                    "INSERT INTO metadata_fields (document_id, key, value) VALUES (?1, ?2, ?3)",
                    params![doc_id, key, val_str],
                )?;
            }

            tx.execute(
                "DELETE FROM document_search WHERE path = ?1",
                params![file.path],
            )?;
            tx.execute(
                "INSERT INTO document_search (path, title, description, headings, body)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    file.path,
                    title.unwrap_or(""),
                    description.unwrap_or(""),
                    headings_text,
                    body_text,
                ],
            )?;

            for err in &parse_errors {
                collected_errors.push((
                    file.path.clone(),
                    err.stage.clone(),
                    err.message.clone(),
                    err.line,
                ));
            }
        }

        tx.commit()?;

        if !collected_errors.is_empty() {
            let tx2 = self.conn.transaction()?;
            for (path, stage, message, line) in collected_errors {
                tx2.execute(
                    "INSERT INTO scan_errors (path, stage, message, line) VALUES (?1, ?2, ?3, ?4)",
                    params![path, stage, message, line.map(|l| l as i64)],
                )?;
            }
            tx2.commit()?;
        }

        Ok(ProcessChangesResult {
            files_added: changes.added.len(),
            files_modified: changes.modified.len(),
            files_deleted: changes.deleted.len(),
            parse_failures,
            broken_links,
            total_links,
        })
    }

    fn parse_front_matter(
        extractor: &FrontMatterExtractor,
        _path: &str,
        body: &str,
    ) -> (Option<FrontMatter>, ParseStatus, Vec<ParseError>, usize) {
        let mut errors = Vec::new();

        let extracted = match extractor.extract(body.as_bytes()) {
            Ok(r) => r,
            Err(e) => {
                errors.push(e);
                return (None, ParseStatus::Partial, errors, 0);
            }
        };

        let (front_matter_end, raw_yaml) = match extracted {
            Some(r) => r,
            None => return (None, ParseStatus::Ok, errors, 0),
        };

        match YamlParser::parse(&raw_yaml) {
            Ok(fm) => (Some(fm), ParseStatus::Ok, errors, front_matter_end),
            Err(e) => {
                errors.push(e);
                (None, ParseStatus::Partial, errors, front_matter_end)
            }
        }
    }

    fn load_file_records(&self) -> Result<Vec<FileRecord>, anyhow::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT path, file_size, modified_at FROM documents")?;
        let records = stmt
            .query_map([], |row| {
                Ok(FileRecord {
                    path: row.get(0)?,
                    absolute_path: String::new(),
                    size: row.get::<_, i64>(1)? as u64,
                    modified_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(records)
    }

    /// Load all known document paths (for link-resolution in incremental updates).
    pub fn load_paths(&self) -> Result<Vec<String>, anyhow::Error> {
        let mut stmt = self.conn.prepare("SELECT path FROM documents")?;
        let paths = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(paths)
    }
}

fn parse_status_to_str(status: &ParseStatus) -> &'static str {
    match status {
        ParseStatus::Ok => "ok",
        ParseStatus::Partial => "partial",
        ParseStatus::Failed => "failed",
    }
}
