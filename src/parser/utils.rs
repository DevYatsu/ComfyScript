use crate::reserved_keywords::RESERVED_KEYWORD;
use nom::{
    character::complete::{alphanumeric1},
    error::VerboseError,
    Err, IResult, bytes::complete::tag, branch::alt,
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
