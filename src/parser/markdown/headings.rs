//! Heading extraction from markdown events.

use crate::model::document::Heading;
use pulldown_cmark::{Event, HeadingLevel};

/// Extract all headings from markdown events.
pub fn extract_headings(events: &[Event]) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut pending_heading = None;
    let mut pending_heading_level = 0;
    let mut heading_counter = 0;

    for event in events {
        match event {
            Event::Start(pulldown_cmark::Tag::Heading { level, .. }) => {
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
            Event::End(pulldown_cmark::TagEnd::Heading(_)) => {
                if let Some(heading_text) = pending_heading.take() {
                    heading_counter += 1;
                    let anchor = Some(slugify(&heading_text));
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
                    pending.push_str(text);
                }
            }
            _ => {}
        }
    }

    headings
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
    use pulldown_cmark::Parser;

    #[test]
    fn test_extract_headings() {
        let markdown = "# H1\n## H2\n### H3";
        let parser = Parser::new(markdown);
        let events: Vec<_> = parser.collect();
        let headings = extract_headings(&events);

        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].title, "H1");
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[1].title, "H2");
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[2].title, "H3");
        assert_eq!(headings[2].level, 3);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Test_123"), "test_123");
        assert_eq!(slugify("Special!@#Chars"), "specialchars");
    }
}
