use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    expression::parse_expression_with0,
};

use nom::{
    branch::alt, character::complete::char, character::complete::multispace0,
    multi::separated_list0, IResult, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

use super::{parenthesized::parse_parenthesized, parse_expression};

pub fn parse_fn_call(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (input, id) = parse_expression_with0(parse_expression_except_fn_call)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = char('(')(input)?;
    let (input, args) = separated_list0(
        char(',').preceded_by(multispace0),
        parse_expression.preceded_by(multispace0),
    )
    .parse(input)?;

    let (input, _) = char(',').preceded_by(multispace0).opt().parse(input)?;

    let (input, _) = char(')').preceded_by(multispace0).cut().parse(input)?;

    let expr = Expression::CallExpression {
        callee: Box::new(id),
        args,
    };

    Ok((input, expr))
}

fn parse_expression_except_fn_call(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = multispace0(i)?;

    let (i, expr) = alt((parse_parenthesized, parse_identifier_expression))(i)?;

    Ok((i, expr))
}
