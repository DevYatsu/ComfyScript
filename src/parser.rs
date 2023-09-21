use nom::{character::complete::multispace0, error::VerboseError, IResult};

use crate::parser::assignment::initial::parse_assignment;

mod assignment;
mod bool;
mod builtins;
mod numbers;
mod operations;
mod strings;

pub fn parse_input(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (input, _) = multispace0(input)?;

    let (input, assignment) = parse_assignment(input)?;

    println!("assignment {:?}", assignment);
    println!("{:?}", input);

    Ok((input, input))
}
