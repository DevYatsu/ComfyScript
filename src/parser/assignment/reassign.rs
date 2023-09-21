use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult, Parser};

fn parse_assignment_op(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alt((tag("="), tag("+="), tag("-="), tag("*="), tag("/="))).parse(input)
}
