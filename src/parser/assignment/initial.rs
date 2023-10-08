use std::fmt;

use crate::parser::{
    ast::{identifier::parse_identifier, vars::VariableDeclarator, ASTNode},
    expression::parse_expression,
    utils::parse_new_lines,
    Span,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::map,
    error::{context, ContextError, VerboseError},
    multi::separated_list1,
    IResult,
};

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

pub fn parse_var_init(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
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

pub fn parse_single_declaration(
    input: Span,
) -> IResult<Span, VariableDeclarator, VerboseError<Span>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier(input)?;

    match id.name.parse::<i32>().is_ok() {
        true => {
            return Err(nom::Err::Error(VerboseError::add_context(
                input,
                "test",
                VerboseError { errors: vec![] },
            )));
        }
        false => (),
    }

    let (input, _) = multispace0(input)?;

    let (input, _) = context("Expected '='", tag("="))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, value) = parse_expression(input)?;
    let declarator = VariableDeclarator { id, init: value };

    Ok((input, declarator))
}

fn parse_variable_keyword(i: Span) -> IResult<Span, VariableKeyword, VerboseError<Span>> {
    alt((
        map(tag("let"), |_| VariableKeyword::Let),
        map(tag("var"), |_| VariableKeyword::Var),
    ))(i)
}
