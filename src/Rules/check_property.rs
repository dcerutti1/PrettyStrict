use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read};
use crate::error::{PrettystrictError};
use crate::lint_rules::LintError;

#[derive(Serialize, Deserialize)]

pub struct Property {
    pub(crate) name: String,
    pub(crate) value: String,
}
#[derive(Serialize, Deserialize)]
pub struct Rule {
    pub selector: String,
    pub declaration: Vec<Property>,
    pub AtRule: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct PropertyList {
    pub properties: Vec<String>,
    #[serde(rename = "at-rules")]
    pub at_rules: Vec<String>,
}
pub fn load_known_props(path: &str) -> Result<PropertyList, LintError> {
    
    let json_content = fs::read_to_string(path).map_err(|e| LintError{
        selector: "".into(),
        property: "".into(),
        message: e.to_string(),
        kind: PrettystrictError::IoError(e),
    })?;
    let props: PropertyList = serde_json::from_str(&json_content).map_err(|e| LintError{
        selector: "".into(),
        property: "".into(),
        message: e.to_string(),
        kind: PrettystrictError::JsonError(e),
    })?;
    Ok(props)
}

pub fn check_props(rule: &Rule, known_props: &PropertyList) -> Vec<LintError> {
    let mut errors = Vec::new();

    for declaration in &rule.declaration {
        if !known_props.properties.contains(&declaration.name) {
            errors.push(LintError{
                selector: rule.selector.clone(),
                property: declaration.name.clone(),
                message: format!("{} is unknown", declaration.name),
                kind: PrettystrictError::UnknownProperty(declaration.name.clone()),
            });
        }
    }
    
    errors
}

pub fn check_at_rule(rule: &Rule, known_props: &PropertyList) -> Vec<LintError> {
    let mut errors = Vec::new();

    for at_rule in &rule.AtRule {
        let at_rule_with_at = if at_rule.starts_with('@') {
            at_rule.clone()
        } else {
            format!("@{}", at_rule)
        };

        if !known_props.at_rules.contains(&at_rule_with_at) {
            errors.push(LintError {
                selector: "".to_string(),
                property: at_rule_with_at.clone(),
                message: format!("Unknown at-rule: {}", at_rule_with_at),
                kind: PrettystrictError::UnknownProperty(at_rule_with_at),
            });
        }
    }

    errors
}





