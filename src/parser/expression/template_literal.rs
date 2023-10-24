use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, value},
    multi::fold_many0,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use nom_supreme::error::ErrorTree;

use crate::parser::ast::Expression;

use super::{
    parse_expression,
    strings::{parse_escaped_char, parse_escaped_whitespace, parse_literal},
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
    ))
    .parse(i)
}

fn parse_literal_expression(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    delimited(char('{'), parse_expression, char('}'))(i)
}
