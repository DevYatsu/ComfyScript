use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    comment::jump_comments,
    expression::parse_expression_with,
    Span,
};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::opt,
    error::VerboseError, multi::separated_list1, IResult,
};

use super::parenthesized::parse_parenthesized;

pub fn parse_fn_call(input: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (input, id) = parse_expression_with(parse_expression_except_fn_call)(input)?;

    let (input, _) = tag("(")(input)?;
    let (input, args) = opt(separated_list1(
        tag(","),
        parse_expression_with(parse_expression_except_fn_call),
    ))(input)?;

    let args = args.unwrap_or_else(|| vec![]);

    let (input, _) = multispace0(input)?;
    let (input, _) = tag(")")(input)?;

    let expr = Expression::CallExpression {
        callee: Box::new(id),
        args,
    };

    Ok((input, expr))
}

fn parse_expression_except_fn_call(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = jump_comments(i)?;

    let (i, expr) = alt((
        parse_parenthesized,
        parse_identifier_expression,
        // avoid adding to many parser here
    ))(i)?;

    Ok((i, expr))
}
