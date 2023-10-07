use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, error::VerboseError,
    IResult,
};

use crate::parser::{
    ast::{literal_value::LiteralValue, Expression},
    composite_types::object::parse_object,
    Span,
};

use super::{composite_types::array::parse_array, primitive_values::bool::parse_bool};
// parsing expressions
pub fn parse_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, expr) = alt((parse_bool, parse_array, parse_object))(i)?;
    // parse any operator following
    let (i, _) = multispace0(i)?;

    todo!();
    Ok((i, Expression::Array { elements: vec![] }))
}
