use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::opt,
    multi::separated_list0, IResult,
};
use nom_supreme::error::ErrorTree;

use crate::parser::{ast::Expression, Span};

use super::parse_expression;

pub fn parse_array(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (i, _) = tag("[")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = separated_list0(tag(","), parse_values)(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = opt(tag(","))(i)?;
    let (i, _) = multispace0(i)?;

    let (i, _) = tag("]")(i)?;

    Ok((i, Expression::Array { elements }))
}

fn parse_values(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (i, _) = multispace0(i)?;
    parse_expression(i)
}
