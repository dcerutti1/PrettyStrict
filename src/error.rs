//provides error for this program

use crate::lint_rules::LintError;
use cssparser::{BasicParseErrorKind, ParseError, ParseErrorKind};
use fmt::Display;
use std::fmt;
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

    #[error("parse error: {0}")]
    ParseError(String),
}

impl<'i, E: Display> From<ParseError<'i, E>> for PrettystrictError {
    fn from(err: ParseError<'i, E>) -> Self {
        match err.kind {
            ParseErrorKind::Basic(kind) => match kind {
                BasicParseErrorKind::UnexpectedToken(token) => {
                    PrettystrictError::UnexpectedToken(format!("{:?}", token))
                }
                BasicParseErrorKind::EndOfInput => PrettystrictError::EndOfFile,
                other => PrettystrictError::Custom(format!("{:?}", other)),
            },
            ParseErrorKind::Custom(e) => PrettystrictError::Custom(format!("{}", e)),
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
