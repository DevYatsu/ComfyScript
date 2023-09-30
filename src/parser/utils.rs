use crate::reserved_keywords::RESERVED_KEYWORD;
use nom::{character::complete::alphanumeric1, error::VerboseError, Err, IResult};

use super::Span;

pub fn alpha_not_reserved(input: Span) -> IResult<Span, Span, VerboseError<Span>> {
    let (input, value) = alphanumeric1(input)?;

    if RESERVED_KEYWORD.contains(&value) {
        Err(Err::Error(VerboseError { errors: vec![] })) // return an error
    } else {
        Ok((input, value))
    }
}
