use nom::{
    branch::alt, bytes::complete::take_until, character::complete::char, combinator::map, IResult,
};
use nom_supreme::error::ErrorTree;

use crate::parser::ast::{literal_value::LiteralValue, Expression};

enum Quote {
    Unique,
    Double,
}

pub fn parse_string(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, quote) = parse_quote(i)?;

    match quote {
        Quote::Unique => {
            let (i, result) = take_until("'")(i)?;
            let (i, c) = char('\'')(i)?;

            return Ok((
                i,
                Expression::Literal {
                    value: LiteralValue::Str(result.to_owned()),
                    raw: c.to_string() + result + &c.to_string(),
                },
            ));
        }
        Quote::Double => {
            let (i, result) = take_until("\"")(i)?;
            let (i, c) = char('\"')(i)?;

            return Ok((
                i,
                Expression::Literal {
                    value: LiteralValue::Str(result.to_owned()),
                    raw: c.to_string() + result + &c.to_string(),
                },
            ));
        }
    }
}
// todo!

fn parse_quote(i: &str) -> IResult<&str, Quote, ErrorTree<&str>> {
    alt((
        map(char('\"'), |_| Quote::Double),
        map(char('\''), |_| Quote::Unique),
    ))(i)
}
