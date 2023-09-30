use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_until, combinator::map,
    error::VerboseError, IResult,
};

use crate::parser::{
    ast::{literal_value::LiteralValue, Expression},
    Span,
};

enum Quote {
    Unique,
    Double,
}

pub fn parse_string(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, quote) = parse_quote(i)?;

    match quote {
        Quote::Unique => {
            let (i, result) = take_until("'")(i)?;
            let (i, c) = tag("'")(i)?;

            return Ok((
                i,
                Expression::Literal {
                    value: LiteralValue::Str(result.fragment().to_string()),
                    raw: c.fragment().to_string() + result.fragment() + c.fragment(),
                },
            ));
        }
        Quote::Double => {
            let (i, result) = take_until("\"")(i)?;
            let (i, c) = tag("\"")(i)?;

            return Ok((
                i,
                Expression::Literal {
                    value: LiteralValue::Str(result.fragment().to_string()),
                    raw: c.fragment().to_string() + result.fragment() + c.fragment(),
                },
            ));
        }
    }
}
// todo!

fn parse_quote(i: Span) -> IResult<Span, Quote, VerboseError<Span>> {
    alt((
        map(tag("\""), |_| Quote::Double),
        map(tag("'"), |_| Quote::Unique),
    ))(i)
}
