use nom::{branch::alt, IResult, Parser};
use nom_supreme::{error::ErrorTree, ParserExt};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, ASTNode, Expression},
    comment::jump_comments,
    expression::{indexing::parse_indexing, member_expr::parse_member_expr, parse_expression},
    operations::assignment::parse_assignment_operator,
    parse_new_lines,
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

    let (i, assigned) = parse_expression.cut().parse(i)?;

    Ok((
        i,
        ASTNode::ExpressionStatement {
            expression: Expression::AssignmentExpression {
                operator: op,
                id,
                assigned: Box::new(assigned),
            },
        },
    ))
}
