use nom::{
    bytes::complete::tag, character::complete::multispace0, error::VerboseError,
    multi::separated_list0, IResult,
};

use crate::parser::{ast::Expression, primitive_values::parse_primitive_value, Span};

pub fn parse_array(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = tag("[")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = separated_list0(tag(","), parse_values)(i)?;
    let (i, _) = multispace0(i)?;

    let (i, _) = tag("]")(i)?;

    Ok((i, Expression::Array { elements }))
}

fn parse_values(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = multispace0(i)?;
    parse_primitive_value(i)
}
