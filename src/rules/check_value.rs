use crate::error::PrettystrictError;
use crate::lint_rules::LintError;
use crate::rules::check_property::Rule;
use serde::Deserialize;
use std::fs;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ValueList {
    #[serde(flatten)]
    pub properties: HashMap<String, ValueRule>,
    pub shorthands: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Deserialize)]
pub struct KeywordRule {
    allowed: Option<Vec<String>>,
    ignores: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValueRule {
    AllowedValues {
        allowed: Vec<String>,
    },
    UnitRange {
        units: Vec<String>,
        range: Range,
    },
    KeywordGroup {
        keywords: HashMap<String, KeywordRule>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Range {
    min: f32,
    max: f32,
}

pub fn load_known_values(path: &str) -> Result<ValueList, LintError> {
    let json_value = fs::read_to_string(path).map_err(|e| LintError {
        selector: "".into(),
        property: "".into(),
        message: e.to_string(),
        kind: PrettystrictError::IoError(e),
    })?;
    let valuelist: ValueList = serde_json::from_str(&json_value).map_err(|e| LintError {
        selector: "".into(),
        property: "".into(),
        message: e.to_string(),
        kind: PrettystrictError::JsonError(e),
    })?;
    Ok(valuelist)
}

pub fn check_value(rule: &Rule, known_values: &ValueList) -> Vec<LintError> {
    let mut errors = Vec::new();
    let value_map = &known_values.properties;
    let re = Regex::new(r"^([0-9]*\.?[0-9]+)([a-zA-Z%]+)$").unwrap();

    for decl in &rule.declaration {
        let value = &decl.value;
        let property = &decl.name;

        match value_map.get(property) {
            Some(ValueRule::AllowedValues { allowed }) => {
                if !allowed.contains(value) {
                    errors.push(LintError {
                        selector: rule.selector.clone(),
                        property: property.clone(),
                        message: format!("‘{}’ is not an allowed value for {}", value, property),
                        kind: PrettystrictError::UnknownValue(value.clone()),
                    });
                }
            }

            Some(ValueRule::UnitRange { units, range }) => {
                let parsed = re.captures(value).and_then(|caps| {
                    let number = caps.get(1)?.as_str().parse::<f32>().ok()?;
                    let unit = caps.get(2)?.as_str();
                    Some((number, unit))
                });

                match parsed {
                    Some((num, unit))
                        if units.contains(&unit.to_string())
                            && num >= range.min
                            && num <= range.max =>
                    {
                        // valid value
                    }
                    _ => {
                        errors.push(LintError {
                            selector: rule.selector.clone(),
                            property: property.clone(),
                            message: format!(
                                "‘{}’ is not a valid unit/range for {}",
                                value, property
                            ),
                            kind: PrettystrictError::UnknownValue(value.clone()),
                        });
                    }
                }
            }

            Some(ValueRule::KeywordGroup { keywords }) => {
                if property == "position" {
                    if let Some(keyword_rule) = keywords.get(value) {
                        if let Some(_allowed) = &keyword_rule.allowed {
                            // (Optional: validate something if needed
                        }
                    } else {
                        errors.push(LintError {
                            selector: rule.selector.clone(),
                            property: property.clone(),
                            message: format!("Invalid value for position: '{}'", value),
                            kind: PrettystrictError::UnknownValue(value.clone()),
                        });
                    }
                }

                let position_value = rule
                    .declaration
                    .iter()
                    .find(|d| d.name == "position")
                    .map(|d| d.value.as_str());

                if let Some("static") = position_value {
                    if let Some(ValueRule::KeywordGroup { keywords }) =
                        known_values.properties.get("position")
                    {
                        if let Some(static_rule) = keywords.get("static") {
                            if let Some(ignores) = &static_rule.ignores {
                                for decl in &rule.declaration {
                                    if ignores.contains(&decl.name) {
                                        errors.push(LintError {
                                            selector: rule.selector.clone(),
                                            property: property.clone(),
                                            message: format!(
                                                "'{}' is not valid for static.",
                                                decl.name
                                            ),
                                            kind: PrettystrictError::UnknownValue(value.clone()),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            None => {
                errors.push(LintError {
                    selector: rule.selector.clone(),
                    property: property.clone(),
                    message: format!("No known values defined for '{}'", property),
                    kind: PrettystrictError::UnknownValue(value.clone()),
                });
            }
        }
    }

    errors
}
