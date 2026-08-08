//! High-level service API for CLI and MCP server.
//!
//! [`OkcService`] provides the main application interface, delegating to
//! the underlying [`RepositoryIndex`] for storage operations.
//!
//! Modules:
//! - [`browse`] - Directory browsing
//! - [`documents`] - Document retrieval (detail, sections, recent)
//! - [`graph`] - Link graph operations (links, backlinks, traversal)
//! - [`search`] - Full-text and metadata search
//! - [`validation`] - Index validation and statistics
//! - [`watch`] - File system watching integration

mod browse;
mod documents;
mod graph;
mod search;
mod validation;
mod watch;

use crate::config::OkcConfig;
use crate::error::Result;
use crate::index::RepositoryIndex;
use crate::model::document::ScanResult;

/// Main service facade for the Open Knowledge Catalog.
///
/// Wraps the repository index and exposes high-level operations
/// for the CLI and MCP transport layers.
pub struct OkcService {
    pub(crate) index: RepositoryIndex,
}

impl OkcService {
    /// Open a service connected to the configured database.
    pub fn open(config: &OkcConfig) -> Result<Self> {
        config.validate()?;
        let index = RepositoryIndex::open(config)?;
        Ok(Self { index })
    }

    /// Open an in-memory service for testing.
    ///
    /// Uses an in-memory SQLite database. Graph store is not available.
    #[allow(dead_code)]
    pub fn open_in_memory(config: &OkcConfig) -> Result<Self> {
        config.validate()?;
        let index = RepositoryIndex::open_in_memory(config)?;
        Ok(Self { index })
    }

    /// Perform a full repository scan.
    ///
    /// Discovers all markdown files, processes changes, and updates indexes.
    /// Returns scan statistics including counts and duration.
    pub fn scan(&mut self) -> Result<ScanResult> {
        self.index.scan()
    }

    /// Export full repository index as JSON for CLI --json flag and benchmarking.
    ///
    /// See OKC-00022 (Add JSON output mode for non-MCP agent consumption) and
    /// OKC-00027 (Add criterion benchmarks for core operations - export_bundle_json target).
    #[allow(dead_code)]
    pub fn export_to_json(&self) -> Result<serde_json::Value> {
        self.index.export_to_json()
    }
}
