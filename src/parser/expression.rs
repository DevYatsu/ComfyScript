use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult, character::complete::multispace0};

use crate::parser::{
    ast::{literal_value::LiteralValue, Expression},
    Span, composite_types::object::parse_object,
};

use super::{primitive_values::bool::parse_bool, composite_types::array::parse_array};
// parsing expressions
pub fn parse_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, expr) = alt((parse_bool, parse_array, parse_object))(i)?;
// parse any operator following
    let (i, _) = multispace0(i)?;


    todo!();
    Ok((
        i,
        Expression::Array { elements: vec![] },
    ))
}
