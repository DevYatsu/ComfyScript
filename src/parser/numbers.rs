use nom::error::VerboseError;
use nom::number::complete::float;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map_res, opt},
    IResult,
};
use std::str;

use super::ast::literal_value::LiteralValue;
use super::ast::Expression;

fn parse_integer(i: &str) -> IResult<&str, f32, VerboseError<&str>> {
    let (i, sign) = opt(alt((char('+'), char('-'))))(i)?;
    let (i, integer) = map_res(digit1, str::parse::<f32>)(i)?;

    let integer_value = match sign {
        Some('-') => -integer,
        _ => integer,
    };

    Ok((i, integer_value))
}

fn parse_float(i: &str) -> IResult<&str, f32, VerboseError<&str>> {
    float(i)
}

pub fn parse_number(i: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (i, num) = alt((parse_float, parse_integer))(i)?;

    Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Number(num),
            raw: num.to_string(),
        },
    ))
}
