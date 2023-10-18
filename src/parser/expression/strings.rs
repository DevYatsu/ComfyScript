use nom::{
    branch::alt, bytes::complete::take_until1, character::complete::char, combinator::map, IResult, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

use crate::{parser::ast::{literal_value::LiteralValue, Expression}, expected};

pub fn parse_string(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, quote) = parse_quote(i)?;

    let (i, result) = take_until1(&*quote.to_string()).context(expected!("an ending quote")).cut().parse(i)?;
    let (i, c) = char(quote)(i)?;

    return Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Str(result.to_owned()),
            raw: c.to_string() + result + &c.to_string(),
        },
    ));
}

fn parse_quote(i: &str) -> IResult<&str, char, ErrorTree<&str>> {
    let simple_quote = '\'';
    let double_quote = '"';

    alt((
        map(char(double_quote), move |_| double_quote),
        map(char(simple_quote), move |_| simple_quote),
    ))(i)
}
