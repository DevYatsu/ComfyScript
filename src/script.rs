use crate::parser::{ast, errors::SyntaxError, parse_input};
use codespan_reporting::{diagnostic::Label, files::SimpleFile};
use nom::{character::complete::alphanumeric1, error::Error as NomError};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ComfyScript<Name: Display + Clone> {
    // a rather simple representation of a comfy script
    /// represents the name of a comfy script
    pub name: Name,
    /// represents the content of a comfy script
    pub content: String,
}

impl<Name: Display + Clone> ComfyScript<Name> {
    pub fn new(name: Name, content: String) -> Self {
        Self { name, content }
    }
    pub fn execute(&self) -> Result<(), (SyntaxError<()>, SimpleFile<Name, String>)> {
        let content = &self.content;

        if content.is_empty() {
            return Ok(());
        }

        let program = match parse_input(&content) {
            Ok(r) => r,
            Err(e) => match e {
                nom_supreme::error::GenericErrorTree::Stack { contexts, .. } => {
                    let ctx = contexts[0].1;
                    println!("ctx {:?}", ctx);
                    match ctx {
                        nom_supreme::error::StackContext::Context(_msg) => {
                            let file = SimpleFile::new(self.name.to_owned(), content.to_owned());
                            let (place, length, found) = self.get_error_data(contexts[0].0);

                            let mut err = SyntaxError::space(found);
                            err.add_label(Label::primary((), place..place + length));

                            return Err((err, file));
                        }
                        _ => unreachable!(),
                    }
                }
                nom_supreme::error::GenericErrorTree::Base { location, kind } => {
                    println!("kind {:?}", kind);
                    let file = SimpleFile::new(self.name.to_owned(), content.to_owned());
                    let (place, length, found) = self.get_error_data(location);

                    let mut err = match kind {
                        nom_supreme::error::BaseErrorKind::Expected(expec) => match expec {
                            nom_supreme::error::Expectation::Tag(expected_token) => {
                                SyntaxError::expected(expected_token, found)
                            }
                            nom_supreme::error::Expectation::Char(expected_token) => {
                                SyntaxError::closing_tag(expected_token.to_string(), found)
                            }
                            nom_supreme::error::Expectation::Alpha => todo!(),
                            nom_supreme::error::Expectation::Digit => todo!(),
                            nom_supreme::error::Expectation::HexDigit => todo!(),
                            nom_supreme::error::Expectation::OctDigit => todo!(),
                            nom_supreme::error::Expectation::AlphaNumeric => todo!(),
                            nom_supreme::error::Expectation::Space => todo!(),
                            nom_supreme::error::Expectation::Multispace => todo!(),
                            nom_supreme::error::Expectation::CrLf => todo!(),
                            nom_supreme::error::Expectation::Eof => todo!(),
                            nom_supreme::error::Expectation::Something => todo!(),
                            _ => todo!(),
                        },
                        _ => unreachable!(),
                    };
                    err.add_label(Label::primary((), place..place + length));

                    return Err((err, file));
                }
                nom_supreme::error::GenericErrorTree::Alt(alt) => {
                    println!("{:?}", alt);

                    std::process::exit(1)
                }
            },
        };

        let program = match program {
            ast::ASTNode::Program { body } => body,
            _ => unreachable!(),
        };

        for node in program {
            println!("{:?}", node);
        }

        Ok(())
    }

    fn get_error_data<'a>(&self, error_content: &'a str) -> (usize, usize, &'a str) {
        let error_place = self.content.len() - error_content.len();
        let error_length = alphanumeric1::<&str, NomError<&str>>(error_content)
            .and_then(|(_, w)| Ok(w.len()))
            .unwrap_or(1);

        let found = &error_content[0..error_length];

        (error_place, error_length, found)
    }

    pub fn minify(&self) -> Result<String, Box<dyn Error>> {
        let content = &self.content;

        if content.is_empty() {
            return Ok(content.to_owned());
        }

        let program = match parse_input(&content) {
            Ok(r) => r,
            Err(_) => return Err("Failed to parse script! Contains an error!".into()),
        };

        let mut buffer = String::new();

        let program = match program {
            ast::ASTNode::Program { body } => body,
            _ => unreachable!(),
        };

        for node in program {
            buffer.push_str(&node.to_string())
        }

        Ok(buffer)
    }
}
