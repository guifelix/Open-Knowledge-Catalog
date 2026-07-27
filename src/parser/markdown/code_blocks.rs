//! Code block extraction from markdown events.

use crate::model::document::CodeBlock;
use pulldown_cmark::{Event, Tag, TagEnd};

/// Extract fenced code blocks from markdown events.
pub fn extract_code_blocks(events: &[Event]) -> Vec<CodeBlock> {
    let mut code_blocks = Vec::new();
    let mut in_code_block = false;
    let mut current_language = None;
    let mut current_filename = None;
    let mut current_content = String::new();

    for (position, event) in events.iter().enumerate() {
        match event {
            Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(info))) => {
                in_code_block = true;
                let info_str = info.to_string();
                // Parse language and optional filename (e.g., "rust:src/main.rs")
                if let Some(colon_idx) = info_str.find(':') {
                    current_language = Some(info_str[..colon_idx].to_string());
                    current_filename = Some(info_str[colon_idx + 1..].to_string());
                } else if !info_str.is_empty() {
                    current_language = Some(info_str);
                }
                current_content.clear();
            }
            Event::Text(text) if in_code_block => {
                current_content.push_str(text);
            }
            Event::End(TagEnd::CodeBlock) if in_code_block => {
                code_blocks.push(CodeBlock {
                    language: current_language.take(),
                    filename: current_filename.take(),
                    content: current_content.trim_end().to_string(),
                    position,
                });
                in_code_block = false;
            }
            _ => {}
        }
    }

    code_blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::Parser;

    #[test]
    fn test_extract_code_block_basic() {
        let markdown = "```rust\nfn main() {}\n```";
        let parser = Parser::new(markdown);
        let events: Vec<_> = parser.collect();
        let blocks = extract_code_blocks(&events);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].language, Some("rust".to_string()));
        assert!(blocks[0].content.contains("fn main()"));
    }

    #[test]
    fn test_extract_code_block_with_filename() {
        let markdown = "```rust:src/main.rs\nfn main() {}\n```";
        let parser = Parser::new(markdown);
        let events: Vec<_> = parser.collect();
        let blocks = extract_code_blocks(&events);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].language, Some("rust".to_string()));
        assert_eq!(blocks[0].filename, Some("src/main.rs".to_string()));
    }

    #[test]
    fn test_extract_code_block_no_language() {
        let markdown = "```\nplain text\n```";
        let parser = Parser::new(markdown);
        let events: Vec<_> = parser.collect();
        let blocks = extract_code_blocks(&events);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].language, None);
        assert_eq!(blocks[0].content.trim(), "plain text");
    }
}
