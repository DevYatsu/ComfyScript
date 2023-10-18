pub mod return_expression;

use self::return_expression::parse_return_statement;

use super::{
    ast::{identifier::Identifier, ASTNode, Expression},
    parse_block, Span,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    bytes::complete::tag,
    character::complete::{char as parse_char, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    IResult,
};
use nom_supreme::error::ErrorTree;

pub fn parse_function(input: Span) -> IResult<Span, ASTNode, ErrorTree<Span>> {
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
pub fn parse_fn_expression(input: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
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

fn parse_fn_params(input: Span) -> IResult<Span, Vec<Identifier>, ErrorTree<Span>> {
    let (input, params) = opt(separated_list1(tag(","), parse_fn_identifier))(input)?;
    let params = params.unwrap_or_else(|| vec![]);

    Ok((input, params))
}

fn parse_fn_body(input: Span) -> IResult<Span, (ASTNode, bool), ErrorTree<Span>> {
    let (input, return_statement) = opt(parse_return_statement)(input)?;

    if let Some(return_statement) = return_statement {
        return Ok((input, (return_statement, true)));
    }

    let (input, body) = parse_block(input)?;

    Ok((input, (body, false)))
}

fn parse_fn_identifier(input: Span) -> IResult<Span, Identifier, ErrorTree<Span>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier(input)?;

    Ok((input, id))
}
