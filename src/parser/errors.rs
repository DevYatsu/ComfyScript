use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use std::{error::Error, fmt::Display, str::FromStr};

pub fn expected_identifier() -> &'static str {
    "E001:"
}
pub fn expected_expression() -> &'static str {
    "E002:"
}
pub fn expected_space() -> &'static str {
    "E003:"
}
pub fn expected_keyword(keyword: &'static str) -> &'static str {
    "E004:"
}
pub fn expected_closing_tag(tag: &'static str) -> &'static str {
    "E005:"
}
pub fn expected(tag: &'static str) -> &'static str {
    "E006:"
}
pub fn expected_import_source() -> &'static str {
    "E007:"
}

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

    /// build an error from a code, an optional expected value, and a found value
    /// may panic if no expected value is given for errors that need one
    pub fn from_err_code(err_code: ErrorCode, val: Option<&'static str>, found: &str) -> Self {
        match err_code {
            ErrorCode(1) => SyntaxError::<()>::identifier(found),
            ErrorCode(2) => SyntaxError::expression(found),
            ErrorCode(3) => SyntaxError::space(found),
            ErrorCode(4) => SyntaxError::keyword(val.expect("Expected a keyword passed as argument when trying to convert a code into an error"), found),
            ErrorCode(5) => SyntaxError::closing_tag(val.expect("Expected a closing tag passed as argument"), found),
            ErrorCode(6) => SyntaxError::expected(val.expect("Expected a tag passed as argument"), found),
            ErrorCode(7) => SyntaxError::import_source(found),
            ErrorCode(_) => unreachable!(),
        };

        todo!()
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
    pub fn space(found: &str) -> Self {
        SyntaxError {
            message: "expected space".to_owned(),
            code: 3.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected space
    found `{found}`"
            )],
        }
    }

    pub fn keyword(keyword: &'static str, found: &str) -> Self {
        SyntaxError {
            message: format!("expected keyword '{}'", keyword),
            code: 4.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected `{keyword}`
    found `{found}`"
            )],
        }
    }
    pub fn closing_tag(tag: &'static str, found: &str) -> Self {
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
    pub fn expected(sth: &'static str, found: &str) -> Self {
        SyntaxError {
            message: format!("expected '{}'", sth),
            code: 6.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected `{sth}`
    found `{found}`"
            )],
        }
    }
    pub fn import_source(found: &str) -> Self {
        SyntaxError {
            message: "expected a valid import source".to_owned(),
            code: 7.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected import source
    found `{found}`"
            )],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ErrorCode(usize);

impl ToString for ErrorCode {
    fn to_string(&self) -> String {
        format!("E{:03}", self.0)
    }
}
impl FromStr for ErrorCode {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove the 'E' prefix if present
        let s = s.trim_start_matches('E');

        // Parse the remaining string as an integer
        let code = s.parse::<usize>()?;

        // Create an ErrorCode from the parsed integer
        Ok(ErrorCode(code))
    }
}

impl From<usize> for ErrorCode {
    fn from(value: usize) -> Self {
        ErrorCode(value)
    }
}

#[derive(Debug, Clone)]
pub struct ErrorDynamicData(ErrorCode, String);

impl ToString for ErrorDynamicData {
    fn to_string(&self) -> String {
        format!("{}:{}", self.0.to_string(), self.1)
    }
}
impl FromStr for ErrorDynamicData {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (code, err_content) = s
            .split_once(':')
            .expect("Expected a good ErrorDynamicData ToString implementation");
        let code = ErrorCode::from_str(code).expect("Invalid Error code");

        // Create an ErrorCode from the parsed integer
        Ok(ErrorDynamicData(code, err_content.to_owned()))
    }
}
