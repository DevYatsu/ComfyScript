use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_until, combinator::map,
    error::VerboseError, IResult,
};

use super::{ast::Expression, Span};

enum Quote {
    Unique,
    Double,
}

pub fn parse_string(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
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
                    value: super::ast::literal_value::LiteralValue::Str(
                        result.fragment().to_string(),
                    ),
                    raw: c.fragment().to_string() + result.fragment() + c.fragment(),
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
                    value: super::ast::literal_value::LiteralValue::Str(
                        result.fragment().to_string(),
                    ),
                    raw: c.fragment().to_string() + result.fragment() + c.fragment(),
                    start,
                    end,
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
