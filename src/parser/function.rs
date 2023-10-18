pub mod return_expression;

use self::return_expression::parse_return_statement;

use super::{
    ast::{identifier::Identifier, ASTNode, Expression},
    parse_block,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    character::complete::{char as parse_char, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    IResult,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

pub fn parse_function(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, _) = tag("fn")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, id) = parse_identifier(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = parse_char('(')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, params) = parse_fn_params(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = parse_char(')')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, (body, is_shortcut)) = map(parse_fn_body, |(b, s)| (Box::new(b), s))(input)?;

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
    let (input, _) = tag("|")(input)?;

    let (input, _) = multispace0(input)?;

    let (input, (body, is_shortcut)) = map(parse_fn_body, |(b, s)| (Box::new(b), s))(input)?;

    let node = Expression::FnExpression {
        params,
        body,
        is_shortcut,
    };

    Ok((input, node))
}

fn parse_fn_params(input: &str) -> IResult<&str, Vec<Identifier>, ErrorTree<&str>> {
    let (input, params) = opt(separated_list1(tag(","), parse_fn_identifier))(input)?;
    let params = params.unwrap_or_else(|| vec![]);

    Ok((input, params))
}

fn parse_fn_body(input: &str) -> IResult<&str, (ASTNode, bool), ErrorTree<&str>> {
    let (input, return_statement) = opt(parse_return_statement)(input)?;

    if let Some(return_statement) = return_statement {
        return Ok((input, (return_statement, true)));
    }

    let (input, body) = parse_block(input)?;

    Ok((input, (body, false)))
}

fn parse_fn_identifier(input: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier(input)?;

    Ok((input, id))
}
