use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::opt, error::VerboseError,
    multi::separated_list0, IResult,
};

use crate::parser::{
    ast::{
        identifier::parse_identifier,
        object::{Property, PropertyKind},
        Expression,
    },
    primitive_values::parse_primitive_value,
    Span,
};

pub fn parse_object(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = tag("{")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = separated_list0(tag(","), parse_property)(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = opt(tag(","))(i)?; 
    let (i, _) = multispace0(i)?;

    let (i, _) = tag("}")(i)?;

    Ok((
        i,
        Expression::Object {
            properties: elements,
        },
    ))
}

fn parse_property(i: Span) -> IResult<Span, Property, VerboseError<Span>> {
    let (i, _) = multispace0(i)?;
    let (i, id) = parse_identifier(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, expr) = parse_primitive_value(i)?; //todo! parse expression

    Ok((
        i,
        Property {
            method: false,
            shorthand: false,
            key: id,
            value: expr,
            kind: PropertyKind::Init,
        },
    ))
    // for now is simplified
}
