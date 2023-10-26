pub mod return_expression;

use std::fmt::Display;

use self::return_expression::parse_return_statement;

use super::{
    ast::{identifier::Identifier, ASTNode, Expression},
    data_type::{parse_data_type, parse_opt_type_assignement, DataType},
    parse_block,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    character::complete::{char as parse_char, multispace0, multispace1},
    multi::separated_list0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParam {
    pub id: Identifier,
    pub param_type: Option<DataType>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionReturnType {
    pub return_type: DataType,
    pub is_fallible: bool, // may fail or not
}

pub fn parse_function(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, _) = multispace1(input)?;

    let (input, id) = parse_identifier.cut().context("identifier").parse(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = parse_char('(')
        .cut()
        .context("open parenthesis")
        .parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, params) = parse_fn_params(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = parse_char(')').cut().parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, return_type) = parse_fn_return_type
        .terminated(multispace0)
        .opt()
        .parse(input)?;

    let (input, (body, is_shortcut)) = parse_fn_body
        .cut()
        .map(|(b, s)| (Box::new(b), s))
        .parse(input)?;

    let node = ASTNode::FunctionDeclaration {
        id,
        params,
        body,
        is_shortcut,
        return_type,
    };

    Ok((input, node))
}

pub fn parse_fn_expression(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (input, _) = tag("|")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, params) = parse_fn_params(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag("|").cut().parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, return_type) = parse_fn_return_type
        .terminated(multispace0)
        .opt()
        .parse(input)?;

    let (input, (body, is_shortcut)) = parse_fn_body.map(|(b, s)| (Box::new(b), s)).parse(input)?;

    let node = Expression::FnExpression {
        params,
        body,
        is_shortcut,
        return_type,
    };

    Ok((input, node))
}

fn parse_fn_params(input: &str) -> IResult<&str, Vec<FunctionParam>, ErrorTree<&str>> {
    let (input, params) = separated_list0(tag(","), parse_fn_param).parse(input)?;

    Ok((input, params))
}

fn parse_fn_body(input: &str) -> IResult<&str, (ASTNode, bool), ErrorTree<&str>> {
    let (input, return_statement) = parse_return_statement.opt().parse(input)?;

    if let Some(return_statement) = return_statement {
        return Ok((input, (return_statement, true)));
    }

    let (input, body) = parse_block.cut().parse(input)?;

    Ok((input, (body, false)))
}

fn parse_fn_param(input: &str) -> IResult<&str, FunctionParam, ErrorTree<&str>> {
    let (input, id) = parse_identifier.preceded_by(multispace0).parse(input)?;
    let (input, param_type) = parse_opt_type_assignement(input)?;

    Ok((input, FunctionParam { id, param_type }))
}

fn parse_fn_return_type(input: &str) -> IResult<&str, FunctionReturnType, ErrorTree<&str>> {
    let (input, _) = tag("->").complete().preceded_by(multispace0).parse(input)?;
    let (input, return_type) = parse_data_type
        .cut()
        .context("valid data type")
        .preceded_by(multispace0)
        .parse(input)?;

    let (input, is_fallible) = parse_char('?')
        .preceded_by(multispace0)
        .opt()
        .map(|x| x.is_some())
        .parse(input)?;

    Ok((
        input,
        FunctionReturnType {
            return_type,
            is_fallible,
        },
    ))
}

impl Display for FunctionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;

        if let Some(param_type) = &self.param_type {
            write!(f, ":{}", param_type)?;
        }

        write!(f, "")
    }
}
impl Display for FunctionReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.return_type)?;

        if self.is_fallible {
            write!(f, "?")?;
        }

        write!(f, "")
    }
}
