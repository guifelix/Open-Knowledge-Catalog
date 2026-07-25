//! Configuration types for the Open Knowledge Catalog indexer.
//!
//! This module defines the [`OkcConfig`] struct which controls all aspects of
//! the indexing process, including root directories, file filtering, size limits,
//! database location, and watcher behavior.

use std::path::PathBuf;

/// Configuration for the OKC indexer and service.
///
/// This struct controls all aspects of the indexing process:
/// - Which directories to scan (`roots`)
/// - Which files to exclude (`exclude_patterns`)
/// - Size limits for files and front-matter
/// - Graph traversal limits
/// - Database location and connection settings
/// - File watcher debouncing and reconciliation intervals
///
/// # Example
///
/// ```rust
/// use okc::config::OkcConfig;
/// use std::path::PathBuf;
///
/// let config = OkcConfig {
///     roots: vec![PathBuf::from("/path/to/knowledge-base")],
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct OkcConfig {
    /// Root directories to scan for markdown files.
    /// Each root is walked recursively (respecting exclude patterns).
    pub roots: Vec<PathBuf>,

    /// Glob patterns for files and directories to exclude from scanning.
    /// Defaults include common VCS, dependency, and build directories.
    pub exclude_patterns: Vec<String>,

    /// Maximum file size in bytes to process. Larger files are skipped.
    /// Default: 2 MiB.
    pub max_file_size: u64,

    /// Maximum front-matter size in bytes. Larger front-matter causes parse failure.
    /// Default: 64 KiB.
    pub max_front_matter_size: usize,

    /// Maximum depth for graph traversal operations.
    /// Default: 5.
    pub max_graph_depth: usize,

    /// Maximum number of nodes to return in graph traversal.
    /// Default: 100.
    pub max_graph_nodes: usize,

    /// Whether to follow symbolic links during scanning.
    /// Default: false (for safety).
    pub follow_symlinks: bool,

    /// Whether to require index files (e.g., `index.md`) for directory browsing.
    /// Default: false.
    pub require_index_files: bool,

    /// Path to the SQLite database file.
    /// Default: `okc_index.db` in the current directory.
    pub db_path: PathBuf,

    /// Debounce window in milliseconds for file watcher events.
    /// Multiple events within this window are batched.
    /// Default: 500ms.
    pub watcher_debounce_ms: u64,

    /// Full reconciliation interval in seconds for the file watcher.
    /// Periodically re-scans all roots to catch missed changes.
    /// Default: 600s (10 minutes).
    pub watcher_reconcile_secs: u64,
}

impl Default for OkcConfig {
    fn default() -> Self {
        Self {
            roots: vec![],
            exclude_patterns: vec![
                ".git/".into(),
                "node_modules/".into(),
                "vendor/".into(),
                "target/".into(),
                ".env".into(),
                "secrets/".into(),
                "credentials/".into(),
            ],
            max_file_size: 2 * 1024 * 1024,
            max_front_matter_size: 64 * 1024,
            max_graph_depth: 5,
            max_graph_nodes: 100,
            follow_symlinks: false,
            require_index_files: false,
            db_path: PathBuf::from("okc_index.db"),
            watcher_debounce_ms: 500,
            watcher_reconcile_secs: 600,
        }
    }
}
