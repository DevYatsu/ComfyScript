use std::fmt::Display;

use super::{
    ast::{BlockStatement, Expression, Statement, StatementKind},
    expression::parse_expression,
    parse_block,
};
use nom::{
    character::complete::{multispace0, multispace1},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement(Expression, BlockStatement, Option<OptionalBlock>);

#[derive(Debug, Clone, PartialEq)]
pub enum OptionalBlock {
    IfStatement(Box<IfStatement>),
    ElseStatement(BlockStatement),
}

pub fn parse_if_statement(input: &str) -> IResult<&str, IfStatement, ErrorTree<&str>> {
    let (input, (test, body)) = parse_if_block(input)?;

    let (else_input, _) = multispace0(input)?;
    let (else_input, else_word) = tag("else").complete().opt().parse(else_input)?;

    if else_word.is_none() {
        let if_statement = IfStatement(test, body, None);

        return Ok((input, if_statement));
    }

    let (else_input, _) = multispace0(else_input)?;
    let (input, other_if) = tag("if").complete().opt().parse(else_input)?;

    if other_if.is_none() {
        let (else_input, alternate) = parse_block
            .map(|s| Some(OptionalBlock::ElseStatement(s)))
            .parse(else_input)?;

        let if_statement = IfStatement(test, body, alternate);

        return Ok((else_input, if_statement));
    }

    let (input, alternate) = parse_if_statement
        .map(|s| Some(OptionalBlock::IfStatement(Box::new(s))))
        .parse(input)?;

    let node = IfStatement(test, body, alternate);

    Ok((input, node))
}

fn parse_if_block(input: &str) -> IResult<&str, (Expression, BlockStatement), ErrorTree<&str>> {
    let (input, _) = multispace1.cut().parse(input)?;

    let (input, test) = parse_expression.cut().parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_block.cut().parse(input)?;

    Ok((input, (test, body)))
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.0, self.1)?;

        if self.2.is_some() {
            write!(f, "{}", self.2.unwrap())?;
        }

        write!(f, "")
    }
}

impl Display for OptionalBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionalBlock::IfStatement(if_statement) => {
                write!(f, "{}", if_statement)
            }
            OptionalBlock::ElseStatement(block_statement) => {
                write!(f, "else {}", block_statement)
            }
        }
    }
}

impl Into<Statement> for IfStatement {
    fn into(self) -> Statement {
        Statement::with_kind(StatementKind::IfStatement(self))
    }
}
