//! File system watching integration for incremental index updates.
//!
//! [`OkcService::watch`] starts a cross-platform file watcher that:
//! - Debounces rapid file changes (configurable)
//! - Filters editor temporary files
//! - Processes changes incrementally via [`ChangeDetector`]
//! - Periodically runs full reconciliation to catch missed events

use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

use tracing::{error, info, warn};

use crate::model::document::FileRecord;
use crate::scanner::changes::FileChanges;
use crate::scanner::watcher::{FileWatcher, WatchEvent};
use crate::service::OkcService;

impl OkcService {
    /// Watch the configured roots for changes and update the index incrementally.
    ///
    /// `initial_scan` – if `true`, runs a full scan before watching.
    /// Returns when the watcher thread exits (currently only on unrecoverable error).
    pub fn watch(&mut self, initial_scan: bool) -> Result<(), anyhow::Error> {
        let roots = self.index.config.roots.clone();
        if roots.is_empty() {
            anyhow::bail!(
                "No root directories configured. Set `roots` in config or pass `--root`."
            );
        }

        if initial_scan {
            info!("Running initial full scan before watching...");
            let result = self.scan()?;
            info!(
                "Initial scan: {} files ({} added, {} modified, {} deleted) in {:.2}s",
                result.total_files,
                result.added,
                result.modified,
                result.deleted,
                result.duration_secs
            );
        }

        let debounce_ms = self.index.config.watcher_debounce_ms;
        let reconcile_secs = self.index.config.watcher_reconcile_secs;

        info!(
            "Starting file watcher (debounce={debounce_ms}ms, reconcile={reconcile_secs}s): {:?}",
            roots
        );

        let watcher = FileWatcher::new(roots.clone(), debounce_ms, reconcile_secs);
        let rx = watcher.start()?;

        loop {
            match rx.recv() {
                Ok(WatchEvent::Changes(paths)) => {
                    if let Err(e) = self.handle_watch_changes(&paths) {
                        error!("Error processing watch changes: {e}");
                    }
                }
                Ok(WatchEvent::Reconcile) => {
                    info!("Running periodic full reconciliation...");
                    match self.scan() {
                        Ok(result) => {
                            info!(
                                "Reconciliation complete: {} files ({:.2}s)",
                                result.total_files, result.duration_secs
                            );
                        }
                        Err(e) => {
                            error!("Reconciliation scan failed: {e}");
                        }
                    }
                }
                Err(_) => {
                    info!("Watch channel closed, exiting.");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Handle an incremental batch of changed file paths from the watcher.
    /// Determines which files were added/modified vs deleted, then processes
    /// them through the index.
    fn handle_watch_changes(
        &mut self,
        changed: &HashSet<std::path::PathBuf>,
    ) -> Result<(), anyhow::Error> {
        let canonical_roots: Vec<std::path::PathBuf> = self
            .index
            .config
            .roots
            .iter()
            .filter_map(|r| std::fs::canonicalize(r).ok())
            .collect();

        // Separate changes into added/modified (still on disk) vs deleted (gone).
        let mut added_or_modified: Vec<FileRecord> = Vec::new();
        let mut deleted: Vec<String> = Vec::new();

        for pb in changed {
            let canonical = std::fs::canonicalize(pb).unwrap_or_else(|_| pb.clone());

            // Compute the relative path used in the index
            let full_path_str = canonical.to_string_lossy().to_string();
            let rel_path = canonical_roots
                .iter()
                .find_map(|root| canonical.strip_prefix(root).ok())
                .and_then(|rel| rel.to_str())
                .unwrap_or(&full_path_str)
                .to_string();

            if canonical.exists() {
                match Self::stat_file(pb, &rel_path) {
                    Ok(record) => added_or_modified.push(record),
                    Err(e) => warn!("Cannot stat changed file {rel_path}: {e}"),
                }
            } else {
                info!("Detected deleted file: {rel_path}");
                deleted.push(rel_path);
            }
        }

        if added_or_modified.is_empty() && deleted.is_empty() {
            return Ok(());
        }

        // Build the known_paths list from the index for link resolution
        let known_paths: Vec<String> = self.index.load_paths()?;

        let changes = FileChanges {
            added: added_or_modified,
            modified: Vec::new(),
            deleted,
            unchanged: Vec::new(),
        };

        let result = self.index.process_changes(&changes, &known_paths)?;

        info!(
            "Incremental update: {} added, {} modified, {} deleted ({} parse failures, {} broken links)",
            result.files_added,
            result.files_modified,
            result.files_deleted,
            result.parse_failures,
            result.broken_links,
        );

        Ok(())
    }

    fn stat_file(path: &Path, rel_path: &str) -> Result<FileRecord, anyhow::Error> {
        let meta = std::fs::metadata(path)?;
        Ok(FileRecord {
            path: rel_path.to_string(),
            absolute_path: std::fs::canonicalize(path)
                .unwrap_or_else(|_| path.to_path_buf())
                .to_string_lossy()
                .to_string(),
            size: meta.len(),
            modified_at: meta
                .modified()?
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::ZERO)
                .as_secs() as i64,
        })
    }
}
