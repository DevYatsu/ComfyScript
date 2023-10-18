use nom::{bytes::complete::tag, IResult};
use nom_supreme::error::ErrorTree;

use crate::parser::{
    ast::{literal_value::LiteralValue, Expression},
    Span,
};

pub fn parse_nil(i: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (i, _) = tag("nil")(i)?;

    Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Nil,
            raw: String::from("nil"),
        },
    ))
}
