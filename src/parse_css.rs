use crate::lint_rules::LintError;
use crate::rules::check_property::{Property, Rule};

pub fn parse_css() -> Result<Vec<Rule>, LintError> {
    // Simple hardcoded CSS for testing - in a real implementation,
    // this would read from a file or take CSS as input
    let css_rules = vec![
        Rule {
            selector: ".foo".to_string(),
            declaration: vec![
                Property {
                    name: "color".to_string(),
                    value: "red".to_string(),
                },
                Property {
                    name: "font-size".to_string(),
                    value: "16px".to_string(),
                },
            ],
            at_rule: vec!["@media (max-width: 600px)".to_string()],
        },
        Rule {
            selector: ".bar".to_string(),
            declaration: vec![Property {
                name: "background".to_string(),
                value: "blue".to_string(),
            }],
            at_rule: vec![],
        },
    ];

    Ok(css_rules)
}

fn parse_declaration(decl: &str) -> Option<Property> {
    let parts: Vec<&str> = decl.splitn(2, ':').collect();
    if parts.len() == 2 {
        Some(Property {
            name: parts[0].trim().to_string(),
            value: parts[1].trim().trim_end_matches(';').to_string(),
        })
    } else {
        None
    }
}

fn parse_selector(selector: &str) -> String {
    selector.trim().to_string()
}

fn extract_at_rules(css: &str) -> Vec<String> {
    let mut at_rules = Vec::new();
    for line in css.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('@') {
            at_rules.push(trimmed.to_string());
        }
    }
    at_rules
}

// Future function to parse actual CSS text
#[allow(dead_code)]
fn parse_css_text(css: &str) -> Result<Vec<Rule>, LintError> {
    let mut rules = Vec::new();
    let mut current_rule: Option<Rule> = None;
    let mut in_rule = false;
    let mut brace_count = 0;

    for line in css.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }
        brace_count += trimmed.chars().filter(|&c| c == '{').count() as i32;
        brace_count -= trimmed.chars().filter(|&c| c == '}').count() as i32;

        if trimmed.contains('{') && !in_rule {
            let selector = trimmed.replace('{', "").trim().to_string();
            current_rule = Some(Rule {
                selector: parse_selector(&selector),
                declaration: Vec::new(),
                at_rule: extract_at_rules(css),
            });
            in_rule = true;
        } else if trimmed.contains('}') && brace_count == 0 {
            if let Some(rule) = current_rule.take() {
                rules.push(rule);
            }
            in_rule = false;
        } else if in_rule && trimmed.contains(':') {
            if let Some(ref mut rule) = current_rule {
                if let Some(prop) = parse_declaration(trimmed) {
                    rule.declaration.push(prop);
                }
            }
        }
    }

    Ok(rules)
}
