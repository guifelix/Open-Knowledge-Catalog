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

/// BM25 search configuration for FTS5 relevance ranking.
///
/// Controls field weights and BM25 algorithm parameters (k1, b).
/// Field weights determine the relative importance of each column in the FTS5 index.
/// Higher weight = more important for relevance scoring.
///
/// Default weights follow the ADR-002 specification:
/// - title: 10.0 (most important)
/// - description: 5.0
/// - headings: 2.0
/// - body: 1.0 (baseline)
/// - concept_type: 0.0 (not used for relevance)
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Bm25Config {
    /// Weight for the title field. Default: 10.0
    pub title_weight: f64,
    /// Weight for the description field. Default: 5.0
    pub description_weight: f64,
    /// Weight for the headings field. Default: 2.0
    pub headings_weight: f64,
    /// Weight for the body field. Default: 1.0
    pub body_weight: f64,
    /// Weight for the concept_type field. Default: 0.0 (ignored for relevance)
    pub concept_type_weight: f64,
    /// BM25 k1 parameter (term frequency saturation). Default: 1.2
    /// Higher values = less saturation, more weight to term frequency.
    pub k1: f64,
    /// BM25 b parameter (length normalization). Default: 0.75
    /// 0.0 = no length normalization, 1.0 = full normalization.
    pub b: f64,
}

impl Default for Bm25Config {
    fn default() -> Self {
        Self {
            title_weight: 10.0,
            description_weight: 5.0,
            headings_weight: 2.0,
            body_weight: 1.0,
            concept_type_weight: 0.0,
            k1: 1.2,
            b: 0.75,
        }
    }
}

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

    /// Maximum depth for graph traversal operations.
    /// Default: 5.
    #[serde(default = "default_max_graph_depth")]
    pub max_graph_depth: usize,

    /// Maximum number of nodes to return in graph traversal.
    /// Default: 100.
    #[serde(default = "default_max_graph_nodes")]
    pub max_graph_nodes: usize,

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

fn default_max_graph_depth() -> usize {
    5
}

