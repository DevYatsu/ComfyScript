use std::fmt;

use crate::reserved_keywords::RESERVED_KEYWORD;
use nom::{
    character::complete::alphanumeric1, multi::separated_list0, IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

pub fn parse_identifier(i: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (i, name) = separated_list0(alphanumeric1, tag("_"))
        .map(|list| list.join(""))
        .verify(|word| !RESERVED_KEYWORD.contains(&word.as_str()))
        .parse(i)?;

    let identifier = Identifier {name};

    Ok((i, identifier))
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
