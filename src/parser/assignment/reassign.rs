use nom::{branch::alt, character::complete::multispace0, combinator::map, IResult, Parser};
use nom_supreme::{error::ErrorTree, ParserExt};

use crate::parser::{
    ast::{identifier::parse_identifier_expression, ASTNode, Expression},
    expression::{indexing::parse_indexing, member_expr::parse_member_expr, parse_expression},
    operations::assignment::parse_assignment_operator,
};

pub fn parse_assignment(i: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (i, id) = map(
        alt((
            parse_indexing,
            parse_member_expr,
            parse_identifier_expression,
        )),
        |e| Box::new(e),
    )(i)?;
    let (i, _) = multispace0(i)?;

    let (i, op) = parse_assignment_operator
        .context("Expected a valid assignment operator such as '=', '+=', '-='")
        .cut()
        .parse(i)?;
    let (i, _) = multispace0(i)?;

    let (i, assigned) = parse_expression
        .context("Expected a valid expression")
        .cut()
        .parse(i)?;

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
