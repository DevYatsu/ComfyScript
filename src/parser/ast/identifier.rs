use std::fmt;

use crate::{parser::Span, reserved_keywords::RESERVED_KEYWORD};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, error::VerboseError,
    Err, IResult,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

pub fn parse_identifier(i: Span) -> IResult<Span, Identifier, VerboseError<Span>> {
    let (mut i, word) = alt((alphanumeric1, tag("_")))(i)?;
    let mut word = word.fragment().to_string();

    loop {
        match alt((alphanumeric1::<Span, VerboseError<Span>>, tag("_")))(i) {
            Ok((input, s)) => {
                word.push_str(s.fragment());

                match alt((alphanumeric1::<Span, VerboseError<Span>>, tag("_")))(input) {
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
        Err(Err::Error(VerboseError { errors: vec![] })) // return an error
    } else {
        Ok((i, Identifier { name: word }))
    }
}

pub fn parse_identifier_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, id) = parse_identifier(i)?;

    Ok((i, Expression::IdentifierExpression(id)))
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
