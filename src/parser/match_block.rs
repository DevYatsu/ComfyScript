use std::fmt::Display;

use crate::parser::parse_statement;

use super::{
    ast::{identifier::Identifier, literal_value::LiteralValue, ASTNode},
    expression::parse_expression,
    parse_new_lines,
};
use nom::{
    character::complete::{char, multispace0, multispace1},
    multi::many0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pattern: MatchPattern,
    body: ASTNode, // ASTNode::BlockStatement
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchBlock {
    cases: Vec<MatchCase>,
}

#[derive(Debug, Clone, PartialEq)]
enum MatchPattern {
    LiteralValue(LiteralValue),
    Variable(Identifier),
    Ok(Box<MatchPattern>),
    Err(Box<MatchPattern>),
}

pub fn parse_match_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, _) = multispace1.cut().parse(input)?;

    let (input, test) = parse_expression.parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_match_block.cut().parse(input)?;

    let node = ASTNode::MatchStatement { test, body };

    Ok((input, node))
}

fn parse_match_block<'a>(input: &'a str) -> IResult<&'a str, MatchBlock, ErrorTree<&'a str>> {
    let (input, _) = char('{').cut().context("block").parse(input)?;

    todo!();

    let (input, _) = parse_new_lines.opt().parse(input)?;

    let (input, statements) = many0(parse_statement.delimited_by(parse_new_lines.opt()))
        .cut()
        .parse(input)?;

    let (input, _) = char('}')
        .opt_preceded_by(parse_new_lines)
        .cut()
        .context("block end")
        .parse(input)?;

    Ok((input, MatchBlock { cases: todo!() }))
}

impl Display for MatchBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for case in &self.cases {
            write!(f, "{},", case)?;
        }

        write!(f, "}}")
    }
}

impl Display for MatchCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pattern)?;
        write!(f, "=>",)?;

        write!(f, "{}", self.body)
        // always a block statement
    }
}

impl Display for MatchPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchPattern::LiteralValue(x) => write!(f, "{}", x),
            MatchPattern::Variable(id) => write!(f, "{}", id),
            MatchPattern::Ok(value) => write!(f, "Ok({})", value),
            MatchPattern::Err(value) => write!(f, "Err({})", value),
        }
    }
}
