use crate::parser::ast::identifier::parse_identifier_expression;
use crate::parser::ast::Expression;
use nom::branch::alt;
use nom::character::complete::{alphanumeric0, char, multispace0};
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;

use super::array::parse_array;
use super::function_call::parse_fn_call;
use super::indexing::parse_indexing;
use super::numbers::parse_number;
use super::object::parse_object;
use super::parenthesized::parse_parenthesized;
use super::parse_expression_with0;
use super::strings::parse_string;
use super::template_literal::parse_template_literal;

pub fn parse_member_expr(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, mut ids) = separated_list1(
        char('.'),
        parse_expression_with0(parse_expression_except_member_expr),
    )(i)?;
    // we are sure that ids length is >= 2 here

    let property = Box::new(ids.pop().unwrap());

    if ids.len() == 0 {
        return Err(nom::Err::Error(ErrorTree::Alt(vec![])));
    }

    let indexed = if ids.len() == 1 {
        Box::new(ids.remove(0))
    } else {
        Box::new(build_member_expr(ids))
    };

    let expr = Expression::MemberExpression {
        indexed,
        property,
        computed: false,
    };

    Ok((i, expr))
}

fn build_member_expr(mut ids: Vec<Expression>) -> Expression {
    if ids.len() == 1 {
        return ids.remove(0);
    }

    let property = Box::new(ids.pop().unwrap());

    let expr = Expression::MemberExpression {
        indexed: Box::new(build_member_expr(ids)),
        property,
        computed: false,
    };

    expr
}

fn parse_expression_except_member_expr(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, _) = multispace0(i)?;

    let (i, found) = alt((
        alt((tag("\""), tag("'"), tag("{"), tag("["), tag("#"), tag("("))),
        alphanumeric0,
    ))
    .peek()
    .parse(i)?;

    let (i, expr) = match found {
        "\"" | "'" => parse_string(i)?,
        "#" => parse_template_literal(i)?,
        "[" => parse_array(i)?,
        "{" => parse_object(i)?,
        "(" => parse_parenthesized(i)?,
        _ => alt((
            parse_number,
            parse_indexing,
            parse_fn_call,
            parse_identifier_expression,
        ))
        .parse(i)?,
    };

    Ok((i, expr))
}
