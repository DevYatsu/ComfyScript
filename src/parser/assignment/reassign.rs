use nom::{character::complete::multispace0, error::VerboseError, IResult};

use crate::parser::{
    ast::{identifier::parse_identifier, ASTNode, Expression},
    operations::assignment::parse_assignment_operator,
    Span, expression::parse_expression,
};

pub fn parse_assignment(i: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (i, id) = parse_identifier(i)?;
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
