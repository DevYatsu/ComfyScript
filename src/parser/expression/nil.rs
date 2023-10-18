use nom::{bytes::complete::tag, IResult};
use nom_supreme::error::ErrorTree;

use crate::parser::ast::{literal_value::LiteralValue, Expression};

pub fn parse_nil(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = tag("nil")(i)?;

    Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Nil,
            raw: String::from("nil"),
        },
    ))
}
