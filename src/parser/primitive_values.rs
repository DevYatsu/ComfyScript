pub mod bool;
pub mod nil;
pub mod numbers;
pub mod strings;

use self::{bool::parse_bool, nil::parse_nil, numbers::parse_number, strings::parse_string};
use crate::parser::{ast::Expression, Span};
use nom::{branch::alt, error::VerboseError, IResult};

pub fn parse_primitive_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((parse_string, parse_bool, parse_number, parse_nil))(i)
}
