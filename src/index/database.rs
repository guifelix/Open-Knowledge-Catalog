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
//!
//! Thread Safety: Uses a connection pool (r2d2) for thread-safe access.
//! All stores share the same pool, enabling concurrent reads through
//! SQLite's WAL mode while serializing writes.

use std::path::Path;
use std::sync::Arc;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use tracing::info;

use crate::config::OkcConfig;
use crate::index::document_store::SqliteDocumentStore;
use crate::index::graph_store::SqliteGraphStore;
use crate::index::parser::{process_changes, DocumentParser};
use crate::index::search_index::SqliteSearchIndex;
use crate::index::traits::{DocumentStore, GraphStore, SearchIndex};
use crate::model::document::{FileRecord, ParseStatus, ProcessChangesResult, ScanResult};
use crate::scanner::changes::{ChangeDetector, FileChanges};
use crate::scanner::walker::Scanner;

/// Primary repository index backed by SQLite with connection pooling.
///
/// Manages document storage, full-text search, link graph, and metadata.
/// Coordinates with [`SqliteGraphStore`] for graph operations and
/// [`SqliteSearchIndex`] for full-text search.
///
/// Thread Safety: Implements `Send + Sync` - all internal components
/// use a shared connection pool with WAL mode for concurrent access.
pub struct RepositoryIndex {
    pool: Arc<Pool<SqliteConnectionManager>>,
    pub(crate) document_store: SqliteDocumentStore,
    pub(crate) search_index: SqliteSearchIndex,
    pub(crate) graph_store: Option<SqliteGraphStore>,
    pub(crate) config: OkcConfig,
}

impl RepositoryIndex {
    /// Get a reference to the connection pool for direct queries.
    pub fn pool(&self) -> &Arc<Pool<SqliteConnectionManager>> {
        &self.pool
    }

    /// Open a new repository index at the configured database path.
    ///
    /// Initializes the schema if needed (via migrations) and prepares
    /// the graph store connection.
    pub fn open(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        // Create connection manager with WAL mode and foreign keys
        let manager = SqliteConnectionManager::file(&config.db_path).with_init(|conn| {
            conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
            Ok(())
        });

        let pool = Arc::new(Pool::new(manager)?);

        // Initialize schema on a connection from the pool
        {
            let conn = pool.get()?;
            crate::index::migrations::run(&conn)?;
        }

        let document_store = SqliteDocumentStore::new(pool.clone());
        let search_index = SqliteSearchIndex::new(pool.clone(), config.bm25.clone());
        let graph_store = SqliteGraphStore::new(pool.clone());

        let index = Self {
            pool,
            document_store,
            search_index,
            graph_store: Some(graph_store),
            config: config.clone(),
        };
        index.ensure_schema()?;
        Ok(index)
    }

    /// Open an in-memory repository index for testing.
    ///
    /// Uses an in-memory SQLite database with shared cache to allow
    /// multiple connections to the same in-memory DB.
    /// The graph store is not available in this mode.
    #[allow(dead_code)]
    pub fn open_in_memory(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        // Use shared cache URI to allow multiple connections to the same in-memory DB
        let manager = SqliteConnectionManager::memory().with_init(|conn| {
            conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
            Ok(())
        });

        let pool = Arc::new(Pool::new(manager)?);

        // Run migrations on a connection from the pool
        {
            let conn = pool.get()?;
            crate::index::migrations::run(&conn)?;
        }

        let document_store = SqliteDocumentStore::new(pool.clone());
        let search_index = SqliteSearchIndex::new(pool.clone(), config.bm25.clone());

        let index = Self {
            pool,
            document_store,
            search_index,
            graph_store: None,
            config: config.clone(),
        };
        index.ensure_schema()?;
        Ok(index)
    }

    fn ensure_schema(&self) -> Result<(), anyhow::Error> {
        self.document_store.init()?;
        self.search_index.init()?;
        if let Some(ref gs) = self.graph_store {
            gs.init()?;
        }
        Ok(())
    }

