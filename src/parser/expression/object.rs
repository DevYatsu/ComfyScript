use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::opt, 
    multi::separated_list0, IResult,
};
use nom_supreme::error::ErrorTree;

use crate::parser::{
    ast::{
        identifier::parse_identifier,
        object::{Property, PropertyKind},
        Expression,
    },
    Span,
};

use super::parse_expression;

pub fn parse_object(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
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

fn parse_property(i: Span) -> IResult<Span, Property, ErrorTree<Span>> {
    let (i, _) = multispace0(i)?;
    let (i, id) = parse_identifier(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, expr) = parse_expression(i)?;

    let is_method = match expr {
        Expression::FnExpression { .. } => true,
        _ => false,
    };

    Ok((
        i,
        Property {
            is_method,
            shorthand: false,
            key: id,
            value: expr,
            kind: PropertyKind::Init,
        },
    ))
    // for now is simplified
}
