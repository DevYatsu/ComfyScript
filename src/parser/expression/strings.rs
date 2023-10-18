use nom::{branch::alt, bytes::complete::take_until, combinator::map, IResult};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

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
            let (i, c) = tag("'")(i)?;

            return Ok((
                i,
                Expression::Literal {
                    value: LiteralValue::Str(result.to_owned()),
                    raw: c.to_owned() + result + c,
                },
            ));
        }
        Quote::Double => {
            let (i, result) = take_until("\"")(i)?;
            let (i, c) = tag("\"")(i)?;

            return Ok((
                i,
                Expression::Literal {
                    value: LiteralValue::Str(result.to_owned()),
                    raw: c.to_owned() + result + c,
                },
            ));
        }
    }
}
// todo!

fn parse_quote(i: &str) -> IResult<&str, Quote, ErrorTree<&str>> {
    alt((
        map(tag("\""), |_| Quote::Double),
        map(tag("'"), |_| Quote::Unique),
    ))(i)
}
