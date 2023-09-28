use nom::{branch::alt, combinator::map, error::VerboseError, IResult, Parser};
use nom_supreme::tag::complete::tag;

use super::ast::{literal_value::LiteralValue, Expression};

pub fn parse_bool(i: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((
        map(tag("true"), |_| Expression::Literal {
            value: LiteralValue::Boolean(true),
            raw: String::from("true"),
        }),
        map(tag("false"), |_| Expression::Literal {
            value: LiteralValue::Boolean(false),
            raw: String::from("false"),
        }),
    ))
    .parse(i)
}
