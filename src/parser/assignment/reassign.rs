use nom::{branch::alt, error::VerboseError, IResult, bytes::complete::tag};

fn parse_assignment_op(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alt((tag("="), tag("+="), tag("-="), tag("*="), tag("/=")))(input)
}
