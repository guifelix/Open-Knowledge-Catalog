//! Core data models for the Open Knowledge Catalog.
//!
//! This module defines the fundamental data structures used throughout the crate:
//!
//! - [`directory`] - Directory browsing types (`DirectoryNode`, `DirectoryDocument`, `BrowseResponse`)
//! - [`document`] - Document types (`FileRecord`, `FrontMatter`, `DocumentSummary`, `DocumentDetail`,
//!   `SearchResult`, `LinkInfo`, `ValidationIssue`, `ValidationReport`, `IndexStats`, `ScanResult`)
//! - [`graph`] - Graph traversal types (`GraphEdge`, `TraverseNode`, `TraverseResponse`)

#[allow(unused_imports)]
pub mod directory;
#[allow(unused_imports)]
pub mod document;
#[allow(unused_imports)]
pub mod graph;

#[allow(unused_imports)]
pub use directory::{BrowseResponse, DirectoryDocument, DirectoryNode};
#[allow(unused_imports)]
pub use document::{
    CheckResult, CheckStatus, DocumentDetail, DocumentMetadata, DocumentSummary, HeadingInfo,
    IndexStats, Link, LinkInfo, MetadataQueryResponse, ParseError, ParseStatus,
    ProcessChangesResult, ScanResult, SearchResponse, SearchResult, ValidationIssue,
    ValidationReport, ValidationSummary,
};
#[allow(unused_imports)]
pub use graph::{GraphEdge, TraverseNode, TraverseResponse};
