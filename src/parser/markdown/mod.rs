//! Markdown parsing sub-modules.

pub mod code_blocks;
pub mod headings;
pub mod links;
pub mod sections;
pub mod tables;

use crate::model::document::{CodeBlock, Heading, Link, Section, Table, TableAlignment};
use pulldown_cmark::{Event, Parser};

/// Parses markdown into structured components.
///
/// Uses pulldown-cmark for compliant CommonMark parsing.
/// Extracts headings, links, body text, and logical sections.
pub struct MarkdownParser;

impl MarkdownParser {
    /// Parse markdown body into headings, links, plain text, sections, tables, and code blocks.
    ///
    /// Returns a tuple of:
    /// - `Vec<Heading>` - All headings with level, title, anchor, position
    /// - `Vec<Link>` - All links (wiki-style and standard) with raw text
    /// - `String` - Plain text body for search indexing
    /// - `Vec<Section>` - Logical sections (heading + content) for granular search
    /// - `Vec<Table>` - All tables with headers, rows, and alignments
    /// - `Vec<CodeBlock>` - All fenced code blocks with language and optional filename
    pub fn parse(
        body: &str,
    ) -> (
        Vec<Heading>,
        Vec<Link>,
        String,
        Vec<Section>,
        Vec<Table>,
        Vec<CodeBlock>,
    ) {
        let parser = Parser::new_ext(body, pulldown_cmark::Options::ENABLE_TABLES);
        let events: Vec<_> = parser.collect();

        let headings = headings::extract_headings(&events);
        let links = links::extract_links(&events, body);
        let tables = tables::extract_tables(&events);
        let code_blocks = code_blocks::extract_code_blocks(&events);
        let sections = sections::build_sections(&events, body);

        // Extract plain text for search
        let searchable_text = events
            .iter()
            .filter_map(|e| match e {
                Event::Text(t) | Event::Code(t) => Some(t.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");

        (
            headings,
            links,
            searchable_text,
            sections,
            tables,
            code_blocks,
        )
    }
}
