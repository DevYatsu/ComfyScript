pub mod bool;
pub mod nil;
pub mod numbers;
pub mod strings;

use nom::{branch::alt, error::VerboseError, IResult};

use crate::parser::{ast::Expression, Span};

use self::{bool::parse_bool, nil::parse_nil, numbers::parse_number, strings::parse_string};

pub fn parse_primitive_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((parse_string, parse_bool, parse_number, parse_nil))(i)
}
