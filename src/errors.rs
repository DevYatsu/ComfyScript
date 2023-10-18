use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ComfyScriptError {
    #[error(transparent)]
    #[diagnostic(code(comfy_error::io_error))]
    IoError(#[from] std::io::Error),

    #[error("syntax error")]
    #[diagnostic(code(comfy_error::parsing_error))]
    ParsingFailed {
        #[source_code]
        input: String,
        #[help]
        advice: String,
        #[label("here")]
        message: SourceSpan,
    },
}
