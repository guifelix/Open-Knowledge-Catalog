//! Front-matter extraction from markdown documents.
//!
//! [`FrontMatterExtractor`] locates and extracts YAML front-matter blocks
//! delimited by `---` markers at the start of markdown files.
//!
//! Handles:
//! - UTF-8 BOM detection and stripping
//! - Opening and closing `---` delimiters
//! - Size limits to prevent DoS from oversized front-matter
//! - Returns byte offset of closing delimiter for body extraction

use memchr::memchr;

use crate::model::document::{LimitError, ParseError};

const DELIMITER: &[u8] = b"---";

/// Extracts YAML front-matter from markdown document bytes.
///
/// The extractor finds the opening `---` delimiter, locates the closing `---`,
/// and returns the raw YAML content between them along with the byte offset
/// of the closing delimiter (for extracting the markdown body).
#[derive(Clone, Debug)]
pub struct FrontMatterExtractor {
    max_size: usize,
}

impl FrontMatterExtractor {
    /// Create a new extractor with a maximum front-matter size limit.
    ///
    /// Front-matter exceeding this size will cause a limit error.
    pub fn new(max_size: usize) -> Self {
        Self { max_size }
    }

    /// Extract front-matter from document bytes.
    ///
    /// Returns `Ok(Some((end_offset, yaml_content)))` if front-matter is found,
    /// `Ok(None)` if no front-matter present, or `Err` if malformed.
    pub fn extract(&self, input: &[u8]) -> Result<Option<(usize, String)>, ParseError> {
        let bom_len = if input.starts_with(b"\xef\xbb\xbf") {
            3
        } else {
            0
        };
        let start = bom_len;

        if input[start..].len() < 6 {
            return Ok(None);
        }

        if &input[start..start + 3] != DELIMITER {
            return Ok(None);
        }

        let search_start = start + 3;
        let mut pos = search_start;

        loop {
            if pos >= input.len() {
                return Err(ParseError {
                    stage: "frontmatter".into(),
                    message: "Missing closing front-matter delimiter".into(),
                    line: None,
                });
            }

            let remaining = &input[pos..];

            let newline_end = if let Some(nl) = memchr(b'\n', remaining) {
                if nl > 0 && remaining[nl - 1] == b'\r' {
                    nl - 1
                } else {
                    nl
                }
            } else {
                remaining.len()
            };

            if newline_end >= 3 && &remaining[..3] == DELIMITER {
                let closing_end = pos + newline_end + 1;
                let yaml_start = search_start;
                let yaml_len = pos - search_start;

                if yaml_len > self.max_size {
                    return Err(ParseError {
                        stage: "frontmatter".into(),
                        message: LimitError::new(
                            "max_front_matter_size",
                            &self.max_size.to_string(),
                            &format!(
                                "Front matter exceeds {} bytes (got {})",
                                self.max_size, yaml_len
                            ),
                        )
                        .message,
                        line: None,
                    });
                }

                let yaml_str = String::from_utf8(input[yaml_start..yaml_start + yaml_len].to_vec())
                    .map_err(|e| ParseError {
                        stage: "frontmatter".into(),
                        message: format!("Invalid UTF-8 in front matter: {}", e),
                        line: None,
                    })?;
                // Trim leading newline if present
                let yaml_str = yaml_str.trim_start_matches('\n').to_string();

                return Ok(Some((closing_end, yaml_str)));
            }

            if let Some(nl) = memchr(b'\n', remaining) {
                pos += nl + 1;
            } else {
                return Err(ParseError {
                    stage: "frontmatter".into(),
                    message: "Missing closing front-matter delimiter".into(),
                    line: None,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic)]
    use super::*;

    #[test]
    fn test_basic_frontmatter() {
        let input = b"---\ntitle: Hello\ntags: [a, b]\n---\n\nBody text";
        let extractor = FrontMatterExtractor::new(4096);
        let result = extractor
            .extract(input)
            .expect("extract succeeded")
            .expect("frontmatter present");
        assert_eq!(result.1, "title: Hello\ntags: [a, b]\n");
        assert!(result.0 > 0);
    }

    #[test]
    fn test_no_frontmatter() {
        let input = b"Just body text\n\nNo front matter here.";
        let extractor = FrontMatterExtractor::new(4096);
        assert!(extractor
            .extract(input)
            .expect("extract succeeded")
            .is_none());
    }

    #[test]
    fn test_missing_closing() {
        let input = b"---\ntitle: Broken\n\nNo closing.";
        let extractor = FrontMatterExtractor::new(4096);
        assert!(extractor.extract(input).is_err());
    }

    #[test]
    fn test_bom_handling() {
        let input = b"\xef\xbb\xbf---\ntitle: BOM\n---\nBody";
        let extractor = FrontMatterExtractor::new(4096);
        let result = extractor
            .extract(input)
            .expect("extract BOM")
            .expect("frontmatter with BOM");
        assert!(result.1.contains("title: BOM"));
    }

    #[test]
    fn test_windows_line_endings() {
        let input = b"---\r\ntitle: CRLF\r\n---\r\nBody";
        let extractor = FrontMatterExtractor::new(4096);
        let result = extractor
            .extract(input)
            .expect("extract CRLF")
            .expect("frontmatter with CRLF");
        assert!(result.1.contains("title: CRLF"));
    }

    #[test]
    fn test_exceeds_max_size() {
        let input = format!("---\n{}\n---\nBody", "a".repeat(100));
        let extractor = FrontMatterExtractor::new(10);
        assert!(extractor.extract(input.as_bytes()).is_err());
    }
}
