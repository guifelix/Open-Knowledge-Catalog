use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OkcConfig {
    pub roots: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub max_file_size: u64,
    pub max_front_matter_size: usize,
    pub max_graph_depth: usize,
    pub max_graph_nodes: usize,
    pub follow_symlinks: bool,
    pub require_index_files: bool,
    pub db_path: PathBuf,
    pub watcher_debounce_ms: u64,
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
