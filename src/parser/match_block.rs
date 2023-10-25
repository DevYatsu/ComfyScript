use std::fmt::Display;

use super::{
    ast::{
        identifier::{parse_identifier, Identifier},
        literal_value::LiteralValue,
        ASTNode,
    },
    expression::{
        numbers::parse_number_literal_value, parse_expression, strings::parse_string_literal_value,
    },
    parse_block,
};
use nom::{
    branch::alt,
    character::complete::{char, multispace0, multispace1},
    multi::many0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

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
pub enum MatchPattern {
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

fn parse_match_block(input: &str) -> IResult<&str, MatchBlock, ErrorTree<&str>> {
    let (input, _) = char('{').cut().context("match block").parse(input)?;

    let (input, cases) = many0(parse_match_case.delimited_by(multispace0))
        .cut()
        .parse(input)?;

    let (input, _) = char('}')
        .opt_preceded_by(multispace0) // to remove useless i believe
        .cut()
        .context("match block end")
        .parse(input)?;

    Ok((input, MatchBlock { cases }))
}

fn parse_match_case(input: &str) -> IResult<&str, MatchCase, ErrorTree<&str>> {
    let (input, pattern) = parse_match_pattern(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=>").complete().parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_block(input)?;

    Ok((input, MatchCase { pattern, body }))
}

fn parse_match_pattern(input: &str) -> IResult<&str, MatchPattern, ErrorTree<&str>> {
    alt((
        parse_ok_pattern_variant,
        parse_err_pattern_variant,
        parse_identifier.map(|id| MatchPattern::Variable(id)),
        tag("nil")
            .complete()
            .value(MatchPattern::LiteralValue(LiteralValue::Nil)),
        tag("true")
            .complete()
            .value(MatchPattern::LiteralValue(LiteralValue::Boolean(true))),
        tag("false")
            .complete()
            .value(MatchPattern::LiteralValue(LiteralValue::Boolean(false))),
        parse_number_literal_value.map(|num| MatchPattern::LiteralValue(num)),
        parse_string_literal_value.map(|s| MatchPattern::LiteralValue(s)),
    ))(input)
}

fn parse_ok_pattern_variant(input: &str) -> IResult<&str, MatchPattern, ErrorTree<&str>> {
    let (input, _) = tag("Ok(").complete().parse(input)?;
    let (input, id) = parse_identifier
        .map(|id| MatchPattern::Ok(Box::new(MatchPattern::Variable(id))))
        .parse(input)?;
    let (input, _) = char(')').parse(input)?;

    Ok((input, MatchPattern::Ok(Box::new(id))))
}
fn parse_err_pattern_variant(input: &str) -> IResult<&str, MatchPattern, ErrorTree<&str>> {
    let (input, _) = tag("Err(").complete().parse(input)?;
    let (input, id) = parse_identifier
        .map(|id| MatchPattern::Err(Box::new(MatchPattern::Variable(id))))
        .parse(input)?;
    let (input, _) = char(')').parse(input)?;

    Ok((input, MatchPattern::Ok(Box::new(id))))
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
