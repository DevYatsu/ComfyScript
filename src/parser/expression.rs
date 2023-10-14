mod array;
mod bool;
pub mod member_expr;
mod nil;
mod numbers;
mod object;
mod parenthesized;
pub mod strings;

use self::{
    array::parse_array, bool::parse_bool, member_expr::parse_member_expr, nil::parse_nil,
    numbers::parse_number, object::parse_object, parenthesized::parse_parenthesized,
    strings::parse_string,
};
use super::{
    ast::{identifier::parse_identifier_expression, ASTNode},
    comment::jump_comments,
    function::function_call::parse_fn_call,
    operations::{binary::parse_binary_operator, build_binary_expression},
};
use crate::parser::{ast::Expression, Span};
use nom::{
    branch::alt,
    character::complete::multispace0,
    error::VerboseError,
    multi::many0,
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn parse_expression_statement(i: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, expr) = parse_expression(i)?;

    let expr_statement = ASTNode::ExpressionStatement { expression: expr };

    Ok((input, expr_statement))
}

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

fn parse_basic_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = jump_comments(i)?;

    let (i, expr) = alt((
        parse_composite_value,
        parse_primitive_value,
        parse_parenthesized,
        parse_fn_call,
        parse_member_expr,
        parse_identifier_expression,
    ))(i)?;

    Ok((i, expr))
}

fn parse_primitive_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((parse_string, parse_bool, parse_number, parse_nil))(i)
}
fn parse_composite_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((parse_array, parse_object))(i)
}
