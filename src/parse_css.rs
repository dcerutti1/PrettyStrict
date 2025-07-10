use crate::lint_rules::LintError;
use crate::rules::check_property::{Property, Rule};
use lightningcss::properties::Property as LightningProperty;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};

pub fn parse_css(css_content: &str) -> Result<Vec<Rule>, LintError> {
    // Parse the CSS using lightningcss
    let stylesheet =
        StyleSheet::parse(css_content, ParserOptions::default()).map_err(|e| LintError {
            selector: "".to_string(),
            property: "".to_string(),
            message: format!("Failed to parse CSS: {:?}", e),
            kind: crate::error::PrettystrictError::ParseError(format!("{:?}", e)),
        })?;

    let mut rules = Vec::new();
    let mut current_at_rules = Vec::new();

    // Traverse the stylesheet rules
    for rule in &stylesheet.rules.0 {
        traverse_rule(rule, &mut rules, &mut current_at_rules);
    }

    Ok(rules)
}

fn traverse_rule(rule: &CssRule, rules: &mut Vec<Rule>, current_at_rules: &mut Vec<String>) {
    match rule {
        CssRule::Style(style_rule) => {
            let selector = format!("{}", style_rule.selectors);
            let mut declarations = Vec::new();

            for property in &style_rule.declarations.declarations {
                if let Some(prop) = extract_property(property) {
                    declarations.push(prop);
                }
            }

            let new_rule = Rule {
                selector,
                declaration: declarations,
                at_rule: current_at_rules.clone(),
            };

            rules.push(new_rule);
        }
        CssRule::Media(media_rule) => {
            let media_query = format!("@media {:?}", media_rule.query);
            current_at_rules.push(media_query);

            for nested_rule in &media_rule.rules.0 {
                traverse_rule(nested_rule, rules, current_at_rules);
            }

            current_at_rules.pop();
        }
        CssRule::Supports(supports_rule) => {
            let supports_query = format!("@supports {:?}", supports_rule.condition);
            current_at_rules.push(supports_query);

            for nested_rule in &supports_rule.rules.0 {
                traverse_rule(nested_rule, rules, current_at_rules);
            }

            current_at_rules.pop();
        }
        CssRule::Keyframes(keyframes_rule) => {
            let keyframes_name = format!("@keyframes {:?}", keyframes_rule.name);
            current_at_rules.push(keyframes_name);

            for keyframe in &keyframes_rule.keyframes {
                let selector = format!("{:?}", keyframe.selectors);

                let mut declarations = Vec::new();
                for property in &keyframe.declarations.declarations {
                    if let Some(prop) = extract_property(property) {
                        declarations.push(prop);
                    }
                }

                let keyframe_rule = Rule {
                    selector,
                    declaration: declarations,
                    at_rule: current_at_rules.clone(),
                };

                rules.push(keyframe_rule);
            }

            current_at_rules.pop();
        }
        CssRule::Import(import_rule) => {
            let import_at_rule = format!("@import {:?}", import_rule.url);
            let import_rule_struct = Rule {
                selector: "".to_string(),
                declaration: Vec::new(),
                at_rule: vec![import_at_rule],
            };
            rules.push(import_rule_struct);
        }
        CssRule::FontFace(font_face_rule) => {
            let font_face_at_rule = "@font-face".to_string();
            let mut declarations = Vec::new();

            for property in &font_face_rule.properties {
                if let Some(prop) = extract_font_face_property(property) {
                    declarations.push(prop);
                }
            }

            let font_face_rule_struct = Rule {
                selector: "".to_string(),
                declaration: declarations,
                at_rule: vec![font_face_at_rule],
            };

            rules.push(font_face_rule_struct);
        }
        _ => {
            // Handle other rule types as needed
        }
    }
}

