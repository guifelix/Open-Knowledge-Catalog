use std::collections::HashSet;

use crate::model::FileRecord;

#[derive(Debug)]
pub struct FileChanges {
    pub added: Vec<FileRecord>,
    pub modified: Vec<FileRecord>,
    pub deleted: Vec<String>,
    pub unchanged: Vec<String>,
}

pub struct ChangeDetector;

impl ChangeDetector {
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
