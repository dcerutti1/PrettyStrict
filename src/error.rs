//provides error for this program

use crate::lint_rules::LintError;
use fmt::Display;
use std::fmt;
use lightningcss::error::ParserError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrettystrictError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    #[error("Unexpected end of file")]
    EndOfFile,

    #[error("Parse error: {0}")]
    Custom(String),

    #[error("file error: {0}")]
    UnknownProperty(String),

    #[error("file error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("file error: {0}")]
    UnknownValue(String),

    #[error("file error")]
    DuplicateProperty,

    #[error("no units have been declared")]
    #[allow(dead_code)]
    NoUnitFound,

    #[error("wrong unit has been declared")]
    WrongUnitDeclared,

    #[error("propery overridden ")]
    ProperyOverride,
    #[error("invalid declaration")]
    InvalidDeclaration,

}

impl From<ParserError<'_>> for PrettystrictError {
    fn from(err: ParserError) -> Self {
        match err {
            ParserError::AtRuleBodyInvalid => {
                PrettystrictError::Custom("invalid rule body".to_string())
            }
            ParserError::AtRulePreludeInvalid => {
                PrettystrictError::Custom("invalid rule prelude".to_string())
            }
            ParserError::AtRuleInvalid(_) => {
                PrettystrictError::Custom("invalid rule".to_string())
            }
            ParserError::EndOfInput => {
                PrettystrictError::EndOfFile
            }
            ParserError::InvalidDeclaration => {
                PrettystrictError::Custom("invalid declaration".to_string())
            }
            ParserError::InvalidMediaQuery => {
                PrettystrictError::Custom("invalid media query".to_string())
            }
            ParserError::InvalidNesting => {
                PrettystrictError::Custom("invalid nesting".to_string())
            }
            ParserError::DeprecatedNestRule => {
                PrettystrictError::Custom("deprecated nesting rule".to_string())
            }
            ParserError::DeprecatedCssModulesValueRule => {
                PrettystrictError::Custom("deprecated css-modules-value-rule".to_string())
            }
            ParserError::InvalidPageSelector => {
                PrettystrictError::Custom("invalid page-selector".to_string())
            }
            ParserError::InvalidValue => {
                PrettystrictError::Custom("invalid value".to_string())
            }
            ParserError::QualifiedRuleInvalid => {
                PrettystrictError::Custom("qualified rule invalid".to_string())
            }
            ParserError::SelectorError(_) => {
                PrettystrictError::Custom("invalid selector".to_string())
            }
            ParserError::UnexpectedImportRule => {
                PrettystrictError::Custom("unexpected import rule".to_string())
            }
            ParserError::UnexpectedNamespaceRule => {
                PrettystrictError::Custom("unexpected namespace rule".to_string())
            }
            ParserError::UnexpectedToken(_) => {
                PrettystrictError::Custom("unexpected token".to_string())
            }
            ParserError::MaximumNestingDepth => {
                PrettystrictError::Custom("maximum nesting depth".to_string())
            }
        }
    }
}
impl From<PrettystrictError> for LintError {
    fn from(err: PrettystrictError) -> Self {
        LintError {
            selector: "".into(), // fill in or refactor later
            property: "".into(),
            message: format!("{}", err),
            kind: err,
        }
    }
}
