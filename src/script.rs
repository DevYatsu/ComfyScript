use crate::parser::{ast, errors::SyntaxError, parse_input};
use codespan_reporting::{diagnostic::Label, files::SimpleFile};
use nom::{character::complete::alphanumeric1, error::Error as NomError};
use std::{error::Error, fmt::Display, process};

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

                    match ctx {
                        nom_supreme::error::StackContext::Context(_msg) => {
                            let file = SimpleFile::new(self.name.to_owned(), content.to_owned());
                            let error_place = content.len() - contexts[0].0.len();
                            let error_length = alphanumeric1::<&str, NomError<&str>>(contexts[0].0)
                                .and_then(|(_, w)| Ok(w.len()))
                                .unwrap_or(1);

                            let found = &contexts[0].0[0..error_length];

                            let mut err = SyntaxError::space(found);
                            err.add_label(Label::primary(
                                (),
                                error_place..error_place + error_length,
                            ));

                            return Err((err, file));
                        }
                        _ => unreachable!(),
                    }
                }
                e => {
                    println!("{:?}", e);
                    process::exit(1)
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
