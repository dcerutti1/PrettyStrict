
use crate::error::PrettystrictError;
use crate::lint_rules::LintError;
use crate::Rules::check_property::Rule;
use regex::Regex;
use lazy_static::lazy_static;
use crate::Rules::check_value::{ValueList, ValueRule};

pub fn unit_check(rule: &Rule, known_values: &ValueList) -> Vec<LintError> {
    lazy_static! {
        static ref UNIT_RE: Regex = Regex::new(r"(?i)^-?\d*\.?\d+\s*([a-z%]+)$").unwrap();
    }

    let mut errors = Vec::new();

    for decl in &rule.declaration {
        let prop = decl.name.as_str();
        let value = decl.value.trim();

       
        let unit_opt = UNIT_RE
            .captures(value)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str());

        // If no unit detected, probably a keyword like "auto"
        if unit_opt.is_none() {
            continue;
        }

        let unit = unit_opt.unwrap();

        
        match known_values.properties.get(prop) {
            Some(ValueRule::UnitRange { units, .. }) => {
                if !units.contains(&unit.to_string()) {
                    errors.push(LintError {
                        selector: rule.selector.clone(),
                        property: decl.name.clone(),
                        message: format!("Unit '{}' is not allowed for '{}'", unit, prop),
                        kind: PrettystrictError::WrongUnitDeclared,
                    });
                }
            }
            Some(ValueRule::AllowedValues { .. }) => {
                // Allowed value properties (e.g. display, text-align) shouldn't have units
                errors.push(LintError {
                    selector: rule.selector.clone(),
                    property: decl.name.clone(),
                    message: format!("Unexpected unit '{}' for keyword-only property '{}'", unit, prop),
                    kind: PrettystrictError::WrongUnitDeclared,
                });
            }
            None => {
                errors.push(LintError {
                    selector: rule.selector.clone(),
                    property: decl.name.clone(),
                    message: format!("Unknown property '{}' — no unit validation rule found", prop),
                    kind: PrettystrictError::UnknownProperty(prop.to_string()),
                });
            },
            Some(&ValueRule::KeywordGroup { .. }) => {
                eprintln!("invalid input")
            }
        }
    }

    errors
}
