use nom::{branch::alt, IResult, Parser};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use crate::parser::ast::{literal_value::LiteralValue, Expression};

pub fn parse_bool(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, (boolean, value)) = alt((
        tag("true")
            .complete()
            .map(|b| (b, LiteralValue::Boolean(true))),
        tag("false")
            .complete()
            .map(|b| (b, LiteralValue::Boolean(false))),
    ))(i)?;

    Ok((
        i,
        Expression::Literal {
            value,
            raw: boolean.to_owned(),
        },
    ))
}
