//! YAML front-matter parsing using saphyr.
//!
//! [`YamlParser`] converts raw YAML strings into structured [`FrontMatter`].
//!
//! Recognized standard keys:
//! - `type` -> `concept_type`
//! - `title` -> `title`
//! - `description` -> `description`
//! - `tags` -> `tags` (sequence of strings)
//! - `typed_links` -> `typed_links` (versioned extension; see `parse_typed_links`)
//!
//! All other keys are stored in `custom` as a `BTreeMap` for round-trip preservation.
//! A `typed_links` block with an unknown version or malformed shape is preserved
//! verbatim in `custom` and never causes a parse failure.

use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::model::document::{FrontMatter, ParseError, TypedLink};
use saphyr::{LoadableYamlNode, Yaml};

/// Parses YAML front-matter into structured data.
///
/// Uses saphyr for safe YAML loading. Extracts known keys and preserves
/// unknown keys in the `custom` map.
///
/// Returns an error if `raw` exceeds `max_size` to prevent pathological
/// inputs from causing out-of-memory conditions in the underlying library.
pub struct YamlParser;

impl YamlParser {
    /// Parse raw YAML string into FrontMatter.
    ///
    /// `max_size` is the maximum allowed input size in bytes. Inputs larger
    /// than this are rejected before reaching the YAML parser as a
    /// defense-in-depth measure against pathological inputs.
    ///
    /// Returns `Err` if YAML is not a mapping at the top level,
    /// contains aliases, or produces a BadValue. Also returns `Err`
    /// if the input exceeds `max_size`.
    pub fn parse(raw: &str, max_size: usize) -> Result<FrontMatter, ParseError> {
        if raw.len() > max_size {
            return Err(ParseError {
                stage: "yaml".into(),
                message: format!(
                    "YAML input too large: {} bytes (max {})",
                    raw.len(),
                    max_size
                ),
                line: None,
            });
        }

        // saphyr's scanner can spin forever on a directive line that reaches
        // end-of-input without a trailing newline (e.g. `%`, `%foo`, or `\n-\n%`)
        // because its character reader pads EOF and the directive loop keeps
        // consuming that padding. Normalizing the input to end in a newline
        // (when absent) guarantees the directive loop terminates. The original
        // `raw` is preserved for `raw_yaml` below.
        let input: Cow<'_, str> = if raw.ends_with('\n') {
            Cow::Borrowed(raw)
        } else {
            let mut terminated = raw.to_string();
            terminated.push('\n');
            Cow::Owned(terminated)
        };

        let docs = Yaml::load_from_str(&input).map_err(|e| ParseError {
            stage: "yaml".into(),
            message: format!("YAML scan error: {:?}", e),
            line: None,
        })?;

        let yaml = docs.into_iter().next().unwrap_or(Yaml::BadValue);

        let mapping = match yaml {
            Yaml::Mapping(m) => m,
            Yaml::BadValue => {
                return Err(ParseError {
                    stage: "yaml".into(),
                    message: "YAML parsing produced BadValue".into(),
                    line: None,
                })
            }
            Yaml::Alias(_) => {
                return Err(ParseError {
                    stage: "yaml".into(),
                    message: "Unexpected YAML alias at top level".into(),
                    line: None,
                })
            }
            other => {
                return Err(ParseError {
                    stage: "yaml".into(),
                    message: format!("Expected mapping at top level, got {:?}", other),
                    line: None,
                });
            }
        };

        let mut concept_type = None;
        let mut title = None;
        let mut description = None;
        let mut tags = vec![];
        let mut typed_links = Vec::new();
        let mut custom = BTreeMap::new();

        let known_keys = ["type", "title", "description", "tags", "typed_links"];

        for (key, value) in &mapping {
            let key_str = yaml_key_to_string(key);
            let is_known = known_keys.contains(&key_str.as_str());

            match key_str.as_str() {
                "type" => concept_type = Some(yaml_value_to_string(value)),
                "title" => title = Some(yaml_value_to_string(value)),
                "description" => description = Some(yaml_value_to_string(value)),
                "tags" => {
                    if let Yaml::Sequence(arr) = value {
                        tags = arr.iter().map(yaml_value_to_string).collect();
                    }
                }
                "typed_links" => match parse_typed_links(value) {
                    Some(links) => typed_links = links,
                    None => {
                        eprintln!(
                            "okc: typed_links block preserved verbatim (unknown version or malformed shape); \
                             see docs/references/okc-typed-links.md"
                        );
                        custom.insert(key_str.clone(), yaml_to_json_value(value));
                    }
                },
                _ => {
                    let json_val = yaml_to_json_value(value);
                    custom.insert(key_str.clone(), json_val);
                }
            }

            if !is_known {
                let json_val = yaml_to_json_value(value);
                custom.insert(key_str, json_val);
            }
        }

