use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OkfConfig {
    pub roots: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub max_file_size: u64,
    pub max_front_matter_size: usize,
    pub max_graph_depth: usize,
    pub max_graph_nodes: usize,
    pub follow_symlinks: bool,
    pub require_index_files: bool,
    pub db_path: PathBuf,
}

impl Default for OkfConfig {
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
            db_path: PathBuf::from("okf_index.db"),
        }
    }
}
