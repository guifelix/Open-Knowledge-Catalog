use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;

use ignore::{WalkBuilder, WalkState};
use tracing::info;

use crate::config::OkcConfig;
use crate::model::document::FileRecord;

pub struct Scanner;

impl Scanner {
    pub fn discover(config: &OkcConfig) -> Vec<FileRecord> {
        let (tx, rx) = mpsc::channel();
        let max_size = config.max_file_size;
        let follow_symlinks = config.follow_symlinks;
        let exclude_patterns = config.exclude_patterns.clone();
        let roots = config.roots.clone();

        thread::spawn(move || {
            for root in roots {
                info!("Scanning root: {:?}", root);
                let mut builder = WalkBuilder::new(&root);
                builder
                    .standard_filters(true)
                    .hidden(false)
                    .follow_links(follow_symlinks)
                    .max_depth(None);

                for pattern in &exclude_patterns {
                    builder.add_ignore(pattern);
                }

                let root_clone = root.clone();
                builder.build_parallel().run(|| {
                    let tx = tx.clone();
                    let root_clone = root_clone.clone();
                    Box::new(move |entry| {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.extension().is_some_and(|e| e == "md") {
                                    if let Ok(metadata) = entry.metadata() {
                                        if metadata.is_file() {
                                            let size = metadata.len();
                                            if size > max_size {
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

                                            let _ = tx.send(FileRecord {
                                                path: rel_path.to_string_lossy().to_string(),
                                                absolute_path: path.to_string_lossy().to_string(),
                                                size,
                                                modified_at,
                                            });
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

        let mut files: Vec<FileRecord> = rx.iter().collect();
        files.sort_by(|a, b| a.path.cmp(&b.path));
        files
    }
}

fn pathdiff(path: &Path, root: &Path) -> Option<PathBuf> {
    path.strip_prefix(root).ok().map(|p| p.to_path_buf())
}
