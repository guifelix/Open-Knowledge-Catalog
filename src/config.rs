//! Configuration types for the Open Knowledge Catalog indexer.
//!
//! This module defines the [`OkcConfig`] struct which controls all aspects of
//! the indexing process, including root directories, file filtering, size limits,
//! database location, and watcher behavior.
//!
//! Configuration can be loaded from:
//! 1. Default values (lowest priority)
//! 2. TOML config file at `./okc.toml` or `~/.config/okc/config.toml`
//! 3. Environment variables with `OKC_` prefix (e.g., `OKC_ROOTS`, `OKC_DB_PATH`)
//! 4. CLI flags (highest priority)

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod bm25;

pub mod search;

#[cfg(test)]
pub mod tests;

pub use bm25::Bm25Config;
pub use search::SearchConfig;

/// Configuration for the OKC indexer and service.
///
/// This struct controls all aspects of the indexing process:
/// - Which directories to scan (`roots`)
/// - Which files to exclude (`exclude_patterns`)
/// - Size limits for files and front-matter
/// - Graph traversal limits
/// - Database location and connection settings
/// - File watcher debouncing and reconciliation intervals
/// - BM25 search relevance ranking parameters
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
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct OkcConfig {
    /// Root directories to scan for markdown files.
    /// Each root is walked recursively (respecting exclude patterns).
    #[serde(default)]
    pub roots: Vec<PathBuf>,

    /// Glob patterns for files and directories to exclude from scanning.
    /// Defaults include common VCS, dependency, and build directories.
    #[serde(default = "default_exclude_patterns")]
    pub exclude_patterns: Vec<String>,

    /// Maximum file size in bytes to process. Larger files are skipped.
    /// Default: 2 MiB.
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,

    /// Maximum front-matter size in bytes. Larger front-matter causes parse failure.
    /// Default: 64 KiB.
    #[serde(default = "default_max_front_matter_size")]
    pub max_front_matter_size: usize,

    /// Maximum YAML input size in bytes. Larger input is rejected before parsing
    /// to prevent pathological inputs from causing OOM in the YAML library.
    /// Default: 8 MiB.
    #[serde(default = "default_max_yaml_input_size")]
    pub max_yaml_input_size: usize,

    /// Maximum depth for graph traversal operations.
    /// Default: 5.
    #[serde(default = "default_max_graph_depth")]
    pub max_graph_depth: usize,

    /// Maximum number of nodes to return in graph traversal.
    /// Default: 100.
    #[serde(default = "default_max_graph_nodes")]
    pub max_graph_nodes: usize,

    /// Maximum serialized character count for a document response.
    /// Default: 500,000 characters.
    #[serde(default = "default_max_response_chars")]
    pub max_response_chars: usize,

    /// Whether to follow symbolic links during scanning.
    /// Default: false (for safety).
    #[serde(default)]
    pub follow_symlinks: bool,

    /// Whether to require index files (e.g., `index.md`) for directory browsing.
    /// Default: false.
    #[serde(default)]
    pub require_index_files: bool,

    /// Path to the SQLite database file.
    /// Default: `okc_index.db` in the current directory.
    #[serde(default = "default_db_path")]
    pub db_path: PathBuf,

    /// Debounce window in milliseconds for file watcher events.
    /// Multiple events within this window are batched.
    /// Default: 500ms.
    #[serde(default = "default_watcher_debounce_ms")]
    pub watcher_debounce_ms: u64,

    /// Full reconciliation interval in seconds for the file watcher.
    /// Periodically re-scans all roots to catch missed changes.
    /// Default: 600s (10 minutes).
    #[serde(default = "default_watcher_reconcile_secs")]
    pub watcher_reconcile_secs: u64,

    /// BM25 search relevance ranking configuration.
    /// Controls field weights and algorithm parameters for FTS5 search.
    #[serde(default)]
    pub bm25: Bm25Config,

    /// Search result configuration.
    /// Controls heading extraction and display in search results.
    #[serde(default)]
    pub search: SearchConfig,
}

fn default_exclude_patterns() -> Vec<String> {
    vec![
        ".git/".into(),
        "node_modules/".into(),
        "vendor/".into(),
        "target/".into(),
        ".env".into(),
        "secrets/".into(),
        "credentials/".into(),
    ]
}

fn default_max_file_size() -> u64 {
    2 * 1024 * 1024
}

fn default_max_front_matter_size() -> usize {
    64 * 1024
}

