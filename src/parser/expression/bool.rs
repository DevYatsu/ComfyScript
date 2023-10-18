use nom::{branch::alt, bytes::complete::tag, IResult};
use nom_supreme::error::ErrorTree;

use crate::parser::{
    ast::{literal_value::LiteralValue, Expression},
    Span,
};

pub fn parse_bool(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (i, boolean) = alt((tag("true"), tag("false")))(i)?;

    let value = match boolean.fragment() {
        &"true" => LiteralValue::Boolean(true),
        &"false" => LiteralValue::Boolean(false),
        _ => unreachable!(),
    };

    Ok((
        i,
        Expression::Literal {
            value,
            raw: boolean.fragment().to_string(),
        },
    ))
}
