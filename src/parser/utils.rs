use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace1, error::VerboseError,
    multi::many1, IResult,
};

use super::Span;

pub fn parse_new_lines(i: Span) -> IResult<Span, String, VerboseError<Span>> {
    let (i, separators) = many1(alt((multispace1, tag(";"))))(i)?;

    let result: String = separators
        .iter()
        .flat_map(|word| word.fragment().chars())
        .collect();

    Ok((i, result))
}
