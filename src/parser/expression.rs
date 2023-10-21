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
use crate::parser::ast::Expression;
use nom::{branch::alt, multi::many0, sequence::separated_pair, IResult, Parser};
use nom_supreme::{error::ErrorTree, ParserExt};

pub fn parse_expression_statement(i: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, expr) = parse_expression(i)?;

    let expr_statement = ASTNode::ExpressionStatement { expression: expr };

    Ok((input, expr_statement))
}

pub fn parse_expression(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    parse_expression_with(parse_basic_expression).parse(i)
}

pub fn parse_expression_with<'a, F>(
    parser: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Expression, ErrorTree<&str>>
where
    F: Fn(&'a str) -> IResult<&'a str, Expression, ErrorTree<&str>>,
{
    move |input| {
        let parser_closure = |i| parser(i);

        let (i, expr) = parser_closure.context("expression").parse(input)?;

        let mut expr_vec = vec![expr];
        let mut operators_vec = Vec::with_capacity(3);

        // Check for binary expr
        let (i, rest) = many0(separated_pair(
            parse_binary_operator.preceded_by(jump_comments),
            jump_comments,
            parser_closure.context("expression"),
        ))
        .cut()
        .parse(i)?;

        for (op, expr) in rest {
            operators_vec.push(op);
            expr_vec.push(expr);
        }

        // Build binary expr with operators precedence
        let final_expr = build_binary_expression(expr_vec, operators_vec);

        Ok((i, final_expr))
    }
}

fn parse_basic_expression(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
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
    ))
    .parse(i)?;

    Ok((i, expr))
}

fn parse_primitive_value(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    alt((parse_string, parse_bool, parse_number, parse_nil))(i)
}
fn parse_composite_value(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    alt((parse_array, parse_object))(i)
}
