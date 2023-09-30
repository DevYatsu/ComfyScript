use nom::{branch::alt, combinator::map, error::VerboseError, IResult, Parser};
use nom_supreme::tag::complete::tag;

use super::ast::{literal_value::LiteralValue, Expression};

enum Bool {
    True,
    False,
}

pub fn parse_bool(i: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let start = i.len();
    let (i, boolean) = alt((
        map(tag("true"), |_| Bool::True),
        map(tag("false"), |_| Bool::False),
    ))
    .parse(i)?;

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
