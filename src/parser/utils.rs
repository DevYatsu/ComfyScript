use nom::{bytes::complete::take_while1, error::VerboseError, IResult};

use super::Span;

pub fn parse_new_lines(i: Span) -> IResult<Span, Span, VerboseError<Span>> {
    let (i, removed) = take_while1(|c: char| c == ';' || c.is_ascii_whitespace())(i)?;

    Ok((i, removed))
}
