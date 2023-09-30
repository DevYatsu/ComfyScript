use nom::error::VerboseError;
use nom::number::complete::float;
use nom::{branch::alt, character::complete::char, combinator::opt, IResult};

use super::ast::literal_value::LiteralValue;
use super::ast::Expression;
use super::Span;

pub fn parse_number(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let start = i.len();
    let (i, sign) = opt(alt((char('+'), char('-'))))(i)?;

    let (i, num) = float(i)?;

    let num = match sign {
        Some('-') => -num,
        _ => num,
    };
    let end = i.len();

    Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Number(num),
            raw: num.to_string(),
            start,
            end,
        },
    ))
}
