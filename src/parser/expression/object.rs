use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::opt,
    error::VerboseError, multi::separated_list0, IResult,
};

use crate::parser::{
    ast::{
        identifier::{parse_identifier, parse_identifier_expression},
        object::{Property, PropertyKind},
        Expression,
    },
    function::parse_anon_fn,
    Span,
};

use super::{member_expr::parse_member_expr, parse_expression};

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

pub fn parse_object_indexing(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
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

fn parse_property(i: Span) -> IResult<Span, Property, VerboseError<Span>> {
    let (i, _) = multispace0(i)?;
    let (i, id) = parse_identifier(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, expr) = alt((parse_expression, parse_method))(i)?;

    let is_method = match expr {
        Expression::Method { .. } => true,
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

fn parse_method(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, f) = parse_anon_fn(i)?;

    Ok((i, f.into()))
}
