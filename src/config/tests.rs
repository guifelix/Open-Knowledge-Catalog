//! Inline tests for configuration module.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use super::{Bm25Config, ConfigError, OkcConfig};
use std::env;
use std::path::PathBuf;
use tempfile::tempdir;

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
    ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn test_bm25_config_default() {
    let config = Bm25Config::default();
    assert_eq!(config.title_weight, 10.0);
    assert_eq!(config.description_weight, 5.0);
    assert_eq!(config.headings_weight, 2.0);
    assert_eq!(config.body_weight, 1.0);
    assert_eq!(config.concept_type_weight, 0.0);
    assert_eq!(config.k1, 1.2);
    assert_eq!(config.b, 0.75);
}

#[test]
fn test_okc_config_default() {
    let config = OkcConfig::default();
    assert!(config.roots.is_empty());
    assert!(!config.exclude_patterns.is_empty());
    assert_eq!(config.max_file_size, 2 * 1024 * 1024);
    assert_eq!(config.max_front_matter_size, 64 * 1024);
    assert_eq!(config.max_yaml_input_size, 8 * 1024 * 1024);
    assert_eq!(config.max_graph_depth, 5);
    assert_eq!(config.max_graph_nodes, 100);
    assert_eq!(config.max_response_chars, 500_000);
    assert!(!config.follow_symlinks);
    assert!(!config.require_index_files);
    assert_eq!(config.db_path, PathBuf::from("okc_index.db"));
    assert_eq!(config.watcher_debounce_ms, 500);
    assert_eq!(config.watcher_reconcile_secs, 600);
    assert_eq!(config.bm25, Bm25Config::default());
}

#[test]
fn test_load_config_from_file() {
    let _lock = env_lock();
    let dir = tempdir().expect("temp dir");
    let config_path = dir.path().join("okc.toml");
    let config_content = format!(
        r#"
roots = [{{
    path = "{}"
}}]
max_file_size = 1048576
max_graph_depth = 3
"#,
        dir.path().display()
    );
    std::fs::write(&config_path, config_content).expect("write config file");

    let config = OkcConfig::load(Some(&config_path)).expect("load config");
    assert_eq!(config.roots.len(), 1);
    assert_eq!(config.roots[0].path, dir.path().to_path_buf());
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
    let _lock = env_lock();
    let dir = tempdir().expect("temp dir");
    let config_dir = dir.path().join("okc");
    std::fs::create_dir_all(&config_dir).expect("create config dir");

    // Temporarily override config dir
    let original_config_dir = dirs::config_dir();
    unsafe { std::env::set_var("XDG_CONFIG_HOME", dir.path()) };

    let result = OkcConfig::create_default_config_file();
    assert!(result.is_ok());
    let config_path = result.expect("config path from create_default");

    // Verify the file was created and has expected content
    let content = std::fs::read_to_string(&config_path).expect("read config file");
    assert!(content.contains("max_file_size = 2097152"));
    assert!(content.contains("max_front_matter_size = 65536"));
    assert!(content.contains("max_yaml_input_size = 8388608"));

    // Restore
    if let Some(original) = original_config_dir {
        unsafe { std::env::set_var("XDG_CONFIG_HOME", original) };
    } else {
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
    }
}

#[test]
fn test_env_overrides_roots() {
    let _lock = env_lock();
    unsafe {
        env::set_var("OKC_ROOTS", "/path/one,/path/two");
    }
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.roots.len(), 2);
    assert_eq!(config.roots[0].path, PathBuf::from("/path/one"));
    assert_eq!(config.roots[1].path, PathBuf::from("/path/two"));
    unsafe { env::remove_var("OKC_ROOTS") };
}

#[test]
fn test_env_overrides_db_path() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_DB_PATH", "/custom/path.db") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.db_path, PathBuf::from("/custom/path.db"));
    unsafe { env::remove_var("OKC_DB_PATH") };
}

#[test]
fn test_env_overrides_max_file_size() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_FILE_SIZE", "1048576") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.max_file_size, 1048576);
    unsafe { env::remove_var("OKC_MAX_FILE_SIZE") };
}

#[test]
fn test_env_overrides_max_front_matter_size() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_FRONT_MATTER_SIZE", "32768") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.max_front_matter_size, 32768);
    unsafe { env::remove_var("OKC_MAX_FRONT_MATTER_SIZE") };
}

#[test]
fn test_env_overrides_max_yaml_input_size() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_YAML_INPUT_SIZE", "4194304") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.max_yaml_input_size, 4194304);
    unsafe { env::remove_var("OKC_MAX_YAML_INPUT_SIZE") };
}

#[test]
fn test_env_overrides_max_graph_depth() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_GRAPH_DEPTH", "10") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.max_graph_depth, 10);
    unsafe { env::remove_var("OKC_MAX_GRAPH_DEPTH") };
}

#[test]
fn test_env_overrides_max_graph_nodes() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_GRAPH_NODES", "500") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.max_graph_nodes, 500);
    unsafe { env::remove_var("OKC_MAX_GRAPH_NODES") };
}

#[test]
fn test_env_overrides_max_response_chars() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_RESPONSE_CHARS", "250000") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.max_response_chars, 250_000);
    unsafe { env::remove_var("OKC_MAX_RESPONSE_CHARS") };
}

#[test]
fn test_env_overrides_follow_symlinks() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_FOLLOW_SYMLINKS", "true") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert!(config.follow_symlinks);
    unsafe { env::remove_var("OKC_FOLLOW_SYMLINKS") };
}

