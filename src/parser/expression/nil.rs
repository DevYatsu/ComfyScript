use nom::{IResult, Parser};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use crate::parser::ast::{literal_value::LiteralValue, Expression};

pub fn parse_nil(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, x) = tag("nil").complete().parse(i)?;

    Ok((i, (LiteralValue::Nil, String::from(x)).into()))
}
