use std::fmt;

use crate::parser::reserved_keywords::RESERVED_KEYWORD;
use nom::{branch::alt, character::complete::alphanumeric1, multi::many1, IResult, Parser};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

pub fn parse_identifier(i: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (i, name) = many1(alt((tag("_"), alphanumeric1)))
        .map(|list| list.join(""))
        .verify(is_id_valid)
        .context("identifier")
        .parse(i)?;

    let identifier = Identifier { name };

    Ok((i, identifier))
}

fn is_id_valid(word: &String) -> bool {
    !RESERVED_KEYWORD.contains(&word.as_str())
}

pub fn parse_unchecked_id(i: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let (i, s) = many1(alt((tag("_"), alphanumeric1)))
        .map(|list| list.join(""))
        .parse(i)?;

    Ok((i, s))
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
