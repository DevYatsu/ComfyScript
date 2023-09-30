use crate::reserved_keywords::RESERVED_KEYWORD;
use nom::{
    character::complete::{alphanumeric1, char as parse_char},
    error::VerboseError,
    Err, IResult,
};

use super::Span;

pub fn parse_identifier(i: Span) -> IResult<Span, String, VerboseError<Span>> {
    let (mut i, word) = alphanumeric1(i)?;
    let mut word = word.fragment().to_string();

    loop {
        match parse_char::<Span, VerboseError<Span>>('_')(i) {
            Ok((input, c)) => {
                word.push(c);

                match alphanumeric1::<Span, VerboseError<Span>>(input) {
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