fn default_max_graph_nodes() -> usize {
    100
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
            max_graph_depth: default_max_graph_depth(),
            max_graph_nodes: default_max_graph_nodes(),
            follow_symlinks: false,
            require_index_files: false,
            db_path: default_db_path(),
            watcher_debounce_ms: default_watcher_debounce_ms(),
            watcher_reconcile_secs: default_watcher_reconcile_secs(),
            bm25: Bm25Config::default(),
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

        // Validate the final configuration
        config.validate()?;

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
    /// - OKC_MAX_GRAPH_DEPTH
    /// - OKC_MAX_GRAPH_NODES
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

        // Watcher debounce
        if let Ok(val) = std::env::var("OKC_WATCHER_DEBOUNCE_MS") {
            self.watcher_debounce_ms = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_WATCHER_DEBOUNCE_MS: invalid u64: {val}"))
            })?;
        }

        // Watcher reconcile
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

        // BM25 config
        if let Ok(val) = std::env::var("OKC_BM25_TITLE_WEIGHT") {
            self.bm25.title_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_TITLE_WEIGHT: invalid f64: {val}"))
            })?;
        }
        if let Ok(val) = std::env::var("OKC_BM25_DESCRIPTION_WEIGHT") {
            self.bm25.description_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!(
                    "OKC_BM25_DESCRIPTION_WEIGHT: invalid f64: {val}"
                ))
            })?;
        }
        if let Ok(val) = std::env::var("OKC_BM25_HEADINGS_WEIGHT") {
            self.bm25.headings_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_HEADINGS_WEIGHT: invalid f64: {val}"))
            })?;
        }
        if let Ok(val) = std::env::var("OKC_BM25_BODY_WEIGHT") {
            self.bm25.body_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_BODY_WEIGHT: invalid f64: {val}"))
            })?;
        }
        if let Ok(val) = std::env::var("OKC_BM25_CONCEPT_TYPE_WEIGHT") {
            self.bm25.concept_type_weight = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!(
                    "OKC_BM25_CONCEPT_TYPE_WEIGHT: invalid f64: {val}"
                ))
            })?;
        }
        if let Ok(val) = std::env::var("OKC_BM25_K1") {
            self.bm25.k1 = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_K1: invalid f64: {val}"))
            })?;
        }
        if let Ok(val) = std::env::var("OKC_BM25_B") {
            self.bm25.b = val.parse().map_err(|_| {
                ConfigError::EnvParseError(format!("OKC_BM25_B: invalid f64: {val}"))
            })?;
        }

        Ok(())
    }

    /// Validate the configuration and return an error if invalid.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate roots exist if specified
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

        // Validate db_path parent directory exists or can be created
        if let Some(parent) = self.db_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                return Err(ConfigError::ValidationError(format!(
                    "Database parent directory does not exist: {}",
                    parent.display()
                )));
            }
        }

        // Validate numeric limits
        if self.max_file_size == 0 {
            return Err(ConfigError::ValidationError(
                "max_file_size must be > 0".into(),
            ));
        }
        if self.max_front_matter_size == 0 {
            return Err(ConfigError::ValidationError(
                "max_front_matter_size must be > 0".into(),
            ));
        }
        if self.max_graph_depth == 0 {
            return Err(ConfigError::ValidationError(
                "max_graph_depth must be > 0".into(),
            ));
        }
        if self.max_graph_nodes == 0 {
            return Err(ConfigError::ValidationError(
                "max_graph_nodes must be > 0".into(),
            ));
        }
        if self.watcher_debounce_ms == 0 {
            return Err(ConfigError::ValidationError(
                "watcher_debounce_ms must be > 0".into(),
            ));
        }
        if self.watcher_reconcile_secs == 0 {
            return Err(ConfigError::ValidationError(
                "watcher_reconcile_secs must be > 0".into(),
            ));
        }

        // Validate BM25 parameters
        if self.bm25.k1 <= 0.0 {
            return Err(ConfigError::ValidationError("bm25.k1 must be > 0".into()));
        }
        if !(0.0..=1.0).contains(&self.bm25.b) {
            return Err(ConfigError::ValidationError(
                "bm25.b must be in range [0.0, 1.0]".into(),
            ));
        }
        if self.bm25.title_weight < 0.0
            || self.bm25.description_weight < 0.0
            || self.bm25.headings_weight < 0.0
            || self.bm25.body_weight < 0.0
            || self.bm25.concept_type_weight < 0.0
        {
            return Err(ConfigError::ValidationError(
                "BM25 weights must be >= 0".into(),
            ));
        }

        Ok(())
    }

    /// Create a default config file at the XDG config directory.
    /// Returns the path where the file was created.
    pub fn create_default_config_file() -> Result<PathBuf, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| {
                ConfigError::ValidationError("Could not determine config directory".into())
            })?
            .join("okc");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let default_config = Self::default();
        let toml_string = toml::to_string_pretty(&default_config).map_err(|e| {
            ConfigError::ValidationError(format!("Failed to serialize default config: {e}"))
        })?;

        std::fs::write(&config_path, toml_string)?;

        Ok(config_path)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic)]
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use tempfile::tempdir;

    // Mutex to serialize tests that modify environment variables
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_default_config() {
        let config = OkcConfig::default();
        assert_eq!(config.max_file_size, 2 * 1024 * 1024);
        assert_eq!(config.max_front_matter_size, 64 * 1024);
        assert_eq!(config.max_graph_depth, 5);
        assert_eq!(config.max_graph_nodes, 100);
        assert!(!config.follow_symlinks);
        assert!(!config.require_index_files);
        assert_eq!(config.watcher_debounce_ms, 500);
        assert_eq!(config.watcher_reconcile_secs, 600);
    }

    #[test]
    fn test_config_validation_success() {
        let dir = tempdir().expect("temp dir creation");
        let db_path = dir.path().join("test.db");
        std::fs::write(&db_path, "").expect("write test db file");

        let config = OkcConfig {
            roots: vec![dir.path().to_path_buf()],
            db_path: db_path.clone(),
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_missing_root() {
        let config = OkcConfig {
            roots: vec![PathBuf::from("/nonexistent/path")],
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_bm25_b() {
        let config = OkcConfig {
            bm25: Bm25Config {
                b: 1.5,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_env_override_roots() {
        let _lock = ENV_MUTEX.lock().expect("env mutex lock");
        let dir = tempdir().expect("temp dir creation");
        let root_path = dir.path().to_string_lossy().to_string();
        env::set_var("OKC_ROOTS", &root_path);
        let mut config = OkcConfig::default();
        config.apply_env_overrides().expect("env override roots");
        assert_eq!(config.roots.len(), 1);
        assert_eq!(config.roots[0], PathBuf::from(&root_path));
        env::remove_var("OKC_ROOTS");
    }

    #[test]
    fn test_env_override_multiple_roots() {
        let _lock = ENV_MUTEX.lock().expect("env mutex lock");
        let dir1 = tempdir().expect("temp dir 1");
        let dir2 = tempdir().expect("temp dir 2");
        let roots = format!("{},{}", dir1.path().display(), dir2.path().display());
        env::set_var("OKC_ROOTS", &roots);
        let mut config = OkcConfig::default();
        config.apply_env_overrides().expect("apply env override");
        assert_eq!(config.roots.len(), 2);
        env::remove_var("OKC_ROOTS");
    }

    #[test]
    fn test_env_override_db_path() {
        let _lock = ENV_MUTEX.lock().expect("env mutex lock");
        env::set_var("OKC_DB_PATH", "/custom/path/db.sqlite");
        let mut config = OkcConfig::default();
        config.apply_env_overrides().expect("env override db_path");
        assert_eq!(config.db_path, PathBuf::from("/custom/path/db.sqlite"));
        env::remove_var("OKC_DB_PATH");
    }

    #[test]
    fn test_env_override_numeric() {
        let _lock = ENV_MUTEX.lock().expect("env mutex lock");
        env::set_var("OKC_MAX_FILE_SIZE", "5242880"); // 5MB
        env::set_var("OKC_MAX_GRAPH_DEPTH", "10");
        let mut config = OkcConfig::default();
        config.apply_env_overrides().expect("env override numeric");
        assert_eq!(config.max_file_size, 5242880);
        assert_eq!(config.max_graph_depth, 10);
        env::remove_var("OKC_MAX_FILE_SIZE");
        env::remove_var("OKC_MAX_GRAPH_DEPTH");
    }

    #[test]
    fn test_env_override_bool() {
        let _lock = ENV_MUTEX.lock().expect("env mutex lock");
        env::set_var("OKC_FOLLOW_SYMLINKS", "true");
        env::set_var("OKC_REQUIRE_INDEX_FILES", "true");
        let mut config = OkcConfig::default();
        config.apply_env_overrides().expect("env override bool");
        assert!(config.follow_symlinks);
        assert!(config.require_index_files);
        env::remove_var("OKC_FOLLOW_SYMLINKS");
        env::remove_var("OKC_REQUIRE_INDEX_FILES");
    }

    #[test]
    fn test_env_override_bm25() {
        let _lock = ENV_MUTEX.lock().expect("env mutex lock");
        env::set_var("OKC_BM25_TITLE_WEIGHT", "15.0");
        env::set_var("OKC_BM25_K1", "1.5");
        env::set_var("OKC_BM25_B", "0.5");
        let mut config = OkcConfig::default();
        config.apply_env_overrides().expect("env override bm25");
        assert_eq!(config.bm25.title_weight, 15.0);
        assert_eq!(config.bm25.k1, 1.5);
        assert_eq!(config.bm25.b, 0.5);
        env::remove_var("OKC_BM25_TITLE_WEIGHT");
        env::remove_var("OKC_BM25_K1");
        env::remove_var("OKC_BM25_B");
    }

    #[test]
    fn test_load_config_from_file() {
        let _lock = ENV_MUTEX.lock().expect("env mutex lock");
        let dir = tempdir().expect("temp dir");
        let config_path = dir.path().join("okc.toml");
        let config_content = format!(
            r#"
roots = ["{}"]
max_file_size = 1048576
max_graph_depth = 3
"#,
            dir.path().display()
        );
        std::fs::write(&config_path, config_content).expect("write config file");

        let config = OkcConfig::load(Some(&config_path)).expect("load config");
        assert_eq!(config.roots.len(), 1);
        assert_eq!(config.roots[0], dir.path().to_path_buf());
        assert_eq!(config.max_file_size, 1048576);
        assert_eq!(config.max_graph_depth, 3);
    }

    #[test]
    fn test_load_config_file_not_found() {
        let result = OkcConfig::load(Some(&PathBuf::from("/nonexistent/config.toml")));
        assert!(result.is_err());
    }

    #[test]
    fn test_create_default_config_file() {
        let _lock = ENV_MUTEX.lock().expect("env mutex lock");
        let dir = tempdir().expect("temp dir");
        let config_dir = dir.path().join("okc");
        std::fs::create_dir_all(&config_dir).expect("create config dir");

        // Temporarily override config dir
        let original_config_dir = dirs::config_dir();
        unsafe { std::env::set_var("XDG_CONFIG_HOME", dir.path()) };

        let result = OkcConfig::create_default_config_file();
        assert!(result.is_ok());
        let config_path = result.expect("config path from create_default");

        // Verify content can be loaded - need to provide a valid db_path
        let mut config = OkcConfig::load(Some(&config_path)).expect("load created config");
        config.db_path = dir.path().join("test.db");
        assert_eq!(config.max_file_size, 2 * 1024 * 1024);

        // Restore
        if let Some(original) = original_config_dir {
            unsafe { std::env::set_var("XDG_CONFIG_HOME", original) };
        } else {
            unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
        }
    }
}
