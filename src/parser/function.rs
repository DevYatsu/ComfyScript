pub mod return_expression;

use self::return_expression::parse_return_statement;

use super::{
    ast::{identifier::Identifier, ASTNode, Expression},
    parse_block,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    character::complete::{char as parse_char, multispace0, multispace1},
    multi::separated_list0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_function(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, _) = tag("fn").complete().parse(input)?;
    let (input, _) = multispace1.parse(input)?;

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

    let (input, (body, is_shortcut)) = parse_fn_body
        .cut()
        .map(|(b, s)| (Box::new(b), s))
        .parse(input)?;

    let node = ASTNode::FunctionDeclaration {
        id,
        params,
        body,
        is_shortcut,
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

    let (input, (body, is_shortcut)) = parse_fn_body.map(|(b, s)| (Box::new(b), s)).parse(input)?;

    let node = Expression::FnExpression {
        params,
        body,
        is_shortcut,
    };

    Ok((input, node))
}

fn parse_fn_params(input: &str) -> IResult<&str, Vec<Identifier>, ErrorTree<&str>> {
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

fn parse_fn_param(input: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier.parse(input)?;

    Ok((input, id))
}
