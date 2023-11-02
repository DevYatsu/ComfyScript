use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, multispace0},
    combinator::{map, value, verify},
    multi::many0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use crate::parser::ast::{Expression, ExpressionKind};

use super::{
    parse_expression1,
    strings::{parse_escaped_char, parse_escaped_whitespace},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateLiteralFragment {
    Literal(String),
    EscapedChar(char),
    Expression(Expression),
    EscapedWS,
}

pub fn parse_template_literal(initial_i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = char('#')(initial_i)?;
    let (i, _) = char('"')(i)?;
    let (i, fragments) = build_template_literal(i)?;
    let (i, _) = char('"')(i)?;

    let result_str = Expression::with_kind(ExpressionKind::TemplateLiteral(
        fragments,
        initial_i[0..initial_i.len() - i.len()].to_string(),
    ));

    Ok((i, result_str))
}

fn build_template_literal(i: &str) -> IResult<&str, Vec<TemplateLiteralFragment>, ErrorTree<&str>> {
    many0(parse_literal_fragment)(i)
}

fn parse_literal_fragment(i: &str) -> IResult<&str, TemplateLiteralFragment, ErrorTree<&str>> {
    alt((
        tag("{{")
            .complete()
            .map(|_| TemplateLiteralFragment::EscapedChar('{')),
        map(parse_literal, TemplateLiteralFragment::Literal),
        map(parse_escaped_char, TemplateLiteralFragment::EscapedChar),
        value(TemplateLiteralFragment::EscapedWS, parse_escaped_whitespace),
        map(
            parse_literal_expression,
            TemplateLiteralFragment::Expression,
        ),
    ))
    .parse(i)
}

pub fn parse_literal(i: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let not_quote_slash = is_not("\"\\{");

    map(
        verify(not_quote_slash, |s: &str| !s.is_empty()),
        |x: &str| x.to_string(),
    )
    .parse(i)
}

fn parse_literal_expression(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = char('{').terminated(multispace0).parse(i)?;

    let (i, expr) = parse_expression1.parse(i)?;

    let (i, _) = char('}').preceded_by(multispace0).parse(i)?;

    Ok((i, expr))
}

impl Display for TemplateLiteralFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateLiteralFragment::Literal(s) => write!(f, "{}", s),
            TemplateLiteralFragment::EscapedChar(escaped) => write!(f, "{}", escaped),
            TemplateLiteralFragment::Expression(expr) => write!(f, "{{{}}}", expr),
            TemplateLiteralFragment::EscapedWS => write!(f, ""),
        }
    }
}

impl TemplateLiteralFragment {
    pub fn is_empty(&self) -> bool {
        match self {
            TemplateLiteralFragment::Literal(s) => s.is_empty(),
            _ => false,
        }
    }
}
