use nom::{
    branch::alt, bytes::complete::tag, combinator::map, error::VerboseError, IResult, Parser,
};

use super::builtins::Atom;

pub fn parse_bool(i: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    alt((
        map(tag("true"), |_| Atom::Boolean(true)),
        map(tag("false"), |_| Atom::Boolean(false)),
    ))
    .parse(i)
}
