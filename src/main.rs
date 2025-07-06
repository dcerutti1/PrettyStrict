

mod check_file;
mod error;
mod Rules;
mod lint_rules;
mod ParseCss;

use clap::{Parser, Subcommand};
use crate::Rules::check_property::{load_known_props, Property, Rule};
use crate::lint_rules::{lint_rules, LintError};
use crate::Rules::check_value::{load_known_values};
use crate::Rules::duplicate_declaration::Location;

#[derive(Parser, Debug)]
#[command(name =  "prettystrict")]
struct Cli {
    #[command(subcommand)]
   command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Check {
        #[arg(value_name = "FILE")]
        file: Option<String>,

        #[arg(short, long, default_value_t = false)]
        watch: bool,
    },
}
fn main() -> Result<(), LintError> {
    let known_props = load_known_props("./src/CSS/Props.json")?;
    let known_values = load_known_values("./src/CSS/Values.json")?;

    let test_rule = Rule {
        selector: ".my-class".to_string(),
        declaration: vec![
            Property {
                name: "color".to_string(),
                value: "blue".to_string(),
            },

        ],
        AtRule: vec!["@media".to_string()],
    };

    let location = Location { line: 1, column: 1 }; // dummy values for testing

    let errors = lint_rules(&test_rule, &known_props, &known_values, &location);

    for error in errors {
        println!(
            "[{}:{}] {}: {}",
            location.line,
            location.column,
            error.property,
            error.message
        );
    }

    Ok(())
}


