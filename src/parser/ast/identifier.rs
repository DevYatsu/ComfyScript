use std::fmt;

use crate::parser::reserved;
use nom::{branch::alt, character::complete::alphanumeric1, multi::many1, IResult, Parser};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use super::{Expression, ExpressionKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

pub fn parse_identifier(i: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (i, name) = many1(alt((tag("_"), alphanumeric1)))
        .map(|list: Vec<&str>| list.join(""))
        .verify(is_id_valid)
        .context("identifier")
        .parse(i)?;

    Ok((i, name.into()))
}

fn is_id_valid(word: &String) -> bool {
    !reserved::KEYWORDS.contains(&word.as_str())
}

pub fn parse_raw_id(i: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let (i, s) = many1(alt((tag("_"), alphanumeric1)))
        .map(|list| list.join(""))
        .parse(i)?;

    Ok((i, s))
}

pub fn parse_identifier_expression(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, id) = parse_identifier(i)?;

    Ok((i, id.into()))
}

impl Identifier {
    pub fn value(&self) -> String {
        self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<Identifier> for String {
    fn into(self) -> Identifier {
        Identifier(self)
    }
}

impl Into<Expression> for Identifier {
    fn into(self) -> Expression {
        Expression::with_kind(ExpressionKind::IdentifierExpression(self))
    }
}
