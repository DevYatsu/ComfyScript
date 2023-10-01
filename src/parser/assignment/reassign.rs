use nom::{character::complete::multispace1, error::VerboseError, IResult};

use crate::parser::{
    ast::{identifier::parse_identifier, ASTNode, Expression},
    operations::parse_assignment_operator,
    Span,
};

fn parse_reassignment(i: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (i, id) = parse_identifier(i)?;
    let (i, _) = multispace1(i)?;

    let (i, op) = parse_assignment_operator(i)?;

    let (i, _) = parse_identifier(i)?; //todo! parse expression

    Ok((
        i,
        ASTNode::ExpressionStatement {
            expression: Expression::AssignmentExpression {
                operator: op,
                id,
                assigned: Box::new(Expression::Array { elements: vec![] }),
            },
        },
    ))
}
