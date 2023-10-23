use nom::{
    branch::alt, bytes::complete::take_until, character::complete::char, combinator::map, IResult,
    Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

use crate::parser::ast::{literal_value::LiteralValue, Expression};

#[derive(Debug, Clone, PartialEq)]
enum StringFragment {
    Literal(String),
    EscapedChar(char),
    EscapedWS,
} // to implement for strings in the future

pub fn parse_string(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, quote) = parse_quote(i)?;

    let (i, result) = take_until(&*quote.to_string()).cut().parse(i)?;
    let (i, c) = char(quote)(i)?;

    return Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Str(result.to_owned()),
            raw: c.to_string() + result + &c.to_string(),
        },
    ));
}

pub fn parse_unchecked_string(i: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let (i, quote) = parse_quote(i)?;

    let (i, result) = take_until(&*quote.to_string()).cut().parse(i)?;
    let (i, c) = char(quote)(i)?;

    return Ok((i, c.to_string() + result + &c.to_string()));
}

fn parse_quote(i: &str) -> IResult<&str, char, ErrorTree<&str>> {
    let simple_quote = '\'';
    let double_quote = '"';

    alt((
        map(char(double_quote), move |_| double_quote),
        map(char(simple_quote), move |_| simple_quote),
    ))(i)
}
