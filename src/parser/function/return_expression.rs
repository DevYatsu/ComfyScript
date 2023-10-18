use nom::{branch::alt, character::complete::multispace0, IResult, Parser};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use crate::{parser::{ast::ASTNode, expression::parse_expression, errors::expected_expression}, expected_keyword};

pub fn parse_return_statement(i: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (i, return_keyword) = alt((tag(">>"), tag("return"))).context(expected_keyword!(">>")).parse(i)?;
    let is_shortcut = return_keyword.to_string().as_str() == ">>";

    let (i, _) = multispace0(i)?;

    let (i, argument) = parse_expression.context(expected_expression()).parse(i)?;

    Ok((
        i,
        ASTNode::ReturnStatement {
            argument,
            is_shortcut,
        },
    ))
}
