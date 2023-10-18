use crate::parser::ast::identifier::parse_identifier_expression;
use crate::parser::ast::range::RangeType;
use crate::parser::ast::Expression;
use crate::parser::Span;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom_supreme::{error::ErrorTree, tag::complete::tag};

use super::function_call::parse_fn_call;
use super::indexing::parse_indexing;
use super::member_expr::parse_member_expr;
use super::parenthesized::parse_parenthesized;
use super::{parse_composite_value, parse_expression_with, parse_primitive_value};

pub fn parse_range(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
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
    )(i)?;

    Ok((i, Expression::Range { from, limits, to }))
}

fn parse_expression_except_range(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    // to avoid recursive calls to range parser
    alt((
        parse_indexing,
        parse_fn_call,
        parse_composite_value,
        parse_primitive_value,
        parse_parenthesized,
        parse_member_expr,
        parse_identifier_expression,
    ))(i)
}

pub fn parse_range_type(i: Span) -> IResult<Span, RangeType, ErrorTree<Span>> {
    let (i, range) = alt((tag("..="), tag("..")))(i)?;

    let range_type = match range.fragment() {
        &"..=" => RangeType::DotEqual,
        _ => RangeType::Dot,
    };

    Ok((i, range_type))
}