fn extract_property(property: &LightningProperty) -> Option<Property> {
    match property {
        LightningProperty::BackgroundColor(color) => Some(Property {
            name: "background-color".to_string(),
            value: format_css_color(color),
        }),
        LightningProperty::Color(color) => Some(Property {
            name: "color".to_string(),
            value: format_css_color(color),
        }),
        LightningProperty::Width(width) => Some(Property {
            name: "width".to_string(),
            value: format!("{:?}", width),
        }),
        LightningProperty::Height(height) => Some(Property {
            name: "height".to_string(),
            value: format!("{:?}", height),
        }),
        LightningProperty::Margin(margin) => Some(Property {
            name: "margin".to_string(),
            value: format!("{:?}", margin),
        }),
        LightningProperty::Padding(padding) => Some(Property {
            name: "padding".to_string(),
            value: format!("{:?}", padding),
        }),
        LightningProperty::Display(display) => Some(Property {
            name: "display".to_string(),
            value: format!("{:?}", display),
        }),
        LightningProperty::Position(position) => Some(Property {
            name: "position".to_string(),
            value: format!("{:?}", position),
        }),
        LightningProperty::FontSize(font_size) => Some(Property {
            name: "font-size".to_string(),
            value: format!("{:?}", font_size),
        }),
        LightningProperty::FontWeight(font_weight) => Some(Property {
            name: "font-weight".to_string(),
            value: format!("{:?}", font_weight),
        }),
        LightningProperty::FontFamily(font_family) => Some(Property {
            name: "font-family".to_string(),
            value: format!("{:?}", font_family),
        }),
        LightningProperty::TextAlign(text_align) => Some(Property {
            name: "text-align".to_string(),
            value: format!("{:?}", text_align),
        }),
        LightningProperty::Border(border) => Some(Property {
            name: "border".to_string(),
            value: format!("{:?}", border),
        }),
        LightningProperty::BorderRadius(border_radius, _) => Some(Property {
            name: "border-radius".to_string(),
            value: format!("{:?}", border_radius),
        }),
        LightningProperty::FlexDirection(flex_direction, _) => Some(Property {
            name: "flex-direction".to_string(),
            value: format!("{:?}", flex_direction),
        }),
        LightningProperty::JustifyContent(justify_content, _) => Some(Property {
            name: "justify-content".to_string(),
            value: format!("{:?}", justify_content),
        }),
        LightningProperty::AlignItems(align_items, _) => Some(Property {
            name: "align-items".to_string(),
            value: format!("{:?}", align_items),
        }),
        LightningProperty::BoxShadow(box_shadow, _) => Some(Property {
            name: "box-shadow".to_string(),
            value: format!("{:?}", box_shadow),
        }),
        LightningProperty::Transform(transform, _) => Some(Property {
            name: "transform".to_string(),
            value: format!("{:?}", transform),
        }),
        LightningProperty::Opacity(opacity) => Some(Property {
            name: "opacity".to_string(),
            value: format!("{:?}", opacity),
        }),
        LightningProperty::ZIndex(z_index) => Some(Property {
            name: "z-index".to_string(),
            value: format!("{:?}", z_index),
        }),
        LightningProperty::Overflow(overflow) => Some(Property {
            name: "overflow".to_string(),
            value: format!("{:?}", overflow),
        }),
        LightningProperty::Cursor(cursor) => Some(Property {
            name: "cursor".to_string(),
            value: format!("{:?}", cursor),
        }),
        LightningProperty::Visibility(visibility) => Some(Property {
            name: "visibility".to_string(),
            value: format!("{:?}", visibility),
        }),
        LightningProperty::Unparsed(unparsed) => Some(Property {
            name: unparsed.property_id.name().to_string(),
            value: format!("{:?}", unparsed.value),
        }),
        LightningProperty::TextDecoration(text_decoration, _) => Some(Property {
            name: "text-decoration".to_string(),
            value: format!("{:?}", text_decoration),
        }),
        LightningProperty::BoxSizing(box_sizing, _) => Some(Property {
            name: "box-sizing".to_string(),
            value: format!("{:?}", box_sizing),
        }),
        _ => {
            // For other properties, we use a generic fallback
            None
        }
    }
}

// Helper function for formatting CSS colors
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
        _ => format!("{:?}", color),
    }
}

fn extract_font_face_property(
    property: &lightningcss::rules::font_face::FontFaceProperty,
) -> Option<Property> {
    Some(Property {
        name: "font-face-property".to_string(),
        value: format!("{:?}", property),
    })
}

// Helper function to parse CSS from a file
pub fn parse_css_file(file_path: &str) -> Result<Vec<Rule>, LintError> {
    let css_content = std::fs::read_to_string(file_path).map_err(|e| LintError {
        selector: "".to_string(),
        property: "".to_string(),
        message: format!("Failed to read CSS file: {}", e),
        kind: crate::error::PrettystrictError::IoError(e),
    })?;

    parse_css(&css_content)
}

// Advanced CSS parsing with error recovery
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

// Fallback parser for when lightningcss fails
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
    let mut at_rules = Vec::new();
    for line in css.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('@') {
            at_rules.push(trimmed.to_string());
        }
    }
    at_rules
}

pub fn parse_css_default() -> Result<Vec<Rule>, LintError> {
    // Simple hardcoded CSS for testing - in a real implementation,
    // this would read from a file or take CSS as input
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
