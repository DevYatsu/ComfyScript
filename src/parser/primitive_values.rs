pub mod bool;
pub mod nil;
pub mod numbers;
pub mod strings;

use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    combinator::opt,
    error::VerboseError,
    IResult,
};

use crate::parser::{ast::Expression, Span};

use self::{bool::parse_bool, nil::parse_nil, numbers::parse_number, strings::parse_string};

pub fn parse_primitive_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, open_paren) = opt(char('('))(i)?;

    if let Some(_) = open_paren {
        let (i, _) = multispace0(i)?;
        let (i, expr) = alt((parse_string, parse_bool, parse_number, parse_nil))(i)?;
        let (i, _) = multispace0(i)?;
        let (i, _) = char(')')(i)?;

        return Ok((i, expr));
    }

    alt((parse_string, parse_bool, parse_number, parse_nil))(i)
}
