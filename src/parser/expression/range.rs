use crate::parser::ast::range::RangeType;
use crate::parser::ast::Expression;
use crate::parser::Span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::VerboseError;
use nom::IResult;

use super::parse_expression;

pub fn parse_range(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, from) = map(parse_expression, |expr| Box::new(expr))(i)?;
    let (i, limits) = parse_range_type(i)?;
    let (i, to) = map(parse_expression, |expr| Box::new(expr))(i)?;

    Ok((i, Expression::Range { from, limits, to }))
}

pub fn parse_range_type(i: Span) -> IResult<Span, RangeType, VerboseError<Span>> {
    let (i, range) = alt((tag("..="), tag("..")))(i)?;

    let range_type = match range.fragment() {
        &"..=" => RangeType::DotEqual,
        _ => RangeType::Dot,
    };

    Ok((i, range_type))
}
