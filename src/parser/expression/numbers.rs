use nom::bytes::complete::{take, take_until1};
use nom::character::complete::alphanumeric1;
use nom::number::complete::float;
use nom::Parser;
use nom::{character::complete::char, IResult};
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

use crate::parser::ast::literal_value::LiteralValue;
use crate::parser::ast::Expression;

pub fn parse_number(initial_i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    // is normal to avoid having things such as Infinity considered numbers
    let (i, num) = float.verify(|num| num.is_normal()).parse(initial_i)?;
    let (_, other_dot) = char('.').opt().parse(i)?;
    let (_, member_expr) = alphanumeric1.opt().parse(i)?;

    // check in case a range or member expr is following
    let (i, num) = if other_dot.is_some() || member_expr.is_some() {
        let (i, num_string) = take_until1(".")(initial_i)?;
        (i, num_string.parse::<f32>().unwrap())
    } else {
        (i, num)
    };

    let (_, raw) = take((initial_i.len() - i.len()) as usize)(initial_i)?;

    Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Number(num),
            raw: raw.to_owned(),
        },
    ))
}

pub fn parse_number_literal_value(initial_i: &str) -> IResult<&str, LiteralValue, ErrorTree<&str>> {
    // is normal to avoid having things such as Infinity considered numbers
    let (i, num) = float.verify(|num| num.is_normal()).parse(initial_i)?;
    let (_, other_dot) = char('.').opt().parse(i)?;

    // check in case a range is following
    let (i, num) = if other_dot.is_some() {
        let (i, num_string) = take_until1(".")(initial_i)?;
        (i, num_string.parse::<f32>().unwrap())
    } else {
        (i, num)
    };

    Ok((i, LiteralValue::Number(num)))
}
