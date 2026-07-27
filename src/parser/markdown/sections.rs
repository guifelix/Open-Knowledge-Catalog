//! Section building from markdown events.

use crate::model::document::{CodeBlock, Heading, Link, Section, Table, TableAlignment};
use pulldown_cmark::{Event, Tag, TagEnd};

/// Build logical sections from markdown events.
///
/// A section is a heading + its content (text, tables, code blocks).
pub fn build_sections(events: &[Event], _source_text: &str) -> Vec<Section> {
    let mut sections = Vec::new();
    let mut current_section_heading = String::from("(top)");
    let mut current_section_level = 0u32;
    let mut current_section_content = String::new();
    let mut current_section_tables = Vec::new();
    let mut current_section_code_blocks = Vec::new();
    let mut in_code_block = false;
    let mut in_heading = false;
    let mut heading_text = String::new();
    let mut position = 0;

    for event in events {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let level_num = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };

                // Save previous section if it has content
                if !current_section_heading.is_empty() && current_section_heading != "(top)" {
                    sections.push(Section {
                        heading: current_section_heading.clone(),
                        level: current_section_level,
                        content: current_section_content.trim().to_string(),
                        start_position: position,
                        tables: std::mem::take(&mut current_section_tables),
                        code_blocks: std::mem::take(&mut current_section_code_blocks),
                    });
                }

                // Start new section
                current_section_level = level_num;
                current_section_content.clear();
                in_heading = true;
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                // Heading text was collected in Text events
                current_section_heading = heading_text.clone();
                in_heading = false;
            }
            Event::Text(text) => {
                if in_heading {
                    heading_text.push_str(text);
                } else if !in_code_block {
                    current_section_content.push_str(text);
                    current_section_content.push(' ');
                }
            }
            Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(_))) => {
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
            }
            _ => {}
        }
        position += 1;
    }

    // Save final section
    if !current_section_heading.is_empty() && current_section_heading != "(top)" {
        sections.push(Section {
            heading: current_section_heading,
            level: current_section_level,
            content: current_section_content.trim().to_string(),
            start_position: position,
            tables: current_section_tables,
            code_blocks: current_section_code_blocks,
        });
    }

    sections
}

// Re-export table and code block extraction for section building
pub use crate::parser::markdown::code_blocks::extract_code_blocks;
pub use crate::parser::markdown::tables::extract_tables;

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::{Options, Parser};

    #[test]
    fn test_build_sections_basic() {
        let markdown = "# H1\n\nContent 1\n\n## H2\n\nContent 2";
        let parser = Parser::new_ext(markdown, Options::ENABLE_TABLES);
        let events: Vec<_> = parser.collect();
        let sections = build_sections(&events, markdown);

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].heading, "H1");
        assert_eq!(sections[0].level, 1);
        assert!(sections[0].content.contains("Content 1"));
        assert_eq!(sections[1].heading, "H2");
        assert_eq!(sections[1].level, 2);
        assert!(sections[1].content.contains("Content 2"));
    }
}
