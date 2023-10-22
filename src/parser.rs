mod assignment;
pub mod ast;
pub mod comment;
pub mod errors;
pub mod expression;
mod function;
mod if_block;
mod import;
mod loop_for;
mod loop_while;
mod operations;

use self::{
    assignment::{initial::parse_var_init, reassign::parse_assignment},
    ast::ASTNode,
    comment::parse_comment_statement,
    expression::parse_expression_statement,
    function::{parse_function, return_expression::parse_return_statement},
    if_block::parse_if_statement,
    loop_for::parse_for_statement,
    loop_while::parse_while_statement,
};
use crate::parser::import::parse_import;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::char,
    multi::{many0, separated_list0},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, final_parser::final_parser, ParserExt};

pub fn parse_input(input: &str) -> Result<ASTNode, ErrorTree<&str>> {
    final_parser(parse_code)(input)
}

fn parse_code<'a>(input: &'a str) -> IResult<&'a str, ASTNode, ErrorTree<&'a str>> {
    let (input, statements) = many0(
        parse_statement
            .preceded_by(parse_new_lines.opt())
            .terminated(parse_new_lines.opt()),
    )
    .cut()
    .all_consuming()
    .parse(input)?;

    Ok((input, ASTNode::Program { body: statements }))
}

fn parse_block<'a>(input: &'a str) -> IResult<&'a str, ASTNode, ErrorTree<&'a str>> {
    let (input, statements) = separated_list0(parse_new_lines.opt(), parse_statement)
        .preceded_by(
            char('{')
                .terminated(parse_new_lines.opt())
                .cut()
                .context("block"),
        )
        .cut()
        .parse(input)?;

    let (input, _) = char('}')
        .preceded_by(parse_new_lines.opt())
        .cut()
        .parse(input)?;

    Ok((input, ASTNode::BlockStatement { body: statements }))
}

fn parse_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    alt((
        parse_var_init,
        parse_assignment,
        parse_import,
        parse_for_statement,
        parse_while_statement,
        parse_if_statement,
        parse_function,
        parse_return_statement,
        parse_comment_statement,
        parse_expression_statement,
    ))
    .parse(input)
}

fn parse_new_lines(i: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (i, removed) = take_while1(|c: char| c == ';' || c.is_ascii_whitespace())(i)?;

    Ok((i, removed))
}
