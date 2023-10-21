use crate::parser::{
    ast::{self, identifier::parse_unchecked_id},
    errors::{get_closing_tag, SyntaxError},
    parse_input,
};
use codespan_reporting::{diagnostic::Label, files::SimpleFile};
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
        let file = SimpleFile::new(self.name.to_owned(), content.to_owned());

        if content.is_empty() {
            return Ok(());
        }

        let program = match parse_input(&content) {
            Ok(r) => r,
            Err(e) => {
                println!("ctx {:?}", e);
                match e {
                    nom_supreme::error::GenericErrorTree::Stack { contexts, .. } => {
                        let ctx = contexts[contexts.len() - 1].1;

                        match ctx {
                            nom_supreme::error::StackContext::Context(msg) => {
                                let (place, length, found) = self.get_error_data(contexts[0].0);
                                let mut err = match msg {
                                    "identifier" => {
                                        // it means base has kind: Kind(Verify)
                                        SyntaxError::identifier(found)
                                    }
                                    "import source" => SyntaxError::import_source(found),
                                    "expression" => SyntaxError::expression(found),
                                    _ => unreachable!(),
                                };

                                err.add_label(Label::primary((), place..place + length));

                                return Err((err, file));
                            }
                            _ => unreachable!(),
                        }
                    }
                    nom_supreme::error::GenericErrorTree::Base { location, kind } => {
                        println!("kind {:?}", kind);
                        println!("loc {:?}", location);
                        let (place, length, found) = self.get_error_data(location);

                        let err = match kind {
                            nom_supreme::error::BaseErrorKind::Expected(expec) => match expec {
                                nom_supreme::error::Expectation::Tag(expected_token) => {
                                    let mut err = SyntaxError::expected(expected_token, found);

                                    err.add_label(Label::primary((), place..place + length));
                                    err
                                }
                                nom_supreme::error::Expectation::Char(expected_token) => {
                                    let opening_tag = expected_token.to_string();
                                    let closing_tag = get_closing_tag(&opening_tag).to_owned();

                                    let mut err =
                                        SyntaxError::closing_tag(opening_tag, closing_tag, found);

                                    err.add_label(Label::primary((), place..place + length));
                                    err
                                }
                                nom_supreme::error::Expectation::Alpha => todo!(),
                                nom_supreme::error::Expectation::Digit => todo!(),
                                nom_supreme::error::Expectation::HexDigit => todo!(),
                                nom_supreme::error::Expectation::OctDigit => todo!(),
                                nom_supreme::error::Expectation::AlphaNumeric => todo!(),
                                nom_supreme::error::Expectation::Space => todo!(),
                                nom_supreme::error::Expectation::Multispace => {
                                    let mut err = SyntaxError::space(found);

                                    err.add_label(Label::primary((), place..place + length));
                                    err
                                }
                                nom_supreme::error::Expectation::CrLf => todo!(),
                                nom_supreme::error::Expectation::Eof => todo!(),
                                nom_supreme::error::Expectation::Something => todo!(),
                                x => {
                                    println!("{:?}", x);

                                    todo!()
                                }
                            },
                            nom_supreme::error::BaseErrorKind::Kind(kind) => {
                                println!("{:?}", kind);

                                match kind {
                                    nom::error::ErrorKind::Tag => todo!(),
                                    nom::error::ErrorKind::Alt => todo!(),
                                    nom::error::ErrorKind::TakeUntil => {
                                        let opening_tag =
                                            if &self.content[place - 2..place - 1] == "/" {
                                                // it means the opening tag is /*
                                                &self.content[place - 2..place]
                                            } else {
                                                &self.content[place - 1..place]
                                            };
                                        let closing_tag = get_closing_tag(opening_tag);
                                        let found = location
                                            .trim_start_matches("\n")
                                            .replace("\n", "..")
                                            .replace("\t", "")
                                            [0..40]
                                            .to_owned()
                                            + "...";

                                        let mut err = SyntaxError::closing_tag(
                                            opening_tag.to_owned(),
                                            closing_tag.to_owned(),
                                            &found,
                                        );

                                        err.add_label(Label::primary((), place..place));

                                        err
                                    }
                                    nom::error::ErrorKind::AlphaNumeric => todo!(),
                                    nom::error::ErrorKind::Space => todo!(),
                                    nom::error::ErrorKind::MultiSpace => todo!(),
                                    nom::error::ErrorKind::Char => todo!(),
                                    nom::error::ErrorKind::CrLf => todo!(),
                                    nom::error::ErrorKind::Verify => todo!(),
                                    nom::error::ErrorKind::Float => todo!(),
                                    _ => unreachable!(),
                                }
                            }
                            nom_supreme::error::BaseErrorKind::External(_) => todo!(),
                        };

                        return Err((err, file));
                    }
                    nom_supreme::error::GenericErrorTree::Alt(alt) => {
                        println!("{:?}", alt);

                        std::process::exit(1)
                    }
                }
            }
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
        let error_length = parse_unchecked_id(error_content)
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
