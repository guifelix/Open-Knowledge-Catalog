//! Fuzz target for query_metadata SQL injection prevention

#![no_main]
use libfuzzer_sys::fuzz_target;
use open_knowledge_catalog::index::RepositoryIndex;
use open_knowledge_catalog::config::OkcConfig;
use std::collections::HashMap;
use tempfile::TempDir;

fuzz_target!(|data: &[u8]| {
    // Create a temporary database for testing
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let mut config = OkcConfig::default();
    config.db_path = db_path;
    config.roots = vec![temp_dir.path().to_path_buf()];
    
    let mut index = match RepositoryIndex::open(&config) {
        Ok(idx) => idx,
        Err(_) => return,
    };
    
    // Try to parse the input as a filter key
    if let Ok(key) = std::str::from_utf8(data) {
        // Skip empty keys
        if key.is_empty() {
            return;
        }
        
        // Try to use the key as a filter - this should not cause SQL injection
        let mut filters = HashMap::new();
        filters.insert(key.to_string(), "test_value".to_string());
        
        // This should not panic or cause SQL injection
        let _ = index.query_metadata(&filters, &["path".to_string()], 10);
    }
});