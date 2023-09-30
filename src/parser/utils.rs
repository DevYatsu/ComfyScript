use crate::reserved_keywords::RESERVED_KEYWORD;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace1},
    error::VerboseError,
    multi::many1,
    Err, IResult,
};

use super::Span;

pub fn parse_identifier(i: Span) -> IResult<Span, String, VerboseError<Span>> {
    let (mut i, word) = alt((alphanumeric1, tag("_")))(i)?;
    let mut word = word.fragment().to_string();

    loop {
        match alt((alphanumeric1::<Span, VerboseError<Span>>, tag("_")))(i) {
            Ok((input, s)) => {
                word.push_str(s.fragment());

                match alt((alphanumeric1::<Span, VerboseError<Span>>, tag("_")))(input) {
                    Ok((input, w)) => {
                        word.push_str(w.fragment());
                        i = input;
                    }
                    Err(_) => {
                        i = input;
                        break;
                    }
                };
            }
            Err(_) => break,
        }
    }

    if RESERVED_KEYWORD.contains(&word.as_str()) {
        Err(Err::Error(VerboseError { errors: vec![] })) // return an error
    } else {
        Ok((i, word))
    }
}

pub fn parse_new_lines(i: Span) -> IResult<Span, String, VerboseError<Span>> {
    let (i, words) = many1(alt((multispace1, tag(";"))))(i)?;

    let result: String = words
        .iter()
        .flat_map(|word| word.fragment().chars())
        .collect();

    Ok((i, result))
}
