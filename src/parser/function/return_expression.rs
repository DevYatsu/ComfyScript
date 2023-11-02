use std::fmt::Display;

use nom::{branch::alt, character::complete::multispace0, IResult, Parser};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use crate::parser::{
    ast::{Expression, Statement, StatementKind},
    expression::parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
/// Expression: argument
/// bool: is_shortcut
pub struct ReturnStatement(pub Expression, pub bool);

pub fn parse_return_statement(i: &str) -> IResult<&str, ReturnStatement, ErrorTree<&str>> {
    let (i, return_keyword) = alt((tag(">>").complete(), tag("return").complete())).parse(i)?;
    let is_shortcut = return_keyword.to_string().as_str() == ">>";

    let (i, _) = multispace0(i)?;

    let (i, argument) = parse_expression.cut().parse(i)?;

    let return_expr = ReturnStatement(argument, is_shortcut);

    Ok((i, return_expr))
}

impl Into<Statement> for ReturnStatement {
    fn into(self) -> Statement {
        Statement::with_kind(StatementKind::ReturnStatement(self))
    }
}
impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 {
            write!(f, ">> ")?;
        } else {
            write!(f, "return ")?;
        }

        write!(f, "{}", self.0)
    }
}
