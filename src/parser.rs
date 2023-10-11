use nom::{branch::alt, bytes::complete::tag, combinator::opt, error::VerboseError, IResult};
use nom_locate::LocatedSpan;

use crate::parser::{import::parse_import, utils::parse_new_lines};

use self::{
    assignment::{initial::parse_var_init, reassign::parse_assignment},
    ast::ASTNode,
    function::{parse_function, return_expression::parse_return_statement},
    loop_for::parse_for_statement,
    loop_while::parse_while_statement, if_block::parse_if_statement,
};

mod assignment;
pub mod ast;
mod composite_types;
mod expression;
mod function;
mod if_block;
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
) -> IResult<Span<'a>, ASTNode, VerboseError<Span<'a>>> {
    let (input, _) = opt(parse_new_lines)(input)?;

    let mut statements = Vec::new();

    let (mut input, limit) = opt(tag(until))(input)?;

    if limit.is_some() {
        return Ok((input, ASTNode::BlockStatement { body: statements }));
    }

    while !input.is_empty() {
        let (new_input, statement) = parse_statement(input)?;
        statements.push(statement);

        let (new_input, _) = opt(parse_new_lines)(new_input)?;
        let (new_input, limit) = opt(tag(until))(new_input)?;

        input = new_input;
        if limit.is_some() {
            break;
        }
    }

    Ok((input, ASTNode::BlockStatement { body: statements }))
}

fn parse_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    alt((
        parse_var_init,
        parse_assignment,
        parse_import,
        parse_assignment,
        parse_for_statement,
        parse_while_statement,
        parse_if_statement,
        parse_function,
        parse_return_statement,
    ))(input)
}
