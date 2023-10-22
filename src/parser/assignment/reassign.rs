use nom::{branch::alt, character::complete::space0, IResult, Parser};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, ASTNode, Expression},
    comment::jump_comments,
    expression::{indexing::parse_indexing, member_expr::parse_member_expr, parse_expression},
    operations::assignment::parse_assignment_operator,
};

pub fn parse_assignment(i: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (i, id) = alt((
        parse_indexing,
        parse_member_expr,
        parse_identifier_expression,
    ))
    .map(|e| Box::new(e))
    .parse(i)?;

    let (i, _) = jump_comments(i)?;

    let (i, op) = parse_assignment_operator.parse(i)?;
    let (i, _) = jump_comments(i)?;

    let (i, assigned) = parse_expression.parse(i)?;

    let expr_statement = ASTNode::ExpressionStatement {
        expression: Expression::AssignmentExpression {
            operator: op,
            id,
            assigned: Box::new(assigned),
        },
    };
    let (i, _) = space0(i)?;

    if i.is_empty() {
        return Ok((i, expr_statement));
    }

    let (i, _) = alt((tag("\n"), tag(","), tag(";"), tag("//").complete()))
        .peek()
        .context("unexpected")
        .cut()
        .parse(i)?;

    Ok((i, expr_statement))
}
