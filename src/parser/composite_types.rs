pub mod array;
pub mod object;

use self::{array::parse_array, object::parse_object};
use crate::parser::{ast::Expression, Span};
use nom::{branch::alt, error::VerboseError, IResult};

pub fn parse_composite_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((parse_array, parse_object))(i)
}
