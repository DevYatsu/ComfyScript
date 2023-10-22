use nom::bytes::complete::{take, take_until1};
use nom::number::complete::float;
use nom::Parser;
use nom::{branch::alt, character::complete::char, combinator::opt, IResult};
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

use crate::parser::ast::literal_value::LiteralValue;
use crate::parser::ast::Expression;

pub fn parse_number(initial_i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (base_input, sign) = opt(alt((char('+'), char('-'))))(initial_i)?;

    // is normal to avoid having things such as Infinity considered numbers
    let (i, num) = float
        .verify(|num| num.is_normal())
        .context("number")
        .parse(base_input)?;
    let (_, other_dot) = opt(char('.'))(i)?;

    // check in case a range is following
    let (i, num) = if other_dot.is_some() {
        let (i, num_string) = take_until1(".")(base_input)?;
        (i, num_string.parse::<f32>().unwrap())
    } else {
        (i, num)
    };

    let (_, raw) = take((initial_i.len() - i.len()) as usize)(initial_i)?;

    let num = match sign {
        Some('-') => -num,
        _ => num,
    };

    Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Number(num),
            raw: raw.to_owned(),
        },
    ))
}
