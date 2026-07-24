mod browse;
mod documents;
mod graph;
mod search;
mod validation;
mod watch;

use crate::config::OkcConfig;
use crate::index::RepositoryIndex;
use crate::model::*;

pub struct OkcService {
    pub(crate) index: RepositoryIndex,
}

impl OkcService {
    pub fn open(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        let index = RepositoryIndex::open(config)?;
        Ok(Self { index })
    }

    /// In-memory storage backend for testing and future trait-based storage abstraction.
    /// See OKC-00014 (Extract trait interfaces for storage, search, and graph layers).
    #[allow(dead_code)]
    pub fn open_in_memory(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        let index = RepositoryIndex::open_in_memory(config)?;
        Ok(Self { index })
    }

    pub fn scan(&mut self) -> Result<ScanResult, anyhow::Error> {
        self.index.scan()
    }

    /// Export full repository index as JSON for CLI --json flag and benchmarking.
    /// See OKC-00022 (Add JSON output mode for non-MCP agent consumption) and
    /// OKC-00027 (Add criterion benchmarks for core operations - export_bundle_json target).
    #[allow(dead_code)]
    pub fn export_to_json(&self) -> Result<serde_json::Value, anyhow::Error> {
        self.index.export_to_json()
    }
}
