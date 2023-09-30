use nom::{branch::alt, bytes::complete::tag, combinator::map, error::VerboseError, IResult};

use crate::parser::{ast::{literal_value::LiteralValue, Expression}, Span};

enum Bool {
    True,
    False,
}

pub fn parse_bool(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, boolean) = alt((
        map(tag("true"), |_| Bool::True),
        map(tag("false"), |_| Bool::False),
    ))(i)?;


    Ok((
        i,
        Expression::Literal {
            value: match boolean {
                Bool::True => LiteralValue::Boolean(true),
                Bool::False => LiteralValue::Boolean(false),
            },
            raw: String::from("true"),
        },
    ))
}
