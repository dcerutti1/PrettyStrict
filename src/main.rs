mod check_file;
mod error;
mod lint_rules;
mod parse_css;
mod rules;

use crate::lint_rules::{LintError, lint_rules};
use crate::parse_css::parse_css;
use crate::rules::check_property::load_known_props;
use crate::rules::check_value::load_known_values;
use crate::rules::duplicate_declaration::Location;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "prettystrict")]
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

    let rules = parse_css()?;
    let location = Location { line: 1, column: 1 }; // dummy values for testing

    let mut all_errors = Vec::new();
    for rule in &rules {
        let errors = lint_rules(rule, &known_props, &known_values, &location);
        all_errors.extend(errors);
    }

    for error in all_errors {
        println!(
            "[{}:{}] {}: {}",
            location.line, location.column, error.property, error.message
        );
    }

    Ok(())
}