        Ok(FrontMatter {
            concept_type,
            title,
            description,
            tags,
            typed_links,
            custom,
            raw_yaml: raw.to_string(),
        })
    }
}

fn yaml_key_to_string(key: &Yaml) -> String {
    match key {
        Yaml::Value(s) => s.as_str().unwrap_or_default().to_string(),
        Yaml::Representation(s, _, _) => s.to_string(),
        other => format!("{:?}", other),
    }
}

/// Extract a scalar (string/number/bool) value as a string, rejecting
/// sequences, mappings, and other composite values.
fn yaml_scalar_to_string(value: &Yaml) -> Option<String> {
    match value {
        Yaml::Value(s) => Some(s.as_str().unwrap_or_default().to_string()),
        Yaml::Representation(s, _, _) => Some(s.to_string()),
        _ => None,
    }
}

/// Parse a `typed_links` front-matter block into [`TypedLink`] entries.
///
/// The block must be a mapping with `version: 1` and a `links` sequence of
/// `{ target, relation, anchor? }` mappings. Any deviation (unknown version,
/// missing `links`, non-mapping entries, or a missing `target`/`relation`)
/// returns `None`, in which case the caller preserves the whole block
/// verbatim in `custom` so the document still parses.
fn parse_typed_links(value: &Yaml) -> Option<Vec<TypedLink>> {
    let block = match value {
        Yaml::Mapping(m) => m,
        _ => return None,
    };

    let version = block
        .iter()
        .find(|(k, _)| yaml_key_to_string(k) == "version")
        .map(|(_, v)| v)?;
    let version_num = match version {
        Yaml::Value(s) => s.as_integer()?,
        Yaml::Representation(s, _, _) => s.as_ref().parse::<i64>().ok()?,
        _ => return None,
    };
    if version_num != 1 {
        return None;
    }

    let links = block
        .iter()
        .find(|(k, _)| yaml_key_to_string(k) == "links")
        .map(|(_, v)| v)?;
    let items = match links {
        Yaml::Sequence(arr) => arr,
        _ => return None,
    };

    let mut out = Vec::with_capacity(items.len());
    for item in items {
        let entry = match item {
            Yaml::Mapping(m) => m,
            _ => return None,
        };
        let mut target = None;
        let mut relation = None;
        let mut anchor = None;
        for (k, v) in entry {
            match yaml_key_to_string(k).as_str() {
                "target" => target = yaml_scalar_to_string(v),
                "relation" => relation = yaml_scalar_to_string(v),
                "anchor" => anchor = yaml_scalar_to_string(v),
                _ => {}
            }
        }
        out.push(TypedLink {
            target: target?,
            relation: relation?,
            anchor,
        });
    }
    Some(out)
}

