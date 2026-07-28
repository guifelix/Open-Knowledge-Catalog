//! Document model for the Open Knowledge Catalog.
//!
//! This module provides the core data structures for representing
//! documents, their metadata, search results, validation, and statistics.

pub mod content;
pub mod frontmatter;
pub mod records;
pub mod search;
pub mod stats;
pub mod validation;

// Re-export commonly used types
pub use content::{
    CodeBlock, Heading, HeadingInfo, Link, LinkInfo, ParsedDocument, Section, Table, TableAlignment,
};
pub use frontmatter::{FrontMatter, LimitError, ParseError, ParseStatus};
pub use records::{DocumentDetail, DocumentMetadata, DocumentSummary, FileRecord};
pub use search::{derive_display_title, MetadataQueryResponse, SearchResponse, SearchResult};
pub use stats::{IndexStats, LimitError as StatsLimitError, ProcessChangesResult, ScanResult};
pub use validation::{
    CheckResult, CheckStatus, ValidationIssue, ValidationReport, ValidationSummary,
};
