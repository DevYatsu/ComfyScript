use nom::{character::complete::char, character::complete::multispace0, IResult, Parser};
use nom_supreme::{error::ErrorTree, multi::collect_separated_terminated, ParserExt};

use crate::parser::ast::Expression;

use super::parse_expression;

pub fn parse_array(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = char('[')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = collect_separated_terminated(
        parse_expression.terminated(multispace0),
        char(',').terminated(multispace0),
        char(']').preceded_by(char(',').terminated(multispace0).opt()),
    )
    .cut()
    .parse(i)?;

    Ok((i, Expression::Array { elements }))
}
