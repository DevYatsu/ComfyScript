use super::{
    ast::identifier::parse_identifier_expression,
    comment::jump_comments,
    operations::{binary::parse_binary_operator, build_binary_expression},
};
use crate::parser::{
    ast::Expression, composite_types::parse_composite_value, parenthesized::parse_parenthesized,
    primitive_values::parse_primitive_value, Span,
};
use nom::{
    branch::alt,
    character::complete::multispace0,
    error::VerboseError,
    multi::many0,
    sequence::{preceded, separated_pair},
    IResult,
};

// parsing expressions
pub fn parse_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, expr) = parse_basic_expression(i)?;

    let mut expr_vec = vec![expr];
    let mut operators_vec = Vec::with_capacity(3);

    let (i, _) = multispace0(i)?;

    // check for binary expr
    let (i, rest) = many0(separated_pair(
        preceded(jump_comments, parse_binary_operator),
        multispace0,
        parse_basic_expression,
    ))(i)?;

    for (op, expr) in rest {
        operators_vec.push(op);
        expr_vec.push(expr);
    }

    // build binary expr with operators precedence
    let final_expr = build_binary_expression(expr_vec, operators_vec);

    Ok((i, final_expr))
}

pub fn parse_basic_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = jump_comments(i)?;

    let (i, expr) = alt((
        parse_composite_value,
        parse_primitive_value,
        parse_parenthesized,
        parse_identifier_expression,
    ))(i)?;

    Ok((i, expr))
}
