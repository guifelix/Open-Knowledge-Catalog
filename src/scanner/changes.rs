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
    pub deleted: Vec<String>,
    /// Files unchanged since previous scan (paths only).
    pub unchanged: Vec<String>,
}

/// Detects file system changes between scans.
///
/// Compares current and previous file records by path, using size and
/// modification time to detect modifications.
pub struct ChangeDetector;

impl ChangeDetector {
    /// Detect changes between current and previous file listings.
    ///
    /// Files are matched by path. A file is considered modified if either
    /// its size or modification timestamp differs from the previous scan.
    pub fn detect(current: &[FileRecord], previous: &[FileRecord]) -> FileChanges {
        let prev_map: std::collections::HashMap<&str, &FileRecord> =
            previous.iter().map(|f| (f.path.as_str(), f)).collect();

        let current_paths: HashSet<&str> = current.iter().map(|f| f.path.as_str()).collect();

        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut unchanged = Vec::new();

        for file in current {
            match prev_map.get(file.path.as_str()) {
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

        let deleted: Vec<String> = previous
            .iter()
            .map(|f| f.path.clone())
            .filter(|p| !current_paths.contains(p.as_str()))
            .collect();

        FileChanges {
            added,
            modified,
            deleted,
            unchanged,
        }
    }
}
