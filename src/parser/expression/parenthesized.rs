use crate::parser::ast::Expression;
use crate::parser::Span;
use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::{character::complete::char, IResult};

use super::parse_expression;

pub fn parse_parenthesized(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = char('(')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, expr) = parse_expression(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(')')(i)?;

    Ok((i, Expression::Parenthesized(Box::new(expr))))
}
