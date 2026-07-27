//! Link extraction from markdown events.

use crate::model::document::Link;
use crate::parser::link_utils::extract_wiki_links;
use pulldown_cmark::{Event, Tag, TagEnd};

/// Extract links from markdown events.
pub fn extract_links(events: &[Event], source_text: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let mut in_link = false;
    let mut link_text = String::new();
    let mut link_url = String::new();

    for event in events {
        match event {
            Event::Start(Tag::Link { dest_url, .. }) => {
                in_link = true;
                link_url = dest_url.to_string();
                link_text.clear();
            }
            Event::Text(text) if in_link => {
                link_text.push_str(text);
            }
            Event::End(TagEnd::Link) if in_link => {
                let is_external = link_url.starts_with("http://")
                    || link_url.starts_with("https://")
                    || link_url.starts_with("mailto:");
                links.push(Link {
                    raw: if link_text.is_empty() {
                        link_url.clone()
                    } else {
                        format!("[{}]({})", link_text, link_url)
                    },
                    target: link_url.clone(),
                    target_anchor: extract_anchor(&link_url),
                    is_external,
                    exists_in_repository: false, // Will be resolved later
                });
                in_link = false;
            }
            _ => {}
        }
    }

    // Also extract wiki-style links from source text
    let wiki_links = extract_wiki_links(source_text);
    for wiki_target in wiki_links {
        let is_external = wiki_target.starts_with("http://")
            || wiki_target.starts_with("https://")
            || wiki_target.starts_with("mailto:");
        links.push(Link {
            raw: format!("[[{}]]", wiki_target),
            target: wiki_target.clone(),
            target_anchor: extract_anchor(&wiki_target),
            is_external,
            exists_in_repository: false,
        });
    }

    links
}

fn extract_anchor(url: &str) -> Option<String> {
    url.find('#').map(|idx| url[idx + 1..].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::Parser;

    #[test]
    fn test_extract_links_basic() {
        let markdown = "[Link](target.md) and [External](https://example.com)";
        let parser = Parser::new(markdown);
        let events: Vec<_> = parser.collect();
        let links = extract_links(&events, markdown);

        assert_eq!(links.len(), 2);
        assert_eq!(links[0].target, "target.md");
        assert!(!links[0].is_external);
        assert_eq!(links[1].target, "https://example.com");
        assert!(links[1].is_external);
    }

    #[test]
    fn test_extract_wiki_links() {
        let markdown = "See [[metrics/revenue]] for details.";
        let parser = Parser::new(markdown);
        let events: Vec<_> = parser.collect();
        let links = extract_links(&events, markdown);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "metrics/revenue");
        assert!(!links[0].is_external);
    }
}
