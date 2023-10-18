use nom::{character::complete::multispace0, combinator::opt, multi::separated_list0, IResult, character::complete::char};
use nom_supreme::{error::ErrorTree};

use crate::parser::ast::{
    identifier::parse_identifier,
    object::{Property, PropertyKind},
    Expression,
};

use super::parse_expression;

pub fn parse_object(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = char('{')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, elements) = separated_list0(char(','), parse_property)(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = opt(char(','))(i)?;
    let (i, _) = multispace0(i)?;

    let (i, _) = char('}')(i)?;

    Ok((
        i,
        Expression::Object {
            properties: elements,
        },
    ))
}

fn parse_property(i: &str) -> IResult<&str, Property, ErrorTree<&str>> {
    let (i, _) = multispace0(i)?;
    let (i, id) = parse_identifier(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(':')(i)?;
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
