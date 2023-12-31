use crate::parser::ast::identifier::parse_identifier_expression;
use crate::parser::ast::range::RangeType;
use crate::parser::ast::Expression;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::{IResult, Parser};
use nom_supreme::ParserExt;
use nom_supreme::{error::ErrorTree, tag::complete::tag};

use super::function_call::parse_fn_call;
use super::indexing::parse_indexing;
use super::member_expr::parse_member_expr;
use super::parenthesized::parse_parenthesized;
use super::{parse_expression_with, parse_primitive_value};

pub fn parse_range(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, from) = map(
        parse_expression_with(parse_expression_except_range),
        |expr| Box::new(expr),
    )(i)?;

    let (i, _) = multispace0(i)?;

    let (i, limits) = parse_range_type(i)?;

    let (i, _) = multispace0(i)?;

    let (i, to) = map(
        parse_expression_with(parse_expression_except_range),
        |expr| Box::new(expr),
    )
    .cut()
    .parse(i)?;

    Ok((i, Expression::Range { from, limits, to }))
}

fn parse_expression_except_range(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    // to avoid recursive calls to range parser
    alt((
        parse_member_expr,
        parse_indexing,
        parse_fn_call,
        parse_primitive_value,
        parse_parenthesized,
        parse_identifier_expression,
    ))(i)
}

fn parse_range_type(i: &str) -> IResult<&str, RangeType, ErrorTree<&str>> {
    let (i, range) = alt((
        tag("..=").complete().map(|_| RangeType::DotEqual),
        tag("..").complete().map(|_| RangeType::Dot),
    ))
    .parse(i)?;

    Ok((i, range))
}
