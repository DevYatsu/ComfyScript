pub fn expected_space() -> &'static str {
    "Expected a space"
}
pub fn expected_expression() -> &'static str {
    "Expected a valid expression"
}

#[macro_export]
macro_rules! expected_keyword {
    ($keyword:expr) => {
        concat!("Expected '{}' keyword", $keyword)
    };
}
#[macro_export]
macro_rules! expected_valid {
    ($x:expr) => {
        concat!("Expected valid ", $x)
    };
}

#[macro_export]
macro_rules! expected {
    ($x:expr) => {
        concat!("Expected ", $x)
    };
}

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum SyntaxError {
    #[error(transparent)]
    #[diagnostic(code(comfy_error::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Expected a specifier")]
    #[diagnostic(help("Add a specifier"))]
    #[diagnostic(code(comfy_error::syntax_error::specifer))]
    ExpectedSpecifier {
        #[source_code]
        input: String,
        #[label("specifier here ?")]
        span: SourceSpan,
    },

    #[error("Expected a valid expression")]
    #[diagnostic(code(comfy_error::syntax_error::expression))]
    ExpectedExpression {
        #[source_code]
        input: String,
        #[help]
        advice: String,
        #[label("expression")]
        span: SourceSpan,
    },

    #[error("Expected a valid keyword")]
    #[diagnostic(code(comfy_error::syntax_error::keyword))]
    ExpectedKeyword {
        #[source_code]
        input: String,
        #[help]
        advice: String,
        #[label("expression")]
        span: SourceSpan,
    },

    #[error("Unexpected character")]
    #[diagnostic(code(comfy_error::syntax_error::unexpected))]
    UnexpectedChar {
        #[source_code]
        input: String,
        #[help]
        advice: String,
        #[label("char here")]
        span: SourceSpan,
    },

    #[error("Expected a closing tag")]
    #[diagnostic(code(comfy_error::syntax_error::closing_tag))]
    ExpectedClosingTag {
        #[source_code]
        input: String,
        #[help]
        advice: String,
        #[label("missing closing tag")]
        span: SourceSpan,
    },
}

#[derive(Debug)]
pub enum ErrorType {
    Space,
    Expression,
    Keyword(String),
    UnexpectedChar(char),
    ExpectedClosingTag(String),
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            ErrorType::Space => "space".to_owned(),
            ErrorType::Expression => "expression".to_owned(),
            ErrorType::Keyword(keyword) => format!("keyword {keyword}"),
            ErrorType::UnexpectedChar(c) => format!("char {c}"),
            ErrorType::ExpectedClosingTag(t) => format!("closing {t}"),
        }
    }
}
