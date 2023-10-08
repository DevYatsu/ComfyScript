pub mod array;
pub mod object;
use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    combinator::opt,
    error::VerboseError,
    IResult,
};

use crate::parser::{ast::Expression, Span};

use self::{array::parse_array, object::parse_object};

pub fn parse_composite_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, open_paren) = opt(char('('))(i)?;

    if let Some(_) = open_paren {
        let (i, _) = multispace0(i)?;
        let (i, expr) = alt((parse_array, parse_object))(i)?;
        let (i, _) = multispace0(i)?;
        let (i, _) = char(')')(i)?;

        return Ok((i, expr));
    }

    alt((parse_array, parse_object))(i)
}
