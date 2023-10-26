use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use std::{error::Error, fmt::Display};

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
    pub fn closing_tag(opening_tag: String, closing_tag: String) -> Self {
        SyntaxError {
            message: format!("expected closing tag for '{}'", opening_tag),
            code: 5.into(),
            labels: Vec::new(),
            notes: vec![format!("expected `{closing_tag}`")],
        }
    }
    pub fn expected(sth: String, found: &str) -> Self {
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
    pub fn unexpected(found: &str) -> Self {
        SyntaxError {
            message: "Unexpected token".to_owned(),
            code: 8.into(),
            labels: Vec::new(),
            notes: vec![format!("Unexpected token `{found}`")],
        }
    }
    pub fn number(found: &str) -> Self {
        SyntaxError {
            message: "expected a valid number expression".to_owned(),
            code: 9.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected a valid number
    found `{}`",
                found
            )],
        }
    }
    pub fn block(found: &str) -> Self {
        SyntaxError {
            message: "expected block".to_owned(),
            code: 10.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected `{{`
    found `{found}`"
            )],
        }
    }

    pub fn unknown_char_escape(found: &str) -> Self {
        SyntaxError {
            message: "unknown character escape".to_owned(),
            code: 11.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "unknown character escape
    at `\\{found}`"
            )],
        }
    }
    pub fn valid_data_type(found: &str) -> Self {
        SyntaxError {
            message: "expected a valid data type".to_owned(),
            code: 12.into(),
            labels: Vec::new(),
            notes: vec![format!(
                "expected a data type
    found `{found}`"
            )],
        }
    }
}

pub fn get_opposing_tag(opening_tag: &str) -> &str {
    match opening_tag {
        "]" => "[",
        "[" => "]",
        "}" => "{",
        "{" => "}",
        ")" => "(",
        "(" => ")",
        "*/" => "/*",
        "/*" => "*/",
        "|" => "|",
        x => x,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ErrorCode(usize);

impl ToString for ErrorCode {
    fn to_string(&self) -> String {
        format!("E{:03}", self.0)
    }
}

impl From<usize> for ErrorCode {
    fn from(value: usize) -> Self {
        ErrorCode(value)
    }
}
