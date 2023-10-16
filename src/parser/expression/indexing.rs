use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, error::VerboseError,
    IResult,
};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    Span,
};

use super::{parenthesized::parse_parenthesized, parse_expression, parse_expression_with};

pub fn parse_indexing(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, indexed) = parse_expression_with(parse_expression_except_indexing)(i)?;
    // to avoid infinite recursive call

    let (i, _) = tag("[")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = parse_expression(i)?;
    let (i, _) = multispace0(i)?;

    let (i, _) = tag("]")(i)?;

    Ok((
        i,
        Expression::MemberExpression {
            indexed: Box::new(indexed),
            property: Box::new(elements),
            computed: true,
        },
    ))
}

fn parse_expression_except_indexing(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((
        parse_parenthesized,
        parse_identifier_expression,
        // avoid adding to many parser here
    ))(i)
}
