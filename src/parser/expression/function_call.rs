use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    comment::jump_comments,
    expression::parse_expression_with,
};

use nom::{
    branch::alt, character::complete::char, character::complete::multispace0,
    multi::separated_list0, IResult, Parser,
};
use nom_supreme::error::ErrorTree;

use super::{parenthesized::parse_parenthesized, parse_expression};

pub fn parse_fn_call(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (input, id) = parse_expression_with(parse_expression_except_fn_call)(input)?;

    let (input, _) = char('(')(input)?;
    let (input, args) = separated_list0(char(','), parse_expression).parse(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;

    let expr = Expression::CallExpression {
        callee: Box::new(id),
        args,
    };

    Ok((input, expr))
}

fn parse_expression_except_fn_call(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = jump_comments(i)?;

    let (i, expr) = alt((
        parse_parenthesized,
        parse_identifier_expression,
        // avoid adding to many parser here
    ))(i)?;

    Ok((i, expr))
}
