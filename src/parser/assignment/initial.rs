use std::fmt;

use crate::parser::{
    ast::{identifier::parse_identifier, vars::VariableDeclarator, ASTNode},
    comment::jump_comments,
    expression::parse_expression,
};
use nom::{
    branch::alt,
    character::complete::{char, multispace1, space0},
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

pub fn parse_var_init(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, keyword) = parse_variable_keyword(input)?;

    let (input, _) = multispace1(input)?;

    let (input, declarations) = separated_list1(
        tag(",").delimited_by(jump_comments),
        parse_single_declaration,
    )
    .parse(input)?;

    let result = (
        input,
        ASTNode::VariableDeclaration {
            declarations,
            kind: keyword,
        },
    );

    Ok(result)
}

pub fn parse_single_declaration(input: &str) -> IResult<&str, VariableDeclarator, ErrorTree<&str>> {
    let (input, _) = jump_comments(input)?;

    let (input, id) = parse_identifier
        .verify(|id| id.name.parse::<i32>().is_err())
        .cut()
        .context("identifier")
        .parse(input)?;

    let (input, _) = jump_comments(input)?;

    let (input, _) = char('=').cut().parse(input)?;
    let (input, _) = jump_comments(input)?;

    let (input, value) = parse_expression.parse(input)?;

    let declarator = VariableDeclarator { id, init: value };
    let (input, _) = space0(input)?;

    if input.is_empty() {
        return Ok((input, declarator));
    }

    let (input, _) = alt((char('\n'), char(','), char(';')))
        .peek()
        .context("unexpected")
        .cut()
        .parse(input)?;

    Ok((input, declarator))
}

fn parse_variable_keyword(i: &str) -> IResult<&str, VariableKeyword, ErrorTree<&str>> {
    alt((
        tag("let").complete().map(|_| VariableKeyword::Let),
        tag("var").complete().map(|_| VariableKeyword::Var),
    ))(i)
}
