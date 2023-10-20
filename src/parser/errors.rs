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

use std::{error::Error, fmt::Display};

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

#[derive(Debug)]
pub struct SyntaxError<FileId> {
    /// the error message
    pub message: String,
    /// the error status code
    pub code: ErrorCode,
    /// error labels
    pub labels: Vec<Label<FileId>>,
    /// additional notes
    pub notes: Vec<String>,
}

impl SyntaxError<()> {
    pub fn print<Name: Display + AsRef<str> + Clone, Content: AsRef<str>>(
        self,
        file: SimpleFile<Name, Content>,
    ) -> Result<(), Box<dyn Error>> {
        let diagnostic = self.generate_diagnostic();
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        term::emit(&mut writer.lock(), &config, &file, &diagnostic)?;
        Ok(())
    }
}

impl<FileId> SyntaxError<FileId> {
    pub fn new(
        message: String,
        code: usize,
        labels: Vec<Label<FileId>>,
        notes: Vec<String>,
    ) -> Self {
        let code = code.into();

        SyntaxError {
            message,
            code,
            labels,
            notes,
        }
    }
    pub fn generate_diagnostic(self) -> Diagnostic<FileId> {
        Diagnostic::error()
            .with_message(self.message)
            .with_code(self.code.to_string())
            .with_labels(self.labels)
            .with_notes(self.notes)
    }
    pub fn add_label(&mut self, label: Label<FileId>) {
        self.labels.push(label);
    }
    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }
    pub fn identifier(found: &str) -> Self {
        SyntaxError {
            message: "expected a valid identifier".to_owned(),
            code: 1.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected identifier
    found `{found}`"
            )],
        }
    }
    pub fn expression(found: &str) -> Self {
        SyntaxError {
            message: "expected a valid expression".to_owned(),
            code: 2.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected expression
    found `{found}`"
            )],
        }
    }
    pub fn keyword(keyword: &'static str, found: &str) -> Self {
        SyntaxError {
            message: format!("expected keyword '{}'", keyword),
            code: 3.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected `{keyword}`
    found `{found}`"
            )],
        }
    }
    pub fn space(found: &str) -> Self {
        SyntaxError {
            message: "expected space".to_owned(),
            code: 4.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected space
    found `{found}`"
            )],
        }
    }

    pub fn expected_closing_tag(tag: &'static str, found: &str) -> Self {
        SyntaxError {
            message: format!("expected closing tag for '{}'", tag),
            code: 5.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected `{tag}`
    found `{found}`"
            )],
        }
    }
}

#[derive(Debug)]
pub struct ErrorCode(usize);

impl ToString for ErrorCode {
    fn to_string(&self) -> String {
        format!("E00{}", self.0)
    }
}

impl From<usize> for ErrorCode {
    fn from(value: usize) -> Self {
        ErrorCode(value)
    }
}
