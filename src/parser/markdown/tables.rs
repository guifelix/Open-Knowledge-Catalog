//! Table extraction from markdown events.

use crate::model::document::{Table, TableAlignment};
use pulldown_cmark::{Alignment, Event, Tag, TagEnd};

/// Extract tables from markdown events.
pub fn extract_tables(events: &[Event]) -> Vec<Table> {
    let mut tables = Vec::new();
    let mut in_table = false;
    let mut in_header = false;
    let mut in_row = false;
    let mut current_headers = Vec::new();
    let mut current_rows = Vec::new();
    let mut current_alignments = Vec::new();
    let mut current_cell = String::new();
    let mut current_row = Vec::new();

    for (position, event) in events.iter().enumerate() {
        match event {
            Event::Start(Tag::Table(alignments)) => {
                in_table = true;
                current_alignments = alignments
                    .iter()
                    .map(|a| match a {
                        Alignment::None => TableAlignment::None,
                        Alignment::Left => TableAlignment::Left,
                        Alignment::Center => TableAlignment::Center,
                        Alignment::Right => TableAlignment::Right,
                    })
                    .collect();
            }
            Event::Start(Tag::TableHead) => {
                in_header = true;
            }
            Event::Start(Tag::TableRow) => {
                in_row = true;
                current_row.clear();
            }
            Event::Start(Tag::TableCell) => {
                current_cell.clear();
            }
            Event::Text(text) if in_table && (in_row || in_header) => {
                current_cell.push_str(text);
            }
            Event::End(TagEnd::TableCell) => {
                if in_table && (in_row || in_header) {
                    current_row.push(current_cell.trim().to_string());
                }
            }
            Event::End(TagEnd::TableRow) => {
                if in_table && in_row {
                    if in_header {
                        current_headers = current_row.clone();
                    } else {
                        current_rows.push(current_row.clone());
                    }
                    in_row = false;
                }
            }
            Event::End(TagEnd::TableHead) => {
                // Header row might be collected directly in TableHead without TableRow
                if in_table && in_header && !current_row.is_empty() {
                    current_headers = current_row.clone();
                    current_row.clear();
                }
                in_header = false;
            }
            Event::End(TagEnd::Table) if in_table => {
                tables.push(Table {
                    headers: current_headers.clone(),
                    rows: current_rows.clone(),
                    alignments: current_alignments.clone(),
                    position,
                });
                in_table = false;
                current_headers.clear();
                current_rows.clear();
                current_alignments.clear();
            }
            _ => {}
        }
    }

    tables
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::{Options, Parser};

    #[test]
    fn test_extract_table_basic() {
        let markdown = "| A | B |\n|---|---|\n| 1 | 2 |";
        let parser = Parser::new_ext(markdown, Options::ENABLE_TABLES);
        let events: Vec<_> = parser.collect();
        let tables = extract_tables(&events);

        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].headers, vec!["A", "B"]);
        assert_eq!(tables[0].rows, vec![vec!["1", "2"]]);
    }

    #[test]
    fn test_extract_table_with_alignments() {
        let markdown = "| A | B |\n|:--|--:|\n| 1 | 2 |";
        let parser = Parser::new_ext(markdown, Options::ENABLE_TABLES);
        let events: Vec<_> = parser.collect();
        let tables = extract_tables(&events);

        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].alignments[0], TableAlignment::Left);
        assert_eq!(tables[0].alignments[1], TableAlignment::Right);
    }
}
