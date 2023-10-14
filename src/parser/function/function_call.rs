use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    expression::parse_expression,
    Span,
};

use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::opt, error::VerboseError,
    multi::separated_list1, IResult,
};

pub fn parse_fn_call(input: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (input, id) = parse_identifier_expression(input)?;

    let (input, _) = tag("(")(input)?;
    let (input, args) = opt(separated_list1(tag(","), parse_expression))(input)?;

    let args = if args.is_none() {
        vec![]
    } else {
        args.unwrap()
    };

    let (input, _) = tag(")")(input)?;
    let (input, _) = multispace0(input)?;

    let expr = Expression::CallExpression {
        callee: Box::new(id),
        args,
    };

    Ok((input, expr))
}
