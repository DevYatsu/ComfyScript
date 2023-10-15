use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::opt,
    error::VerboseError, multi::separated_list0, IResult,
};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, Expression},
    Span,
};

use super::{member_expr::parse_member_expr, parse_expression};

pub fn parse_array(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = tag("[")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = separated_list0(tag(","), parse_values)(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = opt(tag(","))(i)?;
    let (i, _) = multispace0(i)?;

    let (i, _) = tag("]")(i)?;

    Ok((i, Expression::Array { elements }))
}

pub fn parse_array_indexing(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, indexed) = alt((parse_member_expr, parse_identifier_expression))(i)?;

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

fn parse_values(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = multispace0(i)?;
    parse_expression(i)
}
