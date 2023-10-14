use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    expression::{member_expr::parse_member_expr, parse_expression},
    Span,
};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::opt,
    error::VerboseError, multi::separated_list1, IResult,
};

pub fn parse_fn_call(input: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (input, id) = alt((parse_member_expr, parse_identifier_expression))(input)?;

    let (input, _) = tag("(")(input)?;
    let (input, args) = opt(separated_list1(tag(","), parse_expression))(input)?;

    let args = args.unwrap_or_else(|| vec![]);

    let (input, _) = multispace0(input)?;
    let (input, _) = tag(")")(input)?;

    let expr = Expression::CallExpression {
        callee: Box::new(id),
        args,
    };

    Ok((input, expr))
}
