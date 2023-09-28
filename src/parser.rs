use nom::{
    branch::alt,
    character::complete::multispace0,
    combinator::{map, opt},
    error::VerboseError,
    multi::{many0, many1},
    IResult,
};
use nom_supreme::tag::complete::tag;

use crate::parser::{assignment::initial::parse_assignment, import::parse_import};

use self::ast::ASTNode;

mod assignment;
mod ast;
mod bool;
mod for_loop;
mod import;
mod numbers;
mod operations;
mod strings;
mod utils;

pub fn parse_input(input: &str) -> IResult<&str, Vec<ASTNode>, VerboseError<&str>> {
    let (input, _) = multispace0(input)?;
    let mut assignments = Vec::new();

    let (mut input, assignment) = alt((parse_assignment, parse_import))(input)?;

    assignments.push(assignment);

    while !input.is_empty() {
        let (new_input, _) = many0(tag(" "))(input)?;

        let (new_input, _) = alt((tag("\n"), tag(";")))(new_input)?;

        let (new_input, _) = many0(alt((tag("\n"), tag(";"))))(new_input)?;

        let (new_input, assignment) = opt(alt((parse_assignment, parse_import)))(new_input)?;

        if let Some(assignment) = assignment {
            assignments.push(assignment);
        }

        input = new_input;
    }

    println!("{:?}", assignments);

    Ok((input, assignments))
}
