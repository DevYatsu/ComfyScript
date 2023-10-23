use std::fmt;

use crate::parser::{
    ast::{identifier::parse_identifier, vars::VariableDeclarator},
    expression::parse_expression,
};
use nom::{
    branch::alt,
    character::complete::{char, multispace0, multispace1, space0},
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum VariableKeyword {
    Var,
    Let,
}

impl fmt::Display for VariableKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableKeyword::Var => write!(f, "var"),
            VariableKeyword::Let => write!(f, "let"),
        }
    }
}

pub fn parse_var_init(input: &str) -> IResult<&str, Vec<VariableDeclarator>, ErrorTree<&str>> {
    let (input, _) = multispace1(input)?;

    separated_list1(tag(","), parse_single_declaration).parse(input)
}

pub fn parse_single_declaration(input: &str) -> IResult<&str, VariableDeclarator, ErrorTree<&str>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier
        .verify(|id| id.name.parse::<i32>().is_err())
        .cut()
        .context("identifier")
        .parse(input)?;

    let (input, _) = multispace0(input)?;

    let (input, _) = char('=').cut().parse(input)?;

    let (input, _) = multispace0(input)?;

    let (input, value) = parse_expression.parse(input)?;

    let declarator = VariableDeclarator { id, init: value };
    let (input, _) = space0(input)?;

    if input.is_empty() {
        return Ok((input, declarator));
    }

    let (input, _) = alt((tag("\n"), tag(","), tag(";"), tag("//").complete()))
        .peek()
        .context("unexpected")
        .cut()
        .parse(input)?;

    let (input, _) = multispace0(input)?;

    Ok((input, declarator))
}
