use std::collections::BTreeMap;

use crate::model::{FrontMatter, ParseError};
use saphyr::{LoadableYamlNode, Yaml};

pub struct YamlParser;

impl YamlParser {
    pub fn parse(raw: &str) -> Result<FrontMatter, ParseError> {
        let docs = Yaml::load_from_str(raw).map_err(|e| ParseError {
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
        let mut custom = BTreeMap::new();

        let known_keys = ["type", "title", "description", "tags"];

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
