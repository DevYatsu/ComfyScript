use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};
use nom_supreme::error::ErrorTree;

use crate::parser::ast::{literal_value::LiteralValue, Expression};

pub fn parse_bool(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, (boolean, value)) = alt((
        map(tag("true"), |b| (b, LiteralValue::Boolean(true))),
        map(tag("false"), |b| (b, LiteralValue::Boolean(false))),
    ))(i)?;

    Ok((
        i,
        Expression::Literal {
            value,
            raw: boolean.to_owned(),
        },
    ))
}
