use std::fmt;
use lightningcss::error::{ParserError, PrinterErrorKind};
use lightningcss::printer::{Printer, PrinterOptions};
use lightningcss::traits::ToCss;
use lightningcss::properties::Property as LightningProperty;
use lightningcss::rules::CssRule;
use lightningcss::rules::keyframes::{KeyframesName, KeyframesRule};
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use lightningcss::targets::Targets;
use crate::error::PrettystrictError;
use crate::lint_rules::LintError;
use crate::rules::check_property::{Property, Rule};




// === MAIN ENTRY POINT ===
pub fn parse_css(css_content: &str) -> Result<Vec<Rule>, LintError> {
    let stylesheet = StyleSheet::parse(css_content, ParserOptions::default()).map_err(|e| LintError {
        selector: "".to_string(),
        property: "".to_string(),
        message: format!("Failed to parse CSS: {}", e),
        kind: PrettystrictError::Custom("parse_css".to_string()),
    })?;

    let mut rules = Vec::new();
    let mut current_at_rules = Vec::new();

    for rule in &stylesheet.rules.0 {
        traverse_rule(rule, &mut rules, &mut current_at_rules);
    }

    Ok(rules)
}

// === TRAVERSE RULES ===
fn traverse_rule(rule: &CssRule, rules: &mut Vec<Rule>, current_at_rules: &mut Vec<String>) {
    match rule {
        CssRule::Style(style_rule) => {
            let selector = to_css_string(&style_rule.selectors);
            let mut declarations = Vec::new();

            for property in &style_rule.declarations.declarations {
                if let Some(prop) = extract_property(property) {
                    declarations.push(prop);
                }
            }

            rules.push(Rule {
                selector,
                declaration: declarations,
                at_rule: current_at_rules.clone(),
            });
        }

        CssRule::Media(media_rule) => {
            let query = to_css_string_or_error(&media_rule.query);
            let media_query = format!("@media {}", query);
            current_at_rules.push(media_query);

            for nested_rule in &media_rule.rules.0 {
                traverse_rule(nested_rule, rules, current_at_rules);
            }

            current_at_rules.pop();
        }

        CssRule::Supports(supports_rule) => {
            let condition = to_css_string(&supports_rule.condition);
            current_at_rules.push(format!("@supports {}", condition));

            for nested_rule in &supports_rule.rules.0 {
                traverse_rule(nested_rule, rules, current_at_rules);
            }

            current_at_rules.pop();
        }

        CssRule::Keyframes(keyframes_rule) => {
            let mut buffer = String::new();
            let options = PrinterOptions {
                minify: false,
                targets: Targets::default(),
                analyze_dependencies: None,
                pseudo_classes: None,
                source_map: None,
                project_root: None,
            };
            let mut printer = Printer::new(&mut buffer, options);

            // Print the whole keyframes_rule CSS into buffer
            keyframes_rule.to_css(&mut printer).expect("Failed to convert keyframes_rule to CSS");

            current_at_rules.push(buffer);

            for keyframe in &keyframes_rule.keyframes {
                let selector = to_css_string(&keyframe.selectors);

                let mut declarations = Vec::new();
                for property in &keyframe.declarations.declarations {
                    if let Some(prop) = extract_property(property) {
                        declarations.push(prop);
                    }
                }

                rules.push(Rule {
                    selector,
                    declaration: declarations,
                    at_rule: current_at_rules.clone(),
                });
            }

            current_at_rules.pop();
        }




            CssRule::FontFace(font_face_rule) => {
            let mut declarations = Vec::new();
            for property in &font_face_rule.properties {
                if let Some(prop) = extract_font_face_property(property) {
                    declarations.push(prop);
                }
            }

            rules.push(Rule {
                selector: "".to_string(),
                declaration: declarations,
                at_rule: vec!["@font-face".to_string()],
            });
        }

        _ => { /* Skip other rules */ }
    }
}

// === FORMATTERS ===
fn to_css_string<T: ToCss>(value: &T) -> String {
    let mut buffer = String::new();
    let options = PrinterOptions {
        minify: false,
        targets: Targets::default(),
        analyze_dependencies: None,
        pseudo_classes: None,
        source_map: None,
        project_root: None,
    };
    let mut printer = Printer::new(&mut buffer, options);
    if let Err(e) = value.to_css(&mut printer) {
        eprintln!("⚠️ CSS printing failed: {:?}", e);
    }
    buffer
}

fn to_css_string_or_error<T: ToCss>(value: &T) -> String {
    let mut buffer = String::new();
    let options = PrinterOptions {
        minify: false,
        targets: Targets::default(),
        analyze_dependencies: None,
        pseudo_classes: None,
        source_map: None,
        project_root: None,
    };
    let mut printer = Printer::new(&mut buffer, options);
    value.to_css(&mut printer).map_err(|e| match e.kind {
        PrinterErrorKind::AmbiguousUrlInCustomProperty { .. } |
        PrinterErrorKind::FmtError |
        PrinterErrorKind::InvalidComposesNesting |
        PrinterErrorKind::InvalidComposesSelector |
        PrinterErrorKind::InvalidCssModulesPatternInGrid => {
            PrettystrictError::Custom(e.to_string())
        }
    }).expect("Failed to serialize CSS");
    buffer
}

