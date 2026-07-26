//! Markdown parsing using pulldown-cmark.
//!
//! [`MarkdownParser`] extracts structured data from markdown content:
//! - Headings with levels and auto-generated anchors
//! - Links (wiki-style `[[...]]` and standard `[text](url)`)
//! - Tables with headers, rows, and alignments
//! - Fenced code blocks with language and optional filename
//! - Plain text body for full-text search
//! - Logical sections (heading + content) for granular search results

use crate::model::document::{CodeBlock, Heading, Link, Section, Table, TableAlignment};
use pulldown_cmark::{Alignment, Event, HeadingLevel, Tag, TagEnd};

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
        let parser = pulldown_cmark::Parser::new_ext(body, pulldown_cmark::Options::ENABLE_TABLES);

        let mut headings = Vec::new();
        let mut links = Vec::new();
        let mut searchable_text = String::new();
        let mut sections = Vec::new();

        let mut current_section_heading = String::from("(top)");
        let mut current_section_level = 0u32;
        let mut current_section_content = String::new();
        let mut current_section_tables = Vec::new();
        let mut current_section_code_blocks = Vec::new();
        let mut in_code_block = false;
        let mut heading_counter = 0u32;

        let mut pending_heading = None;
        let mut pending_heading_level = 0u32;

        // Table parsing state
        let mut in_table = false;
        let mut table_headers = Vec::new();
        let mut table_rows = Vec::new();
        let mut table_alignments = Vec::new();
        let mut current_row = Vec::new();
        let mut in_table_head = false;
        let mut cell_content = String::new();
        let mut _cell_index = 0;
        let mut header_row_done = false;
        let mut after_table_head = false;

        // Code block parsing state
        let mut code_block_language = None;
        let mut code_block_filename = None;
        let mut code_block_content = String::new();

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
                                tables: std::mem::take(&mut current_section_tables),
                                code_blocks: std::mem::take(&mut current_section_code_blocks),
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
                    } else if in_table {
                        cell_content.push_str(&text);
                    } else if !in_code_block {
                        searchable_text.push_str(&text);
                        searchable_text.push(' ');
                        current_section_content.push_str(&text);
                        current_section_content.push(' ');
                    } else {
                        // Inside a fenced code block
                        code_block_content.push_str(&text);
                    }
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    in_code_block = true;
                    // Extract language and optional filename from fence info string
                    if let pulldown_cmark::CodeBlockKind::Fenced(info) = kind {
                        let info_str = info.to_string();
                        if info_str.contains(':') {
                            let parts: Vec<&str> = info_str.splitn(2, ':').collect();
                            code_block_language = Some(parts[0].to_string());
                            code_block_filename = Some(parts[1].to_string());
                        } else if !info_str.is_empty() {
                            code_block_language = Some(info_str);
                        }
                    }
                    code_block_content = String::new();
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    // Save the code block to current section
                    current_section_code_blocks.push(CodeBlock {
                        language: code_block_language.take(),
                        filename: code_block_filename.take(),
                        content: code_block_content.trim_end().to_string(),
                        position: 0,
                    });
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
                Event::Start(Tag::Table(alignments)) => {
                    in_table = true;
                    table_headers = Vec::new();
                    table_rows = Vec::new();
                    table_alignments = alignments
                        .iter()
                        .map(|a| match a {
                            Alignment::None => TableAlignment::None,
                            Alignment::Left => TableAlignment::Left,
                            Alignment::Center => TableAlignment::Center,
                            Alignment::Right => TableAlignment::Right,
                        })
                        .collect();
                    in_table_head = true;
                    header_row_done = false;
                    after_table_head = false;
                    _cell_index = 0;
                }
                Event::End(TagEnd::Table) => {
                    in_table = false;
                    // Save the table to current section
                    if !table_headers.is_empty() {
                        current_section_tables.push(Table {
                            headers: table_headers.clone(),
                            rows: table_rows.clone(),
                            alignments: table_alignments.clone(),
                            position: 0,
                        });
                    }
                }
                Event::Start(Tag::TableHead) => {
                    in_table_head = true;
                    _cell_index = 0;
                    // Reset current_row for header cells collected directly in TableHead
                    current_row = Vec::new();
                }
                Event::End(TagEnd::TableHead) => {
                    in_table_head = false;
                    after_table_head = true;
                    // Save header cells collected directly in TableHead (no TableRow event for them)
                    if !current_row.is_empty() && !header_row_done {
                        table_headers = current_row.clone();
                        header_row_done = true;
                    }
                    current_row = Vec::new();
                }
                Event::Start(Tag::TableRow) => {
                    current_row = Vec::new();
                    _cell_index = 0;
                }
                Event::End(TagEnd::TableRow) => {
                    if in_table_head || (after_table_head && !header_row_done) {
                        table_headers = current_row.clone();
                        header_row_done = true;
                        after_table_head = false;
                    } else {
                        table_rows.push(current_row.clone());
                    }
                }
                Event::Start(Tag::TableCell) => {
                    cell_content = String::new();
                }
                Event::End(TagEnd::TableCell) => {
                    current_row.push(cell_content.trim().to_string());
                    _cell_index += 1;
                }
                Event::SoftBreak | Event::HardBreak => {
                    if in_table {
                        cell_content.push(' ');
                    } else if !in_code_block {
                        searchable_text.push(' ');
                        current_section_content.push(' ');
                    } else {
                        code_block_content.push('\n');
                    }
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
                tables: current_section_tables,
                code_blocks: current_section_code_blocks,
            });
        }

        (
            headings,
            links,
            searchable_text.trim().to_string(),
            sections,
            Vec::new(),
            Vec::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::document::{CodeBlock, Table, TableAlignment};

    #[test]
    fn test_table_extraction_basic() {
        let markdown = r#"
# Header

| Col1 | Col2 | Col3 |
|------|------|------|
| A    | B    | C    |
| D    | E    | F    |

Some text after table.
"#;
        let (headings, _links, _text, sections, _tables, _code_blocks) =
            MarkdownParser::parse(markdown);

        assert_eq!(headings.len(), 1);
        assert_eq!(headings[0].title, "Header");

        // Find the section with the table
        let section = sections.iter().find(|s| s.heading == "Header").unwrap();
        assert_eq!(section.tables.len(), 1);

        let table = &section.tables[0];
        assert_eq!(table.headers, vec!["Col1", "Col2", "Col3"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["A", "B", "C"]);
        assert_eq!(table.rows[1], vec!["D", "E", "F"]);
        // Default alignment is None
        assert_eq!(
            table.alignments,
            vec![
                TableAlignment::None,
                TableAlignment::None,
                TableAlignment::None
            ]
        );
    }

    #[test]
    fn test_table_extraction_with_alignment() {
        let markdown = r#"
# Header

| Left | Center | Right |
|:-----|:------:|------:|
| A    | B      | C     |
"#;
        let (_headings, _links, _text, sections, _tables, _code_blocks) =
            MarkdownParser::parse(markdown);

        let section = sections.iter().find(|s| s.heading == "Header").unwrap();
        assert_eq!(section.tables.len(), 1);

        let table = &section.tables[0];
        assert_eq!(table.headers, vec!["Left", "Center", "Right"]);
        assert_eq!(
            table.alignments,
            vec![
                TableAlignment::Left,
                TableAlignment::Center,
                TableAlignment::Right,
            ]
        );
    }

    #[test]
    fn test_code_block_extraction_basic() {
        let markdown = r#"
# Header

```rust
fn main() {
    println!("Hello");
}
```

Some text after.
"#;
        let (_headings, _links, _text, sections, _tables, _code_blocks) =
            MarkdownParser::parse(markdown);

        let section = sections.iter().find(|s| s.heading == "Header").unwrap();
        assert_eq!(section.code_blocks.len(), 1);

        let code_block = &section.code_blocks[0];
        assert_eq!(code_block.language, Some("rust".to_string()));
        assert!(code_block.content.contains("fn main()"));
        assert!(code_block.content.contains("println!"));
    }

    #[test]
    fn test_code_block_extraction_with_filename() {
        let markdown = r#"
# Header

```rust:src/main.rs
fn main() {
    println!("Hello");
}
```
"#;
        let (_headings, _links, _text, sections, _tables, _code_blocks) =
            MarkdownParser::parse(markdown);

        let section = sections.iter().find(|s| s.heading == "Header").unwrap();
        assert_eq!(section.code_blocks.len(), 1);

        let code_block = &section.code_blocks[0];
        assert_eq!(code_block.language, Some("rust".to_string()));
        assert_eq!(code_block.filename, Some("src/main.rs".to_string()));
    }

    #[test]
    fn test_code_block_no_language() {
        let markdown = r#"
# Header

```
plain text code block
```
"#;
        let (_headings, _links, _text, sections, _tables, _code_blocks) =
            MarkdownParser::parse(markdown);

        let section = sections.iter().find(|s| s.heading == "Header").unwrap();
        assert_eq!(section.code_blocks.len(), 1);

        let code_block = &section.code_blocks[0];
        assert_eq!(code_block.language, None);
        assert_eq!(code_block.content.trim(), "plain text code block");
    }

    #[test]
    fn test_multiple_tables_and_code_blocks() {
        let markdown = r#"
# Section 1

| A | B |
|---|---|
| 1 | 2 |

```python
print("hello")
```

# Section 2

| X | Y |
|---|---|
| 3 | 4 |

```js
console.log("world");
```
"#;
        let (_headings, _links, _text, sections, _tables, _code_blocks) =
            MarkdownParser::parse(markdown);

        assert_eq!(sections.len(), 2);

        let sec1 = &sections[0];
        assert_eq!(sec1.heading, "Section 1");
        assert_eq!(sec1.tables.len(), 1);
        assert_eq!(sec1.code_blocks.len(), 1);
        assert_eq!(sec1.code_blocks[0].language, Some("python".to_string()));

        let sec2 = &sections[1];
        assert_eq!(sec2.heading, "Section 2");
        assert_eq!(sec2.tables.len(), 1);
        assert_eq!(sec2.code_blocks.len(), 1);
        assert_eq!(sec2.code_blocks[0].language, Some("js".to_string()));
    }

    #[test]
    fn test_nested_headings_with_tables() {
        let markdown = r#"
# H1

## H2

| Col1 | Col2 |
|------|------|
| A    | B    |

### H3

```rust
let x = 1;
```
"#;
        let (headings, _links, _text, sections, _tables, _code_blocks) =
            MarkdownParser::parse(markdown);

        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[2].level, 3);

        // H2 section should have the table
        let h2_section = sections.iter().find(|s| s.heading == "H2").unwrap();
        assert_eq!(h2_section.tables.len(), 1);

        // H3 section should have the code block
        let h3_section = sections.iter().find(|s| s.heading == "H3").unwrap();
        assert_eq!(h3_section.code_blocks.len(), 1);
    }
}
