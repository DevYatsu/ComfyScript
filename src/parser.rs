use nom::{
    branch::alt, bytes::complete::tag, combinator::opt, error::VerboseError, multi::many0,
    sequence::terminated, IResult,
};
use nom_locate::LocatedSpan;

use crate::parser::{import::parse_import, utils::parse_new_lines};

use self::{
    assignment::{initial::parse_var_init, reassign::parse_assignment},
    ast::ASTNode,
    loop_for::parse_for_statement,
    loop_while::parse_while_statement,
};

mod assignment;
pub mod ast;
mod composite_types;
mod expression;
mod import;
mod loop_for;
mod loop_while;
mod operations;
mod parenthesized;
mod primitive_values;
mod utils;

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse_input<'a>(input: Span<'a>) -> IResult<Span, Vec<ASTNode>, VerboseError<Span>> {
    let (mut input, _) = opt(parse_new_lines)(input)?;

    let mut statements = Vec::new();

    while !input.is_empty() {
        let (new_input, statement) = parse_statement(input)?;
        statements.push(statement);

        let (new_input, _) = opt(parse_new_lines)(new_input)?;
        input = new_input;

        if new_input.len() == 0 {
            break;
        }
    }

    for statement in &statements {
        println!("{:?}", statement);
    }
    Ok((input, statements))
}

pub fn parse_block<'a>(
    input: Span<'a>,
    until: &'static str,
) -> IResult<Span<'a>, Vec<ASTNode>, VerboseError<Span<'a>>> {
    let (input, _) = opt(parse_new_lines)(input)?;
    let (input, statements) = many0(terminated(parse_statement, parse_new_lines))(input)?;

    let (input, _) = tag(until)(input)?;

    Ok((input, statements))
}

fn parse_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    alt((
        parse_var_init,
        parse_assignment,
        parse_import,
        parse_assignment,
        parse_for_statement,
        parse_while_statement,
    ))(input)
}
