use nom::{IResult, error::VerboseError, bytes::complete::tag};

use crate::parser::{Span, ast::{Expression, literal_value::LiteralValue}};

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
