use nom::{
    branch::alt, character::complete::char, character::complete::multispace0, IResult, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    errors::{expected_closing_tag, expected_expression},
};

use super::{parenthesized::parse_parenthesized, parse_expression, parse_expression_with};

pub fn parse_indexing(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, indexed) = parse_expression_with(parse_expression_except_indexing)(i)?;
    // to avoid infinite recursive call

    let (i, _) = char('[')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = parse_expression.context(expected_expression()).parse(i)?;
    let (i, _) = multispace0(i)?;

    let (i, _) = char(']').context(expected_closing_tag("]")).parse(i)?;

    Ok((
        i,
        Expression::MemberExpression {
            indexed: Box::new(indexed),
            property: Box::new(elements),
            computed: true,
        },
    ))
}

fn parse_expression_except_indexing(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    alt((
        parse_parenthesized,
        parse_identifier_expression,
        // avoid adding to many parser here
    ))(i)
}
