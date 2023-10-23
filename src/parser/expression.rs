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
use nom::{
    branch::alt,
    character::complete::{alphanumeric0, space0, digit0},
    multi::many0,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_expression_statement(i: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (i, expr) = parse_expression(i)?;

    let expr_statement = ASTNode::ExpressionStatement { expression: expr };

    let (i, _) = space0(i)?;

    if i.is_empty() {
        return Ok((i, expr_statement));
    }

    let (i, _) = alt((tag("\n"), tag(";"), tag("//").complete()))
        .peek()
        .context("unexpected")
        .cut()
        .parse(i)?;

    Ok((i, expr_statement))
}

pub fn parse_expression(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, result) = parse_expression_with(parse_basic_expression).parse(i)?;

    Ok((i, result))
}

pub fn parse_expression_with<'a, F>(
    parser: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Expression, ErrorTree<&str>>
where
    F: Fn(&'a str) -> IResult<&'a str, Expression, ErrorTree<&str>>,
{
    move |i| {
        let parser_closure = |i| parser(i);
println!("{:?}", &i);
        let (i, expr) = parser_closure(i)?;
        println!("{:?}", &i);
        let mut expr_vec = vec![expr];
        let mut operators_vec = Vec::with_capacity(3);

        // Check for binary expr
        let (i, rest) = many0(separated_pair(
            parse_binary_operator.preceded_by(jump_comments),
            jump_comments,
            parser_closure,
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
    let (i, found) = alt((
        alt((tag("\""), tag("'"), tag("("), tag("{"), tag("["), tag("|"))),
        alphanumeric0
    ))
    .peek()
    .parse(i)?;

    let (i, expr) = match found {
        "\"" | "'" => parse_string(i)?, // strings
        "[" => parse_array(i)?,
        "{" => parse_object(i)?,
        "(" => parse_parenthesized(i)?,
        "|" => parse_fn_expression(i)?,
        "true" | "false" => parse_bool(i)?,
        "nil" => parse_nil(i)?,
        _ => alt((
            parse_member_expr,
            parse_indexing,
            parse_fn_call,
            parse_range,
            parse_number, // all numbers are not covered up there, only basic ones: not 1e15 for example
            parse_identifier_expression,
        ))
        .parse(i)?,
    };

    Ok((i, expr))
}

fn parse_primitive_value(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    alt((parse_string, parse_bool, parse_number, parse_nil))(i)
}
