use std::fmt;

use crate::{parser::Span, reserved_keywords::RESERVED_KEYWORD};
use nom::{branch::alt, character::complete::alphanumeric1, Err, IResult};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

pub fn parse_identifier(i: Span) -> IResult<Span, Identifier, ErrorTree<Span>> {
    let (mut i, word) = alt((alphanumeric1, tag("_")))(i)?;
    let mut word = word.fragment().to_string();

    loop {
        match alt((alphanumeric1::<Span, ErrorTree<Span>>, tag("_")))(i) {
            Ok((input, s)) => {
                word.push_str(s.fragment());

                match alt((alphanumeric1::<Span, ErrorTree<Span>>, tag("_")))(input) {
                    Ok((input, w)) => {
                        word.push_str(w.fragment());
                        i = input;
                    }
                    Err(_) => {
                        i = input;
                        break;
                    }
                };
            }
            Err(_) => break,
        }
    }

    if RESERVED_KEYWORD.contains(&word.as_str()) {
        Err(Err::Error(ErrorTree::Stack {
            base: todo!(),
            contexts: todo!(),
        })) // return an error
    } else {
        Ok((i, Identifier { name: word }))
    }
}

pub fn parse_identifier_expression(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (i, id) = parse_identifier(i)?;

    Ok((i, Expression::IdentifierExpression(id)))
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
