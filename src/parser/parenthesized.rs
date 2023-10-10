use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::{branch::alt, character::complete::char, IResult};

use crate::parser::ast::Expression;
use crate::parser::Span;

use super::composite_types::parse_composite_value;
use super::operations::parse_binary_operation;
use super::primitive_values::parse_primitive_value;

pub fn parse_parenthesized(initial_i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = char('(')(initial_i)?;
    let (i, _) = multispace0(i)?;

    let (i, expr) = alt((
        parse_binary_operation,
        parse_composite_value,
        parse_primitive_value,
    ))(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(')')(i)?;

    Ok((i, Expression::Parenthesized(Box::new(expr))))
}
