use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::opt,
    error::VerboseError, multi::many0, IResult,
};
use nom_locate::LocatedSpan;

use crate::parser::{assignment::initial::parse_assignment, import::parse_import};

use self::ast::ASTNode;

mod assignment;
mod ast;
mod for_loop;
mod import;
mod operations;
mod primitive_values;
mod utils;

type Span<'a> = LocatedSpan<&'a str>;

pub fn parse_input(input: &str) -> IResult<Span, Vec<ASTNode>, VerboseError<Span>> {
    let input = Span::new(input);

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
