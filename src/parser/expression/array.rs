use nom::{
    character::complete::char, character::complete::multispace0, combinator::opt,
    multi::separated_list0, IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use crate::parser::ast::Expression;

use super::parse_expression;

pub fn parse_array(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = char('[')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = separated_list0(tag(","), parse_values).cut().parse(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = opt(char(','))(i)?;
    let (i, _) = multispace0(i)?;

    let (i, _) = char(']')(i)?;

    Ok((i, Expression::Array { elements }))
}

fn parse_values(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = multispace0(i)?;
    parse_expression.context("expression").parse(i)
}
