use nom::bytes::complete::take;
use nom::error::VerboseError;
use nom::number::complete::float;
use nom::{branch::alt, character::complete::char, combinator::opt, IResult};

use crate::parser::ast::literal_value::LiteralValue;
use crate::parser::ast::Expression;
use crate::parser::Span;

pub fn parse_number(initial_i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, sign) = opt(alt((char('+'), char('-'))))(initial_i)?;

    let (i, num) = float(i)?;

    let (_, raw) = take((initial_i.len()-i.len()) as usize)(initial_i)?;

    let num = match sign {
        Some('-') => -num,
        _ => num,
    };

    Ok((
        i,
        Expression::Literal {
            value: LiteralValue::Number(num),
            raw: raw.fragment().to_string(),
        },
    ))
}
