use nom::{
    branch::alt, bytes::complete::take_until, combinator::map, error::VerboseError, IResult,
};
use nom_supreme::tag::complete::tag;

use super::ast::Expression;

enum Quote {
    Unique,
    Double,
}

pub fn parse_string(i: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let start = i.len();
    let (i, quote) = parse_quote(i)?;

    match quote {
        Quote::Unique => {
            let (i, result) = take_until("'")(i)?;
            let (i, c) = tag("'")(i)?;
            let end = i.len();

            return Ok((
                i,
                Expression::Literal {
                    value: super::ast::literal_value::LiteralValue::Str(result.to_owned()),
                    raw: c.to_owned() + result + c,
                    start,
                    end,
                },
            ));
        }
        Quote::Double => {
            let (i, result) = take_until("\"")(i)?;
            let (i, c) = tag("\"")(i)?;
            let end = i.len();

            return Ok((
                i,
                Expression::Literal {
                    value: super::ast::literal_value::LiteralValue::Str(result.to_owned()),
                    raw: c.to_owned() + result + c,
                    start,
                    end,
                },
            ));
        }
    }
}
// todo!

fn parse_quote(i: &str) -> IResult<&str, Quote, VerboseError<&str>> {
    alt((
        map(tag("\""), |_| Quote::Double),
        map(tag("'"), |_| Quote::Unique),
    ))(i)
}
