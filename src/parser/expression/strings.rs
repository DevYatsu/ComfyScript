use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::{is_not, take_while_m_n},
    character::complete::{char, multispace1},
    combinator::{map, map_opt, value, verify},
    sequence::{delimited, preceded},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

use crate::parser::ast::{literal_value::LiteralValue, Expression};
use nom::multi::fold_many0;

#[derive(Debug, Clone, PartialEq)]
pub enum StringFragment {
    Literal(String),
    EscapedChar(char),
    EscapedWS,
}

pub fn parse_string(initial_i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, fragments) = delimited(char('"'), build_string, char('"')).parse(initial_i)?;

    let result_str = Expression::Literal {
        value: LiteralValue::Str(fragments),
        raw: initial_i[0..initial_i.len() - i.len()].to_string(),
    };

    Ok((i, result_str))
}

pub fn parse_raw_string(initial_i: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let (i, _) = parse_string(initial_i)?;

    let final_str = initial_i[0..(initial_i.len() - i.len())].to_owned();

    return Ok((i, final_str));
}

fn build_string(i: &str) -> IResult<&str, Vec<StringFragment>, ErrorTree<&str>> {
    fold_many0(parse_fragment, Vec::new, |mut str_vec, fragment| {
        str_vec.push(fragment);
        str_vec
    })(i)
}

pub fn parse_unicode(i: &str) -> IResult<&str, char, ErrorTree<&str>> {
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));

    let parse_u32 = parse_delimited_hex.map_res(|hex| u32::from_str_radix(hex, 16));

    map_opt(parse_u32, std::char::from_u32).parse(i)
}

pub fn parse_escaped_char(i: &str) -> IResult<&str, char, ErrorTree<&str>> {
    preceded(
        char('\\'),
        alt((
            parse_unicode,
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('"', char('"')),
        )),
    )
    .parse(i)
}

pub fn parse_escaped_whitespace(i: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    preceded(char('\\'), multispace1.context("unknown char escape").cut()).parse(i)
}

fn parse_literal(i: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let not_quote_slash = is_not("\"\\");

    map(
        verify(not_quote_slash, |s: &str| !s.is_empty()),
        |x: &str| x.to_string(),
    )
    .parse(i)
}

fn parse_fragment(i: &str) -> IResult<&str, StringFragment, ErrorTree<&str>> {
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
        value(StringFragment::EscapedWS, parse_escaped_whitespace),
    ))
    .parse(i)
}

pub fn parse_string_literal_value(i: &str) -> IResult<&str, LiteralValue, ErrorTree<&str>> {
    let (base_input, s) = build_string(i)?;

    Ok((i, LiteralValue::Str(s)))
}

impl Display for StringFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringFragment::Literal(s) => write!(f, "{}", s),
            StringFragment::EscapedChar(escaped) => write!(f, "{}", escaped),
            StringFragment::EscapedWS => write!(f, ""),
        }
    }
}