#[test]
fn test_env_overrides_require_index_files() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_REQUIRE_INDEX_FILES", "true") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert!(config.require_index_files);
    unsafe { env::remove_var("OKC_REQUIRE_INDEX_FILES") };
}

#[test]
fn test_env_overrides_watcher_debounce_ms() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_WATCHER_DEBOUNCE_MS", "1000") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.watcher_debounce_ms, 1000);
    unsafe { env::remove_var("OKC_WATCHER_DEBOUNCE_MS") };
}

#[test]
fn test_env_overrides_watcher_reconcile_secs() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_WATCHER_RECONCILE_SECS", "1200") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.watcher_reconcile_secs, 1200);
    unsafe { env::remove_var("OKC_WATCHER_RECONCILE_SECS") };
}

#[test]
fn test_env_overrides_exclude_patterns() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_EXCLUDE_PATTERNS", "foo/,bar/") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.exclude_patterns, vec!["foo/", "bar/"]);
    unsafe { env::remove_var("OKC_EXCLUDE_PATTERNS") };
}

#[test]
fn test_env_overrides_bm25_title_weight() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_BM25_TITLE_WEIGHT", "15.0") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.bm25.title_weight, 15.0);
    unsafe { env::remove_var("OKC_BM25_TITLE_WEIGHT") };
}

#[test]
fn test_env_overrides_bm25_description_weight() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_BM25_DESCRIPTION_WEIGHT", "8.0") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.bm25.description_weight, 8.0);
    unsafe { env::remove_var("OKC_BM25_DESCRIPTION_WEIGHT") };
}

#[test]
fn test_env_overrides_bm25_headings_weight() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_BM25_HEADINGS_WEIGHT", "3.0") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.bm25.headings_weight, 3.0);
    unsafe { env::remove_var("OKC_BM25_HEADINGS_WEIGHT") };
}

#[test]
fn test_env_overrides_bm25_body_weight() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_BM25_BODY_WEIGHT", "2.0") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.bm25.body_weight, 2.0);
    unsafe { env::remove_var("OKC_BM25_BODY_WEIGHT") };
}

#[test]
fn test_env_overrides_bm25_concept_type_weight() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_BM25_CONCEPT_TYPE_WEIGHT", "1.0") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.bm25.concept_type_weight, 1.0);
    unsafe { env::remove_var("OKC_BM25_CONCEPT_TYPE_WEIGHT") };
}

#[test]
fn test_env_overrides_bm25_k1() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_BM25_K1", "1.5") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.bm25.k1, 1.5);
    unsafe { env::remove_var("OKC_BM25_K1") };
}

#[test]
fn test_env_overrides_bm25_b() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_BM25_B", "0.5") };
    let mut config = OkcConfig::default();
    config.apply_env_overrides().expect("apply env");
    assert_eq!(config.bm25.b, 0.5);
    unsafe { env::remove_var("OKC_BM25_B") };
}

#[test]
fn test_invalid_env_max_file_size() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_FILE_SIZE", "not_a_number") };
    let mut config = OkcConfig::default();
    let result = config.apply_env_overrides();
    assert!(result.is_err());
    unsafe { env::remove_var("OKC_MAX_FILE_SIZE") };
}

#[test]
fn test_invalid_env_max_front_matter_size() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_FRONT_MATTER_SIZE", "not_a_number") };
    let mut config = OkcConfig::default();
    let result = config.apply_env_overrides();
    assert!(result.is_err());
    unsafe { env::remove_var("OKC_MAX_FRONT_MATTER_SIZE") };
}

#[test]
fn test_invalid_env_max_yaml_input_size() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_YAML_INPUT_SIZE", "not_a_number") };
    let mut config = OkcConfig::default();
    let result = config.apply_env_overrides();
    assert!(result.is_err());
    unsafe { env::remove_var("OKC_MAX_YAML_INPUT_SIZE") };
}

#[test]
fn test_invalid_env_max_graph_depth() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_GRAPH_DEPTH", "not_a_number") };
    let mut config = OkcConfig::default();
    let result = config.apply_env_overrides();
    assert!(result.is_err());
    unsafe { env::remove_var("OKC_MAX_GRAPH_DEPTH") };
}

#[test]
fn test_invalid_env_max_graph_nodes() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_GRAPH_NODES", "not_a_number") };
    let mut config = OkcConfig::default();
    let result = config.apply_env_overrides();
    assert!(result.is_err());
    unsafe { env::remove_var("OKC_MAX_GRAPH_NODES") };
}

#[test]
fn test_invalid_env_max_response_chars() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_MAX_RESPONSE_CHARS", "not_a_number") };
    let mut config = OkcConfig::default();
    let result = config.apply_env_overrides();
    assert!(result.is_err());
    unsafe { env::remove_var("OKC_MAX_RESPONSE_CHARS") };
}

#[test]
fn test_invalid_env_follow_symlinks() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_FOLLOW_SYMLINKS", "not_a_bool") };
    let mut config = OkcConfig::default();
    let result = config.apply_env_overrides();
    assert!(result.is_err());
    unsafe { env::remove_var("OKC_FOLLOW_SYMLINKS") };
}

#[test]
fn test_invalid_env_bm25_title_weight() {
    let _lock = env_lock();
    unsafe { env::set_var("OKC_BM25_TITLE_WEIGHT", "not_a_number") };
    let mut config = OkcConfig::default();
    let result = config.apply_env_overrides();
    assert!(result.is_err());
    unsafe { env::remove_var("OKC_BM25_TITLE_WEIGHT") };
}
