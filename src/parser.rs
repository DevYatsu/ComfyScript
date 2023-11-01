pub mod assignment;
pub mod ast;
pub mod comment;
mod data_type;
pub mod errors;
pub mod expression;
pub mod function;
mod if_block;
mod import;
mod loop_for;
mod loop_while;
mod match_block;
pub mod operations;
mod reserved;

use self::{
    assignment::{
        initial::{parse_var_init, VariableKeyword},
        reassign::parse_assignment,
    },
    ast::{BlockStatement, Program, Statement},
    comment::{parse_line_comment, parse_multiline_comment},
    expression::{parse_expression, parse_expression_statement},
    function::{parse_function, return_expression::ReturnStatement},
    if_block::parse_if_statement,
    loop_for::parse_for_statement,
    loop_while::parse_while_statement,
    match_block::parse_match_statement,
};
use crate::parser::import::parse_import;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alphanumeric0, char, multispace0, multispace1, space0},
    multi::many0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, final_parser::final_parser, tag::complete::tag, ParserExt};

pub fn parse_input(input: &str) -> Result<Program, ErrorTree<&str>> {
    final_parser(parse_code)(input)
}

fn parse_code<'a>(input: &'a str) -> IResult<&'a str, Program, ErrorTree<&'a str>> {
    let (input, _) = parse_new_lines.opt().parse(input)?;

    let (input, statements) = many0(parse_statement.terminated(parse_new_lines.opt()))
        .cut()
        .all_consuming()
        .parse(input)?;

    Ok((input, Program { body: statements }))
}

fn parse_block<'a>(input: &'a str) -> IResult<&'a str, BlockStatement, ErrorTree<&'a str>> {
    let (input, _) = char('{').cut().context("block").parse(input)?;

    let (input, _) = parse_new_lines.opt().parse(input)?;

    let (input, statements) = many0(parse_statement.delimited_by(parse_new_lines.opt()))
        .cut()
        .parse(input)?;

    let (input, _) = char('}')
        .opt_preceded_by(multispace0)
        .cut()
        .context("block end")
        .parse(input)?;

    Ok((input, BlockStatement { body: statements }))
}

fn parse_statement(initial_input: &str) -> IResult<&str, Statement, ErrorTree<&str>> {
    let (input, found) = alt((
        tag("//").complete(),
        tag("/*").complete(),
        tag(">>").complete(),
        alphanumeric0,
    ))
    .parse(initial_input)?;

    let (input, statement) = match found {
        "let" | "var" => {
            let kind = match found {
                "let" => VariableKeyword::Let,
                _ => VariableKeyword::Var,
            };

            let (input, declarations) = parse_var_init(input)?;

            (input, (kind, declarations).into())
        }
        "import" => parse_import(input)?,
        "export" => {
            let (input, _) = multispace1(input)?;
            let (input, _) = tag("fn")
                .complete()
                .cut()
                .context("expected")
                .parse(input)?;

            parse_function(true)(input)?
        }
        "fn" => parse_function(false)(input)?,
        "if" => {
            let (i, if_statement) = parse_if_statement(input)?;

            (i, if_statement.into())
        }
        "for" => parse_for_statement(input)?,
        "while" => parse_while_statement(input)?,
        "match" => parse_match_statement(input)?,
        "return" | ">>" => {
            let is_shortcut = match found {
                ">>" => true,
                _ => false,
            };

            let (input, _) = multispace0(input)?;

            let (input, argument) = parse_expression.cut().parse(input)?;

            (input, (ReturnStatement(argument, is_shortcut)).into())
        }
        "//" => {
            let (input, expression) = parse_line_comment(input)?;

            (input, expression.into())
        }
        "/*" => {
            let (input, expression) = parse_multiline_comment(input)?;

            (input, expression.into())
        }
        _ => alt((parse_assignment, parse_expression_statement))(initial_input)?,
    };

    Ok((input, statement))
}

fn parse_new_lines(i: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (i, _) = space0(i)?;
    let (i, removed) = take_while1(|c: char| c == ';' || c == '\n').parse(i)?;
    let (i, _) = space0(i)?;

    Ok((i, removed))
}
