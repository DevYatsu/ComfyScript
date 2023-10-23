use crate::parser::{
    ast::{self, identifier::parse_unchecked_id},
    comment::jump_comments,
    errors::{get_opposing_tag, SyntaxError},
    expression::strings::parse_unchecked_string,
    parse_input,
};
use codespan_reporting::{diagnostic::Label, files::SimpleFile};
use nom::{branch::alt, Parser};
use nom_supreme::error::GenericErrorTree;
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
                return Err((self.match_error(&e), file));
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

    fn match_error(
        &self,
        e: &GenericErrorTree<&str, &str, &str, Box<dyn Error + Send + Sync>>,
    ) -> SyntaxError<()> {
        match e {
            nom_supreme::error::GenericErrorTree::Stack { contexts, .. } => {
                let ctx = contexts[contexts.len() - 1].1;
                let location = contexts[0].0;

                let location = self.location_with_last_no_whitespace(&location);

                let (place, length, found) = self.get_error_data(location);

                match ctx {
                    nom_supreme::error::StackContext::Context(msg) => {
                        let mut err = match msg {
                            "identifier" => {
                                // it means base has kind: Kind(Verify)
                                SyntaxError::identifier(found)
                            }
                            "import source" => SyntaxError::import_source(found),
                            "expression" => SyntaxError::expression(found),
                            "unexpected" => SyntaxError::unexpected(&found[0..1]),
                            "open parenthesis" => {
                                SyntaxError::expected("(".to_owned(), &found[0..1])
                            }
                            "block" => SyntaxError::block(&found[0..1]),
                            "block end" => SyntaxError::closing_tag("{".to_owned(), "}".to_owned()),
                            _ => unreachable!(),
                        };

                        err.add_label(Label::primary((), place..place + length));

                        err
                    }
                    _ => unreachable!(),
                }
            }
            nom_supreme::error::GenericErrorTree::Base { location, kind } => {
                let location = self.location_with_last_no_whitespace(&location);

                let err = match kind {
                    nom_supreme::error::BaseErrorKind::Expected(expec) => match expec {
                        nom_supreme::error::Expectation::Tag(expected_token) => {
                            let token = expected_token.to_string();
                            let (place, length, found) = self.get_error_data(location);

                            let mut err = SyntaxError::expected(token, found);

                            err.add_label(Label::primary((), place..place + length));
                            err
                        }
                        nom_supreme::error::Expectation::Char(expected_token) => {
                            let closing_tag = expected_token.to_string();
                            let opening_tag = get_opposing_tag(&closing_tag).to_owned();
                            println!("open {}, close {}", opening_tag, closing_tag);

                            let (place, length, _) = self.get_error_data(location);

                            let mut err = SyntaxError::closing_tag(opening_tag, closing_tag);

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
                            let (place, length, found) = self.get_error_data(location);

                            let mut err = SyntaxError::space(found);

                            err.add_label(Label::primary((), place..place + length));
                            err
                        }
                        nom_supreme::error::Expectation::Eof => {
                            let (place, length, found) = self.get_error_data(location);

                            let mut err = SyntaxError::unexpected(found);

                            err.add_label(Label::primary((), place..place + length));
                            err
                        }
                        _ => {
                            todo!()
                        }
                    },
                    nom_supreme::error::BaseErrorKind::Kind(kind) => {
                        println!("{:?}", kind);
                        let (place, length, found) = self.get_error_data(location);

                        match kind {
                            nom::error::ErrorKind::Tag => unreachable!(),
                            nom::error::ErrorKind::Alt => unreachable!(),
                            nom::error::ErrorKind::TakeUntil => {
                                println!("hey takeuntil: {:?}", location);
                                let closing_tag = if &self.content[place - 2..place - 1] == "/" {
                                    // it means the opening tag is /*
                                    &self.content[place - 2..place]
                                } else {
                                    &self.content[place - 1..place]
                                };

                                let opening_tag = get_opposing_tag(closing_tag);

                                let mut err = SyntaxError::closing_tag(
                                    closing_tag.to_owned(),
                                    opening_tag.to_owned(),
                                );

                                err.add_label(Label::primary((), place..place));

                                err
                            }
                            nom::error::ErrorKind::AlphaNumeric => unreachable!(),
                            nom::error::ErrorKind::Space => unreachable!(),
                            nom::error::ErrorKind::MultiSpace => unreachable!(),
                            nom::error::ErrorKind::Char => unreachable!(),
                            nom::error::ErrorKind::CrLf => unreachable!(),
                            nom::error::ErrorKind::Verify => unreachable!(),
                            nom::error::ErrorKind::Float => {
                                let mut err = SyntaxError::number(found);

                                err.add_label(Label::primary((), place..place + length));
                                err
                            }
                            _ => unreachable!(),
                        }
                    }
                    nom_supreme::error::BaseErrorKind::External(_) => unreachable!(),
                };

                err
            }
            nom_supreme::error::GenericErrorTree::Alt(alt) => {
                let x = &alt[0];
                self.match_error(x)
            }
        }
    }
    fn get_error_data<'a>(&self, error_content: &'a str) -> (usize, usize, &'a str) {
        let error_place = self.content.len() - error_content.len();

        let new_error_content = jump_comments(error_content)
            .and_then(|(i, _)| Ok(i))
            .unwrap_or(error_content);

        let error_length = alt((parse_unchecked_id, parse_unchecked_string))
            .parse(&new_error_content)
            .and_then(|(_, w)| Ok(w.len()))
            .unwrap_or(1);
        let found = &new_error_content[0..error_length];

        (
            error_place,
            (error_content.len() - new_error_content.len()) + error_length,
            found,
        )
    }
    fn location_with_last_no_whitespace<'a>(&'a self, location: &'a str) -> &'a str {
        if location.is_empty() {
            let index_last_real_char = self
                .content
                .rfind(|c: char| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
                .unwrap_or(self.content.len() - 1);

            &self.content[index_last_real_char..]
        } else {
            location
        }
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
