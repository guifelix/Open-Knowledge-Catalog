//! Parallel filesystem walker for markdown file discovery.
//!
//! [`Scanner`] uses the `ignore` crate for efficient, parallel directory traversal
//! with support for:
//! - `.gitignore` and standard ignore patterns
//! - Configurable exclude patterns
//! - Symlink following (optional)
//! - File size limits
//! - Extension filtering (`.md` files only)

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;

use ignore::{WalkBuilder, WalkState};
use tracing::info;

use crate::config::{OkcConfig, RootConfig};
use crate::model::document::{FileRecord, LimitError};

/// Discovers markdown files in configured repository roots.
///
/// Runs a parallel walk for each root directory, sending discovered
/// `.md` files through a channel for collection and sorting.
pub struct Scanner;

impl Scanner {
    /// Discover all markdown files under configured roots.
    ///
    /// Returns a sorted vector of [`FileRecord`] with path, size, and mtime.
    /// Respects exclude patterns, size limits, and symlink settings from config.
    pub fn discover(config: &OkcConfig) -> Result<Vec<FileRecord>, LimitError> {
        let (tx, rx) = mpsc::channel();
        let max_size = config.max_file_size;
        let follow_symlinks = config.follow_symlinks;
        let exclude_patterns = config.exclude_patterns.clone();
        let roots = config.roots.clone();

        thread::spawn(move || {
            for root_config in roots {
                let root_path = &root_config.path;
                let root_id = root_config.root_id();
                info!("Scanning root: {:?} (id: {})", root_path, root_id);
                let mut builder = WalkBuilder::new(root_path);
                builder
                    .standard_filters(true)
                    .hidden(false)
                    .follow_links(follow_symlinks)
                    .max_depth(None);

                for pattern in &exclude_patterns {
                    builder.add_ignore(pattern);
                }

                let root_clone = root_path.clone();
                let root_id_clone = root_id.clone();
                builder.build_parallel().run(|| {
                    let tx = tx.clone();
                    let root_clone = root_clone.clone();
                    let root_id_clone = root_id_clone.clone();
                    Box::new(move |entry| {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.extension().is_some_and(|e| e == "md") {
                                    if let Ok(metadata) = entry.metadata() {
                                        if metadata.is_file() {
                                            let size = metadata.len();
                                            if size > max_size {
                                                let _ = tx.send(Err(LimitError::new(
                                                    "max_file_size",
                                                    &max_size.to_string(),
                                                    &format!(
                                                        "File exceeds maximum size of {} bytes",
                                                        max_size
                                                    ),
                                                )
                                                .with_actual(&size.to_string())));
                                                return WalkState::Continue;
                                            }

                                            let modified_at = metadata
                                                .modified()
                                                .ok()
                                                .and_then(|t| {
                                                    t.duration_since(SystemTime::UNIX_EPOCH).ok()
                                                })
                                                .map_or(0, |d| d.as_secs() as i64);

                                            let rel_path = pathdiff(path, &root_clone)
                                                .unwrap_or_else(|| path.to_path_buf());

                                            let _ = tx.send(Ok(FileRecord {
                                                path: rel_path.to_string_lossy().to_string(),
                                                absolute_path: path.to_string_lossy().to_string(),
                                                size,
                                                modified_at,
                                                root_id: root_id_clone.clone(),
                                            }));
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                info!("Walk error: {}", err);
                            }
                        }
                        WalkState::Continue
                    })
                });
            }
        });

        let mut files: Vec<FileRecord> = Vec::new();
        for result in rx.iter() {
            let file = result?;
            files.push(file);
        }
        files.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(files)
    }
}

/// Compute relative path from root to path.
fn pathdiff(path: &Path, root: &Path) -> Option<PathBuf> {
    path.strip_prefix(root).ok().map(|p| p.to_path_buf())
}
