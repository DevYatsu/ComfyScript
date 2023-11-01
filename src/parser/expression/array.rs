use nom::{
    character::complete::char, character::complete::multispace0, multi::separated_list0, IResult,
    Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

use crate::parser::ast::{Expression, ExpressionKind};

use super::parse_expression;

pub fn parse_array(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = char('[')(i)?;

    let (i, elements) =
        separated_list0(char(','), parse_expression.delimited_by(multispace0)).parse(i)?;

    let (i, _) = char(',').terminated(multispace0).opt().parse(i)?;
    let (i, _) = char(']').context("unexpected").cut().parse(i)?;

    Ok((i, elements.into()))
}

impl Into<Expression> for Vec<Expression> {
    fn into(self) -> Expression {
        Expression::with_kind(ExpressionKind::Array(self))
    }
}
