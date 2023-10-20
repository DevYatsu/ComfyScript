use std::fmt;

use crate::parser::{
    ast::{identifier::parse_identifier, vars::VariableDeclarator, ASTNode},
    expression::parse_expression,
};
use nom::{
    branch::alt,
    character::complete::{multispace0, multispace1},
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

    let (input, declarations) = separated_list1(tag(","), parse_single_declaration)(input)?;

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
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier
        .verify(|id| id.name.parse::<i32>().is_err())
        .cut()
        .parse(input)?;

    let (input, _) = multispace0(input)?;

    let (input, _) = tag("=").cut().parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, value) = parse_expression.cut().parse(input)?;
    let declarator = VariableDeclarator { id, init: value };

    Ok((input, declarator))
}

fn parse_variable_keyword(i: &str) -> IResult<&str, VariableKeyword, ErrorTree<&str>> {
    alt((
        tag("let").map(|_| VariableKeyword::Let),
        tag("var").map(|_| VariableKeyword::Var),
    ))(i)
}
