use nom::{
    branch::alt,
    character::complete::{multispace0, space0},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, ASTNode, Expression},
    expression::{
        function_call::parse_fn_call, indexing::parse_indexing, member_expr::parse_opt_member_expr,
        parse_expression,
    },
    operations::assignment::parse_assignment_operator,
};

pub fn parse_assignment(i: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (i, id) = parse_assigned.parse(i)?;

    let (i, _) = multispace0(i)?;

    let (i, op) = parse_assignment_operator.parse(i)?;
    let (i, _) = multispace0(i)?;

    let (i, assigned) = parse_expression.parse(i)?;

    let assignment = ASTNode::Assignment {
        operator: op,
        id,
        assigned,
    };
    let (i, _) = space0(i)?;

    if i.is_empty() {
        return Ok((i, assignment));
    }

    let (i, _) = alt((tag("\n"), tag(","), tag(";"), tag("//").complete()))
        .peek()
        .context("unexpected")
        .cut()
        .parse(i)?;

    Ok((i, assignment))
}

fn parse_assigned(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, expr) = alt((parse_indexing, parse_fn_call, parse_identifier_expression)).parse(i)?;

    let (i, expr) = parse_opt_member_expr(expr)(i)?;

    Ok((i, expr))
}
