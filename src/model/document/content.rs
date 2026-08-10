//! Document content structure types for the Open Knowledge Catalog.
//!
//! This module contains types representing the parsed structure of
//! markdown documents including headings, tables, code blocks, links,
//! and logical sections.

use serde::{Deserialize, Serialize};

/// A heading extracted from markdown content.
#[derive(Debug, Clone)]
pub struct Heading {
    /// Heading level (1-6).
    pub level: u32,
    /// Heading text.
    pub title: String,
    /// Optional anchor/slug for linking.
    pub anchor: Option<String>,
    /// Byte position in the document.
    pub position: usize,
}

/// A table extracted from markdown content.
#[derive(Debug, Clone)]
pub struct Table {
    /// Table headers.
    pub headers: Vec<String>,
    /// Table rows (each row is a vector of cell contents).
    pub rows: Vec<Vec<String>>,
    /// Column alignments (None, Left, Center, Right).
    pub alignments: Vec<TableAlignment>,
    /// Byte position in the document.
    pub position: usize,
}

/// Table column alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableAlignment {
    /// No explicit alignment.
    None,
    /// Left-aligned.
    Left,
    /// Center-aligned.
    Center,
    /// Right-aligned.
    Right,
}

/// A fenced code block extracted from markdown content.
#[derive(Debug, Clone)]
pub struct CodeBlock {
    /// Programming language (from fence info string).
    pub language: Option<String>,
    /// Optional filename (from fence info string, e.g., `rust:filename.rs`).
    pub filename: Option<String>,
    /// Code content.
    pub content: String,
    /// Byte position in the document.
    pub position: usize,
}

/// A link extracted from markdown content.
#[derive(Debug, Clone)]
pub struct Link {
    /// Original link text as written in markdown.
    pub raw: String,
    /// Resolved target path or URL.
    pub target: String,
    /// Optional anchor fragment.
    pub target_anchor: Option<String>,
    /// Whether this is an external link (http/https/mailto).
    pub is_external: bool,
    /// Whether the target exists in the repository (for internal links).
    pub exists_in_repository: bool,
    /// Optional typed relationship from the `typed_links` front-matter extension.
    pub relation: Option<String>,
}

/// Fully parsed document with all extracted structure.
#[derive(Debug, Clone)]
pub struct ParsedDocument {
    /// Document path.
    pub path: String,
    /// Parsed front-matter if present.
    pub front_matter: Option<crate::model::document::frontmatter::FrontMatter>,
    /// All headings in document order.
    pub headings: Vec<Heading>,
    /// All links in document order.
    pub links: Vec<Link>,
    /// Plain text body content.
    pub body_text: String,
    /// Logical sections (heading + content).
    pub sections: Vec<Section>,
    /// Overall parse status.
    pub parse_status: crate::model::document::frontmatter::ParseStatus,
    /// Any parse errors encountered.
    pub parse_errors: Vec<crate::model::document::frontmatter::ParseError>,
}

/// A logical section of a document (heading + content).
#[derive(Debug, Clone)]
pub struct Section {
    /// Section heading text.
    pub heading: String,
    /// Heading level.
    pub level: u32,
    /// Section content (markdown).
    pub content: String,
    /// Byte position of section start.
    pub start_position: usize,
    /// Tables found in this section.
    pub tables: Vec<Table>,
    /// Code blocks found in this section.
    pub code_blocks: Vec<CodeBlock>,
}

/// Heading information for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingInfo {
    /// Heading level (1-6).
    pub level: u32,
    /// Heading text.
    pub title: String,
    /// Optional anchor.
    pub anchor: Option<String>,
}

/// Link information for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    /// Target path (for internal links).
    pub target_path: Option<String>,
    /// Target anchor fragment.
    pub target_anchor: Option<String>,
    /// External URL (for external links).
    pub external_url: Option<String>,
    /// Whether target exists in repository.
    pub exists_in_repository: bool,
    /// Optional typed relationship from the `typed_links` front-matter extension.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
}
