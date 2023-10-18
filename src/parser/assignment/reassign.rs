use nom::{
    branch::alt, character::complete::multispace0, combinator::map, IResult,
};
use nom_supreme::error::ErrorTree;

use crate::parser::{
    ast::{identifier::parse_identifier_expression, ASTNode, Expression},
    expression::{indexing::parse_indexing, member_expr::parse_member_expr, parse_expression},
    operations::assignment::parse_assignment_operator,
    Span,
};

pub fn parse_assignment(i: Span) -> IResult<Span, ASTNode, ErrorTree<Span>> {
    let (i, id) = map(
        alt((
            parse_indexing,
            parse_member_expr,
            parse_identifier_expression,
        )),
        |e| Box::new(e),
    )(i)?;
    let (i, _) = multispace0(i)?;

    let (i, op) = parse_assignment_operator(i)?;
    let (i, _) = multispace0(i)?;

    let (i, assigned) = parse_expression(i)?; //todo! parse expression

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