    /// Perform a full repository scan, detecting and processing all changes.
    ///
    /// Discovers all markdown files under configured roots, compares against
    /// the previous index state, and incrementally processes additions,
    /// modifications, and deletions.
    pub fn scan(&mut self) -> Result<ScanResult, anyhow::Error> {
        let start = std::time::Instant::now();

        let current_files = Scanner::discover(&self.config)?;
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
    /// This method
    /// wraps all changes in a single transaction for atomicity.
    pub fn process_changes(
        &mut self,
        changes: &FileChanges,
        known_paths: &[String],
    ) -> Result<ProcessChangesResult, anyhow::Error> {
        self.process_changes_transactional(changes, known_paths)
    }

    /// Internal method that processes changes within a database transaction.
    fn process_changes_transactional(
        &mut self,
        changes: &FileChanges,
        known_paths: &[String],
    ) -> Result<ProcessChangesResult, anyhow::Error> {
        // Get a connection from the pool and start a transaction
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        // Use the parser module to parse all changed files (outside transaction for I/O)
        let (parsed_docs, result) = process_changes(&self.config, changes, known_paths);

        // Handle deletions within transaction
        for path in &changes.deleted {
            info!("Removing deleted document: {path}");
            self.document_store.delete_document_tx(&tx, path)?;
            self.search_index.remove_document_tx(&tx, path)?;
            if let Some(ref gs) = self.graph_store {
                gs.remove_links_tx(&tx, path)?;
            }
        }

        // Store parsed documents within transaction
        for parsed in parsed_docs {
            self.store_parsed_document_tx(&tx, &parsed)?;
        }

        // Commit the transaction
        tx.commit()?;

        Ok(result)
    }

    fn store_parsed_document(
        &self,
        parsed: &crate::index::parser::ParsedDocument,
    ) -> Result<(), anyhow::Error> {
        use crate::index::traits::DocumentRecord;

        // Create document record
        let doc_record = DocumentRecord {
            id: 0, // Will be assigned by database
            path: parsed.path.clone(),
            parent_path: parsed.parent_path.clone(),
            title: parsed.title.clone(),
            concept_type: parsed.concept_type.clone(),
            description: parsed.description.clone(),
            body_text: parsed.markdown_body.clone(),
            file_size: parsed.size,
            modified_at: parsed.modified_at,
            content_hash: parsed.content_hash.clone(),
            parse_status: parse_status_to_str(&parsed.parse_status).to_string(),
        };

        // Upsert document
        self.document_store.upsert_document(&doc_record)?;

        // Get the document ID
        let doc_id = self.get_doc_id(&parsed.path)?;

        // Store tags
        if !parsed.tags.is_empty() {
            self.document_store.insert_tags(doc_id, &parsed.tags)?;
        }

        // Store headings
        if !parsed.headings.is_empty() {
            self.document_store
                .insert_headings(doc_id, &parsed.headings)?;
        }

        // Store links
        if !parsed.links.is_empty() {
            self.document_store.insert_links(doc_id, &parsed.links)?;
            if let Some(ref gs) = self.graph_store {
                // Convert LinkInfo to Link for graph store
                let links: Vec<crate::model::document::Link> = parsed
                    .links
                    .iter()
                    .map(|l| crate::model::document::Link {
                        raw: String::new(),
                        target: l
                            .target_path
                            .clone()
                            .unwrap_or_else(|| l.external_url.clone().unwrap_or_default()),
                        target_anchor: l.target_anchor.clone(),
                        is_external: l.external_url.is_some(),
                        exists_in_repository: l.exists_in_repository,
                    })
                    .collect();
                gs.store_links(&parsed.path, &links)?;
            }
        }

        // Store metadata fields
        if !parsed.custom_fields.is_empty() {
            self.document_store
                .insert_metadata_fields(doc_id, &parsed.custom_fields)?;
        }

        // Store scan errors
        if !parsed.parse_errors.is_empty() {
            self.document_store
                .insert_scan_errors(&parsed.path, &parsed.parse_errors)?;
        }

        // Index for search
        let searchable = crate::index::traits::SearchableDocument {
            path: parsed.path.clone(),
            title: parsed.title.clone(),
            description: parsed.description.clone(),
            headings: parsed
                .headings
                .iter()
                .map(|h| h.title.clone())
                .collect::<Vec<_>>()
                .join(" "),
            body: parsed.body_text.clone(),
            concept_type: parsed.concept_type.clone(),
        };
        self.search_index.index_document(&searchable)?;

        Ok(())
    }

    /// Transactional version of store_parsed_document for use within a transaction.
    fn store_parsed_document_tx(
        &self,
        tx: &rusqlite::Transaction,
        parsed: &crate::index::parser::ParsedDocument,
    ) -> Result<(), anyhow::Error> {
        use crate::index::traits::DocumentRecord;

        // Create document record
        let doc_record = DocumentRecord {
            id: 0, // Will be assigned by database
            path: parsed.path.clone(),
            parent_path: parsed.parent_path.clone(),
            title: parsed.title.clone(),
            concept_type: parsed.concept_type.clone(),
            description: parsed.description.clone(),
            body_text: parsed.markdown_body.clone(),
            file_size: parsed.size,
            modified_at: parsed.modified_at,
            content_hash: parsed.content_hash.clone(),
            parse_status: parse_status_to_str(&parsed.parse_status).to_string(),
        };

        // Upsert document within transaction
        self.document_store.upsert_document_tx(tx, &doc_record)?;

        // Get the document ID
        let doc_id = self.get_doc_id_tx(tx, &parsed.path)?;

        // Store tags
        if !parsed.tags.is_empty() {
            self.document_store
                .insert_tags_tx(tx, doc_id, &parsed.tags)?;
        }

        // Store headings
        if !parsed.headings.is_empty() {
            self.document_store
                .insert_headings_tx(tx, doc_id, &parsed.headings)?;
        }

        // Store links
        if !parsed.links.is_empty() {
            self.document_store
                .insert_links_tx(tx, doc_id, &parsed.links)?;
            if let Some(ref gs) = self.graph_store {
                // Convert LinkInfo to Link for graph store
                let links: Vec<crate::model::document::Link> = parsed
                    .links
                    .iter()
                    .map(|l| crate::model::document::Link {
                        raw: String::new(),
                        target: l
                            .target_path
                            .clone()
                            .unwrap_or_else(|| l.external_url.clone().unwrap_or_default()),
                        target_anchor: l.target_anchor.clone(),
                        is_external: l.external_url.is_some(),
                        exists_in_repository: l.exists_in_repository,
                    })
                    .collect();
                gs.store_links_tx(tx, &parsed.path, &links)?;
            }
        }

        // Store metadata fields
        if !parsed.custom_fields.is_empty() {
            self.document_store
                .insert_metadata_fields_tx(tx, doc_id, &parsed.custom_fields)?;
        }

        // Store scan errors
        if !parsed.parse_errors.is_empty() {
            self.document_store
                .insert_scan_errors_tx(tx, &parsed.path, &parsed.parse_errors)?;
        }

        // Index for search
        let searchable = crate::index::traits::SearchableDocument {
            path: parsed.path.clone(),
            title: parsed.title.clone(),
            description: parsed.description.clone(),
            headings: parsed
                .headings
                .iter()
                .map(|h| h.title.clone())
                .collect::<Vec<_>>()
                .join(" "),
            body: parsed.body_text.clone(),
            concept_type: parsed.concept_type.clone(),
        };
        self.search_index.index_document_tx(tx, &searchable)?;

        Ok(())
    }

    fn get_doc_id_tx(&self, tx: &rusqlite::Transaction, path: &str) -> Result<i64, anyhow::Error> {
        let mut stmt = tx.prepare("SELECT id FROM documents WHERE path = ?1")?;
        let id: i64 = stmt.query_row([path], |row| row.get(0))?;
        Ok(id)
    }

    fn get_doc_id(&self, path: &str) -> Result<i64, anyhow::Error> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id FROM documents WHERE path = ?1")?;
        let id: i64 = stmt.query_row([path], |row| row.get(0))?;
        Ok(id)
    }

    fn load_file_records(&self) -> Result<Vec<FileRecord>, anyhow::Error> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT path, file_size, modified_at FROM documents")?;
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
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT path FROM documents")?;
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

// Implement Send + Sync for RepositoryIndex since all internal components are thread-safe
unsafe impl Send for RepositoryIndex {}
unsafe impl Sync for RepositoryIndex {}
