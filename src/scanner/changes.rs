//! Change detection between file system scans.
//!
//! [`ChangeDetector`] compares current and previous file listings to identify
//! added, modified, deleted, and unchanged files for incremental indexing.

use std::collections::HashSet;

use crate::model::document::FileRecord;

/// Result of change detection between two scans.
#[derive(Debug)]
pub struct FileChanges {
    /// New files not present in previous scan.
    pub added: Vec<FileRecord>,
    /// Files with changed size or modification time.
    pub modified: Vec<FileRecord>,
    /// Files present in previous scan but not current.
    pub deleted: Vec<FileRecord>,
    /// Files unchanged since previous scan (paths only).
    pub unchanged: Vec<String>,
}

/// Detects file system changes between scans.
///
/// Compares current and previous file records by (root_id, path). A file is considered modified if either
/// its size or modification timestamp differs from the previous scan.
pub struct ChangeDetector;

impl ChangeDetector {
    /// Detect changes between current and previous file listings.
    ///
    /// Files are matched by (root_id, path). A file is considered modified if either
    /// its size or modification timestamp differs from the previous scan.
    pub fn detect(current: &[FileRecord], previous: &[FileRecord]) -> FileChanges {
        let prev_map: std::collections::HashMap<String, &FileRecord> = previous
            .iter()
            .map(|f| (format!("{}::{}", f.root_id, f.path), f))
            .collect();

        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut unchanged = Vec::new();

        for file in current {
            let key = format!("{}::{}", file.root_id, file.path);
            match prev_map.get(&key) {
                Some(prev) => {
                    if prev.modified_at == file.modified_at && prev.size == file.size {
                        unchanged.push(file.path.clone());
                    } else {
                        modified.push(file.clone());
                    }
                }
                None => {
                    added.push(file.clone());
                }
            }
        }

        let current_keys: HashSet<String> = current
            .iter()
            .map(|f| format!("{}::{}", f.root_id, f.path))
            .collect();

        let deleted: Vec<FileRecord> = previous
            .iter()
            .filter(|f| !current_keys.contains(&format!("{}::{}", f.root_id, f.path)))
            .cloned()
            .collect();

        FileChanges {
            added,
            modified,
            deleted,
            unchanged,
        }
    }
}
