use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::opt,
    error::VerboseError, IResult,
};
use nom_locate::LocatedSpan;

use crate::parser::{import::parse_import, utils::parse_new_lines};

use self::{
    assignment::{initial::parse_var_init, reassign::parse_assignment},
    ast::ASTNode,
};

mod assignment;
pub mod ast;
mod composite_types;
mod expression;
mod for_loop;
mod import;
mod operations;
mod primitive_values;
mod utils;

type Span<'a> = LocatedSpan<&'a str>;

pub fn parse_input<'a>(
    input: Span<'a>,
    until: Option<&'static str>,
) -> IResult<Span<'a>, Vec<ASTNode>, VerboseError<Span<'a>>> {
    let (input, _) = multispace0(input)?;
    let (mut input, _) = opt(parse_new_lines)(input)?;

    let mut statements = Vec::new();

    while !input.is_empty() {
        if let Some(limit) = until {
            let (new_input, opt) = opt(tag(limit))(input)?;

            if let Some(_) = opt {
                input = new_input;
                break;
            }
        }

        let (new_input, statement) = parse_statement(input)?;
        statements.push(statement);

        if new_input.len() != 0 {
            let (new_input, _) = parse_new_lines(new_input)?;

            input = new_input;
        } else {
            break;
        }
    }

    for statement in &statements {
        println!("{:?}", statement);
    }

    Ok((input, statements))
}

fn parse_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    alt((parse_var_init, parse_import, parse_assignment))(input)
}
