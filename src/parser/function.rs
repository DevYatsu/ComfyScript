pub mod return_expression;

use self::return_expression::parse_return_statement;

use super::{
    ast::{identifier::Identifier, ASTNode, Expression},
    parse_block, Span,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as parse_char, multispace0, multispace1},
    combinator::opt,
    error::VerboseError,
    multi::separated_list1,
    IResult,
};

pub fn parse_function(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    alt((parse_anon_fn, parse_classic_fn))(input)
}

pub fn parse_anon_fn(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, _) = tag("anon")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, _) = tag("fn")(input)?;

    let id = None;

    let (input, _) = multispace0(input)?;
    let (input, params) = parse_fn_params(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_fn_body(input)?;

    let node = ASTNode::FunctionDeclaration {
        id,
        params,
        body: Box::new(body),
        is_shortcut: false,
    };

    Ok((input, node))
}

pub fn parse_fn_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, f) = parse_anon_fn(i)?;

    Ok((i, f.into()))
}

fn parse_classic_fn(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, _) = tag("fn")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, id) = parse_identifier(input)?;
    let id = Some(id);

    let (input, _) = multispace0(input)?;
    let (input, params) = parse_fn_params(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_fn_body(input)?;

    let node = ASTNode::FunctionDeclaration {
        id,
        params,
        body: Box::new(body),
        is_shortcut: false,
    };

    Ok((input, node))
}

fn parse_fn_params(input: Span) -> IResult<Span, Vec<Identifier>, VerboseError<Span>> {
    let (input, _) = parse_char('(')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, params) = opt(separated_list1(tag(","), parse_fn_identifier))(input)?;
    let params = params.unwrap_or_else(|| vec![]);
    let (input, _) = multispace0(input)?;

    let (input, _) = parse_char(')')(input)?;

    Ok((input, params))
}

fn parse_fn_body(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, return_statement) = opt(parse_return_statement)(input)?;

    if let Some(return_statement) = return_statement {
        return Ok((input, return_statement));
    }

    let (input, body) = parse_block(input)?;

    Ok((input, body))
}

fn parse_fn_identifier(input: Span) -> IResult<Span, Identifier, VerboseError<Span>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier(input)?;

    Ok((input, id))
}
