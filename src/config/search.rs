//! Search result configuration.
//!
//! Controls heading extraction and display in search results.

use serde::{Deserialize, Serialize};

/// Search result configuration.
///
/// Controls heading extraction and display in search results.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SearchConfig {
    /// Maximum number of headings to include per search result.
    /// Default: 1
    pub max_headings: usize,

    /// Maximum heading depth to include (1 = h1 only, 2 = h1+h2, etc.).
    /// Default: 1
    pub heading_depth: u32,
}

fn default_max_headings() -> usize {
    1
}

fn default_heading_depth() -> u32 {
    1
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_headings: default_max_headings(),
            heading_depth: default_heading_depth(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = SearchConfig::default();
        assert_eq!(config.max_headings, 1);
        assert_eq!(config.heading_depth, 1);
    }

    #[test]
    fn test_deserialize() {
        let toml = r#"
            max_headings = 3
            heading_depth = 2
        "#;
        let config: SearchConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.max_headings, 3);
        assert_eq!(config.heading_depth, 2);
    }
}
