//! Directory browsing service.
//!
//! Provides hierarchical directory listings with optional index document
//! summaries and configurable depth/limits.

use crate::error::Result;
use crate::model::directory::BrowseResponse;
use crate::service::OkcService;

impl OkcService {
    /// Browse a directory in the knowledge base.
    ///
    /// Returns subdirectories and documents at the given path, with optional
    /// index document summary. Depth controls recursion into subdirectories.
    pub fn browse(&self, path: &str, depth: usize, limit: usize) -> Result<BrowseResponse> {
        self.index.browse_directory(path, depth, limit)
    }
}
