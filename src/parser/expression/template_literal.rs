use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, multispace0},
    combinator::{map, value, verify},
    multi::fold_many0,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

use crate::parser::ast::Expression;

use super::{
    parse_expression,
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
    let (i, fragments) = preceded(
        char('#'),
        delimited(char('"'), build_template_literal, char('"')),
    )
    .parse(initial_i)?;

    let result_str = Expression::TemplateLiteral {
        raw: initial_i[0..initial_i.len() - i.len()].to_string(),
        value: fragments,
    };

    Ok((i, result_str))
}

fn build_template_literal(i: &str) -> IResult<&str, Vec<TemplateLiteralFragment>, ErrorTree<&str>> {
    fold_many0(parse_literal_fragment, Vec::new, |mut str_vec, fragment| {
        str_vec.push(fragment);
        str_vec
    })(i)
}

fn parse_literal_fragment(i: &str) -> IResult<&str, TemplateLiteralFragment, ErrorTree<&str>> {
    alt((
        map(
            parse_literal_expression,
            TemplateLiteralFragment::Expression,
        ),
        map(parse_literal, TemplateLiteralFragment::Literal),
        map(parse_escaped_char, TemplateLiteralFragment::EscapedChar),
        value(TemplateLiteralFragment::EscapedWS, parse_escaped_whitespace),
        tag("{").map(|value: &str| TemplateLiteralFragment::Literal(value.to_owned())),
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
    delimited(terminated(char('{'), multispace0), parse_expression, preceded(multispace0, char('}')))(i)
}
