//! Parsing pipeline for markdown documents with front-matter.
//!
//! This module provides the parsing components:
//!
//! - [`frontmatter`] - [`FrontMatterExtractor`]: Extracts YAML front-matter from markdown files
//! - [`links`] - [`LinkResolver`]: Resolves and validates wiki-style and external links
//! - [`markdown`] - [`MarkdownParser`]: Parses markdown into headings, links, and sections
//! - [`yaml`] - [`YamlParser`]: Parses YAML front-matter into structured data

pub mod frontmatter;
pub mod links;
pub mod markdown;
pub mod yaml;
