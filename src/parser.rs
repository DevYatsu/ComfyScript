use nom::{character::complete::multispace0, error::VerboseError, IResult};

use crate::parser::assignment::initial::parse_assignment;

mod assignment;
mod ast;
mod bool;
mod for_loop;
mod numbers;
mod operations;
mod strings;

pub fn parse_input(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let mut remaining_input = input;
    let mut assignments = Vec::new();

    while !remaining_input.is_empty() {
        let (new_input, _) = multispace0(remaining_input)?;
        let (new_input, assignment) = parse_assignment(new_input)?;

        remaining_input = new_input;
        assignments.push(assignment);
    }

    println!("{:?}", assignments);

    Ok((remaining_input, input))
}