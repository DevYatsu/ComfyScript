use nom::{bytes::complete::take_while, error::VerboseError, IResult};

use super::Span;

pub fn parse_new_lines(i: Span) -> IResult<Span, Span, VerboseError<Span>> {
    take_while(|c: char| c == ';' || c.is_ascii_whitespace())(i)
}
