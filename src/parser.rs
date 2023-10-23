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
    assignment::{
        initial::{parse_var_init, VariableKeyword},
        reassign::parse_assignment,
    },
    ast::ASTNode,
    comment::{parse_line_comment, parse_multiline_comment},
    expression::{parse_expression, parse_expression_statement},
    function::parse_function,
    if_block::parse_if_statement,
    loop_for::parse_for_statement,
    loop_while::parse_while_statement,
};
use crate::parser::import::parse_import;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alphanumeric0, char, multispace0, space0},
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
    println!("block {}", input);

    let (input, statements) = many0(parse_statement.delimited_by(parse_new_lines.opt()))
        .cut()
        .parse(input)?;
    println!("afterblock {}", input);

    let (input, _) = char('}')
        .opt_preceded_by(parse_new_lines)
        .cut()
        .context("block end")
        .parse(input)?;

    Ok((input, ASTNode::BlockStatement { body: statements }))
}

fn parse_statement(initial_input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, found) = alt((
        alphanumeric0,
        alt((
            tag("//").complete(),
            tag("/*").complete(),
            tag(">>").complete(),
        )),
    ))
    .parse(initial_input)?;
    println!("input {:?}", found);

    let (input, statement) = match found {
        "let" | "var" => {
            let kind = match found {
                "let" => VariableKeyword::Let,
                _ => VariableKeyword::Var,
            };
            println!("working");

            let (input, declarations) = parse_var_init(input)?;
            println!("working");

            (input, ASTNode::VariableDeclaration { declarations, kind })
        }
        "import" => parse_import(input)?,
        "fn" => parse_function(input)?,
        "if" => parse_if_statement(input)?,
        "for" => parse_for_statement(input)?,
        "while" => parse_while_statement(input)?,
        "return" | ">>" => {
            let is_shortcut = match found {
                ">>" => true,
                _ => false,
            };

            let (input, _) = multispace0(input)?;

            let (input, argument) = parse_expression.cut().parse(input)?;

            (
                input,
                ASTNode::ReturnStatement {
                    argument,
                    is_shortcut,
                },
            )
        }
        "//" => {
            let (input, expression) = parse_line_comment(input)?;

            (input, ASTNode::ExpressionStatement { expression })
        }
        "/*" => {
            println!("mutliline input {:?}", input);

            let (input, expression) = parse_multiline_comment(input)?;

            (input, ASTNode::ExpressionStatement { expression })
        }
        _ => {
            println!("expr_statement {:?}", initial_input);

            alt((parse_assignment, parse_expression_statement))(initial_input)?
        }
    };

    Ok((input, statement))
}

fn parse_new_lines(i: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (i, _) = space0(i)?;
    let (i, removed) = take_while1(|c: char| c == ';' || c.is_ascii_whitespace()).parse(i)?;

    Ok((i, removed))
}
