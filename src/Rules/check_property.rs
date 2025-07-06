
use std::error::Error;
use serde::{Deserialize, Serialize};

use std::fs;
use std::io::{Read};
use crate::error::PrettystrictError;

#[derive(Serialize, Deserialize)]
pub struct Property {
    pub(crate) name: String,
    pub(crate) value: String,
}
#[derive(Serialize, Deserialize)]
pub struct Rule {
    pub(crate) selector: String,
    pub(crate) declaration: Vec<Property>,
}
#[derive(Serialize, Deserialize)]
pub struct PropertyList{
    properties: Vec<String>
}
pub fn load_known_props(path: &str) -> Result<PropertyList, Box<dyn Error>> {
    let json_content = fs::read_to_string(path)?;
    let props: PropertyList = serde_json::from_str(&json_content)?;
    Ok(props)
}

pub fn check_props(rule: &Rule, known_props: &PropertyList) -> Vec<PrettystrictError> {
    let mut errors = Vec::new();

    for declaration in &rule.declaration {
        if !known_props.properties.contains(&declaration.name) {
            errors.push(PrettystrictError::UnknownProperty(declaration.name.clone()));
        }
    }
    errors
}

