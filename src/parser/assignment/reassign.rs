use nom::{branch::alt, character::complete::multispace0, IResult, Parser};
use nom_supreme::{error::ErrorTree, ParserExt};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, ASTNode, Expression},
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
    let (i, _) = multispace0(i)?;

    let (i, op) = parse_assignment_operator.parse(i)?;
    let (i, _) = multispace0(i)?;

    let (i, assigned) = parse_expression.cut().context("expression").parse(i)?;

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