fn default_max_yaml_input_size() -> usize {
    8 * 1024 * 1024
}

fn default_max_graph_depth() -> usize {
    5
}

fn default_max_graph_nodes() -> usize {
    100
}

fn default_max_response_chars() -> usize {
    500_000
}

fn default_db_path() -> PathBuf {
    PathBuf::from("okc_index.db")
}

fn default_watcher_debounce_ms() -> u64 {
    500
}

fn default_watcher_reconcile_secs() -> u64 {
    600
}

impl Default for OkcConfig {
    fn default() -> Self {
        Self {
            roots: vec![],
            exclude_patterns: default_exclude_patterns(),
            max_file_size: default_max_file_size(),
            max_front_matter_size: default_max_front_matter_size(),
            max_yaml_input_size: default_max_yaml_input_size(),
            max_graph_depth: default_max_graph_depth(),
            max_graph_nodes: default_max_graph_nodes(),
            max_response_chars: default_max_response_chars(),
            follow_symlinks: false,
            require_index_files: false,
            db_path: default_db_path(),
            watcher_debounce_ms: default_watcher_debounce_ms(),
            watcher_reconcile_secs: default_watcher_reconcile_secs(),
            bm25: Bm25Config::default(),
            search: SearchConfig::default(),
        }
    }
}

/// Errors that can occur during configuration loading and validation.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    NotFound(PathBuf),

    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse TOML config: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Failed to serialize TOML config: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("Config validation failed: {0}")]
    ValidationError(String),

    #[error("Environment variable parse error: {0}")]
    EnvParseError(String),
}

impl OkcConfig {
    /// Load configuration from file, environment variables, and defaults.
    ///
    /// Priority order (highest to lowest):
    /// 1. CLI overrides (applied by caller after this function)
    /// 2. Environment variables (OKC_*)
    /// 3. Config file (./okc.toml or ~/.config/okc/config.toml)
    /// 4. Default values
    ///
    /// If `config_path` is Some, only that file is tried.
    /// If `config_path` is None, the following locations are checked in order:
    /// - ./okc.toml (current directory)
    /// - ~/.config/okc/config.toml (XDG config directory)
    pub fn load(config_path: Option<&Path>) -> Result<Self, ConfigError> {
        let mut config = Self::default();

        // Load from config file if present
        let paths = Self::config_file_paths(config_path);
        let mut loaded = false;
        for path in paths {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                config = toml::from_str(&content)?;
                tracing::info!("Loaded config from: {}", path.display());
                loaded = true;
                break;
            }
        }

        // If explicit path was provided but not found, return error
        if let Some(path) = config_path {
            if !loaded {
                return Err(ConfigError::NotFound(path.to_path_buf()));
            }
        }

        // Apply environment variable overrides
        config.apply_env_overrides()?;

