use crate::parser::ast::Expression;
use nom::character::complete::multispace0;
use nom::Parser;
use nom::{character::complete::char, IResult};
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

use super::parse_expression;

pub fn parse_parenthesized(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = char('(')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, expr) = parse_expression.cut().context("expression").parse(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(')').cut().parse(i)?;

    Ok((i, Expression::Parenthesized(Box::new(expr))))
}