// === PROPERTY EXTRACTORS ===
fn extract_property(property: &LightningProperty) -> Option<Property> {
    use LightningProperty::*;

    let (name, value_opt): (&str, Option<String>) = match property {
        BackgroundColor(color) => ("background-color", Some(format_css_color(color))),
        Color(color) => ("color", Some(format_css_color(color))),
        Width(v) => ("width", Some(to_css_string(v))),
        Height(v) => ("height", Some(to_css_string(v))),
        Margin(v) => ("margin", Some(to_css_string(v))),
        Padding(v) => ("padding", Some(to_css_string(v))),
        Display(v) => ("display", Some(to_css_string(v))),
        Position(v) => ("position", Some(to_css_string(v))),
        FontSize(v) => ("font-size", Some(to_css_string(v))),
        FontWeight(v) => ("font-weight", Some(to_css_string(v))),
        FontFamily(v) => ("font-family", Some(to_css_string(v))),
        TextAlign(v) => ("text-align", Some(to_css_string(v))),
        Border(v) => ("border", Some(to_css_string(v))),
        BorderRadius(v, _) => ("border-radius", Some(to_css_string(v))),
        FlexDirection(v, _) => ("flex-direction", Some(to_css_string(v))),
        JustifyContent(v, _) => ("justify-content", Some(to_css_string(v))),
        AlignItems(v, _) => ("align-items", Some(to_css_string(v))),
        BoxShadow(v, _) => ("box-shadow", Some(to_css_string(v))),
        Transform(v, _) => ("transform", Some(to_css_string(v))),
        Opacity(v) => ("opacity", Some(to_css_string(v))),
        ZIndex(v) => ("z-index", Some(to_css_string(v))),
        Overflow(v) => ("overflow", Some(to_css_string(v))),
        Cursor(v) => ("cursor", Some(to_css_string(v))),
        Visibility(v) => ("visibility", Some(to_css_string(v))),
        BoxSizing(v, _) => ("box-sizing", Some(to_css_string(v))),
        TextDecoration(v, _) => ("text-decoration", Some(to_css_string(v))),
        _ => return None,
    };

    value_opt.map(|value| Property {
        name: name.to_string(),
        value,
    })
}

fn extract_font_face_property(
    property: &lightningcss::rules::font_face::FontFaceProperty,
) -> Option<Property> {
    Some(Property {
        name: "font-face-property".to_string(),
        value: to_css_string(property),
    })
}

// === COLOR FORMATTER ===
fn format_css_color(color: &lightningcss::values::color::CssColor) -> String {
    use lightningcss::values::color::CssColor;
    match color {
        CssColor::RGBA(rgba) => {
            if rgba.alpha == 255 {
                format!("#{:02x}{:02x}{:02x}", rgba.red, rgba.green, rgba.blue)
            } else {
                format!(
                    "rgba({}, {}, {}, {})",
                    rgba.red,
                    rgba.green,
                    rgba.blue,
                    rgba.alpha as f32 / 255.0
                )
            }
        }
        _ => to_css_string(color),
    }
}

// === FILE IO + FALLBACK ===
pub fn parse_css_file(file_path: &str) -> Result<Vec<Rule>, LintError> {
    let css_content = std::fs::read_to_string(file_path).map_err(|e| LintError {
        selector: "".to_string(),
        property: "".to_string(),
        message: format!("Failed to read CSS file: {}", e),
        kind: PrettystrictError::IoError(e),
    })?;

    parse_css(&css_content)
}

pub fn parse_css_with_recovery(css_content: &str) -> Result<Vec<Rule>, LintError> {
    match parse_css(css_content) {
        Ok(rules) => Ok(rules),
        Err(e) => {
            eprintln!(
                "Warning: CSS parsing failed, attempting recovery: {}",
                e.message
            );
            parse_css_fallback(css_content)
        }
    }
}

fn parse_css_fallback(css_content: &str) -> Result<Vec<Rule>, LintError> {
    let mut rules = Vec::new();
    let mut current_rule: Option<Rule> = None;
    let mut in_rule = false;
    let mut brace_count = 0;

    for line in css_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        brace_count += trimmed.chars().filter(|&c| c == '{').count() as i32;
        brace_count -= trimmed.chars().filter(|&c| c == '}').count() as i32;

        if trimmed.contains('{') && !in_rule {
            let selector = trimmed.replace('{', "").trim().to_string();
            current_rule = Some(Rule {
                selector,
                declaration: Vec::new(),
                at_rule: extract_at_rules_simple(css_content),
            });
            in_rule = true;
        } else if trimmed.contains('}') && brace_count == 0 {
            if let Some(rule) = current_rule.take() {
                rules.push(rule);
            }
            in_rule = false;
        } else if in_rule && trimmed.contains(':') {
            if let Some(ref mut rule) = current_rule {
                if let Some(prop) = parse_declaration_simple(trimmed) {
                    rule.declaration.push(prop);
                }
            }
        }
    }

    Ok(rules)
}

fn parse_declaration_simple(decl: &str) -> Option<Property> {
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

fn extract_at_rules_simple(css: &str) -> Vec<String> {
    css.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('@') {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect()
}

pub fn parse_css_default() -> Result<Vec<Rule>, LintError> {
    let css_content = r#"
        .foo {
            color: red;
            font-size: 16px;
        }

        @media (max-width: 600px) {
            .bar {
                background: blue;
            }
        }
    "#;

    parse_css(css_content)
}