fn yaml_value_to_string(value: &Yaml) -> String {
    match value {
        Yaml::Value(s) => s.as_str().unwrap_or_default().to_string(),
        Yaml::Representation(s, _, _) => s.to_string(),
        Yaml::Sequence(arr) => {
            let items: Vec<String> = arr.iter().map(yaml_value_to_string).collect();
            format!("[{}]", items.join(", "))
        }
        Yaml::Mapping(map) => {
            let items: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{}: {}", yaml_key_to_string(k), yaml_value_to_string(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
        other => format!("{:?}", other),
    }
}

fn yaml_to_json_value(value: &Yaml) -> serde_json::Value {
    match value {
        Yaml::Value(s) => {
            if let Some(i) = s.as_integer() {
                serde_json::Value::Number(i.into())
            } else if let Some(f) = s.as_floating_point() {
                serde_json::json!(f)
            } else if let Some(b) = s.as_bool() {
                serde_json::Value::Bool(b)
            } else {
                serde_json::Value::String(s.as_str().unwrap_or_default().to_string())
            }
        }
        Yaml::Representation(s, _, _) => {
            // Try to parse as number/bool
            let str_val = s.as_ref();
            if let Ok(i) = str_val.parse::<i64>() {
                serde_json::Value::Number(i.into())
            } else if let Ok(f) = str_val.parse::<f64>() {
                serde_json::json!(f)
            } else if let Ok(b) = str_val.parse::<bool>() {
                serde_json::Value::Bool(b)
            } else {
                serde_json::Value::String(str_val.to_string())
            }
        }
        Yaml::Sequence(arr) => {
            serde_json::Value::Array(arr.iter().map(yaml_to_json_value).collect())
        }
        Yaml::Mapping(map) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in map {
                let ks = yaml_key_to_string(k);
                json_map.insert(ks, yaml_to_json_value(v));
            }
            serde_json::Value::Object(json_map)
        }
        Yaml::BadValue => serde_json::Value::Null,
        Yaml::Alias(_) => serde_json::Value::Null,
        Yaml::Tagged(_, inner) => yaml_to_json_value(inner),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]
    use super::*;

    /// Inputs that previously caused saphyr to loop forever on `load_from_str`
    /// (a directive line with no trailing newline). Each must now return an
    /// error promptly instead of hanging.
    const HANG_CORPUS: [&str; 4] = ["%", "%foo", "%%", "\n-\n%"];

    #[test]
    fn directive_without_trailing_newline_terminates() {
        for input in HANG_CORPUS {
            let result = YamlParser::parse(input, 8 * 1024 * 1024);
            // Expect an error (BadValue, scan error, or top-level type), never a hang.
            assert!(
                result.is_err(),
                "expected Err for hang-prone input {:?}, got {:?}",
                input,
                result
            );
        }
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn valid_documents_parse_unchanged_after_guard() {
        let cases: [(&str, Option<&str>); 4] = [
            ("title: Greeting", Some("Greeting")),
            ("title: T\ndescription: D\n", Some("T")),
            ("tags:\n  - a\n  - b", None),
            ("type: concept\ncustom_key: value", None),
        ];
        for (input, expected_title) in cases {
            let mut subject = input.to_string();
            if !subject.ends_with('\n') {
                subject.push('\n');
            }
            let result = YamlParser::parse(&subject, 8 * 1024 * 1024)
                .expect("valid YAML mapping should parse");
            assert_eq!(
                result.title.as_deref(),
                expected_title,
                "for input {input:?}"
            );
        }
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn typed_links_version_one_populates_fields() {
        let input = "\
typed_links:
  version: 1
  links:
    - target: /data/metrics.md
      relation: depends-on
      anchor: section-a
    - target: /concepts/base.md
      relation: references
";
        let fm = YamlParser::parse(input, 8 * 1024 * 1024)
            .expect("well-formed v1 typed_links must parse");
        assert_eq!(fm.typed_links.len(), 2);
        assert_eq!(fm.typed_links[0].target, "/data/metrics.md");
        assert_eq!(fm.typed_links[0].relation, "depends-on");
        assert_eq!(fm.typed_links[0].anchor.as_deref(), Some("section-a"));
        assert_eq!(fm.typed_links[1].relation, "references");
        assert_eq!(fm.typed_links[1].anchor, None);
        assert!(
            !fm.custom.contains_key("typed_links"),
            "consumed block must not be preserved in custom"
        );
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn typed_links_unknown_version_preserved_in_custom() {
        let input = "\
typed_links:
  version: 2
  links:
    - target: /x.md
      relation: future-rel
";
        let fm = YamlParser::parse(input, 8 * 1024 * 1024)
            .expect("unknown typed_links version must not fail the document");
        assert!(fm.typed_links.is_empty());
        assert!(
            fm.custom.contains_key("typed_links"),
            "unknown version block must be preserved verbatim"
        );
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn typed_links_missing_version_preserved_in_custom() {
        let input = "\
typed_links:
  links:
    - target: /x.md
      relation: references
";
        let fm = YamlParser::parse(input, 8 * 1024 * 1024)
            .expect("typed_links without version must not fail the document");
        assert!(fm.typed_links.is_empty());
        assert!(fm.custom.contains_key("typed_links"));
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn typed_links_missing_target_preserved_in_custom() {
        let input = "\
typed_links:
  version: 1
  links:
    - relation: depends-on
";
        let fm = YamlParser::parse(input, 8 * 1024 * 1024)
            .expect("missing target must be tolerated, not fatal");
        assert!(fm.typed_links.is_empty());
        assert!(fm.custom.contains_key("typed_links"));
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn typed_links_not_a_mapping_preserved_in_custom() {
        let input = "typed_links: oops-not-a-block\n";
        let fm = YamlParser::parse(input, 8 * 1024 * 1024)
            .expect("non-mapping typed_links must be tolerated");
        assert!(fm.typed_links.is_empty());
        assert!(fm.custom.contains_key("typed_links"));
    }
}
