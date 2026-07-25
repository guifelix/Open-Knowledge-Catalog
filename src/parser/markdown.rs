//! Markdown parsing using pulldown-cmark.
//!
//! [`MarkdownParser`] extracts structured data from markdown content:
//! - Headings with levels and auto-generated anchors
//! - Links (wiki-style `[[...]]` and standard `[text](url)`)
//! - Plain text body for full-text search
//! - Logical sections (heading + content) for granular search results

use crate::model::document::{Heading, Link, Section};
use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};

/// Parses markdown into structured components.
///
/// Uses pulldown-cmark for compliant CommonMark parsing.
/// Extracts headings, links, body text, and logical sections.
pub struct MarkdownParser;

impl MarkdownParser {
    /// Parse markdown body into headings, links, plain text, and sections.
    ///
    /// Returns a tuple of:
    /// - `Vec<Heading>` - All headings with level, title, anchor, position
    /// - `Vec<Link>` - All links (wiki-style and standard) with raw text
    /// - `String` - Plain text body for search indexing
    /// - `Vec<Section>` - Logical sections (heading + content) for granular search
    pub fn parse(body: &str) -> (Vec<Heading>, Vec<Link>, String, Vec<Section>) {
        let parser = pulldown_cmark::Parser::new(body);

        let mut headings = Vec::new();
        let mut links = Vec::new();
        let mut searchable_text = String::new();
        let mut sections = Vec::new();

        let mut current_section_heading = String::from("(top)");
        let mut current_section_level = 0u32;
        let mut current_section_content = String::new();
        let mut in_code_block = false;
        let mut heading_counter = 0u32;

        let mut pending_heading = None;
        let mut pending_heading_level = 0u32;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    let level_num = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    pending_heading_level = level_num;
                    pending_heading = Some(String::new());
                }
                Event::End(TagEnd::Heading(_)) => {
                    if let Some(heading_text) = pending_heading.take() {
                        heading_counter += 1;
                        let anchor = Some(slugify(&heading_text));

                        if !current_section_heading.is_empty() && current_section_heading != "(top)"
                        {
                            sections.push(Section {
                                heading: current_section_heading.clone(),
                                level: current_section_level,
                                content: current_section_content.trim().to_string(),
                                start_position: 0,
                            });
                        }

                        current_section_heading = heading_text.clone();
                        current_section_level = pending_heading_level;
                        current_section_content = String::new();

                        headings.push(Heading {
                            level: pending_heading_level,
                            title: heading_text,
                            anchor,
                            position: heading_counter as usize,
                        });
                    }
                }
                Event::Text(text) | Event::Code(text) => {
                    if let Some(ref mut pending) = pending_heading {
                        if !pending.is_empty() {
                            pending.push(' ');
                        }
                        pending.push_str(&text);
                    } else if !in_code_block {
                        searchable_text.push_str(&text);
                        searchable_text.push(' ');
                        current_section_content.push_str(&text);
                        current_section_content.push(' ');
                    }
                }
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                }
                Event::Start(Tag::Link { dest_url, .. }) => {
                    let target = dest_url.to_string();
                    let is_external = target.starts_with("http://")
                        || target.starts_with("https://")
                        || target.starts_with("mailto:");
                    links.push(Link {
                        raw: target.clone(),
                        target,
                        target_anchor: None,
                        is_external,
                        exists_in_repository: false,
                    });
                }
                Event::SoftBreak | Event::HardBreak if !in_code_block => {
                    searchable_text.push(' ');
                    current_section_content.push(' ');
                }
                _ => {}
            }
        }

        if !current_section_heading.is_empty() && current_section_heading != "(top)" {
            sections.push(Section {
                heading: current_section_heading,
                level: current_section_level,
                content: current_section_content.trim().to_string(),
                start_position: 0,
            });
        }

        (
            headings,
            links,
            searchable_text.trim().to_string(),
            sections,
        )
    }
}

/// Generate a URL-friendly slug from heading text.
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}
