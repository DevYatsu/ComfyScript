use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, 
    IResult,
};
use nom_supreme::error::ErrorTree;

use crate::parser::{ast::ASTNode, expression::parse_expression, Span};

pub fn parse_return_statement(i: Span) -> IResult<Span, ASTNode, ErrorTree<Span>> {
    let (i, return_keyword) = alt((tag(">>"), tag("return")))(i)?;
    let is_shortcut = return_keyword.to_string().as_str() == ">>";

    let (i, _) = multispace0(i)?;

    let (i, argument) = parse_expression(i)?;

    Ok((
        i,
        ASTNode::ReturnStatement {
            argument,
            is_shortcut,
        },
    ))
}
