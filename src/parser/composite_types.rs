pub mod array;
pub mod object;
use nom::{
    branch::alt,
    error::VerboseError,
    IResult,
};

use crate::parser::{ast::Expression, Span};

use self::{array::parse_array, object::parse_object};

pub fn parse_composite_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((parse_array, parse_object))(i)
}
