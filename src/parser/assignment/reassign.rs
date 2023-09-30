use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult};

fn parse_assignment_op(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alt((tag("="), tag("+="), tag("-="), tag("*="), tag("/=")))(input)
}