        Ok(config)
    }

    /// Get the list of config file paths to check, in priority order.
    fn config_file_paths(explicit_path: Option<&Path>) -> Vec<PathBuf> {
        if let Some(path) = explicit_path {
            return vec![path.to_path_buf()];
        }

        let mut paths = vec![PathBuf::from("okc.toml")];

        if let Some(config_dir) = dirs::config_dir() {
            paths.push(config_dir.join("okc").join("config.toml"));
        }

        paths
    }

    /// Apply environment variable overrides to the configuration.
    ///
    /// Environment variables use the `OKC_` prefix with uppercase snake_case names:
    /// - OKC_ROOTS (comma-separated paths)
    /// - OKC_DB_PATH
    /// - OKC_MAX_FILE_SIZE
    /// - OKC_MAX_FRONT_MATTER_SIZE
    /// - OKC_MAX_YAML_INPUT_SIZE
    /// - OKC_MAX_GRAPH_DEPTH
    /// - OKC_MAX_GRAPH_NODES
    /// - OKC_MAX_RESPONSE_CHARS
    /// - OKC_FOLLOW_SYMLINKS (true/false)
    /// - OKC_REQUIRE_INDEX_FILES (true/false)
    /// - OKC_WATCHER_DEBOUNCE_MS
    /// - OKC_WATCHER_RECONCILE_SECS
    /// - OKC_EXCLUDE_PATTERNS (comma-separated)
    /// - OKC_BM25_TITLE_WEIGHT
    /// - OKC_BM25_DESCRIPTION_WEIGHT
    /// - OKC_BM25_HEADINGS_WEIGHT
    /// - OKC_BM25_BODY_WEIGHT
    /// - OKC_BM25_CONCEPT_TYPE_WEIGHT
    /// - OKC_BM25_K1
    /// - OKC_BM25_B
    fn apply_env_overrides(&mut self) -> Result<(), ConfigError> {
        // Roots (comma-separated)
        if let Ok(roots) = std::env::var("OKC_ROOTS") {
            self.roots = roots
                .split(',')
                .map(|s| PathBuf::from(s.trim()))
                .filter(|p| !p.as_os_str().is_empty())
                .collect();
        }

        // Database path
        if let Ok(db_path) = std::env::var("OKC_DB_PATH") {
            self.db_path = PathBuf::from(db_path);
        }

        // Max file size
        if let Ok(val) = std::env::var("OKC_MAX_FILE_SIZE") {
            self.max_file_size = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_MAX_FILE_SIZE: invalid u64: {val}"))
            })?;
        }

        // Max front matter size
        if let Ok(val) = std::env::var("OKC_MAX_FRONT_MATTER_SIZE") {
            self.max_front_matter_size = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!(
                    "OKC_MAX_FRONT_MATTER_SIZE: invalid usize: {val}"
                ))
            })?;
        }

        // Max YAML input size
        if let Ok(val) = std::env::var("OKC_MAX_YAML_INPUT_SIZE") {
            self.max_yaml_input_size = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_MAX_YAML_INPUT_SIZE: invalid usize: {val}"))
            })?;
        }

        // Max graph depth
        if let Ok(val) = std::env::var("OKC_MAX_GRAPH_DEPTH") {
            self.max_graph_depth = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_MAX_GRAPH_DEPTH: invalid usize: {val}"))
            })?;
        }

        // Max graph nodes
        if let Ok(val) = std::env::var("OKC_MAX_GRAPH_NODES") {
            self.max_graph_nodes = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_MAX_GRAPH_NODES: invalid usize: {val}"))
            })?;
        }

        if let Ok(val) = std::env::var("OKC_MAX_RESPONSE_CHARS") {
            self.max_response_chars = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_MAX_RESPONSE_CHARS: invalid usize: {val}"))
            })?;
        }

        // Follow symlinks
        if let Ok(val) = std::env::var("OKC_FOLLOW_SYMLINKS") {
            self.follow_symlinks = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_FOLLOW_SYMLINKS: invalid bool: {val}"))
            })?;
        }

        // Require index files
        if let Ok(val) = std::env::var("OKC_REQUIRE_INDEX_FILES") {
            self.require_index_files = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_REQUIRE_INDEX_FILES: invalid bool: {val}"))
            })?;
        }

        // Watcher debounce ms
        if let Ok(val) = std::env::var("OKC_WATCHER_DEBOUNCE_MS") {
            self.watcher_debounce_ms = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_WATCHER_DEBOUNCE_MS: invalid u64: {val}"))
            })?;
        }

        // Watcher reconcile secs
        if let Ok(val) = std::env::var("OKC_WATCHER_RECONCILE_SECS") {
            self.watcher_reconcile_secs = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!(
                    "OKC_WATCHER_RECONCILE_SECS: invalid u64: {val}"
                ))
            })?;
        }

        // Exclude patterns (comma-separated)
        if let Ok(patterns) = std::env::var("OKC_EXCLUDE_PATTERNS") {
            self.exclude_patterns = patterns
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // Search max headings
        if let Ok(val) = std::env::var("OKC_SEARCH_MAX_HEADINGS") {
            self.search.max_headings = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_SEARCH_MAX_HEADINGS: invalid usize: {val}"))
            })?;
        }

        // Search heading depth
        if let Ok(val) = std::env::var("OKC_SEARCH_HEADING_DEPTH") {
            self.search.heading_depth = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_SEARCH_HEADING_DEPTH: invalid u32: {val}"))
            })?;
        }

        // BM25 title weight
        if let Ok(val) = std::env::var("OKC_BM25_TITLE_WEIGHT") {
            self.bm25.title_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_TITLE_WEIGHT: invalid f64: {val}"))
            })?;
        }

        // BM25 description weight
        if let Ok(val) = std::env::var("OKC_BM25_DESCRIPTION_WEIGHT") {
            self.bm25.description_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!(
                    "OKC_BM25_DESCRIPTION_WEIGHT: invalid f64: {val}"
                ))
            })?;
        }

        // BM25 headings weight
        if let Ok(val) = std::env::var("OKC_BM25_HEADINGS_WEIGHT") {
            self.bm25.headings_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_HEADINGS_WEIGHT: invalid f64: {val}"))
            })?;
        }

        // BM25 body weight
        if let Ok(val) = std::env::var("OKC_BM25_BODY_WEIGHT") {
            self.bm25.body_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_BODY_WEIGHT: invalid f64: {val}"))
            })?;
        }

        // BM25 concept type weight
        if let Ok(val) = std::env::var("OKC_BM25_CONCEPT_TYPE_WEIGHT") {
            self.bm25.concept_type_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!(
                    "OKC_BM25_CONCEPT_TYPE_WEIGHT: invalid f64: {val}"
                ))
            })?;
        }

        // BM25 k1
        if let Ok(val) = std::env::var("OKC_BM25_K1") {
            self.bm25.k1 = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_K1: invalid f64: {val}"))
            })?;
        }

        // BM25 b
        if let Ok(val) = std::env::var("OKC_BM25_B") {
            self.bm25.b = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_B: invalid f64: {val}"))
            })?;
        }

        Ok(())
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.roots.is_empty() {
            return Err(ConfigError::ValidationError(
                "At least one root directory must be specified".into(),
            ));
        }

        for root in &self.roots {
            if !root.exists() {
                return Err(ConfigError::ValidationError(format!(
                    "Root directory does not exist: {}",
                    root.display()
                )));
            }
            if !root.is_dir() {
                return Err(ConfigError::ValidationError(format!(
                    "Root path is not a directory: {}",
                    root.display()
                )));
            }
        }

        if self.max_file_size == 0 {
            return Err(ConfigError::ValidationError(
                "max_file_size must be greater than 0".into(),
            ));
        }

        if self.max_front_matter_size == 0 {
            return Err(ConfigError::ValidationError(
                "max_front_matter_size must be greater than 0".into(),
            ));
        }

        if self.max_yaml_input_size == 0 {
            return Err(ConfigError::ValidationError(
                "max_yaml_input_size must be greater than 0".into(),
            ));
        }

        if self.max_graph_depth == 0 {
            return Err(ConfigError::ValidationError(
                "max_graph_depth must be greater than 0".into(),
            ));
        }

        if self.max_graph_nodes == 0 {
            return Err(ConfigError::ValidationError(
                "max_graph_nodes must be greater than 0".into(),
            ));
        }
        if self.max_response_chars == 0 {
            return Err(ConfigError::ValidationError(
                "max_response_chars must be greater than 0".into(),
            ));
        }

        if self.watcher_debounce_ms == 0 {
            return Err(ConfigError::ValidationError(
                "watcher_debounce_ms must be greater than 0".into(),
            ));
        }

        if self.watcher_reconcile_secs == 0 {
            return Err(ConfigError::ValidationError(
                "watcher_reconcile_secs must be greater than 0".into(),
            ));
        }

        // Validate search config
        if self.search.max_headings == 0 {
            return Err(ConfigError::ValidationError(
                "search.max_headings must be greater than 0".into(),
            ));
        }

        if self.search.heading_depth == 0 {
            return Err(ConfigError::ValidationError(
                "search.heading_depth must be greater than 0".into(),
            ));
        }

        // Validate BM25 config
        if self.bm25.title_weight < 0.0
            || self.bm25.description_weight < 0.0
            || self.bm25.headings_weight < 0.0
            || self.bm25.body_weight < 0.0
            || self.bm25.concept_type_weight < 0.0
        {
            return Err(ConfigError::ValidationError(
                "BM25 weights must be non-negative".into(),
            ));
        }

        if self.bm25.k1 <= 0.0 {
            return Err(ConfigError::ValidationError(
                "BM25 k1 must be positive".into(),
            ));
        }

        if !(0.0..=1.0).contains(&self.bm25.b) {
            return Err(ConfigError::ValidationError(
                "BM25 b must be in range [0.0, 1.0]".into(),
            ));
        }

        // Validate db_path parent directory exists or can be created
        if let Some(parent) = self.db_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                return Err(ConfigError::ValidationError(format!(
                    "Database parent directory does not exist: {}",
                    parent.display()
                )));
            }
        }

        Ok(())
    }

    /// Create a default config file at the XDG config directory.
    pub fn create_default_config_file() -> Result<PathBuf, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| {
                ConfigError::ValidationError("Could not determine config directory".into())
            })?
            .join("okc");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let default_config = Self::default();
        let toml_content = toml::to_string_pretty(&default_config)?;
        std::fs::write(&config_path, toml_content)?;

        Ok(config_path)
    }
}
