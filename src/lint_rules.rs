use crate::error;
use crate::error::PrettystrictError;
use crate::rules::check_property::{PropertyList, Rule, check_at_rule, check_props};
use crate::rules::check_value::{ValueList, check_value};
use crate::rules::duplicate_declaration::{
    Location, check_order, duplicate_declaration, shorthand_detection,
};
use crate::rules::unit_check::unit_check;

#[derive(Debug)]
pub struct LintError {
    pub selector: String,
    pub property: String,
    pub message: String,
    pub kind: PrettystrictError,
}

pub fn lint_rules(
    rule: &Rule,
    known_props: &PropertyList,
    known_values: &ValueList,
    location: &Location,
) -> Vec<LintError> {
    fn convert(rule_errors: Vec<LintError>) -> Vec<LintError> {
        rule_errors
            .into_iter()
            .map(|re| LintError {
                selector: re.selector,
                property: re.property,
                message: format!("{}", re.message),
                kind: error::PrettystrictError::from(re.kind),
            })
            .collect()
    }

    let mut errors = Vec::new();

    let rule_checks: Vec<Vec<LintError>> = vec![
        check_props(rule, known_props),
        check_value(rule, known_values),
        duplicate_declaration(rule, location),
        unit_check(rule, known_values),
        shorthand_detection(rule, known_values),
        check_order(rule),
        check_at_rule(rule, known_props),
    ];

    for rule_errors in rule_checks {
        errors.extend(convert(rule_errors));
    }

    errors
}
