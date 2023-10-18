use std::fmt;

use crate::reserved_keywords::RESERVED_KEYWORD;
use nom::{branch::alt, character::complete::alphanumeric1, IResult};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

pub fn parse_identifier(i: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (mut i, word) = alt((alphanumeric1, tag("_")))(i)?;
    let mut word = word.to_owned();

    loop {
        match alt((alphanumeric1::<&str, ErrorTree<&str>>, tag("_")))(i) {
            Ok((input, s)) => {
                word.push_str(s);

                match alt((alphanumeric1::<&str, ErrorTree<&str>>, tag("_")))(input) {
                    Ok((input, w)) => {
                        word.push_str(w);
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
        Err(nom::Err::Error(ErrorTree::Alt(vec![])))
        // return an error
    } else {
        Ok((i, Identifier { name: word }))
    }
}

pub fn parse_identifier_expression(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, id) = parse_identifier(i)?;

    Ok((i, Expression::IdentifierExpression(id)))
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
