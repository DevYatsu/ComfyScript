use crate::reserved_keywords::RESERVED_KEYWORD;
use nom::{
    character::complete::alphanumeric1,
    error::{ContextError, VerboseError, VerboseErrorKind},
    IResult,
};

pub fn alpha_not_reserved(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (input, value) = alphanumeric1(input)?;

    if RESERVED_KEYWORD.contains(&value) {
        Err(nom::Err::Error(VerboseError::add_context(
            input,
            "Keyword reserved!",
            VerboseError {
                errors: vec![(
                    "Invalid input!",
                    VerboseErrorKind::Context("Invalid input!"),
                )],
            },
        )))
    } else {
        Ok((input, value))
    }
}
