use crate::parser::ast::Expression;
use crate::parser::Span;
use nom::character::complete::multispace0;
use nom::{character::complete::char, IResult};
use nom_supreme::error::ErrorTree;

use super::parse_expression;

pub fn parse_parenthesized(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (i, _) = char('(')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, expr) = parse_expression(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(')')(i)?;

    Ok((i, Expression::Parenthesized(Box::new(expr))))
}
