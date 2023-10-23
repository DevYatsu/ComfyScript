use nom::{character::complete::char, IResult};
use nom_supreme::error::ErrorTree;

use crate::parser::ast::Expression;

#[derive(Debug, Clone, PartialEq)]
enum TemplateLiteralFragment {
    Literal(String),
    EscapedChar(char),
    Expression(Expression),
}

pub fn parse_template_literal(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _hashtag) = char('#')(i)?;

    todo!()
}
