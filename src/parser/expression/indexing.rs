use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, error::VerboseError,
    IResult,
};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    function::function_call::parse_fn_call,
    Span,
};

use super::{
    member_expr::parse_member_expr, parenthesized::parse_parenthesized, parse_composite_value,
    parse_expression, parse_expression_with, parse_primitive_value,
};

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
    // to avoid recursive calls to indexing parser
    alt((
        parse_fn_call,
        parse_composite_value,
        parse_primitive_value,
        parse_parenthesized,
        parse_member_expr,
        parse_identifier_expression,
    ))(i)
}
