mod array;
mod bool;
mod function_call;
pub mod indexing;
pub mod member_expr;
mod nil;
mod numbers;
mod object;
mod parenthesized;
pub mod range;
pub mod strings;

use self::{
    array::parse_array, bool::parse_bool, function_call::parse_fn_call, indexing::parse_indexing,
    member_expr::parse_member_expr, nil::parse_nil, numbers::parse_number, object::parse_object,
    parenthesized::parse_parenthesized, range::parse_range, strings::parse_string,
};
use super::{
    ast::{identifier::parse_identifier_expression, ASTNode},
    comment::jump_comments,
    function::parse_fn_expression,
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

pub fn parse_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    parse_expression_with(parse_basic_expression)(i)
}

pub fn parse_expression_with<'a, F>(
    parser: F,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Expression, VerboseError<Span>>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, Expression, VerboseError<Span>>,
{
    move |input| {
        let parser_closure = |i| parser(i);

        let (i, expr) = parser_closure(input)?;

        let mut expr_vec = vec![expr];
        let mut operators_vec = Vec::with_capacity(3);

        let (i, _) = multispace0(i)?;

        // Check for binary expr
        let (i, rest) = many0(separated_pair(
            preceded(multispace0, parse_binary_operator),
            multispace0,
            &parser_closure,
        ))(i)?;

        for (op, expr) in rest {
            operators_vec.push(op);
            expr_vec.push(expr);
        }

        // Build binary expr with operators precedence
        let final_expr = build_binary_expression(expr_vec, operators_vec);

        Ok((i, final_expr))
    }
}

fn parse_basic_expression(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, _) = jump_comments(i)?;

    let (i, expr) = alt((
        parse_member_expr,
        parse_indexing,
        parse_fn_call,
        parse_range,
        parse_composite_value,
        parse_primitive_value,
        parse_parenthesized,
        parse_identifier_expression,
        parse_fn_expression,
    ))(i)?;

    Ok((i, expr))
}

fn parse_primitive_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((parse_string, parse_bool, parse_number, parse_nil))(i)
}
fn parse_composite_value(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    alt((parse_array, parse_object))(i)
}
