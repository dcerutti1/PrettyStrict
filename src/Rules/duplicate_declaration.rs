
use std::collections::{HashMap, HashSet};
use crate::error::{PrettystrictError};
use crate::lint_rules::LintError;
use crate::Rules::check_value::ValueList;
use super::check_property::Rule;

pub struct Location {
    pub(crate) line: usize,
    pub(crate) column: usize,
}
pub fn duplicate_declaration(rule: &Rule, location: &Location) -> Vec<LintError> {
    let mut errors = Vec::new();
    let mut seen:HashSet<&str> = HashSet::new();
    for declaration in &rule.declaration {
        if !seen.insert(declaration.name.as_str()) {
            errors.push(LintError {
                selector: rule.selector.clone(),
                property: declaration.name.clone(),
                message: "<- duplicate property found.".to_string(),
                kind: PrettystrictError::DuplicateProperty,
            });
        };
    }
    errors
}

pub fn shorthand_detection(rule: &Rule, known_values: &ValueList) -> Vec<LintError> {
    let mut errors = Vec::new();
    let mut seen_props = HashSet::new();

    let shorthand_map = match &known_values.shorthands {
        Some(map) => map,
        None => return errors,
    };

    for decl in &rule.declaration {
        let prop = decl.name.as_str();

        for (shorthand, longhands) in shorthand_map {
            if longhands.contains(&prop.to_string()) && seen_props.contains(shorthand) {
                errors.push(LintError {
                    selector: rule.selector.clone(),
                    property: decl.name.clone(),
                    message: format!("‘{}’ overrides previously defined shorthand ‘{}’", prop, shorthand),
                    kind: PrettystrictError::ProperyOverride,
                });
            }

            if shorthand == prop {
                for longhand in longhands {
                    if seen_props.contains(longhand) {
                        errors.push(LintError {
                            selector: rule.selector.clone(),
                            property: decl.name.clone(),
                            message: format!("‘{}’ overrides previously defined longhand ‘{}’", shorthand, longhand),
                            kind: PrettystrictError::ProperyOverride,
                        });
                    }
                }
            }
        }

        seen_props.insert(prop.to_string());
    }

    errors
}
pub fn check_order(rule: &Rule) -> Vec<LintError> {
    let mut errors = Vec::new();

    let prefered_order = vec![
        "display",
        "position",
        "top",
        "right", "bottom",  "left",
        "z-index",
        "color",
        "background",
    ];
    
    let order_map: HashMap<&str, usize> = prefered_order
        .iter()
        .enumerate()
        .map(|(i, &prop)| (prop, i))
        .collect();

    let mut last_index = 0usize;
    let mut first = true;

    for prop in &rule.declaration {
        if let Some(&current_index) = order_map.get(&prop.name.as_str()) {
            if !first && current_index < last_index {
                errors.push(LintError {
                    selector: rule.selector.clone(),
                    property: prop.name.clone(),
                    message: "invalid property order found.".to_string(),
                    kind: PrettystrictError::ProperyOverride,
                });
            }
            last_index = current_index;
            first = false;
        }
    }
    errors
}
