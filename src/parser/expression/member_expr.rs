use crate::parser::ast::identifier::parse_identifier_expression;
use crate::parser::ast::Expression;
use crate::parser::comment::jump_comments;
use nom::branch::alt;
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;

use super::function_call::parse_fn_call;
use super::indexing::parse_indexing;
use super::parenthesized::parse_parenthesized;
use super::parse_expression_with;

pub fn parse_member_expr(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, mut ids) = separated_list1(
        tag("."),
        parse_expression_with(parse_expression_except_member_expr),
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
    let (i, _) = jump_comments(i)?;

    let (i, expr) = alt((
        parse_parenthesized,
        parse_indexing,
        parse_fn_call,
        parse_identifier_expression,
        // avoid adding to many parser here
    ))
    .parse(i)?;

    Ok((i, expr))
}
