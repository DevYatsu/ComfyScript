use nom::{branch::alt, error::VerboseError, IResult};

use crate::parser::{
    ast::Expression, composite_types::parse_composite_value, operations::parse_binary_operation,
    parenthesized::parse_parenthesized, primitive_values::parse_primitive_value, Span,
};

// parsing expressions
pub fn parse_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, expr) = alt((
        parse_binary_operation,
        // this one following are useless as they are done in the parse_binary_operation anyway
        parse_parenthesized,
        parse_primitive_value,
        parse_composite_value,
    ))(i)?;
    println!("{expr}");

    Ok((i, expr))
}
