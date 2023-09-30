use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, error::VerboseError,
    multi::many0, IResult,
};
use nom_locate::LocatedSpan;

use crate::parser::{
    assignment::initial::parse_assignment, import::parse_import, utils::parse_new_lines,
};

use self::ast::ASTNode;

mod assignment;
pub mod ast;
mod for_loop;
mod import;
mod operations;
mod primitive_values;
mod utils;

type Span<'a> = LocatedSpan<&'a str>;

pub fn parse_input(input: &str) -> IResult<Span, Vec<ASTNode>, VerboseError<Span>> {
    let input = Span::new(input);
    let (input, _) = multispace0(input)?;
    let (mut input, _) = many0(tag(";"))(input)?;

    let mut statements = Vec::new();

    while !input.is_empty() {
        let (new_input, statement) = alt((parse_assignment, parse_import))(input)?;
        
        if new_input.len() != 0 {
            let (new_input, _) = parse_new_lines(new_input)?;
            statements.push(statement);

            input = new_input;
        } else {
            break;
        }
    }

    println!("{:?}", statements);

    Ok((input, statements))
}

fn parse_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    alt((parse_assignment, parse_import))(input)
}
