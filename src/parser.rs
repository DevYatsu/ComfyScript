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
    character::complete::{alphanumeric0, char, space0},
    multi::many0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, final_parser::final_parser, tag::complete::tag, ParserExt};

pub fn parse_input(input: &str) -> Result<ASTNode, ErrorTree<&str>> {
    final_parser(parse_code)(input)
}

fn parse_code<'a>(input: &'a str) -> IResult<&'a str, ASTNode, ErrorTree<&'a str>> {
    let (input, _) = parse_new_lines.opt().parse(input)?;

    let (input, statements) = many0(parse_statement.terminated(parse_new_lines.opt()))
        .cut()
        .all_consuming()
        .parse(input)?;

    Ok((input, ASTNode::Program { body: statements }))
}

fn parse_block<'a>(input: &'a str) -> IResult<&'a str, ASTNode, ErrorTree<&'a str>> {
    let (input, _) = char('{')
        .opt_preceded_by(parse_new_lines)
        .cut()
        .context("block")
        .parse(input)?;

    let (input, _) = parse_new_lines.opt().parse(input)?;

    let (input, statements) = many0(parse_statement.delimited_by(parse_new_lines.opt()))
        .cut()
        .parse(input)?;

    let (input, _) = char('}')
        .opt_preceded_by(parse_new_lines)
        .cut()
        .context("block end")
        .parse(input)?;

    Ok((input, ASTNode::BlockStatement { body: statements }))
}

fn parse_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, found) = alt((
        alt((
            tag("//").complete(),
            tag("/*").complete(),
            tag(">>").complete(),
        )),
        alphanumeric0,
    ))
    .peek()
    .parse(input)?;

    let (input, statement) = match found {
        "let" | "var" => parse_var_init(input)?,
        "import" => parse_import(input)?,
        "fn" => parse_function(input)?,
        "if" => parse_if_statement(input)?,
        "for" => parse_for_statement(input)?,
        "while" => parse_while_statement(input)?,
        "return" | ">>" => parse_return_statement(input)?,
        "//" | "/*" => parse_comment_statement(input)?,
        _ => alt((parse_assignment, parse_expression_statement))(input)?,
    };

    Ok((input, statement))
}

fn parse_new_lines(i: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (i, _) = space0(i)?;
    let (i, removed) = take_while1(|c: char| c == ';' || c.is_ascii_whitespace()).parse(i)?;

    Ok((i, removed))
}
