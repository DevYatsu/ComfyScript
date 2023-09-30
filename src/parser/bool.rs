use nom::{branch::alt, bytes::complete::tag, combinator::map, error::VerboseError, IResult};

use super::{
    ast::{literal_value::LiteralValue, Expression},
    Span,
};

enum Bool {
    True,
    False,
}

pub fn parse_bool(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let start = i.len();
    let (i, boolean) = alt((
        map(tag("true"), |_| Bool::True),
        map(tag("false"), |_| Bool::False),
    ))(i)?;

    let end = i.len();

    Ok((
        i,
        Expression::Literal {
            value: match boolean {
                Bool::True => LiteralValue::Boolean(true),
                Bool::False => LiteralValue::Boolean(false),
            },
            raw: String::from("true"),
            start,
            end,
        },
    ))
}
