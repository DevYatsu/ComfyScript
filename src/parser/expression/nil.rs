use nom::{bytes::complete::tag, error::VerboseError, IResult};

use crate::parser::{
    ast::{literal_value::LiteralValue, Expression},
    Span,
};

pub fn parse_nil(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = tag("nil")(i)?;

    Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Nil,
            raw: String::from("nil"),
        },
    ))
}
